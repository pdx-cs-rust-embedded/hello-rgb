#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use cortex_m_rt::entry;
use microbit::{board::Board, hal::{prelude::*, Timer, gpio::Level::*}};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut pin_r = board.pins.p0_10.into_push_pull_output(Low);
    let mut _pin_g = board.pins.p0_09.into_push_pull_output(Low);
    let mut _pin_b = board.pins.p1_02.into_push_pull_output(Low);

    loop {
        pin_r.set_high().unwrap();
        timer.delay_ms(500u16);
        pin_r.set_low().unwrap();
        timer.delay_ms(500u16);
    }
}
