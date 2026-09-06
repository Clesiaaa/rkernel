use alloc::string::{String, ToString};
use alloc::vec::Vec;
use spin::Mutex;
use lazy_static::lazy_static;
use pc_keyboard::DecodedKey;
use crate::{print, println};
use crate::commands::COMMANDS;
use crate::vfs::{Vfs, VFS};

pub struct Shell {
    buffer: String,
    cwd: String,
}

impl Shell {
    pub fn new() -> Self {
        Shell { buffer: String::new(), cwd: String::from("/") }
    }

    pub fn prompt(&self) {
        print!("{}$ ", self.cwd);
    }

    pub fn on_key(&mut self, key: DecodedKey) {
        match key {
            DecodedKey::Unicode('\n') => {
                println!();
                self.execute();
                self.buffer.clear();
                self.prompt();
            }
            DecodedKey::Unicode('\u{8}') => {
                if self.buffer.pop().is_some() {
                    crate::vga_buffer::backspace();
                }
            }
            DecodedKey::Unicode(c) => {
                self.buffer.push(c);
                print!("{}", c);
            }
            DecodedKey::RawKey(_) => {}
        }
    }

    fn execute(&mut self) {
        let line = self.buffer.trim().to_string();
        if line.is_empty() {
            return;
        }

        let mut parts = line.split_whitespace();
        let name = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        if name == "cd" {
            self.cmd_cd(&args);
            return;
        }

        for command in COMMANDS {
            if command.name == name {
                (command.run)(&args, &mut self.cwd);
                return;
            }
        }

        println!("Unknown command: {}", name);
    }

    fn cmd_cd(&mut self, args: &[&str]) {
        let path = args.get(0).copied().unwrap_or("/");
        let target = Vfs::resolve(&self.cwd, path);
        if VFS.lock().is_dir(&target) {
            self.cwd = target;
        } else {
            println!("cd: directory not found: {}", target);
        }
    }
}

lazy_static! {
    pub static ref SHELL: Mutex<Shell> = Mutex::new(Shell::new());
}
