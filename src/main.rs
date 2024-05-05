#![feature(duration_constructors)]
mod timer;

use std::env;
use std::io::stdin;
use std::process::exit;
use std::{
    io::{self, Read},
    time::Duration,
};
use timer::timer_mockup;

fn main() {
    
    let x: Vec<String>  = env::args().collect();

    if x[1] == "cli".to_string(){
        if let Ok(x) = cli_mode(){
            exit(x)
        }

    }else{
        exit(1);
    }





}

fn cli_mode()-> io::Result<i32>{
    loop {
        println!("Focus Timer - CLI Mode");
        println!("Press q to quit");
        println!("Press s to start");
        let mut buf = String::new();
        let _ =  stdin().read_line(&mut buf);
        match buf.as_str().trim() {
            "q" =>{
                break;
            }
            _ => {
                buf.clear();
                continue;
            }
        }
    }

    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mockup_test() {
        let durration = Duration::from_mins(1);
        assert_eq!((0, 0), timer_mockup(durration));
    }
}
