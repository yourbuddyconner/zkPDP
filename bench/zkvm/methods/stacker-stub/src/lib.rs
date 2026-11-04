//! Stand-in for `stacker` inside the RISC Zero guest.
//!
//! The real crate does not build for `riscv32im-risc0-zkvm-elf`: it needs
//! `mmap`, `mprotect` and `sysconf`. Cedar calls only `remaining_stack`, as a
//! guard against deep recursion. Here it reports "unknown", which Cedar treats
//! as "enough". A policy nested deep enough to exhaust the guest stack then
//! aborts the guest, and an aborted guest produces no proof.

pub fn remaining_stack() -> Option<usize> {
    None
}

pub fn maybe_grow<R, F: FnOnce() -> R>(_red_zone: usize, _stack_size: usize, callback: F) -> R {
    callback()
}

pub fn grow<R, F: FnOnce() -> R>(_stack_size: usize, callback: F) -> R {
    callback()
}
