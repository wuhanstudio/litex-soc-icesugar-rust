#![no_std]
#![no_main]

extern crate panic_halt;

use icesugar_pac;
use riscv_rt::entry;

mod timer;
mod print;

use timer::Timer;

const SYSTEM_CLOCK_FREQUENCY: u32 = 24_000_000;

// This is the entry point for the application.
// It is not allowed to return.
#[entry]
fn main() -> ! {
    let peripherals = unsafe { icesugar_pac::Peripherals::steal() };
    // let peripherals = icesugar_pac::Peripherals::take().unwrap();

    print::print_hardware::set_hardware(peripherals.UART);
    let mut timer = Timer::new(peripherals.TIMER0);

    loop {
        print!("Hello LiteX SoC\r\n");
        msleep(&mut timer, 500);
    }
}

fn msleep(timer: &mut Timer, ms: u32) {
    timer.disable();

    timer.reload(0);
    timer.load(SYSTEM_CLOCK_FREQUENCY / 1_000 * ms);

    timer.enable();

    // Wait until the time has elapsed
    while timer.value() > 0 {}
}
