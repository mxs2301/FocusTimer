use std::io;
use std::{thread::sleep, time::Duration};

use clearscreen::clear;
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

/// Use to get user input to setup necessary durations
/// Returns are signed integers as unsigned could panic on a false input
pub fn get_time() -> (i16, i8) {
    let mut buf = String::new();

    loop {
        println!("Please type in the the duration of a timer session");
        if let Err(err) = io::stdin().read_line(&mut buf) {
            eprintln!("{}", err);
        }

        // let durr: u16 = buf.trim().parse().expect("Conversion failed");
        let durr: i16 = buf.trim().to_string().parse().expect("Conversion fail");
        buf.clear();
        println!("Please type in the amount of sessions you want to have");

        if let Err(err) = io::stdin().read_line(&mut buf) {
            eprintln!("{}", err);
        }

        // let amount: u8 = buf.trim().parse().expect("Conversion failed");
        let amount: i8 = buf.trim().to_string().parse().expect("Conversion failed");

        // Check to see if numbers match
        if durr > 0 && amount > 0 {
            return (durr, amount);
        } else {
            if let Err(err) = clear() {
                eprintln!("{}", err);
            }
            println!("One of the input values was not accetable");
            buf.clear();
        }
    }
}
