#![windows_subsystem = "windows"]
extern crate core;

use crate::emotions_app::EmotionsApp;
use crate::simple_ui::SimpleUI;

mod emotions_app;
mod gif_paintable;
mod gif_paintable_window;
mod res_loader;
mod simple_ui;

mod learn_test;
mod win_clip;

use crate::emotions_app::infra::EmotionFinderClient;

fn main() {
    let scale = std::env::args().nth(1);
    match scale {
        None => std::env::set_var("GDK_SCALE", "2"),
        Some(e) => std::env::set_var("GDK_SCALE", e),
    }
    std::env::set_var("GSK_RENDERER", "cairo");
    // // std::env::set_var("CLUTTER_SCALE", "2");
    // std::env::set_var("GDK_DPI_SCALE", "2");

    let mut simple_ui = SimpleUI::new();
    simple_ui.start(simple_ui.clone());
    // copy_file_demo::
    // WinClip::WinClip::main();
}
