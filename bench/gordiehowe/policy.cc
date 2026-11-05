// E11: any policy set the Cedar compiler handles, as a GordieHowe relation built
// at run time. The compiler writes each request environment's function as text
// (policy.ir). This reads that text and builds the same logic as expressions, so
// a policy is data and needs no C++ compile.
//
//   zkpdp_policy <policy.ir> <inputs dir> <out.jsonl> <set> <commit> [repeats]
//
// Strings use the fixed layout of decision 0026, so one system serves every request.
#include <algorithm>
#include <chrono>
#include <cstdio>
#include <cstring>
#include <filesystem>
#include <fstream>
#include <functional>
#include <map>
#include <memory>
#include <optional>
#include <sstream>
#include <string>
#include <vector>
#include "gf2k/lch14_reed_solomon.h"
#include "gordiehowe/examples/shared/domain_profile.h"
#include "gordiehowe/lib/sha256.h"
#include "gordiehowe/system.h"
#include "params/param_sets.h"
#include "random/secure_random_engine.h"
#include "util/crypto.h"

using namespace proofs;
using namespace proofs::gordiehowe;
using P = param_sets::ROMParams;
using HF = P::HashField;
using PH = P::ProtocolHash;
using RSH = LCH14ReedSolomonFactory<HF>;
using Sys = ConstraintSystem<PH, HF>;
using Clock = std::chrono::steady_clock;
using ByteVec = std::vector<uint8_t>;
using BitE = Expr<HF, Bit>;
using CRef = Ref<HF, Bit, Committed>;
using PRef = Ref<HF, Bit, Public>;
namespace kk = lib::keccak;
namespace sh = lib::sha256;

constexpr size_t kAttrs = 8, kAnc = 16, kDepth = 12, kSet = 8;
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
// One length byte, then the bytes, zero-padded. Decision 0026.
constexpr size_t kCapType = 24, kCapId = 48, kCapAction = 32, kCapNonce = 32;
static ByteVec fixed(const ByteVec &v, size_t cap) {
  check(v.size() <= cap, "a string longer than its capacity");
  ByteVec b{uint8_t(v.size())};
  b.insert(b.end(), v.begin(), v.end());
  b.resize(1 + cap, 0);
  return b;
}
static ByteVec fixed(const std::string &s, size_t cap) { return fixed(str(s), cap); }
static ByteVec unhex(const std::string &hex) {
  ByteVec b;
  for (size_t i = 0; i + 1 < hex.size(); i += 2) b.push_back(uint8_t(std::stoi(hex.substr(i, 2), nullptr, 16)));
  return b;
}

struct Input {
  std::map<std::string, ByteVec> v;
  const ByteVec &operator[](const std::string &k) const { return v.at(k); }
  std::string s(const std::string &k) const { const auto &b = v.at(k); return std::string(b.begin(), b.end()); }
};
static Input read_input(const std::string &path) {
  Input in;
  std::ifstream f(path);
  std::string line;
  while (std::getline(f, line)) {
    auto sp = line.find(' ');
    in.v[line.substr(0, sp)] = sp == std::string::npos ? ByteVec{} : unhex(line.substr(sp + 1));
  }
  return in;
}

struct Env3 { std::string p, a, r; };
struct Group { std::vector<Env3> envs; std::string body; };
struct Program { ByteVec policy_hash, schema_hash; size_t ne = 0, ns = 0, nr = 0; std::vector<Group> groups; };
static Program read_program(const std::string &path) {
  Program pr;
  std::ifstream f(path);
  std::string line;
  while (std::getline(f, line)) {
    std::istringstream is(line);
    std::string k;
    is >> k;
    if (k == "policy_hash") { std::string h; is >> h; pr.policy_hash = unhex(h); }
    else if (k == "schema_hash") { std::string h; is >> h; pr.schema_hash = unhex(h); }
    else if (k == "dims") is >> pr.ne >> pr.ns >> pr.nr;
    else if (k == "group") pr.groups.emplace_back();
    else if (k == "env") { Env3 e; is >> e.p >> e.a >> e.r; pr.groups.back().envs.push_back(e); }
    else if (k == "body") {
      std::string b;
      while (std::getline(f, line) && line != "end") b += line + "\n";
      pr.groups.back().body = b;
    }
  }
  return pr;
}

