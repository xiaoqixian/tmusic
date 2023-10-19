// Date: Wed Oct 18 13:26:53 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use std::{
    //sync::mpsc::{channel, Receiver, Sender}, 
    sync::{Arc, Mutex},
    sync::atomic::{AtomicBool, Ordering},
    collections::VecDeque,
    time::Duration,
    fs::File,
    io::BufReader,
    path::Path, thread::JoinHandle,
};

use kanal::{bounded, Sender, Receiver};

use rodio::{
    source::{Zero, Empty, Source},
    Sample,
    decoder::Decoder,
    cpal::FromSample,
    OutputStream,
    OutputStreamHandle
};

const THRESHOLD: usize = 512;
const BUFFER_SIZE: usize = 64;

#[derive(Debug)]
enum RequestType {
    Sample,
    CurrentFrameLen,
    Channels,
    SampleRate,
    TotalDuration
}
#[derive(Debug)]
enum ResponseType {
    CurrentFrameLen(Option<usize>),
    Channels(u16),
    SampleRate(u32),
    TotalDuration(Option<Duration>)
}

struct Control {
    paused: AtomicBool,
    repeat: AtomicBool
}

// I is the type of sample transfered
// from the channel
// I need to be converted to D in next()
struct SourceStream<I, D> {
    response_receiver: Receiver<ResponseType>,
    request_sender: Sender<RequestType>,
    sample_receiver: Receiver<I>,
    phantom: std::marker::PhantomData<D>
}

pub struct Player<S> {
    play_queue: Arc<PlayQueue<S>>,
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    listener_handle: Option<JoinHandle<Result<(), ()>>>
}

struct NoticeListener<S, I> {
    play_queue: Arc<PlayQueue<S>>,
    response_sender: Sender<ResponseType>,
    request_receiver: Receiver<RequestType>,
    sample_sender: Sender<I>
}

// request all methods in PlayQueue must be immutable
struct PlayQueue<S> {
    current: Mutex<(S, Option<String>)>,
    play_list: Mutex<VecDeque<String>>,
    listened_list: Mutex<Vec<String>>,
    control: Control,
}

impl<I> PlayQueue<Box<dyn Source<Item = I> + Send>>
where I: Sample + Send + 'static + FromSample<i16> + Sized
{
    fn new() -> Self {
        Self {
            current: Mutex::new((Box::new(Empty::<I>::new()) as Box<_>, None)),
            play_list: Mutex::new(VecDeque::new()),
            listened_list: Mutex::new(Vec::new()),
            control: Control { 
                paused: AtomicBool::new(false), 
                repeat: AtomicBool::new(false)
            }
        }
    }

    fn next(&self) -> Option<I> {
        loop {
            if let Some(sample) = self.current.lock().unwrap().0.next() {
                return Some(sample);
            }

            let _ = self.go_next().unwrap();
        }
    }

    // implement next_chunk to avoid acquiring mutex lock frequently
    fn next_chunk(&self, chunk_size: usize) -> Vec<I> {
        (0..chunk_size).into_iter().map(|_| {
            loop {
                if let Some(sample) = self.current.lock().unwrap().0.next() {
                    return sample;
                }

                let _ = self.go_next().unwrap();
            }
        }).collect::<Vec<I>>()
    }

    fn append(&self, path: String) -> Result<(), ()> {
        if !Path::new(&path).is_file() {
            Err(())
        } else {
            self.play_list.lock().unwrap().push_back(path);
            Ok(())
        }
    }

    fn set_paused(&self, paused: bool) {
        self.control.paused.store(paused, Ordering::Release);
    }

    fn go_next(&self) -> Result<(), ()> {
        let mut current = self.current.lock().unwrap();

        if let Some(path) = current.1.take() {
            self.listened_list.lock().unwrap().push(path);
        }

        *current = match self.play_list.lock().unwrap().pop_front() {
            None => (Box::new(Zero::<I>::new_samples(1, 44100, THRESHOLD)) as Box<_>, None),
            Some(path) => {
                let buf = BufReader::new(File::open(path.clone()).unwrap());
                (Box::new(Decoder::new(buf).unwrap().convert_samples()), Some(path))
            }
        };

        Ok(())
    }

    fn go_prev(&self) -> Result<(), ()> {
        if let Some(prev_path) = self.listened_list.lock().unwrap().pop() {
            let mut current = self.current.lock().unwrap();

            if let Some(curr_path) = current.1.take() {
                self.play_list.lock().unwrap().push_front(curr_path);
            }

            *current = ({
                let buf = BufReader::new(File::open(prev_path.clone()).unwrap());
                Box::new(Decoder::new(buf).unwrap().convert_samples())
            }, Some(prev_path));
        }

        Ok(())
    }
}

