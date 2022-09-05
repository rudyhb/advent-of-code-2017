use utils::timer::Timer;

mod day01_inverse_captcha;
mod day02_corruption_checksum;
mod day03_spiral_memory;
mod day04_high_entropy_passphrases;

fn main() {
    env_logger::init();
    let _timer = Timer::start(|elapsed| println!("main took {} ms.", elapsed.as_millis()));
    let day: usize = if let Some(arg1) = std::env::args().nth(1) {
        arg1.parse().expect("argument should be an integer")
    } else {
        4
    };
    println!("running day {}\n", day);
    match day {
        1 => day01_inverse_captcha::run(),
        2 => day02_corruption_checksum::run(),
        3 => day03_spiral_memory::run(),
        4 => day04_high_entropy_passphrases::run(),
        _ => panic!("day {} not found", day),
    }
}
