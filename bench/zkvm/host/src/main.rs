//! E1 harness: run the Cedar guest over the corpus, check it against the
//! oracle, and write measurements to results/ as JSON Lines.
//!
//!   zkvm-host exec  <repo> <out.jsonl>                      correctness and cycles, every request
//!   zkvm-host prove <repo> <out.jsonl> <set> <kind> <n>     prove request 0 of a set n times, verify 20 times
//!   zkvm-host demo  <repo> <set> <kind> <out dir>           write a proof-carrying AuthZEN response and verify it

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

use envelope::{
    canonical_request, parse_hex_hash, response, verify, Journal, Pins, SealVerifier,
    FORMAT_RISC0_COMPOSITE, FORMAT_RISC0_GROTH16, FORMAT_RISC0_SUCCINCT,
};
use envelope::{hex_hash, sorted_entities, uid_text, Layouts, RecordStore};
use methods::{FULL_ELF as CEDAR_GUEST_ELF, FULL_ID as CEDAR_GUEST_ID, RECORDS_ELF, RECORDS_ID};
use risc0_zkvm::{default_executor, default_prover, sha::Digest, ExecutorEnv, ProverOpts, Receipt};
use serde_json::{json, Value};

const SEGMENTS: [&str; 7] =
    ["read", "parse_policies", "parse_schema", "parse_entities", "map_request", "authorize", "hash_commit"];

struct Set {
    name: String,
    policies: Vec<u8>,
    schema: Vec<u8>,
    entities: Vec<u8>,
    requests: Vec<Value>,
    manifest: Value,
}

fn load(repo: &Path, name: &str) -> Set {
    let d = repo.join("corpus/sets").join(name);
    let manifest: Value = serde_json::from_slice(&fs::read(repo.join("corpus/MANIFEST.json")).unwrap()).unwrap();
    let m = manifest["sets"][name].clone();
    // ZKPDP_NO_SCHEMA=1 is the variant where the guest gets no schema. The
    // authority validates entities against the schema before it commits to them.
    let uses_schema = m["uses_schema"].as_bool().unwrap() && std::env::var("ZKPDP_NO_SCHEMA").is_err();
    Set {
        name: name.into(),
        policies: fs::read(d.join("policies.cedar")).unwrap(),
        schema: if uses_schema { fs::read(d.join("schema.cedarschema")).unwrap() } else { vec![] },
        entities: fs::read(d.join("entities.json")).unwrap(),
        requests: fs::read_to_string(d.join("requests.jsonl"))
            .unwrap()
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect(),
        manifest: m,
    }
}

fn set_names(repo: &Path) -> Vec<String> {
    let mut v: Vec<String> = fs::read_dir(repo.join("corpus/sets"))
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    v.sort();
    v
}

fn input(set: &Set, request: &Value, reasons: bool) -> Vec<u8> {
    let req = canonical_request(request);
    let mut b = vec![reasons as u8];
    for f in [&set.policies[..], &set.schema[..], &set.entities[..], &req[..]] {
        b.extend_from_slice(&(f.len() as u32).to_le_bytes());
        b.extend_from_slice(f);
    }
    b
}

fn image_id() -> [u8; 32] {
    Digest::from(CEDAR_GUEST_ID).as_bytes().try_into().unwrap()
}

fn sh(cmd: &str, args: &[&str]) -> String {
    Command::new(cmd)
        .args(args)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_owned())
        .unwrap_or_default()
}

struct Out {
    file: fs::File,
    header: Value,
}

