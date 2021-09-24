# Orusts_bootloader

A legacy BIOS bootloader for Orust System, which is a rust implementation of Orange's OS. 

This bootloader is written in **pure rust**, i.e. no assembly file and all low-level operations are implemented with rust inline assembly.

I tried my best to avoid magic numbers and add documentation for any confusing configuration. Compile-time generated data structures are used as many as possible so that you don't need to deal with annoying bitwise operations without the cost of runtime space and time.

Some code are copied from https://github.com/o8vm/krabs.

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

Run `cargo run`, then you will find the raw bootloader image under `target/bootloader.bin`. Run it with `qemu-system-i386 -drive format=raw,index=0,media=disk,file=target/bootloader.bin -vga std`.

If you want to debug, I recommend debug with gdb. Run `qemu-system-i386 -drive format=raw,index=0,media=disk,file=target/bootloader.bin -vga std -s -S` and `gdb` in **current** directory, then our `.gdbinit` script will automatically switch gdb to i8086 mode for real mode debugging and set a breakpoint at the entrypoint. After entering protect mode, you may want to run `set architecture i386` to go to i386 mode.
