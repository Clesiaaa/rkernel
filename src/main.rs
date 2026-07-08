#![no_std]
#![no_main]

use my_os::vga_buffer;
use my_os::println;
use my_os::printc;
use my_os::print;
use core::panic::PanicInfo;

use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use my_os::memory;
    use my_os::memory::BootInfoFrameAllocator;
    use x86_64::{structures::paging::Page, VirtAddr};

    println!("starting...");

    my_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    println!("Pagination succes");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
