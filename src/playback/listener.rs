// Date: Sat Oct 21 23:28:23 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use std::sync::Arc;
use std::time::Duration;

use rodio::{
    source::Source,
    Sample,
    cpal::FromSample
};

use kanal::{Sender, Receiver};

use super::{
    RequestType,
    ResponseType,
    play_queue::PlayQueue,
    THRESHOLD
};

pub const BUFFER_SIZE: usize = 64;

pub struct NoticeListener<S, I> {
    play_queue: Arc<PlayQueue<S>>,
    response_sender: Sender<ResponseType>,
    request_receiver: Receiver<RequestType>,
    sample_sender: Sender<I>
}

impl<S, I> NoticeListener<S, I> {
    pub fn new(
        play_queue: Arc<PlayQueue<S>>,
        response_sender: Sender<ResponseType>,
        request_receiver: Receiver<RequestType>,
        sample_sender: Sender<I>
    ) -> Self {
        Self {
            play_queue,
            response_sender,
            request_receiver,
            sample_sender
        }
    }

}

impl<I> NoticeListener<Box<dyn Source<Item = I> + Send>, I>
where
    I: Sample + Send + 'static + FromSample<i16>
{
    pub fn run(&mut self) {
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
        if let Some(val) = self.play_queue.current_frame_len() {
            if val > 0 {
                return Some(val);
            } else if self.play_queue.is_empty() {
                return Some(THRESHOLD);
            }
        }

        let (lower_bound, _) = self.play_queue.size_hint();

        if lower_bound > 0 {
            return Some(lower_bound);
        }

        Some(THRESHOLD)
    }

    #[inline]
    fn channels(&self) -> u16 {
        self.play_queue.channels()
    }

    #[inline]
    fn sample_rate(&self) -> u32 {
        self.play_queue.sample_rate()
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        //self.play_queue.current.lock().unwrap().0.total_duration()
        self.play_queue.total_duration()
    }
}