// A value of the generated code.
struct V {
  enum K { kBit, kWord, kArr, kStruct } k = kBit;
  std::optional<bool> c;  // A known scalar.
  BitE e;
  std::vector<V> items;
  std::map<std::string, V> f;
};

// One relation for one request shape.
struct Rel {
  HF hf;
  RSH rsh{hf};
  Sys cs{"zkpdp-policy"};
  PRef request_hash, policy_hash, schema_hash, root, context_hash, decision;
  CRef stype, sid, action, rtype, rid, nfield, pcopy, scopy;
  struct Opening { CRef uid, present, attrs, anc, path, sides, active; std::vector<CRef> left, right, next; };
  std::vector<Opening> ents;
  std::vector<CRef> sets, rec_present, rec_slots;
  CRef ctx_present, ctx_slots;
  std::unique_ptr<typename Sys::Compiled> comp;
  double compile_ms = 0;
  size_t functions = 0;
  Domain<HF> *H = nullptr;
  int counter = 0;
  std::string uniq(const std::string &s) { return s + "." + std::to_string(counter++); }

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

  // ---- values ----
  V konst(bool b) { V v; v.c = b; return v; }
  V bitv(BitE e) { V v; v.e = e; return v; }
  V word(BitE e) { V v; v.k = V::kWord; v.e = e; return v; }
  V bytes32(const ByteVec &b) { return word(BitE(kk::constant_input(*H, kk::constant_bytes(b)))); }
  V zero32() { return bytes32(ByteVec(32, 0)); }
  V arr_of(const CRef &r, size_t n) { V v; v.k = V::kArr; for (size_t i = 0; i < n; i++) v.items.push_back(word(BitE(r.at(i)))); return v; }
  V zero_arr(size_t n) { V v; v.k = V::kArr; for (size_t i = 0; i < n; i++) v.items.push_back(zero32()); return v; }

  V v_and(const V &a, const V &b) {
    if (a.c) return *a.c ? b : konst(false);
    if (b.c) return *b.c ? a : konst(false);
    return bitv(a.e & b.e);
  }
  V v_or(const V &a, const V &b) {
    if (a.c) return *a.c ? konst(true) : b;
    if (b.c) return *b.c ? konst(true) : a;
    return bitv(a.e | b.e);
  }
  V v_not(const V &a) { return a.c ? konst(!*a.c) : bitv(!a.e); }
  V v_eq(const V &a, const V &b) {
    if (a.k == V::kBit && b.k == V::kBit) {
      if (a.c && b.c) return konst(*a.c == *b.c);
      if (a.c) return *a.c ? b : v_not(b);
      if (b.c) return *b.c ? a : v_not(a);
      return bitv(!(a.e ^ b.e));
    }
    return bitv(!(a.e ^ b.e).any(0));
  }
  V v_if(const V &c, const V &t, const V &e) {
    if (c.c) return *c.c ? t : e;
    if (t.k == V::kBit) {
      if (t.c && e.c) return *t.c == *e.c ? t : (*t.c ? c : v_not(c));
      if (t.c) return *t.c ? v_or(c, e) : v_and(v_not(c), e);
      if (e.c) return *e.c ? v_or(v_not(c), t) : v_and(c, t);
      return bitv(c.e.mux(t.e, e.e));
    }
    if (t.k == V::kWord) return word(BitE(c.e.mux(t.e, e.e)));
    V o;
    o.k = t.k;
    if (t.k == V::kArr) for (size_t i = 0; i < t.items.size(); i++) o.items.push_back(v_if(c, t.items[i], e.items[i]));
    else for (const auto &[n, x] : t.f) o.f[n] = v_if(c, x, e.f.at(n));
    return o;
  }
  V in_set(const V &s, const V &x) {
    V found = konst(false);
    for (const auto &m : s.items) found = v_or(found, v_eq(m, x));
    return found;
  }

