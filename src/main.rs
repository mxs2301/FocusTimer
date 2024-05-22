#![feature(duration_constructors)]

mod audio;
mod timer;

use clearscreen::clear;
use std::env;
use std::io::{stdin, stdout, Result, Write};
use std::process::exit;

fn main() {
    let x: Vec<String> = env::args().collect();

    if x[1].trim() == "cli" {
        if let Ok(x) = cli_mode() {
            exit(x)
        }
    } else {
        exit(1);
    }
}

fn cli_mode() -> Result<i32> {
    loop {
        println!("Focus Timer - CLI Mode");
        println!("Press q - quit | s - start");
        print!("Your input: ");
        stdout().flush()?;
        let mut buf = String::new();
        let _ = stdin().read_line(&mut buf);
        match buf.as_str().trim() {
            "q" => {
                if let Err(error) = clear() {
                    eprintln!("{}", error);
                    exit(1);
                }
                println!("Goodbye :)");
                break;
            }
            "s" => {
                timer::start();
            }
            _ => {
                buf.clear();
                if let Err(x) = clear() {
                    eprintln!("{}", x);
                }
                continue;
            }
        }
    }

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::audio::play;
    //use super::timer::get_time;
    //use super::timer::timer_mockup;
    //use std::time::Duration;
    // #[test]
    // fn mockup_test() {
    //     let durration = Duration::from_mins(1);
    //     assert_eq!((0, 0), timer_mockup(durration));
    // }
    #[test]
    fn test_playback() {
        let file = std::fs::File::open("/home/max/Projects/FocusTimer/media/success.mp3").unwrap();
        play(file).unwrap();
    }
}
