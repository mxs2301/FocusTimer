use rodio::*;
use std::collections::HashMap;
use std::io::BufReader;
use std::fs::File;
use std::error::Error;


pub fn play()-> Result<(), Box<dyn Error>> {
   
   let (_stream, handle) = rodio::OutputStream::try_default()?;

   let sink = rodio::Sink::try_new(&handle)?;

   let file = std::fs::File::open("/home/max/Projects/FocusTimer/media/success.mp3")?;

   sink.append(rodio::Decoder::new(BufReader::new(file))?);
   clearscreen::clear()?;
   println!("Sound of break");
   sink.sleep_until_end();

   Ok(())
}