impl<I, D> Iterator for SourceStream<I, D>
where 
    I: Sample + Send + 'static,
    D: FromSample<I> + Sample + Send
{
    type Item = D;

    fn next(&mut self) -> Option<Self::Item> {
        match self.sample_receiver.try_recv().unwrap() {
            Some(sample) => Some(D::from_sample(sample)),
            None => {
                self.request_sender.send(RequestType::Sample).unwrap();
                Some(D::from_sample(self.sample_receiver.recv().unwrap()))
            }
        }
    }
}

impl<I, D> Source for SourceStream<I, D>
where
    I: Sample + Send + 'static,
    D: FromSample<I> + Sample + Send
{
    fn current_frame_len(&self) -> Option<usize> {
        if let Err(e) = self.request_sender.send(RequestType::CurrentFrameLen) {panic!("{}", format!("send error: {:?}", e));}
        match self.response_receiver.recv().unwrap() {
            ResponseType::CurrentFrameLen(cfl) => cfl,
            _ => panic!("INCORRECT ResponseType")
        }
    }

    fn channels(&self) -> u16 {
        self.request_sender.send(RequestType::Channels).unwrap();
        match self.response_receiver.recv().unwrap() {
            ResponseType::Channels(c) => c,
            _ => panic!("INCORRECT ResponseType")
        }
    }

    fn sample_rate(&self) -> u32 {
        self.request_sender.send(RequestType::SampleRate).unwrap();
        match self.response_receiver.recv().unwrap() {
            ResponseType::SampleRate(sr) => sr,
            _ => panic!("INCORRECT ResponseType")
        }
    }

    fn total_duration(&self) -> Option<Duration> {
        self.request_sender.send(RequestType::TotalDuration).unwrap();
        match self.response_receiver.recv().unwrap() {
            ResponseType::TotalDuration(td) => td,
            _ => panic!("INCORRECT ResponseType")
        }
    }
}

impl<I> Iterator for NoticeListener<Box<dyn Source<Item = I> + Send>, I>
where
    I: Sample + Send + 'static + FromSample<i16>,
{
    type Item = I;

    // TODO: next needs to frequently acquire mutex lock
    // implement a next_chunk
    fn next(&mut self) -> Option<Self::Item> {
        self.play_queue.next()
    }
}

impl<I> Source for NoticeListener<Box<dyn Source<Item = I> + Send>, I>
where
    I: Sample + Send + 'static + FromSample<i16>
{
    fn current_frame_len(&self) -> Option<usize> {
        if let Some(val) = self.play_queue.current.lock().unwrap().0.current_frame_len() {
            if val > 0 {
                return Some(val);
            } else if self.play_queue.play_list.lock().unwrap().is_empty() {
                return Some(THRESHOLD);
            }
        }

        let (lower_bound, _) = self.play_queue.current.lock().unwrap().0.size_hint();

        if lower_bound > 0 {
            return Some(lower_bound);
        }

        Some(THRESHOLD)
    }

    #[inline]
    fn channels(&self) -> u16 {
        self.play_queue.current.lock().unwrap().0.channels()
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        self.play_queue.current.lock().unwrap().0.sample_rate()
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        self.play_queue.current.lock().unwrap().0.total_duration()
    }
}

