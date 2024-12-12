use std::time;

use esp_idf_hal::{self, delay::Delay, gpio::PinDriver, prelude::Peripherals};

fn main() {
    let peripherals = Peripherals::take().unwrap();

    let mut trig = PinDriver::output(peripherals.pins.gpio21).unwrap();
    let echo = PinDriver::input(peripherals.pins.gpio22).unwrap();

    let delay = Delay::default();

    loop {
        trig.set_low().unwrap();
        delay.delay_us(5_u32);

        trig.set_high().unwrap();
        delay.delay_us(10_u32);
        trig.set_low().unwrap();

        while !echo.is_high() {}

        let echo_start = time::SystemTime::now();

        while !echo.is_low() {}

        // let echo_end = time::SystemTime::now();

        let echo_dur = echo_start.elapsed().unwrap();

        let distance_cm = echo_dur.as_micros() as f32 * 0.0343 / 2.0;

        println!("Distance {:?} cm", &distance_cm.round());

        delay.delay_ms(1000);
    }

}