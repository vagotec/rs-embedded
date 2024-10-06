#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

// Define an enum that encapsulates all LED pins
enum Led {
    PE9(Output<'static, embassy_stm32::peripherals::PE9>),
    PE10(Output<'static, embassy_stm32::peripherals::PE10>),
    PE11(Output<'static, embassy_stm32::peripherals::PE11>),
    PE12(Output<'static, embassy_stm32::peripherals::PE12>),
    PE13(Output<'static, embassy_stm32::peripherals::PE13>),
    PE14(Output<'static, embassy_stm32::peripherals::PE14>),
    PE15(Output<'static, embassy_stm32::peripherals::PE15>),
    PE8(Output<'static, embassy_stm32::peripherals::PE8>),
}

// Implement an interface for the enum
impl Led {
    fn set_high(&mut self) {
        match self {
            Led::PE9(pin) => pin.set_high(),
            Led::PE10(pin) => pin.set_high(),
            Led::PE11(pin) => pin.set_high(),
            Led::PE12(pin) => pin.set_high(),
            Led::PE13(pin) => pin.set_high(),
            Led::PE14(pin) => pin.set_high(),
            Led::PE15(pin) => pin.set_high(),
            Led::PE8(pin) => pin.set_high(),
        }
    }

    fn set_low(&mut self) {
        match self {
            Led::PE9(pin) => pin.set_low(),
            Led::PE10(pin) => pin.set_low(),
            Led::PE11(pin) => pin.set_low(),
            Led::PE12(pin) => pin.set_low(),
            Led::PE13(pin) => pin.set_low(),
            Led::PE14(pin) => pin.set_low(),
            Led::PE15(pin) => pin.set_low(),
            Led::PE8(pin) => pin.set_low(),
        }
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    info!("blinky1");

    // Initialize the LED pins and add them to the enum array
    let mut leds = [
        Led::PE9(Output::new(p.PE9, Level::Low, Speed::Low)),
        Led::PE10(Output::new(p.PE10, Level::Low, Speed::Low)),
        Led::PE11(Output::new(p.PE11, Level::Low, Speed::Low)),
        Led::PE12(Output::new(p.PE12, Level::Low, Speed::Low)),
        Led::PE13(Output::new(p.PE13, Level::Low, Speed::Low)),
        Led::PE14(Output::new(p.PE14, Level::Low, Speed::Low)),
        Led::PE15(Output::new(p.PE15, Level::Low, Speed::Low)),
        Led::PE8(Output::new(p.PE8, Level::Low, Speed::Low)),
    ];

    let interval = Duration::from_millis(500);

    // Main loop
    loop {
        for led in leds.iter_mut() {
            // Turn on LED
            led.set_high();
            Timer::after(interval).await;

            // Turn off LED
            led.set_low();
            Timer::after(interval).await;
        }
    }
}
