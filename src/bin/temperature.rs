#![no_std]
#![no_main]

use core::fmt::Write;

use cortex_m_rt::entry;
use microbit::hal::delay::Delay;
use microbit::hal::prelude::*;
use microbit::hal::serial;
use microbit::hal::serial::BAUD115200;
use microbit::led;
use panic_halt;

use microfun::display::ScrollImages;
use microfun::font::Font;
use microfun::image::Image;
use microfun::Temperature;

#[entry]
fn main() -> ! {
    if let Some(p) = microbit::Peripherals::take() {
        let gpio = p.GPIO.split();
        let temp = Temperature(p.TEMP);
        let tx = gpio.pin24.into_push_pull_output().downgrade();
        let rx = gpio.pin25.into_floating_input().downgrade();
        let (mut tx, _) = serial::Serial::uart0(p.UART0, tx, rx, BAUD115200).split();
        let mut delay = Delay::new(p.TIMER0);

        // Display
        let row1 = gpio.pin13.into_push_pull_output();
        let row2 = gpio.pin14.into_push_pull_output();
        let row3 = gpio.pin15.into_push_pull_output();
        let col1 = gpio.pin4.into_push_pull_output();
        let col2 = gpio.pin5.into_push_pull_output();
        let col3 = gpio.pin6.into_push_pull_output();
        let col4 = gpio.pin7.into_push_pull_output();
        let col5 = gpio.pin8.into_push_pull_output();
        let col6 = gpio.pin9.into_push_pull_output();
        let col7 = gpio.pin10.into_push_pull_output();
        let col8 = gpio.pin11.into_push_pull_output();
        let col9 = gpio.pin12.into_push_pull_output();
        let mut leds = led::Display::new(
            col1, col2, col3, col4, col5, col6, col7, col8, col9, row1, row2, row3,
        );

        let font = Font::default();

        let mut digits = [Image::default(); 4];
        digits[3] = font.char_bits(b'C');

        loop {
            let current_temp = temp.temperature();
            write!(tx, "Temperature: {}\n\r", current_temp).unwrap();

            digits[0] = if current_temp < 0 {
                font.char_bits(b'-')
            } else {
                font.char_bits(b'+')
            };

            digits[1] = font.char_bits((current_temp / 10) as u8 + 48);
            digits[2] = font.char_bits((current_temp % 10) as u8 + 48);

            leds.scroll_images(&mut delay, &digits, 200);

            delay.delay_ms(1000_u32);
        }
    }
    panic!("END");
}