  // ---- the generated code's grammar ----
  struct Parser {
    Rel &r;
    std::map<std::string, V> &vars;
    std::string src;
    size_t i = 0;
    void ws() { while (i < src.size() && isspace(uchar(src[i]))) i++; }
    using uchar = unsigned char;
    bool at(const std::string &t) { ws(); return src.compare(i, t.size(), t) == 0; }
    bool eat(const std::string &t) { if (at(t)) { i += t.size(); return true; } return false; }
    void need(const std::string &t) { if (!eat(t)) fail("policy.ir: expected '" + t + "' at: " + src.substr(i, 40)); }
    std::string ident() {
      ws();
      size_t b = i;
      while (i < src.size() && (isalnum(uchar(src[i])) || src[i] == '_')) i++;
      return src.substr(b, i - b);
    }
    V expr() { return or_e(); }
    V or_e() { V a = and_e(); while (at("|")) { i++; a = r.v_or(a, and_e()); } return a; }
    V and_e() { V a = eq_e(); while (at("&")) { i++; a = r.v_and(a, eq_e()); } return a; }
    V eq_e() {
      V a = unary();
      if (at("==")) {
        i += 2;
        // SHA-256 of a UID is not all zeros, so an opened request entity is never absent.
        V b = unary();
        return r.v_eq(a, b);
      }
      return a;
    }
    V unary() { if (at("!") ) { i++; return r.v_not(unary()); } return postfix(); }
    V postfix() {
      V a = primary();
      for (;;) {
        if (at(".")) { i++; std::string n = ident(); V x = a.f.at(n); a = x; }
        else if (at("[")) {
          i++;
          size_t idx = std::stoul(ident());
          need("]");
          // A word indexed by byte: a boolean slot's last byte, `slot[31] == 1`.
          if (a.k == V::kWord) { a = r.word(BitE(a.e.slice(8 * idx, 8 * idx + 8))); continue; }
          V x = a.items.at(idx);
          a = x;
        }
        else return a;
      }
    }
    V call(const std::string &n) {
      std::vector<V> a;
      while (!at(")")) { a.push_back(expr()); eat(","); }
      need(")");
      if (n == "zero_entity") {
        V o; o.k = V::kStruct;
        o.f["attrs"] = r.zero_arr(kAttrs); o.f["ancestors"] = r.zero_arr(kAnc); o.f["uid_h"] = r.zero32();
        o.f["present"] = r.word(BitE(kk::constant_input(*r.H, kk::constant_bytes(ByteVec{0}))));
        return o;
      }
      if (n == "zero_rec") {
        V o; o.k = V::kStruct;
        o.f["slots"] = r.zero_arr(kAttrs);
        o.f["present"] = r.word(BitE(kk::constant_input(*r.H, kk::constant_bytes(ByteVec{0}))));
        return o;
      }
      if (n == "in_set") return r.in_set(a[0], a[1]);
      if (n == "intersects") {
        V found = r.konst(false);
        for (const auto &x : a[0].items) found = r.v_or(found, r.v_and(r.v_not(r.v_eq(x, r.zero32())), r.in_set(a[1], x)));
        return found;
      }
      if (n == "subset") {
        V all = r.konst(true);
        for (const auto &x : a[0].items) all = r.v_and(all, r.v_or(r.v_eq(x, r.zero32()), r.in_set(a[1], x)));
        return all;
      }
      if (n == "bit") return r.bitv(BitE(a[0].e.at(*bit_index)));
      if (n == "be_u64") {
        // Bytes 24..31 of the slot, big-endian, as a word least significant bit first.
        std::vector<size_t> order;
        for (size_t b = 0; b < 8; b++) for (size_t j = 0; j < 8; j++) order.push_back(8 * (31 - b) + j);
        return r.word(BitE(a[0].e[order]));
      }
      if (n == "slt" || n == "sle") {
        // Two's complement: compare with the sign bits exchanged.
        auto flip = [&](const BitE &w) -> BitE {
          std::vector<uint64_t> mask(64, 0);
          mask[63] = 1;
          return w ^ r.H->constant(Bit{}, mask, Shape{64});
        };
        auto x = flip(a[0].e).from_bits(), y = flip(a[1].e).from_bits();
        return r.bitv(n == "slt" ? BitE(x < y) : BitE(x <= y));
      }
      fail("policy.ir: unknown function " + n);
    }
    std::optional<size_t> bit_index;
    V primary() {
      if (eat("(")) {
        V a = expr();
        if (at("as")) {  // Only literals are cast.
          fail("policy.ir: unexpected cast");
        }
        need(")");
        return a;
      }
      if (at("[")) {
        i++;
        if (eat("ZERO")) { need(";"); ident(); need("]"); return r.zero_arr(kSet); }
        ws();
        if (isdigit(uchar(src[i]))) {
          ByteVec b;
          while (!at("]")) { b.push_back(uint8_t(std::stoi(ident()))); eat(","); }
          need("]");
          return r.bytes32(b);
        }
        V o; o.k = V::kArr;
        while (!at("]")) { o.items.push_back(expr()); eat(","); }
        need("]");
        return o;
      }
      ws();
      if (isdigit(uchar(src[i]))) {
        std::string lit = ident();
        if (!at("as")) {
          // A byte literal.
          return r.word(BitE(kk::constant_input(*r.H, kk::constant_bytes(ByteVec{uint8_t(std::stoi(lit))}))));
        }
        need("as"); ident();
        uint64_t x = std::stoull(lit);
        std::vector<uint64_t> w;
        for (int b = 0; b < 64; b++) w.push_back((x >> b) & 1);
        return r.word(BitE(r.H->constant(Bit{}, w, Shape{64})));
      }
      std::string n = ident();
      if (n == "if") {
        V c = expr(); need("{"); V t = expr(); need("}"); need("else"); need("{"); V e = expr(); need("}");
        return r.v_if(c, t, e);
      }
      if (n == "true") return r.konst(true);
      if (n == "false") return r.konst(false);
      if (n == "ZERO") return r.zero32();
      if (n == "bit") {
        // bit(present, i): the index is a literal.
        need("(");
        V p = expr(); need(",");
        size_t idx = std::stoul(ident());
        need(")");
        return r.bitv(BitE(p.e.at(idx)));
      }
      if (eat("(")) return call(n);
      auto it = vars.find(n);
      if (it == vars.end()) fail("policy.ir: unknown name " + n);
      return it->second;
    }
    // Statements, then the final expression.
    V run() {
      for (;;) {
        ws();
        if (eat("let ")) {
          std::string n = ident();
          need("=");
          // `let zK = uK == ZERO` where uK is the principal or the resource.
          vars[n] = expr();
          need(";");
        } else if (at("assert(")) {
          i += 7;
          V c = expr();
          need(")");
          need(";");
          if (c.c) { if (!*c.c) fail("policy.ir: an assertion that is always false"); }
          else assert_true(c.e, r.uniq("env.assert"));
        } else {
          return expr();
        }
      }
    }
  };

