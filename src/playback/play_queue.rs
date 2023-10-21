// Date: Sat Oct 21 22:59:07 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use std::{
    collections::VecDeque,
    time::Duration,
    sync::{Mutex, Arc},
    sync::atomic::{AtomicBool, AtomicU16, AtomicU32, Ordering},
    path::Path,
    io::BufReader,
    fs::File
};

use rodio::{
    source::{Source, Empty, Zero},
    Sample,
    cpal::FromSample,
    decoder::Decoder
};

use super::{PlayerError, THRESHOLD};

struct Control {
    paused: AtomicBool,
    repeat: AtomicBool
}


// request all methods in PlayQueue must be immutable
pub struct PlayQueue<S> {
    current: Mutex<(S, Option<String>)>,
    play_list: Arc<Mutex<VecDeque<String>>>,
    listened_list: Mutex<Vec<String>>,
    control: Control,
    duration_tick: AtomicU32,
    total_duration: Mutex<Option<Duration>>,
    channels: AtomicU16,
    sample_rate: AtomicU32
}

impl<S> PlayQueue<S> {
    #[inline]
    pub fn get_playlist(&self) -> Arc<Mutex<VecDeque<String>>> {
        self.play_list.clone()
    }
}

impl<I> Source for PlayQueue<Box<dyn Source<Item = I> + Send>>
where I: Sample + Send + 'static + FromSample<i16> + Sized
{
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        self.current.lock().unwrap().0.current_frame_len()
    }

    #[inline]
    fn channels(&self) -> u16 {
        self.current.lock().unwrap().0.channels()
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        self.current.lock().unwrap().0.sample_rate()
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        self.current.lock().unwrap().0.total_duration()
    }
}

impl<I> PlayQueue<Box<dyn Source<Item = I> + Send>>
where I: Sample + Send + 'static + FromSample<i16> + Sized
{
    pub fn new() -> Self {
        Self {
            current: Mutex::new((Box::new(Empty::<I>::new()) as Box<_>, None)),
            play_list: Arc::new(Mutex::new(VecDeque::new())),
            listened_list: Mutex::new(Vec::new()),
            control: Control { 
                paused: AtomicBool::new(false), 
                repeat: AtomicBool::new(false)
            },
            duration_tick: AtomicU32::new(0),
            total_duration: Mutex::new(None),
            sample_rate: AtomicU32::new(0),
            channels: AtomicU16::new(0)
        }
    }

    #[inline]
    fn next_sample(&self) -> Option<I> {
        if self.control.paused.load(Ordering::Acquire) {
            return Some(I::zero_value());
        }

        let _ = self.duration_tick.fetch_add(1, Ordering::SeqCst);
        self.current.lock().unwrap().0.next()
    }

    pub fn next(&self) -> Option<I> {
        loop {
            if let Some(sample) = self.next_sample() {
                return Some(sample);
            }

            let _ = self.go_next().unwrap();
        }
    }

    #[inline]
    pub fn size_hint(&self) -> (usize, Option<usize>) {
        self.current.lock().unwrap().0.size_hint()
    }

    // implement next_chunk to avoid acquiring mutex lock frequently
    pub fn next_chunk(&self, chunk_size: usize) -> Vec<I> {
        (0..chunk_size).into_iter().map(|_| {
            loop {
                if let Some(sample) = self.next_sample() {
                    return sample;
                }

                let _ = self.go_next().unwrap();
            }
        }).collect::<Vec<I>>()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.play_list.lock().unwrap().is_empty()
    }

    pub fn append(&self, path: String) -> Result<(), PlayerError> {
        if !Path::new(&path).is_file() {
            Err(PlayerError::WrongFileType(path))
        } else {
            self.play_list.lock().unwrap().push_back(path);
            Ok(())
        }
    }

    pub fn get_song(&self) -> Option<String> {
        self.current.lock().unwrap().1.clone()
    }

    #[inline]
    pub fn set_paused(&self, paused: bool) {
        self.control.paused.store(paused, Ordering::Release);
    }

    #[inline]
    pub fn set_repeat(&self, repeat: bool) {
        self.control.repeat.store(repeat, Ordering::Release);
    }

    #[inline]
    pub fn total_duration(&self) -> Option<Duration> {
        self.total_duration.lock().unwrap().clone()
    }

    pub fn progress(&self) -> Option<Duration> {
        if self.total_duration.lock().unwrap().is_none() {
            return None;
        }

        let sample_rate = self.sample_rate.load(Ordering::Acquire);
        let channels = self.channels.load(Ordering::Acquire) as u32;
        let ticks = self.duration_tick.load(Ordering::Acquire);

        Some(Duration::from_secs((ticks / sample_rate / channels) as u64))
    }

    pub fn play(&self, path: String) -> Result<(), PlayerError> {
        let _ = self.play_next(path)?;
        self.go_next_ignore_repeat(true)
    }

    pub fn play_next(&self, path: String) -> Result<(), PlayerError> {
        if !Path::new(&path).is_file() {
            return Err(PlayerError::WrongFileType(path));
        }

        self.play_list.lock().unwrap().push_front(path);
        Ok(())
    }

    pub fn go_next_ignore_repeat(&self, ignore: bool) -> Result<(), PlayerError> {
        let mut current = self.current.lock().unwrap();

        if let Some(path) = current.1.take() {
            if self.control.repeat.load(Ordering::Acquire) && !ignore {
                self.play_list.lock().unwrap().push_front(path);
            } else {
                self.listened_list.lock().unwrap().push(path);
            }
        }

        *current = match self.play_list.lock().unwrap().pop_front() {
            None => (Box::new(Zero::<I>::new_samples(1, 44100, THRESHOLD)) as Box<_>, None),
            Some(path_string) => {
                let buf = BufReader::new(match 
                    File::open(Path::new(&path_string)) {
                    Ok(f) => f,
                    Err(e) => return Err(PlayerError::IOError(e))
                });

                (Box::new(match Decoder::new(buf) {
                    Ok(decoded) => decoded,
                    Err(e) => return Err(PlayerError::DecoderError(e))
                }.convert_samples()), Some(path_string))
            }
        };

        *self.total_duration.lock().unwrap() = match current.1 {
            None => None,
            Some(ref path_string) => 
                mp3_duration::from_path(Path::new(path_string)).ok()
        };

        self.duration_tick.store(0, Ordering::Release);
        let sample_rate = current.0.sample_rate();
        let channels = current.0.channels();
        self.sample_rate.store(sample_rate, Ordering::Release);
        self.channels.store(channels, Ordering::Release);

        Ok(())
    }

    #[inline]
    pub fn go_next(&self) -> Result<(), PlayerError> {
        self.go_next_ignore_repeat(false)
    }

    pub fn go_prev(&self) -> Result<(), PlayerError> {
        if let Some(prev_path) = self.listened_list.lock().unwrap().pop() {
            let mut current = self.current.lock().unwrap();

            if let Some(curr_path) = current.1.take() {
                self.play_list.lock().unwrap().push_front(curr_path);
            }

            self.play_list.lock().unwrap().push_front(prev_path);

            self.go_next_ignore_repeat(true)?;
        }

        Ok(())
    }
}
