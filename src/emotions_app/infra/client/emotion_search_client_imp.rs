use std::io;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{channel, Receiver};
use std::thread::spawn;
use gtk::glib::{clone, MainContext, Priority, PropertyGet, spawn_async};
use gtk::prelude::BufferedInputStreamExt;
use serde_json::{json, Value};
use crate::emotions_app::app::{EmotionSearchClient, EmotionSpec};


struct EmotionSearchClientImpl{
}

fn test_search(){
    let searcher=EmotionSearchClientImpl::new();
    searcher.search_emotions(&EmotionSpec{keywords:"1".to_string()});
    searcher.search_emotions(&EmotionSpec{keywords:"1".to_string()});
}

impl EmotionSearchClientImpl{
    pub fn new() -> Self {
       Self{}
    }

    fn internal_search_emotions(&self, emotion_spec: &EmotionSpec) -> io::Result<Vec<String>> {
        let keyword=urlencoding::encode("");
        let base_url="https://m.baidu.com/sf/vsearch/image/search/wisesearchresult".to_string();
        let search_url=base_url+"?tn=wisejsonala&ie=utf-8&fromsf=1&word="+&keyword+
            "&pn=60&rn=30&gsm=3c&prefresh=undefined&fp=result&searchtype=0&fromfilter=0&tpltype=0";

        let search_result=reqwest::blocking::get(search_url);
        let body=search_result.expect("搜索失败").text();
        let json_body: Value=serde_json::from_str(&body.expect("无效结果")).unwrap();
        let image_res_list:Vec<String> = json_body["linkData"].as_array().unwrap().into_iter()
            .map(|e| e.as_object().unwrap()["objurl"].as_str().unwrap().to_string()).collect();
        Ok(image_res_list)
    }


}

impl EmotionSearchClient for EmotionSearchClientImpl{

    fn search_emotions(&self, emotion_spec: &EmotionSpec)->Receiver<Box<str>>{
        let (sender, receiver) = channel();
        let searcher=Arc::new(EmotionSearchClientImpl::new());
        spawn(move ||{
            searcher.internal_search_emotions(&EmotionSpec { keywords: "123".to_string() }).expect("error");
            sender.send(Box::from("1")).expect("TODO: panic message");
        });
        let (s,r)=MainContext::channel(Priority::default());
        s.send("a");
        r.attach()
        receiver
    }
}
