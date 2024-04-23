#![no_main]
#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::rc::Rc;
use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    embedded_alloc_example::init_global_allocator();

    let mut map = BTreeMap::new();
    map.insert("one", "1");
    map.insert("two", "2");
    match map.get("one") {
        Some(value) => defmt::println!("one is {}", value),
        None => defmt::println!("No value found"),
    }

    let rc = Rc::new(42);
    let rc2 = rc.clone();
    defmt::println!("{}", *rc2);

    let s = String::from("Hello");
    let message = s + " world!";
    defmt::println!("{}", message.as_str());

    embedded_alloc_example::exit();
}
