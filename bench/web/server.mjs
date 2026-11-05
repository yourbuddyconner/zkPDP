// Serve the built page with the headers the multi-threaded prover needs, and
// append posted measurements to results/ as JSON Lines.
//
//   node server.mjs <results file> [port]
//
// Plain HTTP on localhost. HTTPS with a self-signed certificate on port+1 for
// phones on the same network: a page only gets threads in a secure context.
import { createServer } from "node:http";
import { createServer as createTls } from "node:https";
import { execSync } from "node:child_process";
import { appendFileSync, existsSync, readFileSync, statSync } from "node:fs";
import { extname, join, normalize } from "node:path";
import { fileURLToPath } from "node:url";

const here = fileURLToPath(new URL(".", import.meta.url));
const dist = join(here, "dist");
const out = process.argv[2];
const port = Number(process.argv[3] || 8787);
const repo = join(here, "..", "..");
const sh = (c) => { try { return execSync(c, { cwd: repo }).toString().trim(); } catch { return ""; } };
const commit = sh("git rev-parse HEAD");
const types = { ".html": "text/html", ".js": "text/javascript", ".json": "application/json", ".wasm": "application/wasm", ".css": "text/css", ".gz": "application/gzip" };

function handle(req, res) {
  res.setHeader("Cross-Origin-Opener-Policy", "same-origin");
  res.setHeader("Cross-Origin-Embedder-Policy", "require-corp");
  if (req.method === "POST" && req.url === "/result") {
    let body = "";
    req.on("data", (c) => (body += c));
    req.on("end", () => {
      try {
        const r = JSON.parse(body);
        const line = {
          experiment: r.experiment, backend: "circuit-noir-browser", commit,
          timestamp: new Date().toISOString().replace(/\.\d+Z$/, "Z"),
          machine: { device: r.device || "", user_agent: r.env.user_agent, hardware_concurrency: r.env.hardware_concurrency, device_memory_gb: r.env.device_memory_gb },
          toolchain: { noir_js: "1.0.0", bb_js: "6.0.0", threads: r.env.threads, cross_origin_isolated: r.env.cross_origin_isolated, secure: !!req.socket.encrypted },
          set: r.set, circuit: r.label, request: r.request,
          inputs: { policy_hash: r.policy_hash, entity_commitment: r.entities, request_hash: r.request_hash },
          metric: r.metric, value: r.value, repeat: r.repeat, status: r.status, notes: r.notes || "",
        };
        appendFileSync(out, JSON.stringify(line) + "\n");
        res.writeHead(204).end();
      } catch (e) {
        res.writeHead(400).end(String(e));
      }
    });
    return;
  }
  let p = normalize(decodeURIComponent(req.url.split("?")[0]));
  if (p.endsWith("/")) p += "index.html";
  const f = join(dist, p);
  if (!f.startsWith(dist) || !existsSync(f) || !statSync(f).isFile()) return res.writeHead(404).end("not found");
  res.writeHead(200, { "content-type": types[extname(f)] || "application/octet-stream" });
  res.end(readFileSync(f));
}

createServer(handle).listen(port, "0.0.0.0", () => console.log(`http://localhost:${port}`));
const key = join(here, "cert", "key.pem"), cert = join(here, "cert", "cert.pem");
if (existsSync(key) && existsSync(cert)) {
  createTls({ key: readFileSync(key), cert: readFileSync(cert) }, handle).listen(port + 1, "0.0.0.0", () => console.log(`https://<this machine's address>:${port + 1}`));
}
