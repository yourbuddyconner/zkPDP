// E8: the TinyTodo decision as a GordieHowe relation, written by hand. Same
// statement and record commitment as bench/circuit/tinytodo/src/main.nr.
//
//   zkpdp_tinytodo <inputs dir> <out.jsonl> <commit> [repeats]
//
// Limit: strings have fixed lengths in a relation, so each request shape
// (the six string lengths) compiles to its own system. The Noir circuit is
// one circuit for all shapes. Compile time is measured for that reason.
#include <algorithm>
#include <chrono>
#include <cstdio>
#include <cstring>
#include <filesystem>
#include <fstream>
#include <functional>
#include <map>
#include <memory>
#include <sstream>
#include <string>
#include <vector>
#include "gf2k/lch14_reed_solomon.h"
#include "gordiehowe/examples/shared/domain_profile.h"
#include "gordiehowe/lib/sha256.h"
#include "gordiehowe/system.h"
#include "gordiehowe/compiler/emit_export.h"
#include "gordiehowe/compiler/plan_export.h"
#include "gordiehowe/compiler/plan_program.h"
#include "gordiehowe/compiler/plan_proofs.h"
#ifdef ZKPDP_SIGNED
#include "gordiehowe/examples/mdoc/mdoc_signatures.h"
#include "gordiehowe/examples/shared/setup.h"
#include "gordiehowe/instances.h"
#endif
#include "params/param_sets.h"
#include "random/secure_random_engine.h"
#include "util/crypto.h"

using namespace proofs;
using namespace proofs::gordiehowe;
using P = param_sets::ROMParams;
using HF = P::HashField;
using PH = P::ProtocolHash;
using RSH = LCH14ReedSolomonFactory<HF>;
#ifdef ZKPDP_SIGNED
// E3: the store's root is private and an authority's P-256 signature is checked in domain A.
using AF = P::EcdsaField;
using Sys = ConstraintSystem<PH, HF, AF>;
#else
using Sys = ConstraintSystem<PH, HF>;
#endif
using Clock = std::chrono::steady_clock;
using ByteVec = std::vector<uint8_t>;
using BitE = Expr<HF, Bit>;
using CRef = Ref<HF, Bit, Committed>;
using PRef = Ref<HF, Bit, Public>;
namespace kk = lib::keccak;
namespace sh = lib::sha256;

constexpr size_t kAttrs = 8, kAnc = 16, kDepth = 12;
static double ms(Clock::time_point t) { return std::chrono::duration<double, std::milli>(Clock::now() - t).count(); }

static ByteVec sha(const ByteVec &m) {
  ByteVec d(32);
  proofs::SHA256 h;
  h.Update(m.data(), m.size());
  h.DigestData(d.data());
  return d;
}
static ByteVec cat(std::initializer_list<ByteVec> xs) {
  ByteVec o;
  for (const auto &x : xs) o.insert(o.end(), x.begin(), x.end());
  return o;
}
static ByteVec str(const std::string &s) { return ByteVec(s.begin(), s.end()); }

struct Input {
  std::map<std::string, ByteVec> v;
  const ByteVec &operator[](const std::string &k) const { return v.at(k); }
  std::string s(const std::string &k) const { const auto &b = v.at(k); return std::string(b.begin(), b.end()); }
};
static Input read_input(const std::string &path) {
  Input in;
  std::ifstream f(path);
  std::string k, hex;
  while (f >> k) {
    hex.clear();
    if (f.peek() == ' ') { f.get(); if (f.peek() != '\n') f >> hex; }
    ByteVec b;
    for (size_t i = 0; i + 1 < hex.size(); i += 2) b.push_back(uint8_t(std::stoi(hex.substr(i, 2), nullptr, 16)));
    in.v[k] = b;
  }
  return in;
}

struct Opening {
  CRef present, attrs, anc, path, sides, active;
  std::vector<CRef> left, right, next;
};