impl Out {
    fn new(repo: &Path, path: &str, experiment: &str) -> Out {
        let r = repo.to_str().unwrap();
        let dirty = !sh("git", &["-C", r, "status", "--porcelain", "--", "bench", "corpus"]).is_empty();
        let header = json!({
            "experiment": experiment,
            "backend": "zkvm-risc0",
            "commit": sh("git", &["-C", r, "rev-parse", "HEAD"]),
            "tree_dirty": dirty,
            "machine": {
                "cpu": sh("sysctl", &["-n", "machdep.cpu.brand_string"]),
                "cores": sh("sysctl", &["-n", "hw.ncpu"]),
                "ram_gb": sh("sysctl", &["-n", "hw.memsize"]).parse::<u64>().unwrap_or(0) >> 30,
                "os": format!("macOS {}", sh("sw_vers", &["-productVersion"])),
            },
            "toolchain": {
                "rustc": sh("rustc", &["--version"]),
                "cargo_risczero": sh("cargo", &["risczero", "--version"]),
                "r0vm": sh("r0vm", &["--version"]),
                "risc0_zkvm_crate": "3.0",
                "cedar_policy": "4.14.1",
                "prover": std::env::var("RISC0_PROVER").unwrap_or_else(|_| "default".into()),
                "dev_mode": std::env::var("RISC0_DEV_MODE").unwrap_or_default(),
            },
            "image_id": hex::encode(image_id()),
            "variant": if std::env::var("ZKPDP_NO_SCHEMA").is_ok() { "no-schema" } else { "schema" },
            "prover_build": "r0vm prebuilt, ipc, cpu",
        });
        let file = fs::OpenOptions::new().create(true).append(true).open(path).unwrap();
        Out { file, header }
    }

    /// One line per measurement, written at once so an aborted run keeps its record.
    fn line(&mut self, set: &Set, request: &Value, metric: &str, value: Value, repeat: usize, status: &str, notes: &str) {
        let mut l = self.header.clone();
        let o = l.as_object_mut().unwrap();
        o.insert("timestamp".into(), json!(sh("date", &["-u", "+%Y-%m-%dT%H:%M:%SZ"])));
        o.insert("set".into(), json!(set.name));
        o.insert("request".into(), request["name"].clone());
        o.insert(
            "inputs".into(),
            json!({
                "policy_hash": set.manifest["policy_hash"],
                "schema_hash": if set.schema.is_empty() { Value::Null } else { set.manifest["schema_hash"].clone() },
                "entity_commitment": set.manifest["entities_sha256_jcs_v0"],
                "request_hash": request["request_hash"],
            }),
        );
        o.insert("metric".into(), json!(metric));
        o.insert("value".into(), value);
        o.insert("repeat".into(), json!(repeat));
        o.insert("status".into(), json!(status));
        o.insert("notes".into(), json!(notes));
        writeln!(self.file, "{}", serde_json::to_string(&l).unwrap()).unwrap();
        self.file.flush().unwrap();
    }
}

/// Run the guest without proving. Returns the journal, total cycles and the per-segment cycles.
fn execute(set: &Set, request: &Value) -> Result<(Journal, u64, Vec<u64>, f64), String> {
    let blob = input(set, &request["request"], true);
    let mut stdout = Vec::new();
    let t = Instant::now();
    let session = {
        let env = ExecutorEnv::builder()
            .write_slice(&blob)
            .stdout(&mut stdout)
            .build()
            .map_err(|e| e.to_string())?;
        default_executor().execute(env, CEDAR_GUEST_ELF).map_err(|e| format!("{e:#}"))?
    };
    let secs = t.elapsed().as_secs_f64();
    let journal: Journal = session.journal.decode().map_err(|e| e.to_string())?;
    let marks: Vec<u64> = stdout.chunks(8).map(|c| u64::from_le_bytes(c.try_into().unwrap())).collect();
    let segs = marks.windows(2).map(|w| w[1] - w[0]).collect();
    Ok((journal, session.cycles(), segs, secs))
}

