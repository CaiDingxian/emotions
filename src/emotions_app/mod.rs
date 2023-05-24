// A custom GDK paintable capable of rendering a GIF
// The paintable makes uses of the awesome image
// crate to read a gif file and transform it to a Vec<Frame>
// which are then rendered by the paintable at different snapshots

use gtk::prelude::*;
use gtk::{gdk, glib};

use crate::gif_paintable_window::GifPaintableWindow;
use gtk::prelude::{ApplicationExt, ApplicationExtManual};
pub(crate) mod app;
pub(crate) mod core;
mod domain;
pub(crate) mod infra;
mod start;

pub struct EmotionsApp {}

impl EmotionsApp {
    //使用默认的GPU绘图方式时，GTK会申请很多的内存，并不乐意释放，这并非内存泄露
    //GSK_RENDERER=cairo可以避免内存申请过多
    //也有人说 OpenGL 驱动程序通过将 GPU 的内存映射到它们的内存空间来玩一些技巧，然后内核将其计为额外内存。
    //受影响的不是 RAM，所有变化都是一个数字。
    pub fn start() -> glib::ExitCode {
        // gdk::set_allowed_backends("wayland");
        let application = gtk::Application::new(Some("com.voidgeek.emotions"), Default::default());

        application.connect_activate(|app| {
            let win = GifPaintableWindow::new(app);
            win.present()
        });

        application.run()
    }
}
