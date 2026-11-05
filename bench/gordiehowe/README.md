# GordieHowe backend

TinyTodo as a hand-written GordieHowe relation. Experiment E8.

## Build on this machine

Upstream documents x86-64 Linux with Nix only. It also builds natively on Apple silicon with three local changes, in `local.patch`, against the commit in `UPSTREAM_COMMIT`. This build is not upstream's supported one.

- `packed_decimal` takes any integer vector, because `size_t` and `uint64_t` differ on macOS.
- The test framework is dropped from the precompiled header and link line.
- Two local executables are added to the CMake file.

```sh
git clone https://github.com/cmlsharp/gordiehowe && cd gordiehowe
git checkout "$(cat <repo>/bench/gordiehowe/UPSTREAM_COMMIT)"
git apply <repo>/bench/gordiehowe/local.patch
cmake -S lib -B build -G Ninja -DCMAKE_BUILD_TYPE=Release \
  -DCMAKE_CXX_COMPILER=/opt/homebrew/opt/llvm/bin/clang++ \
  -DCMAKE_C_COMPILER=/opt/homebrew/opt/llvm/bin/clang \
  -DCMAKE_CXX_FLAGS=-mcpu=apple-m1 -DOPENSSL_ROOT_DIR=/opt/homebrew/opt/openssl@3
ninja -C build gordiehowe/zkpdp_tinytodo
python3 <repo>/bench/gordiehowe/make_inputs.py <repo>/bench/circuit/tinytodo inputs
build/gordiehowe/zkpdp_tinytodo inputs out.jsonl <commit> 5
```

The CMake entries in the patch name `$ENV{HOME}/code/zk-pdp`. Change them if the repository lives elsewhere.
