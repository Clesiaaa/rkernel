#![no_std]
#![no_main]
 
use my_os::vga_buffer;
use my_os::println;
use my_os::printc;
use my_os::print;
use core::panic::PanicInfo;
 
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
 
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    
    printc!(vga_buffer::Color::Brown, "R");
    printc!(vga_buffer::Color::LightGray, "kernel\n"); 
    
    my_os::init();
 
    loop {}
}
