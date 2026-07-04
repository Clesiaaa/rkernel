#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
 
mod vga_buffer;
 
use core::panic::PanicInfo;
 
/// This function is called on panic situations.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
 
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    
    printc!(vga_buffer::Color::Pink, "Hello World!\n");
    printc!(vga_buffer::Color::Green, "Hello World!\n"); 
    
    #[cfg(test)]
    test_main();
 
    loop {}
}
 
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests \n", tests.len());
    for test in tests {
        test();
    }
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
