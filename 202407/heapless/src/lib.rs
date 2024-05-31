#![no_main]
#![no_std]
#![allow(dead_code)]

use cortex_m_semihosting::debug;
use defmt_rtt as _; // global logger
use nrf52840_hal as _; // memory layout
use panic_probe as _;

// same panicking *behavior* as `panic-probe` but doesn't print a panic message
// this prevents the panic message being printed *twice* when `defmt::panic` is invoked
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}

/// Terminates the application and makes a semihosting-capable debug tool exit
/// with status code 0.
pub fn exit() -> ! {
    loop {
        debug::exit(debug::EXIT_SUCCESS);
    }
}

/// Hardfault handler.
///
/// Terminates the application and makes a semihosting-capable debug tool exit
/// with an error. This seems better than the default, which is to spin in a
/// loop.
#[cortex_m_rt::exception]
unsafe fn HardFault(_frame: &cortex_m_rt::ExceptionFrame) -> ! {
    loop {
        debug::exit(debug::EXIT_FAILURE);
    }
}

fn values<T, const N: usize>(default_value: T) -> heapless::Vec::<T, N>
    where T: Copy + core::fmt::Debug,
{
    let mut xs = heapless::Vec::<T, N>::new();
    for _ in 0..N {
        xs.push(default_value).unwrap();
    }
    xs
}

#[cfg(test)]
#[defmt_test::tests]
mod unit_tests {
    use super::*;
    use defmt::assert_eq;
    use heapless::Vec;

    #[test]
    fn test_heapless_vec() -> Result<(), usize> {
        let mut xs = Vec::<usize, 8>::new();
        xs.push(0)?;
        xs.push(1)?;

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 0);
        assert_eq!(xs[1], 1);

        assert_eq!(xs.pop(), Some(1));
        assert_eq!(xs.pop(), Some(0));
        assert_eq!(xs.len(), 0);

        for i  in 0..xs.capacity() {
            xs.push(i)?;
        }

        // キャパシティを使い切っているときは、エラーで追加しようとした値が返ってくる
        assert_eq!(xs.push(8), Err(8));

        Ok(())
    }

    #[test]
    fn test_heapless_vec_generics() {
        let three_bytes = values::<u8, 3>(42);
        assert_eq!(three_bytes.len(), 3);
        defmt::println!("three_bytes = {}", three_bytes);
        
        let ten_characters = values::<char, 10>('𝄞');
        assert_eq!(ten_characters.len(), 10);
        defmt::println!("ten_characters = {}", ten_characters);
    }
}
