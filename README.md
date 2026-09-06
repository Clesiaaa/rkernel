# Rkernel

Minimal x86_64 kernel written in Rust, built on the bootloader crate.

## Features

- VGA text mode driver, colored output
- Timer and keyboard interrupts (PIC 8259)
- Paging and heap allocator
- In-memory virtual file system
- Simple shell

## Run

qemu-system-x86_64 -drive format=raw,file=target/x86_64-my_os/debug/bootimage-my_os.bin

or you can use :

cargo run

## Shell commands

help                   list commands
echo <text>            print text
pwd                    print current directory
cd <path>              change directory
ls [path]              list directory
cat <file>             print file contents
touch <file>           create empty file
mkdir <dir>            create directory
rm <path>              remove file or directory
write <file> <text>    write text to file
clear                  clear the screen
fetch                  display system information


## Nota Bene

Maybe in the future I will implement a text editor, and a tiny compiler. Perhaps, I think it's more than enough for a first
Operating System. I would also to build my own bootloader, and have control on every aspect and build on top of it something more
usable.

Hope you enjoy my work!