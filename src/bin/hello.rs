#![no_std]
#![no_main]

use core::fmt::Write;

use cortex_m_rt::entry;
use microbit::hal::delay::Delay;
use microbit::hal::prelude::*;
use microbit::hal::serial;
use microbit::hal::serial::BAUD115200;
use panic_halt as _;

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        let gpio = p.GPIO.split();
        let tx = gpio.pin24.into_push_pull_output().downgrade();
        let rx = gpio.pin25.into_floating_input().downgrade();
        let (mut tx, _) = serial::Serial::uart0(p.UART0, tx, rx, BAUD115200).split();
        let mut delay = Delay::new(p.TIMER0);
        loop {
            write!(tx, "Hello world!\n\r").ok();
            delay.delay_ms(1000_u32);
        }
    }
    panic!("END");
}
