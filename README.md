# rust kernel

This is a simple kernel written in rust.
It display "Hello World!".

You will need "QEMU" to run this.

to run this :

- cargo build
- cargo bootimage
- qemu-system-x86_64 -drive format=raw,file=target/x86_64-my_os/debug/bootimage-my_os.bin



