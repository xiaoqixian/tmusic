// Date: Sat Oct 21 22:46:28 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use std::time::Duration;

use mp3_duration::MP3DurationError;

use rodio::decoder::DecoderError;

mod source_stream;
mod play_queue;
mod listener;
mod player;

const THRESHOLD: usize = 512;

#[derive(Debug)]
pub enum PlayerError {
    IOError(std::io::Error),
    WrongFileType(String),
    DecoderError(DecoderError),
    MP3DurationError(MP3DurationError)
}

#[derive(Debug)]
enum RequestType {
    Sample,
    CurrentFrameLen,
    Channels,
    SampleRate,
    TotalDuration
}

#[allow(dead_code)]
#[derive(Debug)]
enum ResponseType {
    CurrentFrameLen(Option<usize>),
    Channels(u16),
    SampleRate(u32),
    TotalDuration(Option<Duration>)
}

pub trait Playback {
    type ListContainer;
    type ListHandle;

    fn append_list(&mut self, path: String) -> Result<(), PlayerError>;

    fn go_next(&mut self) -> Result<(), PlayerError>;

    fn go_prev(&mut self) -> Result<(), PlayerError>;

    fn play_next(&mut self, path: String) -> Result<(), PlayerError>;

    fn play(&mut self, path: String) -> Result<(), PlayerError>;

    fn set_paused(&mut self, paused: bool) -> Result<(), PlayerError>;

    fn total_duration(&self) -> Option<Duration>;

    fn progress(&self) -> Option<Duration>;

    fn get_song(&self) -> Option<String>;

    fn get_playlist(&self) -> Self::ListHandle
        where Self::ListContainer: IntoIterator,
              <<Self as Playback>::ListContainer as IntoIterator>::Item: Into<String>;
}
