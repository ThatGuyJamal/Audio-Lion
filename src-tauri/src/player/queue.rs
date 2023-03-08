use std::error::Error;
use std::ffi::OsStr;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::sync::{Arc, Mutex};

use rand::seq::SliceRandom;
use rand::thread_rng;
use symphonia::core::formats::FormatReader;

use super::Player;
use super::music_track::MusicTrack;
use super::types::{NError, TrackTime};

struct QueueTrack {
    format: Arc<Mutex<Box<dyn FormatReader>>>,
    name: String,
}

pub struct QueuePlayer {
    queue: Vec<QueueTrack>,
    player: Player,
    index: usize,
}

impl QueuePlayer {
    pub fn new(app_name: String) -> Self {
        let player = Player::new(app_name, 1.0, 1.0);

        QueuePlayer {
            queue: vec![],
            player,
            index: usize::MAX - 1,
        }
    }

    pub fn add<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Box<dyn Error>>
    where
        P: AsRef<OsStr>,
    {
        let track = MusicTrack::new(path)?;

        self.queue.push(QueueTrack {
            format: Arc::new(Mutex::new(track.get_format())),
            name: track.name().to_string(),
        });

        Ok(())
    }

    pub fn remove(&mut self, index: usize) {
        self.queue.remove(index);
    }

    pub fn clear(&mut self) {
        self.queue.clear();
        self.index = usize::MAX - 1;
    }

    pub fn shuffle(&mut self) {
        self.queue.shuffle(&mut thread_rng());
    }

    pub fn current_track_name(&self) -> &str {
        if self.index == usize::MAX - 1 {
            return &self.queue.get(0).unwrap().name;
        }

        &self.queue.get(self.index).unwrap().name
    }

    pub fn play(&mut self, app_handle: tauri::AppHandle, index: usize) {
        self.index = index;

        self.player.play(app_handle, self.queue[self.index].format.clone());
    }

    pub fn play_next(&mut self, app_handle: tauri::AppHandle) {
        self.index += 1;

        if self.index >= self.queue.len() {
            self.index = 0;
        }

        self.player.play(app_handle, self.queue[self.index].format.clone());
    }

    pub fn play_previous(&mut self, app_handle: tauri::AppHandle) {
        if self.index == 0 {
            self.index = self.queue.len();
        }

        self.index -= 1;

        self.player.play(app_handle, self.queue[self.index].format.clone());
    }

    pub fn get_index_from_track_name(&self, name: &str) -> Result<usize, NError> {
        for (index, track) in self.queue.iter().enumerate() {
            if track.name == name {
                return Ok(index);
            }
        }

        Err(NError::NoTrack)
    }

    pub fn get_duration_for_track(&self, index: usize) -> TrackTime {
        let tmp = self.queue.get(index).unwrap().format.clone();
        let format = tmp.lock().unwrap();

        let track = format.default_track().expect("Can't load tracks");
        let time_base = track.codec_params.time_base.unwrap();

        let duration = track
            .codec_params
            .n_frames
            .map(|frames| track.codec_params.start_ts + frames)
            .unwrap();
        let time = time_base.calc_time(duration);

        TrackTime {
            ts_secs: 0,
            ts_frac: 0.0,
            dur_secs: time.seconds,
            dur_frac: time.frac,
        }
    }
}

impl Deref for QueuePlayer {
    type Target = Player;

    fn deref(&self) -> &Self::Target {
        &self.player
    }
}

impl DerefMut for QueuePlayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.player
    }
}