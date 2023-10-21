// Date: Sat Oct 21 23:48:23 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use std::{
    sync::{Arc, Mutex},
    collections::VecDeque,
    time::Duration
};

use rodio::{
    OutputStream,
    OutputStreamHandle,
    Sample,
    source::Source,
    cpal::FromSample
};

use kanal::bounded;

use super::{
    PlayerError,
    Playback,
    play_queue::PlayQueue,
    source_stream::SourceStream,
    RequestType,
    ResponseType,
    listener::{BUFFER_SIZE, NoticeListener}
};

pub struct Player<S> {
    play_queue: Arc<PlayQueue<S>>,
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
}

impl<I> Player<Box<dyn Source<Item = I> + Send>>
where 
    I: Sample + Send + FromSample<i16> + 'static,
    f32: FromSample<I>
{
    fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();

        let (request_sender, request_receiver) = bounded::<RequestType>(0);
        let (response_sender, response_receiver) = bounded::<ResponseType>(0);
        let (sample_sender, sample_receiver) = bounded::<I>(BUFFER_SIZE);

        let source_stream = SourceStream::<I, f32>::new(
            response_receiver,
            request_sender,
            sample_receiver,
        );

        let play_queue = Arc::new(PlayQueue::<Box<dyn Source<Item = I> + Send>>::new());

        let mut listener = NoticeListener::<Box<dyn Source<Item = I> + Send>, I>::new(
            play_queue.clone(),
            response_sender,
            request_receiver,
            sample_sender
        );

        // spawn a notice listener
        let _ = std::thread::spawn(move || listener.run());

        let _ = stream_handle.play_raw(source_stream).unwrap();

        Self {
            play_queue,
            _stream,
            _stream_handle: stream_handle,
        }
    }
}

impl<I> Playback for Player<Box<dyn Source<Item = I> + Send>>
where 
    I: Sample + Send + FromSample<i16> + 'static,
    f32: FromSample<I>
{
    type ListContainer = VecDeque<String>;
    type ListHandle = Arc<Mutex<Self::ListContainer>>;

    #[inline]
    fn get_song(&self) -> Option<String> {
        self.play_queue.get_song()
    }

    #[inline]
    fn get_playlist(&self) -> Self::ListHandle {
        self.play_queue.get_playlist()
    }

    #[inline]
    fn append_list(&mut self, path: String) -> Result<(), PlayerError>{
        self.play_queue.append(path)
    }

    #[inline]
    fn go_next(&mut self) -> Result<(), PlayerError> {
        self.play_queue.go_next_ignore_repeat(true)
    }

    #[inline]
    fn go_prev(&mut self) -> Result<(), PlayerError> {
        self.play_queue.go_prev()
    }

    #[inline]
    fn play(&mut self, path: String) -> Result<(), PlayerError> {
        self.play_queue.play(path)
    }

    #[inline]
    fn play_next(&mut self, path: String) -> Result<(), PlayerError> {
        self.play_queue.play_next(path)
    }

    #[inline]
    fn set_paused(&mut self, paused: bool) -> Result<(), PlayerError> {
        self.play_queue.set_paused(paused);
        Ok(())
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        self.play_queue.total_duration()
    }

    #[inline]
    fn progress(&self) -> Option<Duration> {
        self.play_queue.progress()
    }
}
