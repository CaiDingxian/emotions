use crate::emotions_app::core::common_error::CommonError;
use crate::emotions_app::domain::ImageInfo;

use serde_json::Value;
use std::io;
use std::io::{Error, ErrorKind};
use std::sync::mpsc::Receiver;
use std::thread::{spawn, JoinHandle};

#[derive(Copy, Clone)]
pub struct PageSpec {
    pub(crate) current_page: i32,
    pub(crate) page_size: i32,
}

#[derive(Clone)]
pub struct EmotionSpec {
    pub(crate) keywords: String,
    pub page: PageSpec,
}

pub trait FindEmotion {
    fn load_emotion(&self, url: &String) -> Result<Box<[u8]>, CommonError>;
    fn search_emotions(&self, emotion_spec: &EmotionSpec) -> Result<Vec<String>, CommonError>;

    fn clone_box(&self) -> Box<dyn FindEmotion>;
}
