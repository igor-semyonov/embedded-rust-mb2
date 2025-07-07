#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::{delay::DelayNs, digital::OutputPin};
use microbit::hal::{gpio, timer};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let board = microbit::Board::take().unwrap();

    let _row1 = board
        .display_pins
        .row1
        .into_push_pull_output(gpio::Level::High);
    let _col1 = board
        .display_pins
        .col1
        .into_push_pull_output(gpio::Level::Low);
    let mut row5 = board
        .display_pins
        .row5
        .into_push_pull_output(gpio::Level::High);

    let mut timer = timer::Timer::new(board.TIMER0);

    loop {
        timer.delay_ms(100);
        row5.set_high()
            .unwrap();
        timer.delay_ms(100);
        row5.set_low()
            .unwrap();
    }
}