  explicit Rel(const Program &pr, const Input &in) {
    auto &D = cs.domain(examples::domain_profile<P>(hf, rsh), "H");
    H = &D;
    auto batch = sh::sha256_batch(cs[D], "sha");
    auto cb = [&](const std::string &s) { return kk::constant_bytes(str(s)); };
    auto constant = [&](const ByteVec &b) { return kk::constant_input(D, kk::constant_bytes(b)); };
    request_hash = D.pub(Bit{}, "request_hash", Shape{256});
    policy_hash = D.pub(Bit{}, "policy_hash", Shape{256});
    schema_hash = D.pub(Bit{}, "schema_hash", Shape{256});
    root = D.pub(Bit{}, "root", Shape{256});
    context_hash = D.pub(Bit{}, "context_hash", Shape{256});
    decision = D.pub(Bit{}, "decision", Shape{1});
    auto field = [&](const char *nm, size_t cap) { return D.priv(Bit{}, nm, Shape{1 + cap, 8}); };
    stype = field("stype", kCapType);
    sid = field("sid", kCapId);
    action = field("action", kCapAction);
    rtype = field("rtype", kCapType);
    rid = field("rid", kCapId);
    nfield = field("nfield", kCapNonce);
    pcopy = D.priv(Bit{}, "pcopy", Shape{256});
    scopy = D.priv(Bit{}, "scopy", Shape{256});
    assert_equal(pcopy, policy_hash, "policy.pub");
    assert_equal(pcopy, constant(pr.policy_hash), "policy_const");
    assert_equal(scopy, schema_hash, "schema.pub");
    assert_equal(scopy, constant(pr.schema_hash), "schema_const");
    auto rh = sh::sha256(batch, "req", kk::sponge_input(cb("zk-pdp-req-v1"), action, nfield, rid, rtype, sid, stype));
    assert_equal(rh, request_hash, "request");
    auto p_uid = sh::sha256(batch, "p_uid", kk::sponge_input(stype, sid));
    auto r_uid = sh::sha256(batch, "r_uid", kk::sponge_input(rtype, rid));

    std::map<std::string, V> shared;
    V ents_v, ok_v, sets_v, set_h_v, recs_v, rec_h_v;
    ents_v.k = ok_v.k = sets_v.k = set_h_v.k = recs_v.k = rec_h_v.k = V::kArr;
    auto digest = [&](const std::string &nm, auto msg) { return sh::sha256(batch, nm, msg); };
    for (size_t k = 0; k < pr.ne; k++) {
      Opening o;
      std::string nm = "e" + std::to_string(k);
      o.uid = D.priv(Bit{}, nm + ".uid", Shape{256});
      o.present = D.priv(Bit{}, nm + ".present", Shape{8});
      o.attrs = D.priv(Bit{}, nm + ".attrs", Shape{kAttrs, 256});
      o.anc = D.priv(Bit{}, nm + ".anc", Shape{kAnc, 256});
      o.path = D.priv(Bit{}, nm + ".path", Shape{kDepth, 256});
      o.sides = D.priv(Bit{}, nm + ".sides", Shape{kDepth});
      o.active = D.priv(Bit{}, nm + ".active", Shape{kDepth});
      auto ah = digest(nm + ".ah", kk::sponge_input(o.present, o.attrs));
      auto ch = digest(nm + ".ch", kk::sponge_input(o.anc));
      auto leaf = digest(nm + ".leaf", kk::sponge_input(o.uid, ah, ch));
      BitE h = BitE(leaf);
      for (size_t i = 0; i < kDepth; i++) {
        std::string lv = nm + ".l" + std::to_string(i);
        CRef l = D.priv(Bit{}, lv + ".left", Shape{256});
        CRef r = D.priv(Bit{}, lv + ".right", Shape{256});
        CRef n = D.priv(Bit{}, lv + ".next", Shape{256});
        auto side = o.sides.at(i);
        auto sib = o.path.at(i);
        assert_equal(l, side.mux(sib, h), lv + ".l");
        assert_equal(r, side.mux(h, sib), lv + ".r");
        auto d = digest(lv + ".node", kk::sponge_input(kk::constant_bytes(ByteVec{1}), l, r));
        assert_equal(n, o.active.at(i).mux(d, h), lv + ".n");
        if (i + 1 < kDepth) assert_true(!o.active.at(i + 1) | o.active.at(i), lv + ".a");
        o.left.push_back(l); o.right.push_back(r); o.next.push_back(n);
        h = BitE(n);
      }
      V st; st.k = V::kStruct;
      st.f["uid_h"] = word(BitE(o.uid));
      st.f["present"] = word(BitE(o.present));
      st.f["attrs"] = arr_of(o.attrs, kAttrs);
      st.f["ancestors"] = arr_of(o.anc, kAnc);
      ents_v.items.push_back(st);
      ok_v.items.push_back(bitv(!(h ^ root).any(0)));
      ents.push_back(o);
    }
    for (size_t j = 0; j < pr.ns; j++) {
      CRef s = D.priv(Bit{}, "set" + std::to_string(j), Shape{kSet, 256});
      sets.push_back(s);
      sets_v.items.push_back(arr_of(s, kSet));
      set_h_v.items.push_back(word(BitE(digest("set" + std::to_string(j) + ".h", kk::sponge_input(s)))));
    }
    auto rec = [&](const std::string &nm, CRef &present, CRef &slots) {
      present = D.priv(Bit{}, nm + ".present", Shape{8});
      slots = D.priv(Bit{}, nm + ".slots", Shape{kAttrs, 256});
      V st; st.k = V::kStruct;
      st.f["present"] = word(BitE(present));
      st.f["slots"] = arr_of(slots, kAttrs);
      return st;
    };
    for (size_t j = 0; j < pr.nr; j++) {
      CRef p, s;
      std::string nm = "rec" + std::to_string(j);
      V st = rec(nm, p, s);
      rec_present.push_back(p); rec_slots.push_back(s);
      recs_v.items.push_back(st);
      rec_h_v.items.push_back(word(BitE(digest(nm + ".h", kk::sponge_input(p, s)))));
    }
    V ctx_v = rec("ctx", ctx_present, ctx_slots);
    assert_equal(digest("ctx.h", kk::sponge_input(ctx_present, ctx_slots)), context_hash, "ctx.hash");

    // Selection by types and action. Environments this shape cannot match are left out.
    auto is_str = [&](const CRef &f, size_t cap, const std::string &s) -> BitE {
      return !(f.flat() ^ constant(fixed(s, cap))).any(0);
    };
    V dec = konst(false), any_sel = konst(false);
    for (const auto &g : pr.groups) {
      V sel = konst(false);
      for (const auto &e : g.envs) sel = v_or(sel, bitv(is_str(stype, kCapType, e.p) & is_str(action, kCapAction, e.a) & is_str(rtype, kCapType, e.r)));
      functions++;
      std::map<std::string, V> vars;
      vars["sel"] = sel;
      vars["p_uid"] = word(BitE(p_uid));
      vars["r_uid"] = word(BitE(r_uid));
      vars["ents"] = ents_v; vars["ent_ok"] = ok_v; vars["sets"] = sets_v; vars["set_h"] = set_h_v;
      vars["recs"] = recs_v; vars["rec_h"] = rec_h_v; vars["ctx"] = ctx_v;
      // A SHA-256 output is never the zero slot, so `uK == ZERO` for the request's own
      // entities is false, and the entity needs no multiplexer.
      std::string body = g.body;
      Parser ps{*this, vars, body};
      V d = run_body(ps, body, vars);
      dec = v_or(dec, v_and(sel, d));
      any_sel = v_or(any_sel, sel);
    }
    assert_true(any_sel.e, "fits_schema");
    if (dec.c) assert_true(*dec.c ? BitE(decision.at(0)) : BitE(!decision.at(0)), "decision");
    else assert_true(!(BitE(decision.at(0)) ^ dec.e), "decision");
    auto t = Clock::now();
    comp = compile(cs);
    compile_ms = ms(t);
  }

