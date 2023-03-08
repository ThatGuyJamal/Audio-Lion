use serde::{Deserialize, Serialize};
use specta::Type;
use symphonia::core::units::Time;

#[derive(Debug)]
pub enum NError {
    NoTrack,
}

/// Messages sent inside the `Player`
pub enum Message {
    Play,
    Pause,
    End,
    Exit,
    Seek(Time),
    Time(TrackTime),
    Volume(f32),
    PlaybackSpeed(f32),
}

/// Used to represent the timestamp
/// ts_* is used to represent the *current* timestamp (as in where is currently the player playing inside the track)
/// dur_* is used to represent the *entire* timestamp (as is how long is the track)
#[derive(Clone, Debug)]
pub struct TrackTime {
    pub ts_secs: u64,
    pub ts_frac: f64,
    pub dur_secs: u64,
    pub dur_frac: f64,
}