fn check(set: &Set, request: &Value, j: &Journal) -> Result<(), String> {
    let o = &request["oracle"];
    if j.decision != o["decision"].as_bool().unwrap() {
        return Err(format!("decision {} differs from oracle", j.decision));
    }
    let reasons: Vec<String> = o["reasons"].as_array().unwrap().iter().map(|r| r.as_str().unwrap().into()).collect();
    if j.reasons.as_ref() != Some(&reasons) {
        return Err(format!("reasons {:?} differ from oracle {:?}", j.reasons, reasons));
    }
    let want = |k: &str| parse_hex_hash(set.manifest[k].as_str().unwrap()).unwrap();
    if j.policy_hash != want("policy_hash") || j.entities.root != want("entities_sha256_jcs_v0") {
        return Err("policy or entity hash differs from manifest".into());
    }
    if j.request_hash != parse_hex_hash(request["request_hash"].as_str().unwrap()).unwrap() {
        return Err("request hash differs".into());
    }
    let nonce = request["request"].pointer("/context/nonce").and_then(Value::as_str).unwrap_or("");
    if j.nonce != nonce {
        return Err("nonce differs".into());
    }
    Ok(())
}

fn cmd_exec(repo: &Path, out: &str) {
    let mut o = Out::new(repo, out, "E1");
    let (mut ok, mut bad) = (0, 0);
    let only = std::env::var("ZKPDP_SETS").ok();
    for name in set_names(repo) {
        if let Some(o) = &only {
            if !o.split(',').any(|x| x == name) {
                continue;
            }
        }
        let set = load(repo, &name);
        for r in &set.requests {
            match execute(&set, r).and_then(|x| check(&set, r, &x.0).map(|_| x)) {
                Ok((_, cycles, segs, secs)) => {
                    ok += 1;
                    o.line(&set, r, "total_cycles", json!(cycles), 0, "ok", "rounded up to segment boundaries by the executor");
                    o.line(&set, r, "user_cycles", json!(segs.iter().sum::<u64>()), 0, "ok", "sum of the guest's own marks");
                    for (n, c) in SEGMENTS.iter().zip(&segs) {
                        o.line(&set, r, &format!("cycles_{n}"), json!(c), 0, "ok", "");
                    }
                    o.line(&set, r, "execute_time_s", json!(secs), 0, "ok", "executor only, no proof");
                }
                Err(e) => {
                    bad += 1;
                    eprintln!("FAIL {name} {}: {e}", r["name"]);
                    o.line(&set, r, "total_cycles", Value::Null, 0, "failed", &e);
                }
            }
        }
        eprintln!("{name}: done");
    }
    println!("matched oracle: {ok}  failed: {bad}");
    if bad > 0 {
        std::process::exit(1);
    }
}

fn opts(kind: &str) -> (ProverOpts, &'static str) {
    match kind {
        "composite" => (ProverOpts::composite(), FORMAT_RISC0_COMPOSITE),
        "succinct" => (ProverOpts::succinct(), FORMAT_RISC0_SUCCINCT),
        "groth16" => (ProverOpts::groth16(), FORMAT_RISC0_GROTH16),
        k => panic!("unknown receipt kind {k}"),
    }
}

fn prove(set: &Set, request: &Value, kind: &str, reasons: bool) -> Result<(Receipt, f64), String> {
    let blob = input(set, &request["request"], reasons);
    let env = ExecutorEnv::builder().write_slice(&blob).build().map_err(|e| e.to_string())?;
    let t = Instant::now();
    let info = default_prover().prove_with_opts(env, CEDAR_GUEST_ELF, &opts(kind).0).map_err(|e| format!("{e:#}"))?;
    Ok((info.receipt, t.elapsed().as_secs_f64()))
}

fn peak_children_gb() -> f64 {
    let mut ru: libc::rusage = unsafe { std::mem::zeroed() };
    unsafe { libc::getrusage(libc::RUSAGE_CHILDREN, &mut ru) };
    // macOS reports bytes.
    ru.ru_maxrss as f64 / (1u64 << 30) as f64
}

