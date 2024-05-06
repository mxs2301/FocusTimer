use std::io;
use std::{thread::sleep, time::Duration};
pub fn timer_mockup(duration: Duration) -> (u64, u64) {
    let mut countdown = duration.as_secs();

    let mut minutes = duration.as_secs() / 60;

    let mut seconds = 0;
    while countdown != 0 {
        println!("{}:{}", minutes, seconds);

        if seconds == 0 {
            seconds = 59;
            minutes -= 1;
        } else {
            seconds -= 1;
        }
        countdown -= 1;
        sleep(Duration::from_secs(1));
    }

    (minutes, seconds)
}

fn get_time() -> (u16, u8) {
    let mut buf = String::new();

    println!("Please type in the the duration of a timer session");
    if let Err(err) = io::stdin().read_line(&mut buf) {
        eprintln!("{}", err);
    }

    let durr: u16 = buf.trim().parse().expect("Conversion failed");

    println!("Please type in the amount of sessions you want to have");

    if let Err(err) = io::stdin().read_line(&mut buf) {
        eprintln!("{}", err);
    }

    let amount: u8 = buf.trim().parse().expect("Conversion failed");

    (durr, amount)
}
