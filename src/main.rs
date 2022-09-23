#![feature(hash_drain_filter)]

use utils::timer::Timer;

mod day01_inverse_captcha;
mod day02_corruption_checksum;
mod day03_spiral_memory;
mod day04_high_entropy_passphrases;
mod day05_a_maze_of_twisty_trampolines;
mod day06_memory_reallocation;
mod day07_recursive_circus;
mod day08_i_heard_you_like_registers;
mod day09_stream_processing;
mod day10_knot_hash;
mod day11_hex_ed;
mod day12_digital_plumber;
mod day13_packet_scanners;
mod day14_disk_defragmentation;
mod day15_dueling_generators;

fn main() {
    env_logger::init();
    let _timer = Timer::start(|elapsed| println!("main took {} ms.", elapsed.as_millis()));
    let day: usize = if let Some(arg1) = std::env::args().nth(1) {
        arg1.parse().expect("argument should be an integer")
    } else {
        15
    };
    println!("running day {}\n", day);
    match day {
        1 => day01_inverse_captcha::run(),
        2 => day02_corruption_checksum::run(),
        3 => day03_spiral_memory::run(),
        4 => day04_high_entropy_passphrases::run(),
        5 => day05_a_maze_of_twisty_trampolines::run(),
        6 => day06_memory_reallocation::run(),
        7 => day07_recursive_circus::run(),
        8 => day08_i_heard_you_like_registers::run(),
        9 => day09_stream_processing::run(),
        10 => day10_knot_hash::run(),
        11 => day11_hex_ed::run(),
        12 => day12_digital_plumber::run(),
        13 => day13_packet_scanners::run(),
        14 => day14_disk_defragmentation::run(),
        15 => day15_dueling_generators::run(),
        _ => panic!("day {} not found", day),
    }
}
