# rust kernel

This is a kernel written in rust.

You will need "QEMU" to run this.

implemented :

- text printing
- colored text printing
- cpu interrupt
- keyword entry
- entry points
- backspace()
- paging

next :

1. memory allocation
2. vfs
3. ???

to run this :

- cargo build
- cargo bootimage
- qemu-system-x86_64 -drive format=raw,file=target/x86_64-my_os/debug/bootimage-my_os.bin



