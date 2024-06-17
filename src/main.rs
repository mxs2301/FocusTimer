#![feature(duration_constructors)]

mod audio;
mod cli;
mod gui;
mod timer;

use clearscreen::clear;
use cli::{cli_mode, empty_args, help};
use std::{env, process::exit};

fn main() {
    let x: Vec<String> = env::args().collect();

    if x.len() == 1 {
        clear().expect("Screen clearing failed");
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
