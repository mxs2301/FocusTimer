use rodio::*;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

pub fn play(file: File) -> Result<(), Box<dyn Error>> {
    let (_stream, handle) = rodio::OutputStream::try_default()?;
    let sink = rodio::Sink::try_new(&handle)?;
    sink.append(Decoder::new(BufReader::new(file))?);
    clearscreen::clear()?;
    println!("Sound of break");

    sink.sleep_until_end();

    Ok(())
}
