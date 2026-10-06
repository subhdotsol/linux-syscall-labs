# no\_std Rust Linux Syscall Experiment

 A small Rust experiment to understand how a program can interact with the Linux kernel without using the Rust standard library.

 The project uses:

 - Rust `no_std`
- AArch64 Linux
- Direct Linux syscalls
- Inline assembly
- A custom `_start` entry point

 Currently, the program writes a message to stdout and then exits by directly invoking the Linux `write` and `exit` syscalls.

 The syscall interface is kept separate so I can experiment with more Linux syscalls later.

 This is purely a learning project to explore what happens underneath the usual Rust standard library and runtime abstractions.

 ## Goal

 The goal is simply to have fun experimenting with low-level Rust, Linux syscalls, assembly, and the kernel interface.

 More experiments will be added as I learn.

 ## Useful reference 
 For looking up Linux syscalls, I'm using Filippo's Linux syscall table:
 
## Useful Reference

[Linux Syscall Table — Filippo Valsorda](https://filippo.io/linux-syscall-table/)
 
 It provides a searchable list of Linux syscalls and their numbers across architectures.