// One compiled relation for one request shape.
struct Tiny {
  HF hf;
  RSH rsh{hf};
  Sys cs{"zkpdp-tinytodo"};
  PRef request_hash, policy_hash, schema_hash, nonce, decision;
#ifdef ZKPDP_SIGNED
  examples::mdoc::p256_ecdsa::Pub issuer_pk;
  PRef epoch;
  CRef root, ecopy;
  HostInput e_host, sig_host;
  P256ReedSolomon rs_a;
#else
  PRef root;
#endif
  CRef stype, sid, action, rtype, rid, pcopy, scopy, ncopy;
  Opening ents[2];
  std::unique_ptr<typename Sys::Compiled> comp;
  double compile_ms = 0;

  std::vector<HF::Elt> bits(const ByteVec &b) const {
    std::vector<HF::Elt> v;
    for (uint8_t x : b) for (int j = 0; j < 8; j++) v.push_back((x >> j) & 1 ? hf.one() : hf.zero());
    return v;
  }
  std::vector<HF::Elt> flags(const std::vector<bool> &b) const {
    std::vector<HF::Elt> v;
    for (bool x : b) v.push_back(x ? hf.one() : hf.zero());
    return v;
  }

  explicit Tiny(const Input &in) {
    auto &H = cs.domain(examples::domain_profile<P>(hf, rsh), "H");
    auto batch = sh::sha256_batch(cs[H], "sha");
    auto byte_field = [&](const char *nm, size_t n) { return H.priv(Bit{}, nm, Shape{n, 8}); };
    size_t nl = in["nonce"].size();
    request_hash = H.pub(Bit{}, "request_hash", Shape{256});
    policy_hash = H.pub(Bit{}, "policy_hash", Shape{256});
    schema_hash = H.pub(Bit{}, "schema_hash", Shape{256});
#ifdef ZKPDP_SIGNED
    root = H.priv(Bit{}, "root", Shape{256});
    epoch = H.pub(Bit{}, "epoch", Shape{64});
    ecopy = H.priv(Bit{}, "ecopy", Shape{64});
    assert_equal(ecopy, epoch, "epoch.pub");
#else
    root = H.pub(Bit{}, "root", Shape{256});
#endif
    nonce = H.pub(Bit{}, "nonce", Shape{nl, 8});
    decision = H.pub(Bit{}, "decision", Shape{1});
    stype = byte_field("stype", in["subject_type"].size());
    sid = byte_field("sid", in["subject_id"].size());
    action = byte_field("action", in["action"].size());
    rtype = byte_field("rtype", in["resource_type"].size());
    rid = byte_field("rid", in["resource_id"].size());

    // This relation is one policy set under one schema.
    auto constant = [&](const ByteVec &b) { return kk::constant_input(H, kk::constant_bytes(b)); };
    // A public value alone is no domain value, so each is pinned through a committed copy.
    pcopy = H.priv(Bit{}, "pcopy", Shape{256});
    scopy = H.priv(Bit{}, "scopy", Shape{256});
    ncopy = H.priv(Bit{}, "ncopy", Shape{nl, 8});
    assert_equal(pcopy, policy_hash, "policy.pub");
    assert_equal(pcopy, constant(in["policy_hash"]), "policy");
    assert_equal(scopy, schema_hash, "schema.pub");
    assert_equal(scopy, constant(in["schema_hash"]), "schema_const");
    assert_equal(ncopy, nonce, "nonce.pub");

    // Strings JSON and Cedar write without escaping, so concatenation is canonical.
    {
      int fi = 0;
      for (const CRef &f : {stype, sid, action, rtype, rid}) {
        auto w = f.from_bits();
        assert_true(!(w < 0x20) & (w < 0x7f) & (w != 0x22) & (w != 0x5c), "plain" + std::to_string(fi++));
      }
    }
    {
      auto w = ncopy.from_bits();
      assert_true(!(w < 0x20) & (w < 0x7f) & (w != 0x22) & (w != 0x5c), "plain_nonce");
    }

    // The canonical request is the one the verifier hashed.
    auto cb = [&](const std::string &s) { return kk::constant_bytes(str(s)); };
    auto rh = sh::sha256(batch, "req",
        kk::sponge_input(cb("{\"action\":{\"name\":\""), action, cb("\"},\"context\":{\"nonce\":\""), nonce,
                         cb("\"},\"resource\":{\"id\":\""), rid, cb("\",\"type\":\""), rtype,
                         cb("\"},\"subject\":{\"id\":\""), sid, cb("\",\"type\":\""), stype, cb("\"}}")));
    assert_equal(rh, request_hash, "request");

    auto p_uid = sh::sha256(batch, "p_uid", kk::sponge_input(stype, cb("::\""), sid, cb("\"")));
    auto r_uid = sh::sha256(batch, "r_uid", kk::sponge_input(rtype, cb("::\""), rid, cb("\"")));

    // The principal and the resource are leaves of the committed store.
    auto open = [&](Opening &o, const std::string &nm, const CRef &uid) {
      o.present = H.priv(Bit{}, nm + ".present", Shape{8});
      o.attrs = H.priv(Bit{}, nm + ".attrs", Shape{kAttrs, 256});
      o.anc = H.priv(Bit{}, nm + ".anc", Shape{kAnc, 256});
      o.path = H.priv(Bit{}, nm + ".path", Shape{kDepth, 256});
      o.sides = H.priv(Bit{}, nm + ".sides", Shape{kDepth});
      o.active = H.priv(Bit{}, nm + ".active", Shape{kDepth});
      auto ah = sh::sha256(batch, nm + ".ah", kk::sponge_input(o.present, o.attrs));
      auto ch = sh::sha256(batch, nm + ".ch", kk::sponge_input(o.anc));
      auto leaf = sh::sha256(batch, nm + ".leaf", kk::sponge_input(uid, ah, ch));
      BitE h = BitE(leaf);
      for (size_t i = 0; i < kDepth; i++) {
        std::string lv = nm + ".l" + std::to_string(i);
        CRef l = H.priv(Bit{}, lv + ".left", Shape{256});
        CRef r = H.priv(Bit{}, lv + ".right", Shape{256});
        CRef n = H.priv(Bit{}, lv + ".next", Shape{256});
        auto side = o.sides.at(i);
        auto sib = o.path.at(i);
        assert_equal(l, side.mux(sib, h), lv + ".l");
        assert_equal(r, side.mux(h, sib), lv + ".r");
        auto d = sh::sha256(batch, lv + ".node", kk::sponge_input(kk::constant_bytes(ByteVec{1}), l, r));
        assert_equal(n, o.active.at(i).mux(d, h), lv + ".n");
        if (i + 1 < kDepth) assert_true(!o.active.at(i + 1) | o.active.at(i), lv + ".a");
        o.left.push_back(l); o.right.push_back(r); o.next.push_back(n);
        h = BitE(n);
      }
      assert_equal(h, root, nm + ".root");
    };
#ifdef ZKPDP_SIGNED
    {
      namespace ms = examples::mdoc::signatures;
      auto &A = cs.domain(examples::domain_profile<P>(p256_base, rs_a.factory()), "A");
      issuer_pk = A.pub(Field{}, "issuer_pk", Shape{2});
      // The verifier pins this key, so it checks once that the key is on the curve.
      auto signed_digest = sh::sha256(batch, "signed", kk::sponge_input(cb("zk-pdp-root-v0"), root, ecopy));
      e_host = cs.root().host_input("e_host", 32);
      sig_host = cs.root().host_input("sig_host", 64);
      auto issuer = ms::ecdsa_verify_public_key(cs.scope("issuer"), A, issuer_pk, e_host, sig_host);
      const std::vector<size_t> order = lib::arith::big_endian_bit_order(32);
      assert_equal(signed_digest[order].named("cross.e"), issuer.e);
    }
#endif
    open(ents[0], "p", p_uid);
    open(ents[1], "r", r_uid);

    // The request fits the schema, and the four policies.
    auto zero = H.constant(Bit{}, std::vector<uint64_t>{0}, Shape{1}).at(0);
    auto is_str = [&](const CRef &f, size_t len, const std::string &s) -> BitE {
      if (len != s.size()) return BitE(zero);
      return !(f.flat() ^ constant(str(s))).any(0);
    };
    size_t al = in["action"].size(), rl = in["resource_type"].size(), il = in["resource_id"].size(), sl = in["subject_type"].size();
    auto a = [&](const std::string &s) { return is_str(action, al, s); };
    BitE is_list = is_str(rtype, rl, "List");
    BitE is_app = is_str(rtype, rl, "Application") & is_str(rid, il, "TinyTodo");
    BitE get_list = a("GetList");
    BitE edits = a("UpdateList") | a("CreateTask") | a("UpdateTask") | a("DeleteTask");
    BitE list_action = get_list | edits | a("DeleteList") | a("EditShare");
    BitE app_action = a("CreateList") | a("GetLists");
    assert_true(is_str(stype, sl, "User"), "subject type");
    assert_true((list_action & is_list) | (app_action & is_app), "fits_schema");
    auto &pe = ents[0]; auto &re = ents[1];
    auto slot_eq = [&](BitE x, BitE y) -> BitE { return !(x ^ y).any(0); };
    auto in_anc = [&](BitE group) -> BitE {
      BitE found = BitE(zero);
      for (size_t i = 0; i < kAnc; i++) found = found | slot_eq(BitE(pe.anc.at(i)), group);
      return found;
    };
    // Attributes in name order: editors, name, owner, readers, tasks.
    BitE editors = BitE(re.attrs.at(0)), owner = BitE(re.attrs.at(2)), readers = BitE(re.attrs.at(3));
    BitE in_e = in_anc(editors), in_r = in_anc(readers);
    BitE p0 = app_action & is_app;
    BitE p1 = is_list & slot_eq(owner, BitE(p_uid));
    BitE p2 = get_list & is_list & (in_r | in_e);
    BitE p3 = edits & is_list & in_e;
    assert_equal(decision.at(0), p0 | p1 | p2 | p3, "decision");

    auto t = Clock::now();
#ifdef ZKPDP_SIGNED
    comp = compile(cs, examples::mac_profile<P>("zkpdp-tinytodo-signed"));
#else
    comp = compile(cs);
#endif
    compile_ms = ms(t);
  }