  // Folds `let zK = uK == ZERO` for uK bound to p_uid or r_uid.
  V run_body(Parser &ps, std::string &body, std::map<std::string, V> &vars) {
    std::istringstream is(body);
    std::string line, out;
    std::set<std::string> request_uids;
    while (std::getline(is, line)) {
      std::string t = line;
      t.erase(0, t.find_first_not_of(' '));
      if (t.rfind("let u", 0) == 0 && (t.find("= p_uid;") != std::string::npos || t.find("= r_uid;") != std::string::npos)) {
        request_uids.insert(t.substr(4, t.find(' ', 4) - 4));
      }
      if (t.rfind("let z", 0) == 0) {
        auto eq = t.find(" = ");
        std::string lhs = t.substr(4, eq - 4), rhs = t.substr(eq + 3, t.find(" ==") - eq - 3);
        if (request_uids.count(rhs)) { out += "let " + lhs + " = false;\n"; continue; }
      }
      out += line + "\n";
    }
    ps.src = out;
    ps.i = 0;
    return ps.run();
  }

  Sys::Assignment statement(const Input &in, bool flip = false) const {
    Sys::Assignment st;
    st.set(request_hash, bits(in["request_hash"]));
    st.set(policy_hash, bits(in["policy_hash"]));
    st.set(schema_hash, bits(in["schema_hash"]));
    st.set(root, bits(in["entities_root"]));
    st.set(context_hash, bits(in["context_hash"]));
    bool d = in["decision"][0] != 0;
    st.set(decision, flags({flip ? !d : d}));
    return st;
  }
  Sys::Assignment witness(Input in, const std::function<void(Input &)> &mut = {}) const {
    if (mut) mut(in);
    Sys::Assignment w;
    w.set(pcopy, bits(in["policy_hash"]));
    w.set(scopy, bits(in["schema_hash"]));
    w.set(stype, bits(fixed(in["subject_type"], kCapType)));
    w.set(sid, bits(fixed(in["subject_id"], kCapId)));
    w.set(action, bits(fixed(in["action"], kCapAction)));
    w.set(rtype, bits(fixed(in["resource_type"], kCapType)));
    w.set(rid, bits(fixed(in["resource_id"], kCapId)));
    w.set(nfield, bits(fixed(in["nonce"], kCapNonce)));
    for (size_t k = 0; k < ents.size(); k++) {
      const Opening &o = ents[k];
      std::string e = "ents." + std::to_string(k);
      w.set(o.uid, bits(in[e + ".uid_h"]));
      w.set(o.present, bits(in[e + ".present"]));
      w.set(o.attrs, bits(in[e + ".attrs"]));
      w.set(o.anc, bits(in[e + ".ancestors"]));
      w.set(o.path, bits(in[e + ".path"]));
      std::vector<bool> side, act;
      size_t len = in[e + ".path_len"][0];
      for (size_t i = 0; i < kDepth; i++) { side.push_back(in[e + ".sides"][i] != 0); act.push_back(i < len); }
      w.set(o.sides, flags(side));
      w.set(o.active, flags(act));
      ByteVec h = sha(cat({in[e + ".uid_h"], sha(cat({in[e + ".present"], in[e + ".attrs"]})), sha(in[e + ".ancestors"])}));
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
    }
    for (size_t j = 0; j < sets.size(); j++) w.set(sets[j], bits(in["sets." + std::to_string(j)]));
    for (size_t j = 0; j < rec_present.size(); j++) {
      w.set(rec_present[j], bits(in["recs." + std::to_string(j) + ".present"]));
      w.set(rec_slots[j], bits(in["recs." + std::to_string(j) + ".slots"]));
    }
    w.set(ctx_present, bits(in["ctx.present"]));
    w.set(ctx_slots, bits(in["ctx.slots"]));
    return w;
  }
};

