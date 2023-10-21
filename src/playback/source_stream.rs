// Date: Sat Oct 21 22:48:35 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use std::time::Duration;

use rodio::{
    source::Source,
    Sample,
    cpal::FromSample
};

use kanal::{Receiver, Sender, ReceiveError};

use super::{RequestType, ResponseType};

// I is the type of sample transfered
// from the channel
// I need to be converted to D in next()
pub struct SourceStream<I, D> {
    response_receiver: Receiver<ResponseType>,
    request_sender: Sender<RequestType>,
    sample_receiver: Receiver<I>,
    phantom: std::marker::PhantomData<D>
}

impl<I, D> SourceStream<I, D> {
    pub fn new(
        rsp_rx: Receiver<ResponseType>,
        req_tx: Sender<RequestType>,
        smp_rx: Receiver<I>
    ) -> Self {
        Self {
            response_receiver: rsp_rx,
            request_sender: req_tx,
            sample_receiver: smp_rx,
            phantom: std::marker::PhantomData
        }
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

                match self.sample_receiver.recv() {
                    Ok(smp) => Some(D::from_sample(smp)),
                    Err(ReceiveError::Closed) => panic!("both sides closed"),
                    Err(ReceiveError::SendClosed) => panic!("sender closed")
                }
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
        self.request_sender
            .send(RequestType::CurrentFrameLen)
            .expect("request_sender send error");

        match self.response_receiver
            .recv()
            .expect("response_receiver recv error") 
        {
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
