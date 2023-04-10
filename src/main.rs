use crate::emotions_app::EmotionsApp;
use crate::simple_ui::SimpleUI;

mod emotions_app;
mod gif_paintable;
mod gif_paintable_window;
mod res_loader;
mod simple_ui;

fn main() {
    SimpleUI::start();
}
