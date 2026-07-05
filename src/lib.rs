#![no_std]
#![feature(abi_x86_interrupt)]

pub mod vga_buffer;
pub mod interrupts;

pub fn init() {
    interrupts::init_idt();
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}
