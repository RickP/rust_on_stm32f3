#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]
#![no_main]

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate cortex_m;
extern crate f3;
extern crate panic_semihosting;

use f3::hal::delay::Delay;
use f3::hal::prelude::*;
use f3::hal::stm32f30x;
use f3::hal::i2c::I2c;
use f3::led::Leds;
use f3::Lsm303dlhc;
use rt::ExceptionFrame;


entry!(main);

fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();
    let gpioe = dp.GPIOE.split(&mut rcc.ahb);

    // clock configuration using the default settings (all clocks run at 8 MHz)
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // TRY this alternate clock configuration (all clocks run at 16 MHz)
    // let clocks = rcc.cfgr.sysclk(16.mhz()).freeze(&mut flash.acr);

    let mut leds = Leds::new(gpioe);
    let mut delay = Delay::new(cp.SYST, clocks);

    // The `Lsm303dlhc` abstraction exposed by the `f3` crate requires a specific pin configuration
    // to be used and won't accept any configuration other than the one used here. Trying to use a
    // different pin configuration will result in a compiler error.
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);
    let scl = gpiob.pb6.into_af4(&mut gpiob.moder, &mut gpiob.afrl);
    let sda = gpiob.pb7.into_af4(&mut gpiob.moder, &mut gpiob.afrl);

    let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 400.khz(), clocks, &mut rcc.apb1);

    let mut lsm303dlhc = Lsm303dlhc::new(i2c).unwrap();


    let n = leds.len();
    loop {
        let _accel = lsm303dlhc.accel().unwrap();
        let _mag = lsm303dlhc.mag().unwrap();
        let _temp = lsm303dlhc.temp().unwrap();

        if _accel.z > 15000 {
            for curr in 0..n {
                let next = (curr + 1) % n;
                leds[curr].off();
                leds[next].on();

                delay.delay_ms(50_u8);
            }
        }
    }
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
