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
    let npins = pins.len();
    let mut pins_on = 0u8;
    
    loop {
        for (i, p) in pins.iter_mut().enumerate() {
            if (pins_on >> i) & 1 == 1 {
                p.set_high().unwrap();
            } else {
                p.set_low().unwrap();
            }
        }
        timer.delay_ms(500u16);
        pins_on = (pins_on + 1) % (1 << npins);
    }
}
