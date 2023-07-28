#![no_main]
#![no_std]

use panic_rtt_target as _;
use rtt_target::rtt_init_print;

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    hal::{
        prelude::*,
        Timer,
        gpio::Level,
    },
};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let pin_r = board.pins.p0_10.into_push_pull_output(Level::Low);
    let pin_g = board.pins.p0_09.into_push_pull_output(Level::Low);
    let pin_b = board.pins.p1_02.into_push_pull_output(Level::Low);
    let mut pins = [pin_r.degrade(), pin_g.degrade(), pin_b.degrade()];
    let mut index = 0;
    let states = [4, 2, 1, 6, 3, 5, 7, 0];
    let nstates = states.len();
    
    loop {
        let state = states[index];
        for (i, p) in pins.iter_mut().enumerate() {
            if (state >> i) & 1 == 1 {
                p.set_high().unwrap();
            } else {
                p.set_low().unwrap();
            }
        }
        timer.delay_ms(500u16);
        index = (index + 1) % nstates;
    }
}
