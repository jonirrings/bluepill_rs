// src/main.rs

// Blinks the Bluepill's onboard LED on PC13 of the STM32F103C8 microcontroller.

// Derived from stm32f1xx-hal 0.9.0 delay.rs example:
// https://github.com/stm32-rs/stm32f1xx-hal/blob/master/examples/delay.rs
// and an older set of code from Jonathan Klimt which lead me to the useful flash tool:
// https://jonathanklimt.de/electronics/programming/embedded-rust/rust-on-stm32-2/

// Pre-requisites:
// rustup update
// rustup target install thumbv7m-none-eabi
// cargo install cargo-flash
// git clone this project

// Usage:
// cargo build --release
// cargo flash --chip STM32F103C8 --connect-under-reset --release
// cargo clean (If updating linker script or things Cargo might not notice)

#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;
use cortex_m::{asm, singleton};
use cortex_m_rt::entry;
// The runtime
use stm32f1xx_hal::{
    pac,
    prelude::*,
    serial::{Serial, Config},
}; // STM32F1 specific functions (Can optionally include timer::Timer - see below)

use core::fmt::Write;

#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding HAL structs
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);
    let mut afio = dp.AFIO.constrain();
    let channels = dp.DMA1.split();
    // Acquire the GPIOC peripheral
    let mut gpioc = dp.GPIOC.split();
    let mut gpioa = dp.GPIOA.split();
    // Configure gpio C pin 13 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led = gpioc.pc13.into_push_pull_output(&mut gpioc.crh);
    // USART1
    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10;

    // USART1
    // let tx = gpiob.pb6.into_alternate_push_pull(&mut gpiob.crl);
    // let rx = gpiob.pb7;

    // USART2
    // let tx = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);
    // let rx = gpioa.pa3;

    // USART3
    // let tx = gpiob.pb10.into_alternate_push_pull(&mut gpiob.crh);
    // let rx = gpiob.pb11;
    let serial = Serial::new(
        dp.USART1,
        (tx, rx),
        &mut afio.mapr,
        Config::default().baudrate(115200.bps()),
        &clocks,
    );

    let tx = serial.tx.with_dma(channels.4);
    let rx = serial.rx.with_dma(channels.5);
    let buf = singleton!(: [u8; 8] = [0; 8]).unwrap();

    let (_, tx) = tx.write(b"The quick brown fox").wait();
    let (_, tx) = tx.write(b" jumps").wait();
    let (_, tx) = tx.write(b" over the lazy dog.").wait();
    tx.write(b"\r\n").wait();
    let (_buf, _rx) = rx.read(buf).wait();


    // Configure the syst timer for a blocking delay (also check out blink.rs example)
    //let mut delay = Timer::syst(cp.SYST, &clocks).delay();
    // or
    let mut delay = cp.SYST.delay(&clocks);

    loop {
        led.set_high();
        // Use `embedded_hal::DelayMs` trait
        delay.delay_ms(1_000_u16);
        led.set_low();
        // or use `fugit` duration units
        delay.delay(1.secs());
    }
}