impl<I> NoticeListener<Box<dyn Source<Item = I> + Send>, I>
where
    I: Sample + Send + 'static + FromSample<i16>
{
    fn run(&mut self) -> Result<(), ()> {
        loop {
            let req = self.request_receiver.recv().unwrap();

            if let RequestType::Sample = req {
                self.play_queue.next_chunk(BUFFER_SIZE).into_iter().for_each(|smp| self.sample_sender.send(smp).unwrap());
                continue;
            }

            let resp = match req {
                RequestType::CurrentFrameLen => {
                    ResponseType::CurrentFrameLen(self.current_frame_len())
                },
                RequestType::Channels => {
                    ResponseType::Channels(self.channels())
                },
                RequestType::SampleRate => {
                    ResponseType::SampleRate(self.sample_rate())
                },
                RequestType::TotalDuration => {
                    ResponseType::TotalDuration(self.total_duration())
                },
                _ => panic!("Unexpected ResponseType")
            };

            let _ = self.response_sender.send(resp).unwrap();
        }
        Ok(())
    }
}

impl<I> Player<Box<dyn Source<Item = I> + Send>>
where 
    I: Sample + Send + FromSample<i16> + 'static,
    f32: FromSample<I>
{
    fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        //let (request_sender, request_receiver) = channel::<RequestType>();
        //let (response_sender, response_receiver) = channel::<ResponseType<I>>();
        let (request_sender, request_receiver) = bounded::<RequestType>(0);
        let (response_sender, response_receiver) = bounded::<ResponseType>(0);
        let (sample_sender, sample_receiver) = bounded::<I>(BUFFER_SIZE);

        let source_stream = SourceStream::<I, f32> {
            response_receiver,
            request_sender,
            sample_receiver,
            phantom: std::marker::PhantomData
        };

        let play_queue = Arc::new(PlayQueue::<Box<dyn Source<Item = I> + Send>>::new());

        //let current: Arc<Mutex<(Box<dyn Source<Item = I> + Send>, Option<String>)>> = Arc::new(Mutex::new((Box::new(Empty::<I>::new()) as Box<_>, None)));
        //let play_list = Arc::new(Mutex::new(VecDeque::new()));
        //let listened_list = Arc::new(Mutex::new(Vec::new()));
        //let control = Arc::new(Control {
            //paused: AtomicBool::new(false),
            //repeat: AtomicBool::new(false),
        //});

        let mut listener = NoticeListener::<Box<dyn Source<Item = I> + Send>, I> {
            play_queue: play_queue.clone(),
            response_sender,
            request_receiver,
            sample_sender
        };

        // spawn a notice listener
        let listener_handle = std::thread::spawn(move || listener.run());

        let _ = stream_handle.play_raw(source_stream).unwrap();

        Self {
            play_queue,
            _stream,
            stream_handle,
            listener_handle: Some(listener_handle)
        }
    }

    pub fn append(&mut self, path: String) -> Result<(), ()>{
        self.play_queue.append(path)
    }

    pub fn go_next(&mut self) -> Result<(), ()> {
        self.play_queue.go_next()
    }

    pub fn go_prev(&mut self) -> Result<(), ()> {
        self.play_queue.go_prev()
    }

    pub fn play(&mut self, path: String) -> Result<(), ()> {
        Ok(())
    }

    pub fn join_listener(&mut self) {
        self.listener_handle.take().unwrap().join().unwrap().expect("listener exit abnormally");
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.play_queue.set_paused(paused);
    }
}

pub fn mp3player() -> Player::<Box<dyn Source<Item = i16> + Send>> {
    Player::<Box<dyn Source<Item = i16> + Send>>::new()
}
