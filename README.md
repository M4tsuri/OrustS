# OrustS

An i386 operation system written in pure rust (for fun and no profit).

This operation system is under active developing.

## Checklist

- [ ] implement a bootloader
  - [x] build a staged bootloader demo
  - [ ] enable the A20 line
  - [x] setup GDT
  - [ ] load kernel image into ram
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