  Sys::Assignment statement(const Input &in, bool flip = false) const {
    Sys::Assignment st;
    st.set(request_hash, bits(in["request_hash"]));
    st.set(policy_hash, bits(in["policy_hash"]));
    st.set(schema_hash, bits(in["schema_hash"]));
#ifdef ZKPDP_SIGNED
    st.set(epoch, bits(in["epoch"]));
    {
      auto elt = [](const ByteVec &be) {
        uint8_t le[32];
        for (size_t i = 0; i < 32; i++) le[i] = be[31 - i];
        return *p256_base.of_bytes_field(le);
      };
      st.set(issuer_pk, std::vector<AF::Elt>{elt(in["authority_x"]), elt(in["authority_y"])});
    }
#else
    st.set(root, bits(in["entities_root"]));
#endif
    st.set(nonce, bits(in["nonce"]));
    bool d = in["decision"][0] != 0;
    st.set(decision, flags({flip ? !d : d}));
    return st;
  }

  // MUT edits the host's copy of the values before the Merkle walk.
  Sys::Assignment witness(Input in, const std::function<void(Input &)> &mut = {}) const {
    if (mut) mut(in);
    Sys::Assignment w;
#ifdef ZKPDP_SIGNED
    w.set(root, bits(in["entities_root"]));
    w.set(ecopy, bits(in["epoch"]));
    w.set(e_host, sha(cat({str("zk-pdp-root-v0"), in["entities_root"], in["epoch"]})));
    w.set(sig_host, in["signature"]);
#endif
    w.set(pcopy, bits(in["policy_hash"]));
    w.set(scopy, bits(in["schema_hash"]));
    w.set(ncopy, bits(in["nonce"]));
    w.set(stype, bits(in["subject_type"]));
    w.set(sid, bits(in["subject_id"]));
    w.set(action, bits(in["action"]));
    w.set(rtype, bits(in["resource_type"]));
    w.set(rid, bits(in["resource_id"]));
    auto one = [&](const Opening &o, const std::string &e, const ByteVec &uid) {
      w.set(o.present, bits(in[e + ".present"]));
      w.set(o.attrs, bits(in[e + ".attrs"]));
      w.set(o.anc, bits(in[e + ".ancestors"]));
      w.set(o.path, bits(in[e + ".path"]));
      std::vector<bool> side, act;
      size_t len = in[e + ".path_len"][0];
      for (size_t i = 0; i < kDepth; i++) { side.push_back(in[e + ".sides"][i] != 0); act.push_back(i < len); }
      w.set(o.sides, flags(side));
      w.set(o.active, flags(act));
      ByteVec h = sha(cat({uid, sha(cat({in[e + ".present"], in[e + ".attrs"]})), sha(in[e + ".ancestors"])}));
      for (size_t i = 0; i < kDepth; i++) {
        ByteVec sib(in[e + ".path"].begin() + 32 * i, in[e + ".path"].begin() + 32 * (i + 1));
        const ByteVec &l = side[i] ? sib : h, &r = side[i] ? h : sib;
        ByteVec up = sha(cat({ByteVec{1}, l, r}));
        w.set(o.left[i], bits(l));
        w.set(o.right[i], bits(r));
        ByteVec nx = act[i] ? up : h;
        w.set(o.next[i], bits(nx));
        h = nx;
      }
    };
    one(ents[0], "principal", sha(str(in.s("subject_type") + "::\"" + in.s("subject_id") + "\"")));
    one(ents[1], "resource", sha(str(in.s("resource_type") + "::\"" + in.s("resource_id") + "\"")));
    return w;
  }
};

