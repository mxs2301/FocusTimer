use crate::audio::play;
use clearscreen::clear;
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
use std::{thread::sleep, time::Duration};

fn timer(duration: Duration) -> (u64, u64) {
    let mut countdown = duration.as_secs();
    let mut minutes = duration.as_secs() / 60;
    let mut seconds = 0;

    clear().expect("Clear failed");

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
        clear().expect("Clear failed");
    }

    (minutes, seconds)
}
pub fn start() {
    let (session_time, break_time, session_amount) = setup_session_details().expect("False input");
    let audio_file = setup_audio().expect("File not found or at wrong location");

    session_notification(
        "Your session will start in 5 seconds\nHave fun focusing! :)",
        5,
    );

    for _ in 0..session_amount {
        timer(create_duration(session_time));
        play(audio_file.try_clone().expect("Cloning failed"))
            .expect("Audio was not able to be played");
        session_notification("Your break will start in 5 seconds", 5);
        timer(create_duration(break_time));
        session_notification("Your break is over in 5 seconds", 5);
    }
    clear().expect("Access to clear failed horribly");
    session_notification(
        "You did it!, you will be brought back to the main menun in 5 seconds",
        5,
    );
}

fn session_notification(msg: &str, time: u64) {
    println!("{}", msg);
    sleep(Duration::from_secs(time));
}

fn setup_session_details() -> Result<(i32, i32, i32), Box<dyn Error>> {
    let mut buf = String::new();

    print!("How long should a session be: ");
    io::stdout().flush()?;
    let _ = io::stdin().read_line(&mut buf);
    let session = buf.trim().parse::<i32>()?;
    buf.clear();

    print!("How long should a break be: ");
    io::stdout().flush()?;
    let _ = io::stdin().read_line(&mut buf);
    let pause = buf.trim().parse::<i32>()?;
    buf.clear();

    print!("How many rounds do you want to have: ");
    io::stdout().flush()?;
    let _ = io::stdin().read_line(&mut buf);
    let amount = buf.trim().parse::<i32>()?;

    Ok((session, pause, amount))
}

fn create_duration(time: i32) -> Duration {
    Duration::from_mins(time as u64)
}

fn setup_audio() -> Result<File, Box<dyn Error>> {
    let path = std::path::Path::new("/home/max/Projects/FocusTimer/media/success.mp3");

    let file = File::open(path)?;

    Ok(file)
}
