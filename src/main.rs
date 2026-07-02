#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;



/// This funciton is called on panic situations.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    
    //vga_buffer::println("Hello");
    //vga_buffer::println("World");
    
    println!("Hello World{}", "!");

    loop {}
}
