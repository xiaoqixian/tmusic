//use std::fs::File;
//use std::io::BufReader;
//use rodio::{Decoder, OutputStream, source::Source};

mod player;
//use rodio::{OutputStream, Decoder, Source};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut mp3 = player::mp3player();
    mp3.append(String::from("files/bicycle.mp3")).unwrap();
    mp3.append(String::from("files/THANATOS.mp3")).unwrap();
    sleep(Duration::from_secs(10));
    mp3.go_next().unwrap();
    sleep(Duration::from_secs(10));
    mp3.go_prev().unwrap();
    sleep(Duration::from_secs(10));
}

#[test]
fn test_next_prev() {
}
