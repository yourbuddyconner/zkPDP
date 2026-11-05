import { defineConfig } from "vite";

// SharedArrayBuffer, which the multi-threaded prover needs, requires these headers.
const isolate = { "Cross-Origin-Opener-Policy": "same-origin", "Cross-Origin-Embedder-Policy": "require-corp" };

export default defineConfig({
  build: { target: "esnext" },
  optimizeDeps: { esbuildOptions: { target: "esnext" }, exclude: ["@noir-lang/noirc_abi", "@noir-lang/acvm_js"] },
  server: { headers: isolate },
  preview: { headers: isolate },
});
