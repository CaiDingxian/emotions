use std::collections::HashMap;
use std::io::Read;
extern crate reqwest;
pub struct ResLoader {}

impl ResLoader {
    pub fn test_load() -> reqwest::Result<()> {
        let text = reqwest::blocking::get("http://www.baidu.com")?
            .text()
            .expect("TODO: panic message");
        println!("{}", text);
        Ok(())
    }
}