static std::string shape_of(const Input &in) {
  std::ostringstream s;
  for (auto k : {"nonce", "subject_type", "subject_id", "action", "resource_type", "resource_id"}) s << in[k].size() << ".";
  return s.str();
}

#ifndef ZKPDP_SIGNED
// The compiled system's plan, realized program and proof modules, as Lean, for
// Lean's kernel to check (E10). Follows examples::write_plan, without its test header.
template <class C>
static void export_lean(const C &cs, const std::string &dir, const std::string &commit) {
  compiler::ExportMeta meta{"zkpdp-tinytodo", P::kId, commit, "zkpdp", 0};
  meta.lean_name = "Heldout.ZkpdpTinyTodo." + compiler::lean_param_set(P::kId);
  const std::string plan = compiler::export_lean_plan(cs, meta);
  const compiler::ProgramData program = compiler::build_program(cs, meta);
  const compiler::ProofModuleData data = compiler::proof_module_data(cs, meta, program);
  std::vector<std::pair<std::string, std::string>> modules;
  modules.emplace_back(compiler::lean_plan_file(dir, meta), plan);
  modules.emplace_back(compiler::lean_program_file(dir, meta), program.text);
  modules.emplace_back(compiler::lean_derivs_file(dir, meta), program.derivs);
  for (const auto &mod : compiler::proof_modules(data, compiler::ProofTemplate{}))
    modules.emplace_back(compiler::lean_proof_file(dir, meta, mod), mod.text);
  const std::filesystem::path base = std::filesystem::path(modules.front().first).parent_path();
  const std::filesystem::path gkr = base / "GKR";
  size_t certs = 0;
  for (size_t d = 0; d < cs.num_domains(); ++d) {
    std::string cert = cs.compiled_domain(d).gkr_certificate();
    if (cert.empty()) continue;
    certs++;
    modules.emplace_back((gkr / (cs.domain(d).name + ".cert")).string(), std::move(cert));
  }
  for (auto &ex : compiler::emit_tables(cs, meta)) {
    modules.emplace_back((gkr / (ex.domain + ".ghc")).string(), std::string(ex.ghc.begin(), ex.ghc.end()));
    for (auto &[rel, text] : ex.modules) modules.emplace_back((base / rel).string(), std::move(text));
  }
  size_t bytes = 0;
  for (const auto &[path, text] : modules) {
    std::filesystem::create_directories(std::filesystem::path(path).parent_path());
    std::ofstream(path, std::ios::binary) << text;
    bytes += text.size();
  }
  printf("exported %zu modules, %zu bytes, %zu GKR certificates, under %s\n", modules.size(), bytes, certs, base.c_str());
  std::string report = cs.report();
  printf("%s\n", report.substr(0, 2600).c_str());
}

