use gtk::prelude::*;
use gtk::{
    gdk, gdk_pixbuf, gio, glib, gsk, ActionBar, Align, BoxLayout, Grid, GridLayout, Image, Inhibit,
    ListView, TextView,
};
use std::ops::Index;
use std::ptr::null;

use gtk::builders::GridLayoutBuilder;
use gtk::gdk::Display;
use gtk::glib::translate::FromGlibPtrFull;
use gtk::gsk::ffi::GskRenderNode;
use gtk::gsk::{ffi, CairoRenderer};
use gtk::{
    Application, ApplicationWindow, Box as Box_, Button, ComboBoxText, CssProvider, Entry,
    Orientation, StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION,
};

use gtk::gsk::RenderNode;
use gtk::gsk::RenderNodeType;
use gtk::gsk::Renderer;
use gtk::prelude::*;

pub struct SimpleUI {}

impl SimpleUI {
    pub fn start() -> glib::ExitCode {
        // gdk::set_allowed_backends("cairo");
        let application = Application::new(Some("com.github.css"), Default::default());
        application.connect_startup(|app| {
            // The CSS "magic" happens here.
            let provider = CssProvider::new();
            provider.load_from_data(include_str!("style.css"));
            // We give the CssProvided to the default screen so the CSS rules we added
            // can be applied to our window.
            StyleContext::add_provider_for_display(
                &Display::default().expect("Could not connect to a display."),
                &provider,
                STYLE_PROVIDER_PRIORITY_APPLICATION,
            );

            // We build the application UI.
            SimpleUI::build_ui(app);
        });
        application.run()
    }

    fn build_ui(application: &Application) {
        let grid = Grid::builder()
            .width_request(300)
            .height_request(300)
            .column_spacing(2)
            .row_spacing(2)
            .column_homogeneous(true)
            .orientation(Orientation::Horizontal)
            .css_classes(["image-grid"])
            .build();

        let vbox = Box_::new(Orientation::Vertical, 0);
        vbox.append(
            &TextView::builder()
                .width_request(300)
                .height_request(50)
                .build(),
        );
        vbox.append(&grid);
        vbox.set_margin_start(10);
        vbox.set_margin_end(10);
        vbox.set_margin_bottom(10);
        vbox.set_margin_top(10);

        let window = ApplicationWindow::builder()
            .application(application)
            .title("表情包管理器")
            .child(&vbox)
            .build();

        for (index, image) in SimpleUI::get_image_widgets().iter().enumerate() {
            println!(" {}-{} ", index % 3, index / 3);
            grid.attach(image, (index % 3) as i32, (index / 3) as i32, 1, 1)
        }

        // vbox.append(
        //     &Box_::builder()
        //         .width_request(100)
        //         .height_request(100)
        //         .css_classes(["image-box"])
        //         .vexpand(true)
        //         .hexpand(true)
        //         .build(),
        // );

        window.show();
        // Then we add the container inside our window.
        application.connect_activate(move |_| window.present());
    }

    fn build_image_by_url(url: &str) -> Image {
        let file = gio::File::for_uri(url)
            .read(None::<&gio::Cancellable>)
            .unwrap();

        let pixbuf = gdk_pixbuf::Pixbuf::from_stream(&file, None::<&gio::Cancellable>).unwrap();

        let image = Image::builder()
            .halign(Align::Center)
            .valign(Align::Center)
            .width_request(100)
            .height_request(100)
            .css_classes(["image"])
            .file(url)
            .build();
        image.set_from_pixbuf(Some(&pixbuf));
        image
    }

    fn get_image_widgets() -> Vec<Image> {
        let urls = [
            "https://img2.baidu.com/it/u=4130459079,902939219&fm=253&fmt=auto&app=120&f=JPEG?w=500&h=500",
            "https://img0.baidu.com/it/u=2973981695,449639631&fm=253&fmt=auto&app=120&f=JPEG?w=456&h=328",
            "https://img0.baidu.com/it/u=1355305794,2447146551&fm=253&fmt=auto&app=120&f=JPEG?w=300&h=300",
            "https://img2.baidu.com/it/u=2200213970,3195987466&fm=253&fmt=auto&app=138&f=JPEG?w=440&h=440",
            "https://img1.baidu.com/it/u=925909949,2724448435&fm=253&fmt=auto&app=120&f=JPEG?w=500&h=500",
            "https://img1.baidu.com/it/u=1434603982,4190874381&fm=253&fmt=auto&app=138&f=JPEG?w=500&h=500",
            "https://img0.baidu.com/it/u=3196165980,1615390016&fm=253&fmt=auto&app=138&f=JPEG?w=500&h=500"
        ];
        urls.map(SimpleUI::build_image_by_url).to_vec()
    }
}
