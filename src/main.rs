#![feature(duration_constructors)]

mod timer;
use std::env;
use std::io;
use std::process::exit;
use clearscreen::clear;
use clearscreen::ClearScreen;

fn main() {
    
    let x: Vec<String>  = env::args().collect();

    if x[1].trim() == "cli"{
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
        println!("Press q - quit, s - start\n");
        let mut buf = String::new();
        let _ =  io::stdin().read_line(&mut buf);
        match buf.as_str().trim() {
            "q" =>{
                if let Err(x) = clear(){
                    eprintln!("{}", x);
                    exit(1);
                }
                println!("Goodbye :)");
                break;
            },
            "s" => {
                todo!();
            },
            _ => {
                buf.clear();
                if let Err(x) = clear(){
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
    use super::timer::timer_mockup;
    use std::time::Duration;

    #[test]
    fn mockup_test() {
        let durration = Duration::from_mins(1);
        assert_eq!((0, 0), timer_mockup(durration));
    }
}