fn cmd_prove(repo: &Path, out: &str, name: &str, kind: &str, n: usize) {
    let mut o = Out::new(repo, out, "E1");
    let set = load(repo, name);
    let r = &set.requests[0];
    for i in 0..n {
        match prove(&set, r, kind, true) {
            Ok((receipt, secs)) => {
                let j: Journal = receipt.journal.decode().unwrap();
                if let Err(e) = check(&set, r, &j) {
                    o.line(&set, r, &format!("prove_time_s_{kind}"), json!(secs), i, "failed", &e);
                    continue;
                }
                o.line(&set, r, &format!("prove_time_s_{kind}"), json!(secs), i, "ok", "end to end from inputs, includes execution");
                let bytes = bincode::serialize(&receipt).unwrap();
                o.line(&set, r, &format!("receipt_bytes_{kind}"), json!(bytes.len()), i, "ok", "bincode of the whole receipt, journal included");
                o.line(&set, r, &format!("seal_bytes_{kind}"), json!(receipt.seal_size()), i, "ok", "");
                if i == n - 1 {
                    let mut ms: Vec<f64> = (0..20)
                        .map(|_| {
                            let t = Instant::now();
                            receipt.verify(CEDAR_GUEST_ID).expect("receipt verifies");
                            t.elapsed().as_secs_f64() * 1e3
                        })
                        .collect();
                    for (k, m) in ms.iter().enumerate() {
                        o.line(&set, r, &format!("verify_time_ms_{kind}"), json!(m), k, "ok", "");
                    }
                    ms.sort_by(|a, b| a.partial_cmp(b).unwrap());
                    println!("{name} {kind}: verify median {:.2} ms", (ms[9] + ms[10]) / 2.0);
                }
                println!("{name} {kind} #{i}: {secs:.1} s, receipt {} bytes", bytes.len());
            }
            Err(e) => {
                eprintln!("{name} {kind} #{i}: FAILED {e}");
                o.line(&set, r, &format!("prove_time_s_{kind}"), Value::Null, i, "failed", &e);
            }
        }
    }
    o.line(&set, r, &format!("peak_memory_gb_{kind}"), json!(peak_children_gb()), 0, "ok", "largest prover child process across the repeats of this run");
}

/// The RISC Zero seal: a bincode receipt. The journal inside it must be the journal claimed.
struct Risc0Seals;
impl SealVerifier for Risc0Seals {
    fn verify(&self, _format: &str, evaluator: &[u8; 32], journal: &Journal, seal: &[u8]) -> Result<(), String> {
        let receipt: Receipt = bincode::deserialize(seal).map_err(|e| e.to_string())?;
        receipt.verify(Digest::from(*evaluator)).map_err(|e| e.to_string())?;
        let proven: Journal = receipt.journal.decode().map_err(|e| e.to_string())?;
        if &proven != journal {
            return Err("journal in the response is not the proven journal".into());
        }
        Ok(())
    }
}

