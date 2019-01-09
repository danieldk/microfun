#![no_std]

use microbit::TEMP;

pub mod display;

pub mod font;

pub struct Temperature(pub TEMP);

impl Temperature {
    pub fn temperature(&self) -> u32 {
        self.0.tasks_start.write(|w| unsafe { w.bits(1) });
        while self.0.events_datardy.read().bits() == 0 {}
        self.0.events_datardy.reset();
        let t = self.0.temp.read().bits() / 4;
        self.0.tasks_stop.write(|w| unsafe { w.bits(1) });
        t
    }
}
