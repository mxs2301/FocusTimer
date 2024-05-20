use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
use std::{thread::sleep, time::Duration};
fn timer(duration: Duration) -> (u64, u64) {
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
pub fn start() {}

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

fn setup_audio() -> Result<File, Box<dyn Error>> {
    let path = std::path::Path::new("/home/max/Projects/FocusTimer/media/success.mp3");

    let file = File::open(path)?;

    Ok(file)
}
