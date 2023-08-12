#[allow(unused_imports)]
use rand::Rng;
use bars::Bars;
use std::{thread, time};

pub mod bars;

#[allow(dead_code)]
const LENGTH: usize = 5;
const BOX: char = '■';

fn main() {
    let barray: Vec<u8> = gen_random_vec(LENGTH);
    let mut my_bar = Bars::new(
        barray,
        BOX);
    my_bar.show_bars();
    let delay_amt = 200;
    let t = time::Duration::from_millis(delay_amt);

    loop {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        my_bar.barvec = gen_random_vec(LENGTH);
        my_bar.show_bars();
        thread::sleep(t);
    }
}

fn gen_random_vec(size: usize) -> Vec<u8> {
    (0..size).map(|_| rand::thread_rng().gen_range(1..11)).collect::<Vec<u8>>()
}

