//! Based on an example from https://github.com/jamwaffles/ssd1306
//! 
//! Ported to STMF030 by nebelgrau77
//! 
//! Send random raw data to the display, emulating an old untuned TV. This example `release()`s the
//! underlying display properties struct and allows calling of the low-level `draw()` method,
//! sending a 1024 byte buffer straight to the display.
//!
//! This example is for the STM32F030F4P6 board board using I2C1.
//!
//! Wiring connections are as follows for a 128x32 unbranded display:
//!
//! ```
//! Display -> Board
//! GND -> GND
//! +3.3V -> VCC
//! SDA -> PA10
//! SCL -> PA9
//! ```
//!
//! Run with `cargo run --example noise_i2c`. Best results when using `--release`.


#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_halt;
extern crate stm32f0xx_hal as hal;

use cortex_m_rt::entry;
use rand::prelude::*;
use ssd1306::{mode::displaymode::DisplayModeTrait, prelude::*, Builder as SSD1306Builder};


use crate::hal::{
    prelude::*,
    stm32,
    i2c::I2c,
    
};

#[entry]
fn main() -> ! {

    if let Some(mut dp) = stm32::Peripherals::take() {
        
        cortex_m::interrupt::free(move |cs| {

        // set the clocks to have the MCU run at full speed
        let mut rcc = dp.RCC.configure().sysclk(48.mhz()).freeze(&mut dp.FLASH);
        
        let gpioa = dp.GPIOA.split(&mut rcc);
        let scl = gpioa.pa9.into_alternate_af4(cs);
        let sda = gpioa.pa10.into_alternate_af4(cs);
        let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), &mut rcc);
              
        let mut disp: GraphicsMode<_> = SSD1306Builder::new().size(DisplaySize::Display128x32).connect_i2c(i2c).into();
        
        disp.init().unwrap();

        
        let mut props = disp.release();

        let mut buf = [0x00u8; 1024];

        let mut rng = SmallRng::seed_from_u64(0x0101_0303_0808_0909);

        loop {
            rng.fill_bytes(&mut buf);

            props.draw(&buf);
        }
       
    });
    
}

    loop {continue;}
    
}