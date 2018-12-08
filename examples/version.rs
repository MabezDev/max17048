#![no_std]
#![no_main]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate panic_semihosting;
extern crate stm32l4xx_hal as hal;

use crate::hal::prelude::*;
use crate::hal::stm32l4::stm32l4x2;

use crate::hal::i2c::I2c;
use crate::rt::ExceptionFrame;
use crate::rt::entry;

use core::fmt::Write;
use crate::sh::hio;
use max17048::Max17048;

#[entry]
fn main() -> ! {

    let mut hstdout = hio::hstdout().unwrap();

    // writeln!(hstdout, "Hello, world!").unwrap();

    // let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32l4x2::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpioa = dp.GPIOA.split(&mut rcc.ahb2);

    let mut scl = gpioa.pa9.into_open_drain_output(&mut gpioa.moder, &mut gpioa.otyper);
    scl.internal_pull_up(&mut gpioa.pupdr, true);
    let scl = scl.into_af4(&mut gpioa.moder, &mut gpioa.afrh);

    let mut sda = gpioa.pa10.into_open_drain_output(&mut gpioa.moder, &mut gpioa.otyper);
    sda.internal_pull_up(&mut gpioa.pupdr, true);
    let sda = sda.into_af4(&mut gpioa.moder, &mut gpioa.afrh);
    
    let i2c = I2c::i2c1(dp.I2C1, (scl, sda), 100.khz(), clocks, &mut rcc.apb1r1);

    let mut max17048 = Max17048::new(i2c);

    writeln!(hstdout, "Version: {}", max17048.version().unwrap());
    writeln!(hstdout, "Voltage: {}", max17048.vcell().unwrap());
    writeln!(hstdout, "State of charge: {}%", max17048.soc().unwrap());

    // // i2c.write(0x3C, &[0xCC, 0xAA]).unwrap();
    // let mut buffer = [0u8; 2];
    // // 0x08 is version reg
    // // i2c.write(0x6C, &[0x08],).unwrap();
    // // let val = i2c.read(0x36, &mut buffer).unwrap();
    // const MAX17048_ADDR: u8 = 0x6C;
    // i2c.write_read(MAX17048_ADDR, &[0x08], &mut buffer).unwrap();
    // let version: u16 = (buffer[0] as u16) << 8 | buffer[1] as u16;
    // writeln!(hstdout,"Silicon Version: {}", version);

    // // let soc: u16 = (buffer[0] as u16) + (buffer[1] as u16 / 256);  //& 0xFF00
    // // let soc: u16 = (buffer[0] as u16) << 8 & 0xFF00 | (buffer[1] as u16) & 0x00FF;
    // i2c.write_read(MAX17048_ADDR, &[0x04], &mut buffer).unwrap();
    // let soc: u16 = (buffer[0] as u16) << 8 | buffer[1] as u16;
    // writeln!(hstdout,"Batt SoC: {}%", soc / 256);

    // i2c.write_read(MAX17048_ADDR, &[0x02], &mut buffer).unwrap();
    // let vlt: u16 = (buffer[0] as u16) << 8 | buffer[1] as u16;
    // writeln!(hstdout,"Volt: {}", vlt as f32 * 0.000078125);

    loop {}
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}