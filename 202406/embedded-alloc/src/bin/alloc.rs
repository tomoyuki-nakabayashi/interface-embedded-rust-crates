#![no_main]
#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    embedded_alloc_example::init_global_allocator();

    let mut xs = Vec::new();
    xs.push(1);
    defmt::println!("{}", xs[0]);

    embedded_alloc_example::exit();
}
