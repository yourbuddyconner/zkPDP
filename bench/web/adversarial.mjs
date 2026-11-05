// E9, part A: input-level attacks on a generated circuit.
//
//   node adversarial.mjs <circuit dir> <out.jsonl> <set> [max requests] [workers]
//
// Each attack changes the private or public inputs of an honest request, then
// runs the witness solver twice: with the oracle's decision and with the
// opposite one. The circuit must never accept the opposite decision. It may
// accept the oracle's decision only if the changed value has no effect on the
// statement, for example an entity slot this request's function does not read.
//
// The solver runs unconstrained helpers honestly, so this cannot show a
// malicious helper output is rejected. Part B, adversarial_witness.py, covers
// that at the witness level.
import { Noir } from "@noir-lang/noir_js";
import { createHash } from "node:crypto";
import { fork } from "node:child_process";
import { appendFileSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { fileURLToPath } from "node:url";

const clone = (x) => JSON.parse(JSON.stringify(x));
const flip = (bytes, i = 31) => { bytes[i] ^= 1; };
const sha = (s) => [...createHash("sha256").update(s).digest()];
const str = (bv) => Buffer.from(bv.storage.slice(0, bv.len)).toString();

// The attacks, each a name and a function that mutates a copy of the input.
function attacks(input, others) {
  const out = [];
  const add = (name, f) => out.push({ name, f });
  const pUid = sha(`${str(input.subject_type)}::"${str(input.subject_id)}"`);

  input.ents.forEach((e, k) => {
    e.attrs.forEach((_, i) => {
      add(`ent${k}.attr${i}.flip`, (x) => flip(x.ents[k].attrs[i]));
      add(`ent${k}.attr${i}.forge_principal`, (x) => { x.ents[k].attrs[i] = pUid; });
    });
    for (let b = 0; b < 8; b++) add(`ent${k}.present.bit${b}`, (x) => { x.ents[k].present ^= 1 << b; });
    add(`ent${k}.uid.flip`, (x) => flip(x.ents[k].uid_h));
    add(`ent${k}.path0.flip`, (x) => flip(x.ents[k].path[0]));
    add(`ent${k}.side0.toggle`, (x) => { x.ents[k].sibling_is_left[0] = !x.ents[k].sibling_is_left[0]; });
    add(`ent${k}.pathlen.zero`, (x) => { x.ents[k].path_len = 0; });
    add(`ent${k}.pathlen.minus1`, (x) => { x.ents[k].path_len = Math.max(0, x.ents[k].path_len - 1); });
    add(`ent${k}.pathlen.plus1`, (x) => { x.ents[k].path_len += 1; });
    add(`ent${k}.pathlen.over`, (x) => { x.ents[k].path_len = 13; });
    add(`ent${k}.zeroed`, (x) => {
      const z = x.ents[k];
      z.uid_h = z.uid_h.map(() => 0); z.present = 0; z.path_len = 0;
      z.attrs = z.attrs.map((a) => a.map(() => 0));
      z.ancestors = z.ancestors.map((a) => a.map(() => 0));
    });
    // Claim membership of a group by writing another record's attribute into the ancestor slots.
    input.ents.forEach((o, m) => o.attrs.forEach((slot, i) => {
      if (slot.some((b) => b)) {
        add(`ent${k}.anc.forge_from_ent${m}.attr${i}`, (x) => {
          const free = x.ents[k].ancestors.findIndex((a) => a.every((b) => b === 0));
          x.ents[k].ancestors[free < 0 ? 0 : free] = clone(slot);
        });
      }
    }));
    // A valid opening of a different entity from the same store.
    others.forEach((other, n) => other.ents.forEach((oe, m) => {
      if (oe.uid_h.some((b) => b) && oe.uid_h.join() !== e.uid_h.join() && m === 0 && n < 3) {
        add(`ent${k}.swap_with_other_opening${n}`, (x) => { x.ents[k] = clone(oe); });
      }
    }));
  });
  (input.sets || []).forEach((set, j) => {
    add(`set${j}.elem0.flip`, (x) => flip(x.sets[j][0]));
    add(`set${j}.add_principal`, (x) => {
      const free = x.sets[j].findIndex((a) => a.every((b) => b === 0));
      x.sets[j][free < 0 ? 0 : free] = pUid;
    });
    add(`set${j}.emptied`, (x) => { x.sets[j] = x.sets[j].map((a) => a.map(() => 0)); });
  });
  (input.recs || []).forEach((_, j) => {
    add(`rec${j}.slot0.flip`, (x) => flip(x.recs[j].slots[0]));
    add(`rec${j}.present.flip`, (x) => { x.recs[j].present ^= 1; });
    add(`rec${j}.emptied`, (x) => { x.recs[j].present = 0; x.recs[j].slots = x.recs[j].slots.map((a) => a.map(() => 0)); });
  });
  if (input.ctx) {
    input.ctx.slots.forEach((_, i) => add(`ctx.slot${i}.flip`, (x) => flip(x.ctx.slots[i])));
    add("ctx.present.flip", (x) => { x.ctx.present ^= 1; });
    add("ctx.emptied", (x) => { x.ctx.present = 0; x.ctx.slots = x.ctx.slots.map((a) => a.map(() => 0)); });
    add("context_hash.flip", (x) => flip(x.context_hash));
  }
  for (const f of ["subject_type", "subject_id", "action", "resource_type", "resource_id", "nonce"]) {
    add(`${f}.flip`, (x) => { x[f].storage[0] ^= 1; });
    if (input[f].len > 0) add(`${f}.shorter`, (x) => { x[f].storage[x[f].len - 1] = 0; x[f].len -= 1; });
    add(`${f}.padding_nonzero`, (x) => { x[f].storage[x[f].storage.length - 1] = 65; });
    add(`${f}.not_plain`, (x) => { x[f].storage[0] = 0x22; });
  }
  for (const f of ["request_hash", "policy_hash", "schema_hash", "entities_root"]) add(`${f}.flip`, (x) => flip(x[f]));
  return out;
}

async function solve(noir, input) {
  try {
    await noir.execute(input);
    return { accepted: true };
  } catch (e) {
    return { accepted: false, why: String(e.message || e).slice(0, 60) };
  }
}

async function worker() {
  const { dir, reqs } = JSON.parse(process.argv[3]);
  const noir = new Noir(JSON.parse(readFileSync(join(dir, "target/policy.json"))));
  const inputs = reqs.map((r) => JSON.parse(readFileSync(join(dir, "inputs", `${r.n}.json`))));
  for (let idx = 0; idx < reqs.length; idx++) {
    if (idx % Number(process.argv[5]) !== Number(process.argv[4])) continue;
    const input = inputs[idx];
    const honest = await solve(noir, input);
    process.send({ baseline: true, request: reqs[idx].name, ok: honest.accepted, why: honest.why });
    for (const { name, f } of attacks(input, inputs.filter((_, i) => i !== idx))) {
      const same = clone(input); f(same);
      const opposite = clone(same); opposite.decision = !opposite.decision;
      const s = await solve(noir, same);
      const o = s.accepted ? { accepted: false, why: "skipped: solver is deterministic" } : await solve(noir, opposite);
      process.send({ request: reqs[idx].name, decision: input.decision, attack: name, same: s.accepted, opposite: o.accepted, why: s.why });
    }
  }
}

async function main() {
  const [dir, outPath, set, maxReq = "8", workers = "8"] = process.argv.slice(2);
  const idx = JSON.parse(readFileSync(join(dir, "inputs/index.json")));
  // Keep permits and denies balanced when sampling.
  const all = idx.requests.filter((r) => !r.skipped);
  const permit = all.filter((r) => r.decision), deny = all.filter((r) => !r.decision);
  const reqs = [];
  while (reqs.length < Number(maxReq) && (permit.length || deny.length)) {
    if (permit.length) reqs.push(permit.shift());
    if (deny.length && reqs.length < Number(maxReq)) reqs.push(deny.shift());
  }
  const me = fileURLToPath(import.meta.url);
  const totals = { attacks: 0, soundness: 0, free: 0 };
  const free = new Map();
  await Promise.all(Array.from({ length: Number(workers) }, (_, w) => new Promise((done, fail) => {
    const c = fork(me, ["worker", JSON.stringify({ dir, reqs }), String(w), workers]);
    c.on("message", (m) => {
      if (m.baseline) {
        if (!m.ok) { console.error("HONEST INPUT REJECTED", m.request, m.why); process.exitCode = 1; }
        return;
      }
      totals.attacks++;
      const outcome = m.opposite ? "SOUNDNESS_FAILURE" : m.same ? "accepted_no_effect" : "rejected";
      if (m.opposite) { totals.soundness++; console.error("SOUNDNESS FAILURE", m.request, m.attack); }
      if (m.same) { totals.free++; const k = m.attack.replace(/\d+/g, "N"); free.set(k, (free.get(k) || 0) + 1); }
      appendFileSync(outPath, JSON.stringify({ experiment: "E9", part: "A", set, request: m.request, decision: m.decision, attack: m.attack, outcome, why: m.why || "" }) + "\n");
    });
    c.on("exit", (code) => (code ? fail(new Error("worker " + code)) : done()));
  })));
  console.log(set, "requests", reqs.length, JSON.stringify(totals));
  console.log("accepted with the same decision, by class:", JSON.stringify([...free.entries()].sort()));
}

if (process.argv[2] === "worker") worker().then(() => process.exit(0)); else main();
