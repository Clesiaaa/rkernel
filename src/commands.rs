use alloc::string::String;
use alloc::format;
use crate::println;
use crate::printc;
use crate::vga_buffer::Color;
use crate::vfs::{Vfs, VFS};
use crate::allocator::HEAP_SIZE;

pub struct Command {
    pub name: &'static str,
    pub help: &'static str,
    pub run: fn(&[&str], &mut String),
}

pub static COMMANDS: &[Command] = &[
    Command { name: "help", help: "Display this list of commands", run: cmd_help },
    Command { name: "echo", help: "echo <text>", run: cmd_echo },
    Command { name: "pwd", help: "Display the current directory", run: cmd_pwd },
    Command { name: "ls", help: "ls [path]", run: cmd_ls },
    Command { name: "cat", help: "cat <file>", run: cmd_cat },
    Command { name: "touch", help: "touch <file>", run: cmd_touch },
    Command { name: "mkdir", help: "mkdir <directory>", run: cmd_mkdir },
    Command { name: "rm", help: "rm <path>", run: cmd_rm },
    Command { name: "write", help: "write <file> <text>", run: cmd_write },
    Command { name: "clear", help: "Clear the screen", run: cmd_clear },
    Command { name: "fetch", help: "Display system information", run: cmd_fetch },
];

fn cmd_help(_args: &[&str], _cwd: &mut String) {
    println!("Available commands:");
    for command in COMMANDS {
        println!("  {:<10}{}", command.name, command.help);
    }
    println!("  {:<10}{}", "cd", "cd <path>");
}

fn cmd_echo(args: &[&str], _cwd: &mut String) {
    println!("{}", args.join(" "));
}

fn cmd_pwd(_args: &[&str], cwd: &mut String) {
    println!("{}", cwd);
}

fn cmd_ls(args: &[&str], cwd: &mut String) {
    let path = args.get(0).copied().unwrap_or(".");
    let target = Vfs::resolve(cwd, path);
    match VFS.lock().ls(&target) {
        Ok(entries) => {
            for entry in entries {
                println!("{}", entry);
            }
        }
        Err(e) => println!("ls: {}", e),
    }
}

fn cmd_cat(args: &[&str], cwd: &mut String) {
    let Some(path) = args.get(0).copied() else {
        println!("cat: missing path");
        return;
    };
    let target = Vfs::resolve(cwd, path);
    match VFS.lock().read(&target) {
        Ok(data) => match core::str::from_utf8(&data) {
            Ok(text) => println!("{}", text),
            Err(_) => println!("cat: binary content"),
        },
        Err(e) => println!("cat: {}", e),
    }
}

fn cmd_touch(args: &[&str], cwd: &mut String) {
    let Some(path) = args.get(0).copied() else {
        println!("touch: missing path");
        return;
    };
    let target = Vfs::resolve(cwd, path);
    if let Err(e) = VFS.lock().touch(&target) {
        println!("touch: {}", e);
    }
}

fn cmd_mkdir(args: &[&str], cwd: &mut String) {
    let Some(path) = args.get(0).copied() else {
        println!("mkdir: missing path");
        return;
    };
    let target = Vfs::resolve(cwd, path);
    if let Err(e) = VFS.lock().mkdir(&target) {
        println!("mkdir: {}", e);
    }
}

fn cmd_rm(args: &[&str], cwd: &mut String) {
    let Some(path) = args.get(0).copied() else {
        println!("rm: missing path");
        return;
    };
    let target = Vfs::resolve(cwd, path);
    if let Err(e) = VFS.lock().remove(&target) {
        println!("rm: {}", e);
    }
}

fn cmd_write(args: &[&str], cwd: &mut String) {
    let Some(path) = args.get(0).copied() else {
        println!("write: missing path");
        return;
    };
    let target = Vfs::resolve(cwd, path);
    let content = args[1..].join(" ");
    if let Err(e) = VFS.lock().write(&target, content.into_bytes()) {
        println!("write: {}", e);
    }
}

fn cmd_clear(_args: &[&str], _cwd: &mut String) {
    crate::vga_buffer::clear_screen();
}

fn cmd_fetch(_args: &[&str], cwd: &mut String) {
    let logo: [&str; 5] = ["#####", "#   #", "#####", "#  # ", "#   #"];
    let info: [String; 5] = [
        String::from("os: my_os"),
        String::from("arch: x86_64"),
        String::from("shell: my_os-shell"),
        format!("memory: {} KiB heap", HEAP_SIZE / 1024),
        format!("cwd: {}", cwd),
    ];

    for i in 0..5 {
        printc!(Color::LightCyan, "{}", logo[i]);
        println!("  {}", info[i]);
    }
}