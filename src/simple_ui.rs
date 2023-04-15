use gtk::prelude::*;
use gtk::{gdk, gdk_pixbuf, gio, glib, gsk, ActionBar, Align, BoxLayout, Grid, GridLayout, Image, Inhibit, ListView, TextView, Fixed, GestureClick, HeaderBar, DragSource, DropTarget, Window, DropTargetAsync, pango, TextBuffer};
use std::ops::Index;
use std::path::PathBuf;
use std::ptr::null;

use gtk::builders::{FixedBuilder, GridLayoutBuilder};
use gtk::gdk::{ContentProvider, Display};
use gtk::glib::translate::FromGlibPtrFull;
use gtk::gsk::ffi::GskRenderNode;
use gtk::gsk::{ffi, CairoRenderer};
use gtk::{
    Application, ApplicationWindow, Box as Box_, Button, ComboBoxText, CssProvider, Entry,
    Orientation, StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use gtk::ffi::GtkHeaderBar;
use gtk::gdk::ffi::gdk_content_provider_new_typed;
use gtk::gio::ffi::{g_file_get_type, g_subprocess_launcher_set_environ};
use gtk::gio::File;
use gtk::glib::{Bytes, PropertyGet};
use gtk::glib::ffi::g_private_get;

use gtk::gsk::RenderNode;
use gtk::gsk::RenderNodeType;
use gtk::gsk::Renderer;
use gtk::pango::ffi::{pango_context_get_font_map, pango_context_new, pango_font_description_from_string, pango_font_description_set_family, PangoContext, PangoFontMapClass};
use gtk::pango::{Font, FontDescription, FontFamily, FontMap, Language};

use reqwest::Url;
use gtk::prelude::*;
use gtk::subclass::text_buffer;
use gtk::SystemSetting::FontConfig;
use pango::prelude::{FontFamilyExt};
pub struct SimpleUI {}

impl SimpleUI {

    pub fn start() -> glib::ExitCode {
        std::env::set_var("GSK_RENDERER","cairo");
        gio::resources_register_include!("composite.gresource")
            .expect("Failed to register resources.");
        // unsafe {
        //     let pango_context = pango_context_new();
        //     let font_map=font_m;
        //     let font_map_rs=FontMap::from_glib_full(font_map);
        //     let font_map_context=font_map_rs.create_context();
        //     font_map_rs.load_fontset(&font_map_context,&FontDescription::from_string(""),&Language::from_string(""));
        //
        // }
        let start_msg = "abc";

        let application = Application::new(Some("com.github.css"), Default::default());
        application.connect_startup(move |app| {
            // The CSS "magic" happens here.
            println!("{}",&start_msg);
            let provider = CssProvider::new();
            provider.load_from_resource(&("/resources/estyle.css"));
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

        println!("{}",start_msg);

        application.run_with_args(&vec![""])
    }

    fn build_ui(application: &Application) {
        let grid = Grid::builder()
            .width_request(300)
            .height_request(300)
            .column_spacing(2)
            .row_spacing(2)
            .hexpand(false)
            .hexpand_set(true)
            .vexpand(false)
            .vexpand_set(true)
            .orientation(Orientation::Horizontal)
            .css_classes(["image-grid"])
            .build();

        let vbox = Box_::builder().orientation(Orientation::Vertical)
            .width_request(300)
            .height_request(300)
            .hexpand(false)
            .hexpand_set(true)
            .build();
        let text_buffer=TextBuffer::builder().text("\u{E781}")
            .build();
        vbox.add_css_class("material-symbols-outlined");
        vbox.append(
            &TextView::builder()
                .buffer(&text_buffer)
                .css_classes(["material-symbols-outlined"])
                .width_request(300)
                .height_request(50)
                .build(),
        );
        vbox.append(&grid);
        vbox.set_margin_start(10);
        vbox.set_margin_end(10);
        vbox.set_margin_bottom(10);
        vbox.set_margin_top(10);
        vbox.set_size_request(300,300);

        dbg!(gtk::IconTheme::default().icon_names());

        let window = ApplicationWindow::builder()
            .application(application)
            .title("表情包管理器")
            .titlebar(&HeaderBar::builder().title_widget(
                &Button::builder()
                    .hexpand(true)
                    .hexpand_set(true)
                    .halign(Align::End)
                    .css_classes(["close-button","material-symbols-outlined"])
                    .icon_name("window-close")
                    .build()
            ).show_title_buttons(false).build())
            .child(&vbox)
            .build();

        window.remove_css_class("csd");
        dbg!(window.css_classes());
        dbg!(window.scale_factor());
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

    fn generate_uri_list(paths: &Vec<PathBuf>) -> Bytes {
        return Bytes::from_owned(
            paths
                .iter()
                .map(|path| -> String {
                    Url::from_file_path(path.canonicalize().unwrap())
                        .unwrap()
                        .to_string()
                })
                .reduce(|accum, item| [accum, item].join("\n"))
                .unwrap(),
        );
    }

    fn generate_content_provider_from_path(path: &PathBuf) -> ContentProvider {
        unsafe {
            let gfile = File::for_path(path);
            glib::translate::from_glib_full(gdk_content_provider_new_typed(
                g_file_get_type(),
                gfile,
            ))
        }
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

        let gesture_click=gtk::GestureClick::new();
        let url_string=url.to_string();

        // //text/uri-list
        //let file_list_content_provider =ContentProvider::for_bytes("text/uri-list;charset=utf-8", &uri_list);
        {
            let path_buf =PathBuf::from("C:/Users/Geek/Pictures/3.jpg");
            // let uri_list=SimpleUI::generate_uri_list(&vec![path_buf]);
            Display::default().unwrap().clipboard().set_content(Some(&SimpleUI::generate_content_provider_from_path(&path_buf))).unwrap();
        }

        // gesture_click.connect_pressed(move |gesture_click:&GestureClick, _:i32, _:f64, _:f64|{
        //     println!("{}","1");
        //     let path_buf =PathBuf::from("C:/Users/Geek/Pictures/3.jpg");
        //     // let uri_list=SimpleUI::generate_uri_list(&vec![path_buf]);
        //     // //text/uri-list
        //     // let file_list_content_provider =ContentProvider::for_bytes("text/uri-list;charset=utf-8", &uri_list);
        //     Display::default().unwrap().clipboard().set_content(Some(&SimpleUI::generate_content_provider_from_path(&path_buf))).unwrap();
        // });

        image.add_controller(gesture_click);
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
