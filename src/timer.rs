use std::{thread::sleep, time::Duration};

pub fn timer_mockup(duration: Duration) -> (u64, u64) {
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
}pub fn start(){

    use crate::audio::play;
    println!("This is fun");


    let path = std::path::Path::new("/home/max/Projects/FocusTimer/media/success.mp3");
    let file = std::fs::File::open(path).unwrap();

    let _ = play(file);


}