fn cmd_demo(repo: &Path, name: &str, kind: &str, outdir: &str) {
    let set = load(repo, name);
    let r = &set.requests[0];
    let (receipt, secs) = prove(&set, r, kind, true).expect("prove");
    let journal: Journal = receipt.journal.decode().unwrap();
    let format = opts(kind).1;
    let resp = response(&journal, format, &image_id(), &bincode::serialize(&receipt).unwrap());
    let dir = PathBuf::from(outdir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("request.json"), serde_json::to_string_pretty(&r["request"]).unwrap()).unwrap();
    fs::write(dir.join("response.json"), serde_json::to_string_pretty(&resp).unwrap()).unwrap();

    let pins = Pins {
        evaluators: vec![(format.into(), image_id())],
        policy_hashes: vec![parse_hex_hash(set.manifest["policy_hash"].as_str().unwrap()).unwrap()],
        schema_hash: if set.schema.is_empty() { None } else { Some(parse_hex_hash(set.manifest["schema_hash"].as_str().unwrap()).unwrap()) },
        entity_roots: vec![parse_hex_hash(set.manifest["entities_sha256_jcs_v0"].as_str().unwrap()).unwrap()],
        nonce: r["request"].pointer("/context/nonce").and_then(Value::as_str).map(str::to_owned),
    };
    println!("proved in {secs:.1} s");
    println!("honest response:    {:?}", verify(&r["request"], &resp, &pins, &Risc0Seals));
    let mut flipped = resp.clone();
    flipped["decision"] = json!(!journal.decision);
    println!("flipped decision:   {:?}", verify(&r["request"], &flipped, &pins, &Risc0Seals));
    let mut forged = resp.clone();
    forged["decision"] = json!(!journal.decision);
    forged["context"]["proof"]["journal"]["decision"] = json!(!journal.decision);
    println!("forged journal:     {:?}", verify(&r["request"], &forged, &pins, &Risc0Seals));
    let mut other = r["request"].clone();
    other["resource"]["id"] = json!("l1");
    println!("different request:  {:?}", verify(&other, &resp, &pins, &Risc0Seals));
    let mut stale = pins.clone();
    stale.policy_hashes = vec![[0; 32]];
    println!("unlisted policy:    {:?}", verify(&r["request"], &resp, &stale, &Risc0Seals));
}

/// The record guest's input: the two entities with closed ancestors as parents, and their paths.
fn records_input(repo: &Path, name: &str, request: &Value) -> (Vec<u8>, [u8; 32]) {
    let set = load(repo, name);
    let d = repo.join("corpus/sets").join(name);
    let entities: Value = serde_json::from_slice(&set.entities).unwrap();
    let store = RecordStore::new(&entities, &Layouts::new()).expect("record store");
    let sorted = sorted_entities(&entities).unwrap();
    // Parents by UID text, for the closure.
    let parents: std::collections::BTreeMap<String, Vec<Value>> = sorted
        .iter()
        .map(|e| (uid_text(&e["uid"]).unwrap(), e["parents"].as_array().cloned().unwrap_or_default()))
        .collect();
    let one = |who: &Value| -> (Vec<u8>, Vec<u8>) {
        let u = uid_text(who).unwrap();
        let e = sorted.iter().find(|e| uid_text(&e["uid"]).as_deref() == Some(&u)).expect("entity in store");
        let (mut seen, mut todo, mut closed) = (std::collections::BTreeSet::new(), parents[&u].clone(), Vec::new());
        while let Some(p) = todo.pop() {
            let t = uid_text(&p).unwrap();
            if seen.insert(t.clone()) {
                todo.extend(parents.get(&t).cloned().unwrap_or_default());
                closed.push(p);
            }
        }
        let mut x = e.clone();
        x["parents"] = Value::Array(closed);
        let (_, path) = store.open(&u).unwrap();
        let mut pb = Vec::new();
        for st in &path {
            pb.extend_from_slice(&st.sibling);
            pb.push(st.sibling_is_left as u8);
        }
        (serde_json::to_vec(&x).unwrap(), pb)
    };
    let (pj, pp) = one(&request["subject"]);
    let (rj, rp) = one(&request["resource"]);
    let schema = fs::read(d.join("schema.cedarschema")).unwrap();
    let req = canonical_request(request);
    let mut b = vec![1u8];
    for f in [&set.policies[..], &schema[..], &req[..], &store.root[..], &pj[..], &pp[..], &rj[..], &rp[..]] {
        b.extend_from_slice(&(f.len() as u32).to_le_bytes());
        b.extend_from_slice(f);
    }
    (b, store.root)
}

const RECORD_SEGMENTS: [&str; 6] = ["read", "open_records", "parse_policies", "build_entities", "map_request", "authorize"];

