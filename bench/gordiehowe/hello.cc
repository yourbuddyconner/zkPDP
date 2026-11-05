// Smallest GordieHowe program, to prove the toolchain works on this machine:
// knowledge of a SHA-256 preimage.
#include <chrono>
#include <cstdio>
#include <memory>
#include <vector>
#include "gf2k/lch14_reed_solomon.h"
#include "gordiehowe/examples/sha256/sha256_preimage.h"
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
static double ms(Clock::time_point t) { return std::chrono::duration<double, std::milli>(Clock::now() - t).count(); }

int main() {
  HF hf;
  RSH rsh{hf};
  std::vector<uint8_t> msg(55, 7);
  uint8_t d[32];
  proofs::SHA256 h; h.Update(msg.data(), msg.size()); h.DigestData(d);
  auto bits = [&](const uint8_t *p, size_t n) {
    std::vector<HF::Elt> v;
    for (size_t i = 0; i < n; i++) for (int j = 0; j < 8; j++) v.push_back((p[i] >> j) & 1 ? hf.one() : hf.zero());
    return v;
  };
  Sys cs{"zkpdp-hello"};
  auto &H = cs.domain(examples::domain_profile<P>(hf, rsh), "H");
  auto r = examples::sha256_preimage::sha256_preimage(cs, H, msg.size(), lib::sha256::Sha256Mode::kBatched);
  auto t0 = Clock::now();
  auto comp = compile(cs);
  printf("compile %.1f ms\n", ms(t0));
  Sys::Assignment st; st.set(r.statement.digest, bits(d, 32));
  Sys::Assignment w; w.set(r.witness.message, bits(msg.data(), msg.size()));
  auto prep = comp->prepare(st);
  SecureRandomEngine rng;
  t0 = Clock::now();
  auto proof = prep.prove(w, rng);
  printf("prove %.1f ms\n", ms(t0));
  t0 = Clock::now();
  bool ok = prep.verify(proof);
  printf("verify %.1f ms ok=%d\n", ms(t0), ok);
  uint8_t e[32]; memcpy(e, d, 32); e[0] ^= 1;
  Sys::Assignment bad; bad.set(r.statement.digest, bits(e, 32));
  printf("wrong digest rejected=%d\n", !comp->prepare(bad).verify(proof));
  return ok ? 0 : 1;
}
