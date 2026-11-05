const q = new URLSearchParams(self.location.search);
const set = q.get("set"), repeats = q.get("repeats"), commit = q.get("commit") || "unknown";
const say = (line) => postMessage({ line });
const finish = async (code) => {
  const mem = Module.HEAPU8 ? Module.HEAPU8.length : (self.wasmMemory ? self.wasmMemory.buffer.byteLength : 0);
  say(`exit ${code}, wasm memory ${(mem / 2 ** 20).toFixed(0)} MiB`);
  const text = new TextDecoder().decode(Module.FS.readFile("/out.jsonl"));
  const meta = JSON.stringify({ meta: true, set, code, wasm_memory_bytes: mem, user_agent: navigator.userAgent, threads: 1, commit }) + "\n";
  await fetch("/raw", { method: "POST", body: meta + text });
  postMessage({ done: true });
};
self.Module = {
  arguments: [`/data/${set}/policy.ir`, `/data/${set}/inputs`, "/out.jsonl", set, commit, repeats, "0"],
  print: say, printErr: (l) => { if (!/^\[INFO|^GH_/.test(l)) say(l); },
  onExit: (code) => { finish(code); },
  onAbort: (r) => { say("ABORT " + r); postMessage({ done: true }); },
};
importScripts("zkpdp_policy.js");