/// E1b: correctness and cycles of the record guest, every request of a set.
fn cmd_exec_records(repo: &Path, out: &str, name: &str) {
    let mut o = Out::new(repo, out, "E1b");
    let set = load(repo, name);
    let (mut ok, mut bad) = (0, 0);
    let mut medians: Vec<u64> = Vec::new();
    for r in &set.requests {
        let (blob, root) = records_input(repo, name, &r["request"]);
        let mut stdout = Vec::new();
        let session = {
            let env = ExecutorEnv::builder().write_slice(&blob).stdout(&mut stdout).build().unwrap();
            default_executor().execute(env, RECORDS_ELF).expect("execute")
        };
        let j: Journal = session.journal.decode().unwrap();
        let reasons: Vec<String> = r["oracle"]["reasons"].as_array().unwrap().iter().map(|x| x.as_str().unwrap().into()).collect();
        let good = j.decision == r["oracle"]["decision"].as_bool().unwrap() && j.reasons.as_ref() == Some(&reasons) && j.entities.root == root
            && j.request_hash == parse_hex_hash(r["request_hash"].as_str().unwrap()).unwrap();
        if good { ok += 1 } else { bad += 1; eprintln!("FAIL {}", r["name"]); }
        let marks: Vec<u64> = stdout.chunks(8).map(|c| u64::from_le_bytes(c.try_into().unwrap())).collect();
        let total = *marks.last().unwrap();
        medians.push(total);
        o.line(&set, r, "user_cycles", json!(total), 0, if good { "ok" } else { "failed" }, &hex_hash(&root));
        for (n, w) in RECORD_SEGMENTS.iter().zip(marks.windows(2)) {
            o.line(&set, r, &format!("cycles_{n}"), json!(w[1] - w[0]), 0, "ok", "");
        }
    }
    medians.sort();
    println!("{name}: matched oracle {ok}, failed {bad}, median cycles {}", medians[medians.len() / 2]);
    if bad > 0 { std::process::exit(1); }
}

/// E1b: prove one request with the record guest.
fn cmd_prove_records(repo: &Path, out: &str, name: &str, n: usize, which: usize) {
    let mut o = Out::new(repo, out, "E1b");
    let set = load(repo, name);
    let r = &set.requests[which];
    let (blob, _) = records_input(repo, name, &r["request"]);
    for i in 0..n {
        let env = ExecutorEnv::builder().write_slice(&blob).build().unwrap();
        let t = Instant::now();
        let info = default_prover().prove_with_opts(env, RECORDS_ELF, &ProverOpts::composite()).expect("prove");
        let secs = t.elapsed().as_secs_f64();
        let j: Journal = info.receipt.journal.decode().unwrap();
        assert_eq!(j.decision, r["oracle"]["decision"].as_bool().unwrap());
        o.line(&set, r, "prove_time_s_composite", json!(secs), i, "ok", "record guest");
        o.line(&set, r, "receipt_bytes_composite", json!(bincode::serialize(&info.receipt).unwrap().len()), i, "ok", "");
        println!("{name} #{i}: {secs:.1} s, {} bytes", bincode::serialize(&info.receipt).unwrap().len());
        if i == n - 1 {
            for k in 0..20 {
                let t = Instant::now();
                info.receipt.verify(RECORDS_ID).expect("verifies");
                o.line(&set, r, "verify_time_ms_composite", json!(t.elapsed().as_secs_f64() * 1e3), k, "ok", "");
            }
        }
    }
    o.line(&set, r, "peak_memory_gb_composite", json!(peak_children_gb()), 0, "ok", "");
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let repo = PathBuf::from(&a[2]);
    match a[1].as_str() {
        "exec" => cmd_exec(&repo, &a[3]),
        "exec-records" => cmd_exec_records(&repo, &a[3], &a[4]),
        "prove-records" => cmd_prove_records(&repo, &a[3], &a[4], a[5].parse().unwrap(), a[6].parse().unwrap()),
        "prove" => cmd_prove(&repo, &a[3], &a[4], &a[5], a[6].parse().unwrap()),
        "demo" => cmd_demo(&repo, &a[3], &a[4], &a[5]),
        c => panic!("unknown command {c}"),
    }
}
