#![no_std]
#![feature(abi_x86_interrupt)]

extern crate alloc;

pub mod vga_buffer;
pub mod interrupts;
pub mod memory;
pub mod allocator;

pub fn init() {
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