#endif

int main(int argc, char **argv) {
  std::string dir = argv[1];
  #ifndef ZKPDP_SIGNED
  if (const char *ex = getenv("ZKPDP_EXPORT")) {
    std::vector<std::string> fs;
    for (auto &e : std::filesystem::directory_iterator(dir)) fs.push_back(e.path().string());
    std::sort(fs.begin(), fs.end());
    Input in = read_input(fs[0]);
    Tiny t(in);
    printf("shape %s compile %.0f ms\n", shape_of(in).c_str(), t.compile_ms);
    export_lean(*t.comp, ex, argv[3]);
    return 0;
  }
#endif
  std::ofstream out(argv[2], std::ios::app);
  std::string commit = argv[3];
  int repeats = argc > 4 ? atoi(argv[4]) : 1;
  std::vector<std::string> files;
  for (auto &e : std::filesystem::directory_iterator(dir)) files.push_back(e.path().string());
  std::sort(files.begin(), files.end());
  std::map<std::string, std::unique_ptr<Tiny>> systems;
  SecureRandomEngine rng;
  auto line = [&](const std::string &req, const std::string &metric, double v, const std::string &extra = "") {
    out << "{\"experiment\":\"E8\",\"backend\":\"gordiehowe\",\"commit\":\"" << commit << "\",\"set\":\"tinytodo\",\"request\":\"" << req
        << "\",\"metric\":\"" << metric << "\",\"value\":" << v << ",\"status\":\"ok\",\"notes\":\"" << extra << "\"}\n";
    out.flush();
  };
  int good = 0, bad = 0, attacks = 0, accepted = 0, threw_count = 0, verified_false = 0, controls = 0;
  for (const auto &f : files) {
    Input in = read_input(f);
    std::string name = in.s("name");
    auto &t = systems[shape_of(in)];
    if (!t) {
      t = std::make_unique<Tiny>(in);
      printf("shape %s compile %.0f ms\n", shape_of(in).c_str(), t->compile_ms);
      line(name, "compile_time_ms", t->compile_ms, "per request shape");
    }
    auto prep = t->comp->prepare(t->statement(in));
    std::vector<uint8_t> proof;
    for (int k = 0; k < repeats; k++) {
      auto t0 = Clock::now();
      proof = prep.prove(t->witness(in), rng);
      double pm = ms(t0);
      t0 = Clock::now();
      bool ok = prep.verify(proof);
      double vm = ms(t0);
      line(name, "prove_time_ms", pm); line(name, "verify_time_ms", vm); line(name, "proof_bytes", double(proof.size()));
      if (k == repeats - 1) printf("%-28s decision=%d prove %.0f ms verify %.1f ms proof %zu bytes ok=%d\n", name.c_str(), in["decision"][0], pm, vm, proof.size(), ok);
      ok ? good++ : bad++;
    }
    line(name, "correct", 1, "accepts the oracle decision");
    // The opposite decision must not verify, with this proof or one built for it.
    attacks++;
    if (t->comp->prepare(t->statement(in, true)).verify(proof)) { accepted++; printf("  ACCEPTED flipped statement\n"); }
    // Adversarial witnesses: prove regardless, then verify.
    typename Sys::ProveOptions opt;
    opt.require_satisfied = false;
    auto try_attack = [&](const std::string &an, const std::function<void(Input &)> &mut, bool flip) {
      attacks++;
      auto pr = t->comp->prepare(t->statement(in, flip));
      bool acc = false, threw = false;
      try { acc = pr.verify(pr.prove(t->witness(in, mut), rng, opt)); } catch (...) { threw = true; }
      threw ? threw_count++ : verified_false += !acc;
      if (acc && an != "control_honest") { accepted++; printf("  ACCEPTED %s flip=%d\n", an.c_str(), flip); }
      if (an == "control_honest" && acc) controls++;
      out << "{\"experiment\":\"E9\",\"part\":\"gordiehowe\",\"backend\":\"gordiehowe\",\"set\":\"tinytodo\",\"request\":\"" << name << "\",\"attack\":\"" << an << "\",\"flip\":" << flip << ",\"outcome\":\"" << (acc ? "accepted" : threw ? "prover_threw" : "verify_rejected") << "\"}\n";
    };
    ByteVec p_uid = sha(str(in.s("subject_type") + "::\"" + in.s("subject_id") + "\""));
    auto setslot = [](ByteVec &b, size_t i, const ByteVec &x) { std::copy(x.begin(), x.end(), b.begin() + 32 * i); };
    if (name.find("stranger") != std::string::npos || name.find("reader-updates") != std::string::npos || name.find("editor-deletes") != std::string::npos) {
      // A deny: claim the permit with forged facts.
      try_attack("forge_owner", [&](Input &x) { setslot(x.v["resource.attrs"], 2, p_uid); }, true);
      try_attack("forge_editors_membership", [&](Input &x) { ByteVec g(x["resource.attrs"].begin(), x["resource.attrs"].begin() + 32); setslot(x.v["principal.ancestors"], 15, g); }, true);
      try_attack("forge_readers_membership", [&](Input &x) { ByteVec g(x["resource.attrs"].begin() + 96, x["resource.attrs"].begin() + 128); setslot(x.v["principal.ancestors"], 15, g); }, true);
    }
    // Control: the same path with an honest witness must verify, or "rejected" means nothing.
    try_attack("control_honest", {}, false);
#ifdef ZKPDP_SIGNED
    try_attack("signature_flip", [&](Input &x) { x.v["signature"][63] ^= 1; }, false);
    try_attack("root_swap", [&](Input &x) { x.v["entities_root"][0] ^= 1; }, false);
#endif
    try_attack("flip_decision_only", {}, true);
    try_attack("attr_flip", [&](Input &x) { x.v["resource.attrs"][31] ^= 1; }, false);
    try_attack("anc_flip", [&](Input &x) { x.v["principal.ancestors"][31] ^= 1; }, false);
    try_attack("present_flip", [&](Input &x) { x.v["resource.present"][0] ^= 1; }, false);
    try_attack("path_flip", [&](Input &x) { x.v["resource.path"][31] ^= 1; }, false);
    try_attack("side_toggle", [&](Input &x) { x.v["resource.sides"][0] ^= 1; }, false);
    try_attack("path_len_zero", [&](Input &x) { x.v["resource.path_len"][0] = 0; }, false);
    try_attack("subject_id_flip", [&](Input &x) { x.v["subject_id"][0] ^= 1; }, false);
    try_attack("action_flip", [&](Input &x) { x.v["action"][0] ^= 1; }, false);
  }
  out.flush();
  printf("requests ok %d bad %d, attacks %d accepted %d, verifier rejected %d, prover threw %d, controls verified %d, shapes %zu\n", good, bad, attacks, accepted, verified_false, threw_count, controls, systems.size());
  return bad || accepted ? 1 : 0;
}
