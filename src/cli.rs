use clearscreen::clear;

use crate::timer;
use std::io::{stdin, stdout, Write};
use std::process::exit;

#[derive(Debug)]
pub struct CLI_ERROR {}

impl std::fmt::Display for CLI_ERROR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A panic happened in the cli, check for issues with the audio and io sections"
        )
    }
}

pub fn cli_mode() -> Result<i32, CLI_ERROR> {
    clear().expect("Screen clearing fialed");
    loop {
        println!("Focus Timer - CLI Mode");
        println!("Press q - quit | s - start");
        print!("Your input: ");
        stdout().flush().expect("Flushing of buffer failed");
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

pub fn empty_args() {
    println!("FocusTimer is an application to setup timers to focus and breaks in between");
    println!("To Start the program in CLI-Mode pass CLI or cli as an argument when running the application");
    println!("To get help, pass the -h option when running FocusTimer");
}

pub fn help() {
    println!("To start FocusTimer in CLi-mode run 'FocusTimer --cli'");
    println!("To start FocusTimer in GUI-Mode run 'FocusTimer --gui (iced | egui | slint)'");
}
