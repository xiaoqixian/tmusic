// Date: Mon Oct 16 17:55:45 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use std::collections::VecDeque;
use std::cell::RefCell;

use rodio::source::{Empty, Source, Zero};
use rodio::Sample;

use super::player::PlayerCommand;

/// Builds a queue.
pub fn queue<S>(keep_alive_if_empty: bool)
    -> (Arc<SourcesQueueInput<S>>, 
        SourcesQueueOutput<S>, 
        Sender<PlayerCommand>) 
where S: Sample + Send + 'static {

    let input = Arc::new(SourcesQueueInput {
        next_sounds: Mutex::new(VecDeque::new()),
        prev_sounds: RefCell::new(Vec::new()),
        keep_alive_if_empty: AtomicBool::new(keep_alive_if_empty)
    });

    let output = SourcesQueueOutput {
        current: Box::new(Empty::<S>::new()) as Box<_>,
        signal_after_end: None,
        input: input.clone()
    };

    let (tx, rx) = mpsc::channel();
    //spawn a thread to receive commands
    std::thread::spawn(move || command_process(input.clone(), rx));

    (input, output, tx)
}

fn command_process<S>(
    input: Arc<SourcesQueueInput<S>>, 
    rx: Receiver<PlayerCommand>) {
    loop {
        if let Ok(cmd) = rx.try_recv() {
            match cmd {
                _ => {}
            }
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

pub struct SourcesQueueInput<S> {
    next_sounds: Mutex<VecDeque<(Box<dyn Source<Item = S> + Send>, Option<Sender<()>>)>>,

    /// only accessible in this struct
    prev_sounds: RefCell<Vec<Box<dyn Source<Item = S>>>>,

    keep_alive_if_empty: AtomicBool
}

impl<S> SourcesQueueInput<S>
where
    S: Sample + Send + 'static,
{
    #[inline]
    pub fn append<T>(&self, source: T)
    where
        T: Source<Item = S> + Send + 'static
    {
        self.next_sounds
            .lock()
            .unwrap()
            .push_back((Box::new(source) as Box<_>, None));
    }

    #[inline]
    pub fn append_with_signal<T>(&self, source: T) -> Receiver<()>
    where
        T: Source<Item = S> + Send + 'static
    {
        let (tx, rx) = mpsc::channel();
        self.next_sounds
            .lock()
            .unwrap()
            .push_back((Box::new(source) as Box<_>, Some(tx)));
        rx
    }

    pub fn clear(&self) -> usize {
        let prev_len = self.prev_sounds.borrow().len();
        let mut sounds = self.next_sounds.lock().unwrap();
        let next_len = sounds.len();
        sounds.clear();
        prev_len + next_len
    }

    pub fn set_keep_alive_if_empty(&self, keep_alive_if_empty: bool) {
        self.keep_alive_if_empty
            .store(keep_alive_if_empty, Ordering::Release);
    }
}

pub struct SourcesQueueOutput<S> {
    current: Box<dyn Source<Item = S> + Send>,
    signal_after_end: Option<Sender<()>>,
    input: Arc<SourcesQueueInput<S>>
}

const THRESHOLD: usize = 512;

impl<S> Source for SourcesQueueOutput<S>
where
    S: Sample + Send + 'static
{
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        if let Some(val) = self.current.current_frame_len() {
            if val != 0 {
                return Some(val);
            } else if self.input.keep_alive_if_empty.load(Ordering::Acquire)
                && self.input.next_sounds.lock().unwrap().is_empty() {
                    // The next source will be a filler silence which will
                    // have the length of `THRESHOLD`
                    return Some(THRESHOLD);
            }
        }

        let (lower_bound, _) = self.current.size_hint();

        if lower_bound > 0 {
            return Some(lower_bound);
        }

        Some(THRESHOLD)
    }

    #[inline]
    fn channels(&self) -> u16 {
        self.current.channels()
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        self.current.sample_rate()
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        self.current.total_duration()
    }
}

impl<S> Iterator for SourcesQueueOutput<S>
where
    S: Sample + Send + 'static
{
    type Item = S;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(sample) = self.current.next() {
                return Some(sample);
            }

            // pick the next sound in the go_next() method
            //TODO there is a possibility that go_next just happened
            // after user jumped to the next song.
            if self.go_next().is_err() {
                return None;
            }
        }
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.current.size_hint().0, None)
    }
}

impl<S> SourcesQueueOutput<S>
where
    S: Sample + Send + 'static
{
    fn go_next(&mut self) -> Result<(), ()> {
        if let Some(signal_after_end) = self.signal_after_end.take() {
            let _ = signal_after_end.send(());
        }

        let (next, signal_after_end) = {
            let mut next = self.input.next_sounds.lock().unwrap();

            if next.len() == 0 {
                let silence = Box::new(Zero::<S>::new_samples(1, 44100, THRESHOLD)) as Box<_>;

                if self.input.keep_alive_if_empty.load(Ordering::Acquire) {
                    (silence, None)
                } else {
                    return Err(());
                }
            } else {
                next.pop_front().unwrap()
            }
        };

        self.current = next;
        self.signal_after_end = signal_after_end;
        Ok(())
    }
}
