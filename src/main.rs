#![no_std]
#![no_main]

use my_os::println;
use my_os::printc;
use my_os::vga_buffer::Color;
use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};

const LETTER_R: [&str; 5] = ["#### ", "#   #", "#### ", "#  # ", "#   #"];
const LETTER_K: [&str; 5] = ["#   #", "#  # ", "###  ", "#  # ", "#   #"];
const LETTER_E: [&str; 5] = ["#####", "#    ", "###  ", "#    ", "#####"];
const LETTER_N: [&str; 5] = ["#   #", "##  #", "# # #", "#  ##", "#   #"];
const LETTER_L: [&str; 5] = ["#    ", "#    ", "#    ", "#    ", "#####"];

const BANNER: [[&str; 5]; 7] = [
    LETTER_R, LETTER_K, LETTER_E, LETTER_R, LETTER_N, LETTER_E, LETTER_L,
];

const BANNER_COLORS: [Color; 5] = [
    Color::LightRed,
    Color::Yellow,
    Color::LightGreen,
    Color::LightCyan,
    Color::LightBlue,
];

fn print_banner() {
    for row in 0..5 {
        for letter in BANNER.iter() {
            printc!(BANNER_COLORS[row], "{} ", letter[row]);
        }
        println!();
    }
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use my_os::memory;
    use my_os::memory::BootInfoFrameAllocator;
    use my_os::allocator;
    use x86_64::VirtAddr;

    println!("starting...");

    my_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    print_banner();
    my_os::shell::SHELL.lock().prompt();

    loop {
        x86_64::instructions::hlt();
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}