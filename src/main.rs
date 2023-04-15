use crate::emotions_app::EmotionsApp;
use crate::simple_ui::SimpleUI;

mod emotions_app;
mod gif_paintable;
mod gif_paintable_window;
mod res_loader;
mod simple_ui;

mod win_clip;

fn main() {

    let u="123";

    let lam=||println!("{}",u);
    p(u);
    lam();
     SimpleUI::start();
    // copy_file_demo::
    // win_clip::win_clip::main();
}

fn p(s:&str){
    println!("{}",s)
}