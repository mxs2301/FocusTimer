#![feature(duration_constructors)]

mod audio;
mod gui;
mod timer;

use clearscreen::clear;
use std::env;
use std::io::{stdin, stdout, Result, Write};
use std::process::exit;

fn main() {
    let x: Vec<String> = env::args().collect();

    if x.len() == 1 {
        clearscreen::clear().expect("Screen clearing failed");
        empty_args();
        exit(3);
    }

    match x[1].trim() {
        "--cli" | "--CLI" => {
            if let Ok(x) = cli_mode() {
                exit(x);
            }
        }

        "--gui" | "--GUI" => {
            todo!()
        }

        "--help" | "-h" => {
            help();
            exit(0);
        }

        _ => {
            exit(1);
        }
    }
}

fn empty_args() {
    println!("FocusTimer is an application to setup timers to focus and breaks in between");
    println!("To Start the program in CLI-Mode pass CLI or cli as an argument when running the application");
    println!("To get help, pass the -h option when running FocusTimer");
}

fn help() {
    println!("To start FocusTimer in CLi-mode run 'FocusTimer --cli'");
    println!("To start FocusTimer in GUI-Mode run 'FocusTimer --gui (iced | egui | slint)'");
}

fn gui_mode() -> Result<i32> {
    todo!();
    Ok(0)
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
    use super::timer::select_ringtone;
    //use super::timer::get_time;
    //use super::timer::timer_mockup;
    //use std::time::Duration;
    // #[test]
    // fn mockup_test() {
    //     let durration = Duration::from_mins(1);
    //     assert_eq!((0, 0), timer_mockup(durration));
    // }
    // #[test]
    // fn test_playback() {
    //     let file = std::fs::File::open("/home/max/Projects/FocusTimer/media/success.mp3").unwrap();
    //     play(file).unwrap();
    // }

    #[test]
    fn list_audio() {
        println!("Print");
        select_ringtone();
    }
}
