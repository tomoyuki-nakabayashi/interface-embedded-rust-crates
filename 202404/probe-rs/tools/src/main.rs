#![no_main]
#![no_std]

use probe_rs_example as _; // global logger + panicking-behavior + memory layout

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    defmt::trace!("trace");
    defmt::debug!("debug");
    defmt::info!("info");
    defmt::warn!("warn");
    defmt::error!("error");

    let x = 42;
    defmt::println!("x = {}", x);

    probe_rs_example::exit()
}
