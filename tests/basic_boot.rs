#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(my_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use my_os::{println, print, printc};
use my_os::vga_buffer;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    my_os::init();
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion...");
    assert_eq!(1, 1);
    print!("[ok]");
}

#[test_case]
fn color_green() {
    printc!(vga_buffer::Color::Green, "this text is green...");
    assert_eq!(1, 1);
    print!("[ok]\n");
}

#[test_case]
fn color_cyan() {
    printc!(vga_buffer::Color::Cyan, "this text is cyan...");
    assert_eq!(1, 1);
    print!("[ok]\n");
}

#[test_case]
fn interrupts() {
    x86_64::instructions::interrupts::int3();
    assert_eq!(1, 1);
    print!("[ok]\n");
}
