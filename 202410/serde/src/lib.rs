#![no_main]
#![no_std]

use cortex_m_semihosting::debug;
use defmt_rtt as _;
use nrf52840_hal as _;
use panic_probe as _;

#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

pub fn exit() -> ! {
    loop {
        debug::exit(debug::EXIT_SUCCESS);
    }
}

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, defmt::Format)]
struct Point {
    x: i32,
    y: i32,
}

use core::fmt::Write;
use heapless::String;

pub struct Number(f32);

impl serde::Serialize for Number {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut buf: String<32> = String::new();
        write!(buf, "{:.2}", self.0).unwrap();
        serializer.serialize_bytes(&buf.as_bytes())
    }
}

#[cfg(test)]
#[defmt_test::tests]
mod unit_tests {
    use super::*;
    use defmt::assert_eq;

    #[test]
    fn serialize_point() {
        let mut buf = [0u8; 32];
        let bytes = serde_json_core::to_slice(&Point{ x: 1, y: 2 }, &mut buf).unwrap();
        assert_eq!(bytes, 13);
        assert_eq!(core::str::from_utf8(&buf[..bytes]).unwrap(), "{\"x\":1,\"y\":2}");
    }

    #[test]
    fn serialize_point_to_string() {
        let json = serde_json_core::to_string::<_, 32>(&Point{ x: 1, y: 2 }).unwrap();
        assert_eq!(json.as_str(), "{\"x\":1,\"y\":2}");
    }

    #[test]
    fn deserialize_point() {
        let value = serde_json_core::from_str::<Point>("{\"x\":1,\"y\":2}").unwrap();
        assert_eq!(value, (Point{ x: 1, y: 2 }, 13));
    }

    #[test]
    fn custom_serialize() {
        let sd = serde_json_core::to_string::<_, 32>(&Number(1.55555)).unwrap();
        assert_eq!(sd.as_str(), "1.56");
    }
}
