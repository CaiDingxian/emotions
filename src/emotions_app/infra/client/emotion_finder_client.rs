use crate::emotions_app::app::{EmotionSpec, FindEmotion};
use crate::emotions_app::core::common_error::{CommonError, ErrorType};
use gtk::glib::clone;
use image::EncodableLayout;
use reqwest::blocking::{Client, Request};
use reqwest::header::HeaderMap;
use reqwest::Method;
use serde_json::Value;
use std::io::{Bytes, Error, ErrorKind, Read};
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use std::thread;
use std::thread::{spawn, JoinHandle, Thread};
use std::time::Duration;

#[derive(Clone)]
pub struct EmotionFinderClient {
    client: Arc<Client>,
}
unsafe impl Sync for EmotionFinderClient {}
unsafe impl Send for EmotionFinderClient {}
impl EmotionFinderClient {
    pub fn new() -> Self {
        let client = Client::builder()
            .gzip(false)
            .connect_timeout(Duration::from_millis(1500))
            .timeout(Duration::from_millis(2500))
            .build()
            .unwrap();
        Self {
            client: Arc::new(client),
        }
    }

    fn build_url(emotion_spec: &EmotionSpec) -> String {
        //百度图片搜索基本地址
        let base_url = "https://m.baidu.com/sf/vsearch/image/search/wisesearchresult".to_string();

        //格式化字符串
        let search_url = format!(
            "{base_url}?tn=wisejsonala&ie=utf-8&fromsf=1&word={keywords}&pn={item_no}&rn={page_size}&gsm=3c&prefresh=undefined&fp=result&searchtype=0&fromfilter=0&tpltype=0",
            base_url = base_url,
            keywords = urlencoding::encode(&format!("{},{}",emotion_spec.keywords," 表情 gif")).clone(),
            item_no = emotion_spec.page.current_page * emotion_spec.page.page_size,
            page_size = emotion_spec.page.page_size
        );
        search_url
    }

    fn preflight_emotion(
        client: Arc<Client>,
        url: String,
    ) -> Result<HeaderMap, Box<dyn std::error::Error + Send + Sync>> {
        let client = client.clone();

        let response = client.execute(Request::new(Method::HEAD, url.parse()?))?;
        let header_map = response.headers();
        let header_map = header_map.clone();
        return Ok(header_map);
    }
}

impl FindEmotion for EmotionFinderClient {
    fn load_emotion(&self, url: &String) -> Result<Box<[u8]>, CommonError> {
        let url_clone = url.clone();

        let client = self.client.clone();

        let headers =
            EmotionFinderClient::preflight_emotion(client.clone(), url_clone.parse().unwrap())
                .unwrap();

        println!("{:?} - {}", thread::current().id(), url_clone.clone());

        let size = headers
            .get("content-length")
            .expect("无效的大小信息")
            .to_str()
            .map_err(|e| CommonError::new(ErrorType::IoError, None, None))?
            .parse::<u32>()
            .map_err(|e| CommonError::new(ErrorType::IoError, None, None))?;
        println!("{:?}大小：{}", thread::current().id(), size);

        if (size > 500_000) {
            Err(CommonError::new(ErrorType::ParingError, None, None))
        } else {
            // let bytes = reqwest::blocking::get(url_clone)?.bytes()?;
            let bytes = client
                .execute(Request::new(Method::GET, url_clone.parse().unwrap()))
                .map_err(|e| CommonError::new(ErrorType::IoError, None, None))?
                .bytes()
                .map_err(|e| CommonError::new(ErrorType::IoError, None, None))?;

            Ok(Box::from(bytes.as_ref()))
        }
    }

    fn search_emotions(&self, emotion_spec: &EmotionSpec) -> Result<Vec<String>, CommonError> {
        //对搜索关键字进行Url编码
        let keyword = urlencoding::encode(&emotion_spec.keywords);

        //拼接搜索Url
        let search_url = Self::build_url(emotion_spec);
        //阻塞方式请求URL
        let search_result = reqwest::blocking::get(search_url);

        //定义加载失败构建
        let make_load_error = || Error::new(ErrorKind::NotFound, "加载失败");

        //执行搜索，获取结果文本
        let body = search_result?.text()?;
        //解析json
        let json_body: Value = serde_json::from_str(&body)?;
        //从json中获取图片url列表
        let image_res_list: Vec<Result<String, Error>> = json_body["linkData"]
            .as_array()
            .ok_or(make_load_error())?
            .into_iter()
            .map(|e| {
                e.as_object().ok_or(make_load_error())?["objurl"]
                    .as_str()
                    .ok_or(make_load_error())
                    .map(|e| e.to_string())
            })
            .collect();

        //若其中有一个url失败，显示加载失败
        if !image_res_list.iter().all(|e| e.is_ok()) {
            Err(CommonError::from(Error::new(
                ErrorKind::NotFound,
                "加载失败",
            )))
        } else {
            Ok(image_res_list.into_iter().map(|e| e.unwrap()).collect())
        }
    }

    fn clone_box(&self) -> Box<dyn FindEmotion> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::emotions_app::app::queries::emotion_search_client::FindEmotion;
    use crate::emotions_app::app::PageSpec;
    use windows::h;
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test_search() {
        let client = EmotionFinderClient::new();
        let spec = EmotionSpec {
            keywords: "动态表情".to_string(),
            page: PageSpec {
                current_page: 0,
                page_size: 10,
            },
        };
        let result = client.search_emotions(&spec).unwrap();
        println!("{:?}", result)
    }

    #[test]
    fn test_head() {
        let response = reqwest::blocking::Client::builder()
            .gzip(false)
            .connect_timeout(Duration::from_millis(1500))
            .build()
            .unwrap()
            .execute(Request::new(Method::HEAD, "https://5b0988e595225.cdn.sohucs.com/images/20190707/b135698746f242868e0d62a611217aac.gif".parse().unwrap()))
            .unwrap();

        println!("{:?}", response.headers());
    }
}
