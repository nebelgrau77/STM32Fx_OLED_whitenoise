//! Based on an example from https://github.com/jamwaffles/ssd1306
//! 
//! Ported to STMF411
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
//! SDA -> PB9
//! SCL -> PB8
//! ```
//!
//! Best results when using `--release`.

#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate panic_halt;
extern crate stm32f4xx_hal as hal;

use cortex_m_rt::entry;
use rand::prelude::*;
use ssd1306::{mode::displaymode::DisplayModeTrait, prelude::*, Builder as SSD1306Builder};

use crate::hal::{i2c::I2c, prelude::*, stm32};

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(_cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the system clock. We want to run at 100MHz for this one.
        // external crystal on this board is 25MHz
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.use_hse(25.mhz()).sysclk(100.mhz()).pclk2(25.mhz()).freeze();

        // Set up I2C - SCL is PB8 and SDA is PB9; they are set to Alternate Function 4
        
        let gpiob = dp.GPIOB.split();
        let scl = gpiob.pb8.into_alternate_af4().set_open_drain();
        let sda = gpiob.pb9.into_alternate_af4().set_open_drain();
        let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks);

        
        // Set up the display
        
        let mut disp: GraphicsMode<_> = SSD1306Builder::new().connect_i2c(i2c).into();
        disp.init().unwrap();
        
               
        let mut props = disp.release();

        let mut buf = [0x00u8; 1024];

        let mut rng = SmallRng::seed_from_u64(0xfedc_ba98_7654_3210);

        loop {
            rng.fill_bytes(&mut buf);

            props.draw(&buf);
        }
        
    }

    loop {}
}
