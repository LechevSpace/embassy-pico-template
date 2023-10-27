#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), no_main)]
#![feature(type_alias_impl_trait)]

#[cfg(feature = "defmt")]
use defmt_rtt as _;

use panic_probe as _;

#[cfg(feature = "rp2040")]
use embassy_executor::{Executor, Spawner};
use embassy_time::{Duration, Timer};

#[cfg(feature = "rp2040")]
use embassy_rp::{
    gpio::{Level, Output},
    multicore::{spawn_core1, Stack},
    peripherals::PIN_25,
};


#[cfg(feature = "cortex-m")]
use cortex_m_rt::{exception, ExceptionFrame};

use embassy_pico_template::info;

use static_cell::StaticCell;

#[cfg(feature = "cortex-m")]
/// Stack - Core1 stack = Core 0 stack size.
static CORE0_EXECUTOR: StaticCell<Executor> = StaticCell::new();
#[cfg(feature = "cortex-m")]
static CORE1_EXECUTOR: StaticCell<Executor> = StaticCell::new();
#[cfg(feature = "cortex-m")]
// TODO: Set a stack size for the second core
static mut CORE1_STACK: Stack<{ 30 * 1024 }> = Stack::new();

#[cfg(feature = "cortex-m")]
#[cortex_m_rt::entry]
fn main() -> ! {
    embassy_rp::pac::SIO.spinlock(31).write_value(1);

    let peripherals = embassy_rp::init(Default::default());
    let led = Output::new(peripherals.PIN_25, Level::Low);

    spawn_core1(peripherals.CORE1, unsafe { &mut CORE1_STACK }, move || {
        let core1_executor = CORE1_EXECUTOR.init(Executor::new());

        core1_executor.run(|spawner| spawner.must_spawn(print()))
    });

    let core0_executor = CORE0_EXECUTOR.init(Executor::new());
    core0_executor.run(|spawner| spawner.must_spawn(blinky(led)))
}

#[cfg(feature = "rp2040")]
#[embassy_executor::task()]
async fn print() {
    loop {
        info!("Printing on Core 1 every 2 secs...");
        Timer::after(Duration::from_secs(2)).await;
    }
}


#[cfg(feature = "rp2040")]
#[embassy_executor::task()]
async fn blinky(mut led: Output<'static, PIN_25>) {
    loop {
        info!("led on!");
        led.set_high();
        Timer::after(Duration::from_secs(1)).await;

        info!("led off!");
        led.set_low();
        Timer::after(Duration::from_secs(1)).await;
    }
}

#[cfg(feature = "cortex-m")]
#[exception]
unsafe fn HardFault(ef: &ExceptionFrame) -> ! {
    use embassy_pico_template::error;

    #[cfg(feature = "defmt")]
    error!("HardFault: {:#?}", defmt::Debug2Format(ef));

    #[cfg(not(feature = "defmt"))]
    error!("HardFault: {:#?}", ef);

    loop {}
}

#[cfg(not(feature = "cortex-m"))]
fn main() {}
