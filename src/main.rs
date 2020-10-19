#![no_main]
#![no_std]

use panic_halt as _;

use crate::board::hal;
use crate::board::led::{LedColor, Leds};
use hal::spi::{Mode, Phase, Polarity, Spi};
use hal::{delay::Delay, prelude::*, stm32};
use stm32f407g_disc as board;

use cortex_m::peripheral::Peripherals;

use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};

// Accelerometer stuff
use accelerometer::{Orientation, Tracker};
use fk_lis3dsh::{RawAccelerometer, LIS3DSH};

#[entry]
fn main() -> ! {
    if let (Some(p), Some(cp)) = (stm32::Peripherals::take(), Peripherals::take()) {
        let gpiod = p.GPIOD.split();
        let gpioa = p.GPIOA.split();
        let gpioe = p.GPIOE.split();

        // Initialize on-board LEDs
        let mut leds = Leds::new(gpiod);

        // Constrain clock registers
        let rcc = p.RCC.constrain();

        // Configure clock to 168 MHz (i.e. the maximum) and freeze it
        let clocks = rcc.cfgr.sysclk(168.mhz()).freeze();

        // Get delay provider
        let mut delay = Delay::new(cp.SYST, clocks);

        rtt_init_print!();
        rprintln!("Hello embedded Rust");

        // Create SPI module
        let sck = gpioa.pa5.into_alternate_af5();
        let miso = gpioa.pa6.into_alternate_af5();
        let mosi = gpioa.pa7.into_alternate_af5();
        let spi = Spi::spi1(
            p.SPI1,
            (sck, miso, mosi),
            Mode {
                polarity: Polarity::IdleLow,
                phase: Phase::CaptureOnFirstTransition,
            },
            hal::time::KiloHertz(400).into(),
            clocks,
        );
        let mut cs = gpioe.pe3.into_push_pull_output();
        cs.set_high().unwrap();

        // Create accelerometer
        let mut acc =
            LIS3DSH::new_with_interface(fk_lis3dsh::commbus::SPIBus::new(spi, cs), &mut delay)
                .unwrap();

        let mut tracker = Tracker::new(4200.0);
        let mut counter = 0_u32;
        loop {
            if acc.has_data().unwrap() {
                let accel = acc.accel_raw().unwrap();
                tracker.update(accel);
                update_leds_orientation(&mut leds, tracker.orientation());

                if counter % 50 == 0 {
                    rprintln!(
                        "{}\t{}\t{}\t{:?}",
                        accel.x,
                        accel.y,
                        accel.z,
                        tracker.orientation()
                    );
                }
            } else {
                if counter % 100 == 0 {
                    rprintln!("{}\tNo Data", counter);
                }
            }

            counter += 1;
            delay.delay_ms(20_u16);
        }
    }

    loop {
        continue;
    }
}

fn update_leds_orientation(leds: &mut Leds, orientation: Orientation) {
    match orientation {
        Orientation::FaceDown => leds_all(leds, true),
        Orientation::LandscapeUp => leds_on(leds, LedColor::Red),
        Orientation::PortraitDown => leds_on(leds, LedColor::Blue),
        Orientation::LandscapeDown => leds_on(leds, LedColor::Green),
        Orientation::PortraitUp => leds_on(leds, LedColor::Orange),
        _ => leds_all(leds, false),
    };
}

fn leds_all(leds: &mut Leds, on: bool) {
    if on {
        leds[LedColor::Orange].on();
        leds[LedColor::Green].on();
        leds[LedColor::Blue].on();
        leds[LedColor::Red].on();
    } else {
        leds[LedColor::Orange].off();
        leds[LedColor::Green].off();
        leds[LedColor::Blue].off();
        leds[LedColor::Red].off();
    }
}

fn leds_on(leds: &mut Leds, color: LedColor) {
    leds_all(leds, false);
    leds[color].on();
}
