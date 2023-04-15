use std::io;
use std::sync::mpsc::Receiver;

pub struct PageSpec{
    current_page:i32,
    item_per_page:i32
}

pub struct EmotionSpec {
    pub(crate) keywords:String
}

pub trait EmotionSearchClient {

    fn search_emotions(&self, page_query:&EmotionSpec)->Receiver<Box<str>>;
}