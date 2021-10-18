# Orusts_bootloader

A legacy BIOS bootloader for Orust System, which is a rust implementation of Orange's OS. 

This bootloader is written in **pure rust**, i.e. no assembly file and all low-level operations are implemented with rust inline assembly.

I tried my best to avoid magic numbers and add documentation for them when nessessary. Many data structures, for example, GDT, are generated at compile-time to avoid runtime cost (with nice abstraction).

Some ideas come from https://github.com/o8vm/krabs.

## Dependency

This crate works with the newest Rust Nightly:

```
$ cargo --version --verbose
1.55.0-nightly (cebef2951 2021-07-22)
release: 1.55.0
commit-hash: cebef2951ee69617852844894164b54ed478a7da
commit-date: 2021-07-22
```

with `rust-src` and `llvm-tools-preview` installed.

## Usage

Run `cargo run`, then you will find the raw bootloader image at `target/bootloader.bin`. 

- Run: `qemu-system-i386 -drive format=raw,index=0,media=disk,file=target/bootloader.bin -vga std`.
- Debug: I recommend debug with gdb. Run `qemu-system-i386 -drive format=raw,index=0,media=disk,file=target/bootloader.bin -vga std -s -S` and attach gdb to it. Note if you want to debug real mode 8086 assembly, you need to add a description file with `set tdesc filename target.xml`.
