import { Noir } from "@noir-lang/noir_js";
import { Barretenberg, UltraHonkBackend } from "@aztec/bb.js";

const $ = (id) => document.getElementById(id);
const log = (m) => ($("log").textContent += m + "\n");

const threads = self.crossOriginIsolated ? Math.min(navigator.hardwareConcurrency || 1, 16) : 1;
const env = {
  user_agent: navigator.userAgent,
  hardware_concurrency: navigator.hardwareConcurrency || null,
  device_memory_gb: navigator.deviceMemory || null,
  cross_origin_isolated: !!self.crossOriginIsolated,
  threads,
};
$("env").textContent =
  `Threads: ${threads}${env.cross_origin_isolated ? "" : " (page is not cross-origin isolated, so the prover is single-threaded)"}\n${env.user_agent}`;

const index = await (await fetch("/circuits/index.json")).json();
for (const c of index) {
  const o = document.createElement("option");
  o.value = c.name;
  o.textContent = c.label;
  $("circuit").append(o);
}

async function post(line) {
  try {
    await fetch("/result", { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify(line) });
  } catch (e) {
    log("could not send result: " + e);
  }
}

$("run").onclick = async () => {
  $("run").disabled = true;
  $("log").textContent = "";
  const meta = index.find((c) => c.name === $("circuit").value);
  const target = $("mode").value;
  const mode = target.endsWith("no-zk") ? "plain" : "zk";
  const repeats = Number($("repeats").value);
  const device = $("device").value.trim();
  const base = { ...meta, mode, env, device };
  const body = $("out").querySelector("tbody");
  body.textContent = "";
  $("out").hidden = false;
  try {
    log("loading circuit");
    const circuit = await (await fetch(`/circuits/${meta.name}/circuit.json`)).json();
    const input = await (await fetch(`/circuits/${meta.name}/input.json`)).json();
    let t = performance.now();
    const api = await Barretenberg.new({ threads });
    const backend = new UltraHonkBackend(circuit.bytecode, api);
    const noir = new Noir(circuit);
    await post({ ...base, metric: "setup_time_s", value: (performance.now() - t) / 1e3, repeat: 0, status: "ok" });
    for (let i = 0; i < repeats; i++) {
      log(`run ${i + 1}: witness`);
      const t0 = performance.now();
      const { witness } = await noir.execute(input);
      const t1 = performance.now();
      log(`run ${i + 1}: proving`);
      const proof = await backend.generateProof(witness, { verifierTarget: target });
      const t2 = performance.now();
      const valid = await backend.verifyProof(proof, { verifierTarget: target });
      const t3 = performance.now();
      const r = { witness: (t1 - t0) / 1e3, prove: (t2 - t1) / 1e3, verify: t3 - t2, bytes: proof.proof.length };
      const tr = document.createElement("tr");
      for (const v of [i + 1, r.witness.toFixed(2), r.prove.toFixed(2), r.verify.toFixed(0), r.bytes, valid ? "yes" : "NO"]) {
        const td = document.createElement("td");
        td.textContent = v;
        tr.append(td);
      }
      body.append(tr);
      const status = valid ? "ok" : "failed";
      await post({ ...base, metric: "witness_time_s", value: r.witness, repeat: i, status });
      await post({ ...base, metric: `prove_time_s_${mode}`, value: r.prove, repeat: i, status });
      await post({ ...base, metric: `verify_time_ms_${mode}`, value: r.verify, repeat: i, status });
      await post({ ...base, metric: `proof_bytes_${mode}`, value: r.bytes, repeat: i, status });
    }
    await api.destroy();
    log("done");
  } catch (e) {
    log("FAILED: " + (e && e.message ? e.message : e));
    await post({ ...base, metric: `prove_time_s_${mode}`, value: null, repeat: 0, status: "failed", notes: String(e && e.message ? e.message : e).slice(0, 400) });
  }
  $("run").disabled = false;
};
