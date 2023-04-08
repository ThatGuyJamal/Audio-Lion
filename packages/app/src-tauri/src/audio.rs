use lazy_static::lazy_static;
use rodio::{Decoder, OutputStream, Sink, Source};
use specta::Type;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};

use crate::types::AppError;

pub struct AudioPlayer {}

impl AudioPlayer {}
