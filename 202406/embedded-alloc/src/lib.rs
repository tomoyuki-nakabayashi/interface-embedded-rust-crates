#![no_std]
#![no_main]

extern crate alloc;

use embedded_alloc::Heap;
use defmt_rtt as _; // global logger
use nrf52840_hal as _; // memory layout
use panic_probe as _;
use cortex_m_semihosting::debug;

#[global_allocator]
static HEAP: Heap = Heap::empty();

pub fn init_global_allocator() {
    use core::mem::MaybeUninit;
    const HEAP_SIZE: usize = 1024;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
}

pub fn exit() -> ! {
    loop {
        debug::exit(debug::EXIT_SUCCESS);
    }
}

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}
