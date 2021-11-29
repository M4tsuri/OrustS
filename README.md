# OrustS

An i386 operation system written in pure rust (for fun and no profit).

This operation system is under active developing.

## Usage

- `cargo kbuild build` to build
- `cargo kbuild run` to run it with qemu
- `cargo kbuild debug` to wait for gdb attach on port 1234

## Checklist

- [ ] implement a bootloader
  - [x] build a staged bootloader demo
  - [x] enable the A20 line
  - [x] setup GDT
  - [x] load kernel image into ram
  - [x] transfer into protect mode
  - [ ] [optional] setup page table
  - [ ] transfer control to kernel
- [ ] implement a simple kernel
  - [ ] setup a larger GDT
  - [ ] [optional] add support for paging
  - [ ] setup IDT
  - [ ] add support for multitasking
- [ ] implement advanced features
  - [ ] add support for user mode applications
  - [ ] add support for filesystem 
