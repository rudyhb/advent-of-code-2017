use utils::timer::Timer;

mod day01_inverse_captcha;
mod day02_corruption_checksum;

fn main() {
    env_logger::init();
    let _timer = Timer::start(|elapsed| println!("main took {} ms.", elapsed.as_millis()));
    let day: usize = if let Some(arg1) = std::env::args().nth(1) {
        arg1.parse().expect("argument should be an integer")
    } else {
        2
    };
    println!("running day {}\n", day);
    match day {
        1 => day01_inverse_captcha::run(),
        2 => day02_corruption_checksum::run(),
        _ => panic!("day {} not found", day),
    }
}
