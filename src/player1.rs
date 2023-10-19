// Date: Wed Oct 18 13:26:53 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use std::{
    sync::{Arc, Mutex, atomic::AtomicBool}, 
    collections::VecDeque,
    time::Duration,
    fs::File,
    io::BufReader
};

use rodio::{
    source::{Zero, Empty, Source},
    Sample,
    decoder::Decoder,
    cpal::FromSample,
    OutputStream,
    OutputStreamHandle
};

const THRESHOLD: usize = 512;

struct Control {
    paused: bool,
    repeat: bool
}

struct SourceStream<S> {
    current: S,
    play_list: Arc<Mutex<VecDeque<String>>>,
    listened_list: Vec<String>,
    control: Arc<Mutex<Control>>
}

pub struct Mp3Player {
    play_list: Arc<Mutex<VecDeque<String>>>,
    control: Arc<Mutex<Control>>,
    _stream: OutputStream,
    stream_handle: OutputStreamHandle
}

impl<S> Iterator for Arc<Mutex<SourceStream<S>>> 
where S: Source + Send,
      S::Item: Sample + Send
{
    type Item = S::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.lock().unwrap().current.next()
    }
}