static std::string shape_of(const Input &) { return "all"; }

int main(int argc, char **argv) {
  Program pr = read_program(argv[1]);
  std::string dir = argv[2];
  std::ofstream out(argv[3], std::ios::app);
  std::string set = argv[4], commit = argv[5];
  int repeats = argc > 6 ? atoi(argv[6]) : 1;
  const bool run_attacks = argc > 7 ? atoi(argv[7]) != 0 : true;  // off for browser timing
  std::vector<std::string> files;
  for (auto &e : std::filesystem::directory_iterator(dir)) files.push_back(e.path().string());
  std::sort(files.begin(), files.end());
  std::map<std::string, std::unique_ptr<Rel>> systems;
  SecureRandomEngine rng;
  auto line = [&](const std::string &req, const std::string &metric, double v, const std::string &extra = "") {
    out << "{\"experiment\":\"E11\",\"backend\":\"gordiehowe-interpreted\",\"commit\":\"" << commit << "\",\"set\":\"" << set << "\",\"request\":\"" << req
        << "\",\"metric\":\"" << metric << "\",\"value\":" << v << ",\"status\":\"ok\",\"notes\":\"" << extra << "\"}\n";
    out.flush();
  };
  int good = 0, bad = 0, attacks = 0, accepted = 0, threw = 0, rejected = 0, controls = 0;
  for (const auto &f : files) {
    Input in = read_input(f);
    std::string name = in.s("name");
    auto &t = systems[shape_of(in)];
    if (!t) {
      t = std::make_unique<Rel>(pr, in);
      printf("shape %s compile %.0f ms, %zu functions\n", shape_of(in).c_str(), t->compile_ms, t->functions);
      line(name, "compile_time_ms", t->compile_ms, "per request shape");
    }
    auto prep = t->comp->prepare(t->statement(in));
    std::vector<uint8_t> proof;
    double pm = 0, vm = 0;
    bool ok = false;
    for (int k = 0; k < repeats; k++) {
      auto t0 = Clock::now();
      proof = prep.prove(t->witness(in), rng);
      pm = ms(t0);
      t0 = Clock::now();
      ok = prep.verify(proof);
      vm = ms(t0);
      line(name, "prove_time_ms", pm); line(name, "verify_time_ms", vm); line(name, "proof_bytes", double(proof.size()));
    }
    printf("%-44s decision=%d prove %.0f ms verify %.1f ms proof %zu bytes ok=%d\n", name.c_str(), in["decision"][0], pm, vm, proof.size(), ok);
    ok ? good++ : bad++;
    typename Sys::ProveOptions opt;
    opt.require_satisfied = false;
    auto attack = [&](const std::string &an, const std::function<void(Input &)> &mut, bool flip) {
      if (!run_attacks) return;
      auto pr2 = t->comp->prepare(t->statement(in, flip));
      bool acc = false, th = false;
      try { acc = pr2.verify(pr2.prove(t->witness(in, mut), rng, opt)); } catch (...) { th = true; }
      if (an == "control_honest") { controls += acc; if (!acc) printf("  CONTROL FAILED\n"); return; }
      attacks++;
      th ? threw++ : (rejected += !acc);
      if (acc) { accepted++; printf("  ACCEPTED %s flip=%d\n", an.c_str(), flip); }
      out << "{\"experiment\":\"E9\",\"part\":\"gordiehowe-interpreted\",\"set\":\"" << set << "\",\"request\":\"" << name << "\",\"attack\":\"" << an << "\",\"flip\":" << flip
          << ",\"outcome\":\"" << (acc ? "accepted" : th ? "prover_threw" : "verify_rejected") << "\"}\n";
    };
    attack("control_honest", {}, false);
    attack("flip_decision", {}, true);
    // Forge facts of every opened slot, with the opposite decision and with the same one.
    for (size_t k = 0; k < pr.ne; k++) {
      std::string e = "ents." + std::to_string(k);
      for (bool flip : {false, true}) {
        for (size_t i = 0; i < kAttrs; i++) attack(e + ".attr" + std::to_string(i), [&, i](Input &x) { x.v[e + ".attrs"][32 * i + 31] ^= 1; }, flip);
        attack(e + ".anc", [&](Input &x) { x.v[e + ".ancestors"][32 * kAnc - 1] ^= 1; }, flip);
        attack(e + ".uid", [&](Input &x) { x.v[e + ".uid_h"][31] ^= 1; }, flip);
      }
    }
    for (size_t j = 0; j < pr.ns; j++) attack("set" + std::to_string(j), [&, j](Input &x) { x.v["sets." + std::to_string(j)][31] ^= 1; }, true);
    for (size_t j = 0; j < pr.nr; j++) attack("rec" + std::to_string(j), [&, j](Input &x) { x.v["recs." + std::to_string(j) + ".slots"][31] ^= 1; }, true);
    attack("ctx", [&](Input &x) { x.v["ctx.slots"][31] ^= 1; }, false);
    attack("ctx_flip", [&](Input &x) { x.v["ctx.slots"][31] ^= 1; }, true);
    attack("subject_id", [&](Input &x) { if (!x.v["subject_id"].empty()) x.v["subject_id"][0] ^= 1; }, false);
    out.flush();
  }
  printf("%s: requests ok %d bad %d, controls verified %d, attacks %d accepted %d, verifier rejected %d, prover threw %d, shapes %zu\n", set.c_str(), good, bad, controls, attacks, accepted, rejected, threw, systems.size());
  return bad ? 1 : 0;
}
