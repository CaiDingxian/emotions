use std::cell::{Cell, RefCell};
use std::env::args;
use std::error::Error;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::ops::{Deref, DerefMut, Index, Not};
use std::path::{Path, PathBuf};
use std::ptr::null;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread::{spawn, JoinHandle};
use std::time::Instant;

use gtk::builders::{FixedBuilder, GridLayoutBuilder};
use gtk::ffi::GtkHeaderBar;
use gtk::gdk::ffi::gdk_content_provider_new_typed;
use gtk::gdk::{ContentProvider, Display};
use gtk::gio::ffi::{g_file_get_type, g_subprocess_launcher_set_environ};
use gtk::gio::SimpleAction;
use gtk::glib::ffi::g_private_get;
use gtk::glib::translate::FromGlibPtrFull;
use gtk::glib::value::FromValue;
use gtk::glib::{
    clone, closure_local, Bytes, MainContext, PropertyGet, PropertySet, RustClosure, VariantTy,
};
use gtk::gsk::ffi::GskRenderNode;
use gtk::gsk::RenderNode;
use gtk::gsk::RenderNodeType;
use gtk::gsk::Renderer;
use gtk::gsk::{ffi, CairoRenderer};
use gtk::pango::ffi::{
    pango_context_get_font_map, pango_context_new, pango_font_description_from_string,
    pango_font_description_set_family, PangoContext, PangoFontMapClass,
};
use gtk::pango::{Font, FontDescription, FontFamily, FontMap, Language};
use gtk::prelude::*;
use gtk::prelude::*;
use gtk::subclass::{application, text_buffer};
use gtk::AccessibleRole::TextBox;
use gtk::ConstraintStrength::Weak;
use gtk::SystemSetting::FontConfig;
use gtk::{
    gdk, gdk_pixbuf, gio, glib, gsk, pango, ActionBar, Align, BoxLayout, DragSource, DropTarget,
    DropTargetAsync, Fixed, GestureClick, Grid, GridLayout, HeaderBar, Image, Inhibit, Label,
    ListView, Picture, PolicyType, ScrolledWindow, Text, TextBuffer, TextView, Widget, Window,
    WrapMode,
};
use gtk::{
    Application, ApplicationWindow, Box as Box_, Button, ComboBoxText, CssProvider, Entry,
    Orientation, StyleContext, STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use image::EncodableLayout;
use pango::prelude::FontFamilyExt;
use reqwest::Url;
use windows::s;

use crate::emotions_app::app::{EmotionSpec, FindEmotion, PageSpec};
use crate::emotions_app::core::*;
use crate::emotions_app::infra::EmotionFinderClient;
use crate::gif_paintable;
use crate::gif_paintable::{Frame, GifPaintable};
use crate::gif_paintable_window::GifPaintableWindow;
use crate::win_clip::WinClip;

pub struct SimpleUI {
    client: Arc<dyn FindEmotion + Send + Sync>,
    application: Application,
    query: RefCell<EmotionSpec>,
    loading: Rc<Mutex<bool>>,
}

impl Clone for Box<dyn FindEmotion> {
    fn clone(&self) -> Box<dyn FindEmotion> {
        self.clone_box()
    }
}

impl SimpleUI {
    pub fn new() -> Arc<SimpleUI> {
        gio::resources_register_include!("composite.gresource")
            .expect("Failed to register resources.");
        let s = Self {
            client: Arc::new(EmotionFinderClient::new()),
            application: Application::new(Some("com.github.css"), Default::default()),
            query: RefCell::new(EmotionSpec {
                keywords: "".to_string(),
                page: PageSpec {
                    current_page: 0,
                    page_size: 18,
                },
            }),
            loading: Rc::new(Mutex::new(false)),
        };
        Arc::new(s)
    }

    pub fn start(&self, self_arc: Arc<Self>) {
        let mut current_dir = std::env::current_dir().unwrap();
        current_dir.push("emo_res");
        std::fs::create_dir_all(current_dir.clone()).unwrap();
        // unsafe {
        //     let pango_context = pango_context_new();
        //     let font_map=font_m;
        //     let font_map_rs=FontMap::from_glib_full(font_map);
        //     let font_map_context=font_map_rs.create_context();
        //     font_map_rs.load_fontset(&font_map_context,&FontDescription::from_string(""),&Language::from_string(""));
        //
        // }

        // let rc_self = Arc::new(self);
        // let rc_self2 = rc_self.clone();
        let self2 = self_arc.clone();
        let lambda = move |temp_app: &Application| {
            // The CSS "magic" happens here.
            let provider = CssProvider::new();
            provider.load_from_resource(&("/resources/estyle.css"));
            // We give the CssProvided to the default screen so the CSS rules we added
            // can be applied to our window.
            StyleContext::add_provider_for_display(
                &Display::default().expect("Could not connect to a display."),
                &provider,
                STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
            Self::build_ui(&self2, self2.clone(), temp_app);

            // We build the application UI.
        };

        self.application.connect_startup(lambda);
        self.application.run();
        //self.application.run_with_args(&vec![""]);
    }

    fn gen_res_url(net_url: &str, ext_name: &str) -> String {
        let mut current_dir = std::env::current_dir().unwrap();
        current_dir.push("emo_res");

        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        net_url.hash(&mut hasher);
        let hash_str = hasher.finish();

        current_dir.push(hash_str.to_string() + ext_name);

        let url = format!("{}", current_dir.to_str().unwrap());
        url
    }

    fn fetch_emotions(
        client: Arc<dyn FindEmotion + Send + Sync>,
        spec: &EmotionSpec,
    ) -> Arc<Vec<(String, (Box<[u8]>, Vec<Frame>))>> {
        let start = Instant::now();
        //
        let emotions = client.clone().search_emotions(spec);

        println!(
            "搜索用时：{}",
            Instant::now().duration_since(start.clone()).as_millis()
        );

        let client = client.clone();

        let emotions: Vec<_> = emotions
            .unwrap()
            .into_iter()
            .map(|url| {
                let client = client.clone();
                let url_clone = url.clone();
                let join_handle: JoinHandle<Result<(Box<[u8]>, Vec<Frame>), CommonError>> =
                    spawn(move || {
                        let bytes = client.load_emotion(&url.clone())?;
                        let frames = GifPaintable::decode_to_frames(bytes.as_ref())?;
                        Ok((bytes, frames))
                    });

                (url_clone, join_handle)
            })
            .collect();

        //
        let emotions: Vec<_> = emotions
            .into_iter()
            .map(|(url, e)| (url, e.join()))
            .filter(|(url, e)| e.is_ok())
            .map(|(url, e)| (url, e.unwrap()))
            .filter(|(url, e)| e.is_ok())
            .map(|(url, e)| (url, e.unwrap()))
            .collect();

        let emotions = Arc::new(emotions);

        let emotions2 = emotions.clone();

        for (index, (url, (bytes, frames))) in emotions2.iter().enumerate() {
            let url = Arc::new(url.clone());
            let image = Arc::new(bytes.clone());
            spawn(move || {
                println!("{}", &url);

                File::create(Path::new(&Self::gen_res_url(&url, ".gif")))
                    .unwrap()
                    .write(&image);
            });
        }

        emotions
    }

    fn build_ui(&self, self_arc: Arc<Self>, app: &Application) {
        println!("OK");
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

        let vbox = Box_::builder()
            .orientation(Orientation::Vertical)
            .width_request(300)
            .height_request(300)
            .hexpand(false)
            .hexpand_set(true)
            .build();

        vbox.add_css_class("material-symbols-outlined");
        let text_entry = Entry::builder()
            .css_classes(["material-symbols-outlined", "search-box"])
            .width_request(300)
            .height_request(40)
            .build();

        vbox.append(&text_entry);
        let scrolled_win = ScrolledWindow::builder()
            .hscrollbar_policy(PolicyType::Never)
            .vexpand(true)
            .vexpand_set(true)
            .child(&grid)
            .build();

        let g1 = Rc::new(grid.clone());

        let mut a = Mutex::new(true);
        let g2 = Rc::new(grid.clone());

        let client = self.client.clone();
        let query = Rc::new(self.query.clone().into_inner());

        let self_arc2 = self_arc.clone();

        vbox.append(&scrolled_win);
        vbox.set_margin_start(10);
        vbox.set_margin_end(10);
        vbox.set_margin_bottom(10);
        vbox.set_margin_top(10);
        vbox.set_size_request(300, 300);

        dbg!(gtk::IconTheme::default().icon_names());

        let title_label = Rc::new(
            Label::builder()
                .width_request(120)
                .height_request(20)
                .css_classes(["title-label"])
                .label("表情管理器")
                .build(),
        );

        let title_label_clone = title_label.clone();

        let header_box = HeaderBar::builder()
            .title_widget(title_label_clone.as_ref())
            .css_classes(["header-box"])
            .height_request(20)
            .show_title_buttons(false)
            .build();

        header_box.pack_start(&Box_::builder().width_request(80).build());

        let title_label_clone = title_label.clone();
        scrolled_win
            .vadjustment()
            .connect_value_changed(move |adj| {
                let last = g2.last_child();

                let (x, y, w, h) = match last {
                    None => (0, 0, 1, 1),
                    Some(e) => g2.query_child(&e),
                };

                let value = adj.value();
                let upper = adj.upper();
                let page_size = adj.page_size();
                let position = value / (upper - page_size);

                if position >= 1.0 && { !*self_arc2.loading.lock().unwrap() } {
                    {
                        self_arc2.loading.set(true);
                    }
                    self_arc2
                        .application
                        .activate_action("my_fetch_emotions", Some(&y.to_variant()));
                }
            });

        let self_arc2 = self_arc.clone();
        let g2 = g1.clone();
        let title_label_clone = title_label.clone();

        let fetch_action = SimpleAction::new("my_fetch_emotions", Some(VariantTy::INT32));

        self_arc2.application.add_action(&fetch_action);
        fetch_action.connect_activate(move |args, y| {
            let page = self_arc2.query.clone().into_inner().page.clone();
            let y: i32 = y.unwrap().get().unwrap();
            //args[0].get::<i32>().unwrap();
            let keyword = self_arc2.query.clone().into_inner().keywords.clone();
            self_arc2.query.replace(EmotionSpec {
                keywords: keyword,
                page: PageSpec {
                    page_size: page.page_size,
                    current_page: page.current_page + 1,
                },
            });

            let emotions =
                Self::fetch_emotions(client.clone(), self_arc2.query.borrow().deref()).to_vec();

            for (index, (url, (bytes, frames))) in emotions.clone().into_iter().enumerate() {
                g2.attach(
                    &Self::build_image_by_url(&url.clone(), frames.clone()),
                    (index % 3) as i32,
                    ((index / 3) as i32 + y),
                    1,
                    1,
                )
            }
            {
                self_arc2.loading.set(false);
            }
        });

        let g2 = g1.clone();
        let client = self.client.clone();
        let g2 = g1.clone();
        let self2 = self_arc.clone();

        &text_entry.connect_activate(move |e| {
            let search = e.text().to_string();
            loop{
                match &g2.first_child() {
                    Some(v)=>g2.remove(v),
                    None => break,
                }
            }
            if search.is_empty() {
                return;
            }
            let main_context = MainContext::default();
            let client = client.clone();
            let self2=self2.clone();
            let query = Rc::new(self2.query.clone().into_inner());

            let page=self2.clone().query.clone().into_inner().page;

            self2.clone().query.replace(EmotionSpec{
                keywords:search.clone(),
                page:PageSpec{
                    page_size:page.page_size,
                    current_page:0
                }
            });

            let self3=self2.clone();
            let handle = main_context.spawn_local(glib::clone!(@weak g2 => async move {

                let emotions=Self::fetch_emotions(client,&self2.clone().query.clone().into_inner()).clone().to_vec();


                for (index,(url,(bytes,frames))) in emotions.clone().into_iter().enumerate() {
                    g2.attach(&Self::build_image_by_url(&url.clone(),frames.clone()), (index % 3) as i32, (index / 3) as i32, 1, 1)
                }
            }));
            self3.loading.set(false);
        });

        let close_btn = &Button::builder()
            .css_classes(["close-button"])
            .margin_end(10)
            .build();

        let app2 = app.clone();

        close_btn.connect_clicked(move |_| {
            app2.quit();
        });

        header_box.pack_end(close_btn);

        let window = ApplicationWindow::builder()
            .application(app)
            .titlebar(&header_box)
            .child(&vbox)
            .build();

        window.remove_css_class("csd");
        dbg!(window.css_classes());
        dbg!(window.scale_factor());
        // for (index, image) in SimpleUI::get_image_widgets().iter().enumerate() {
        //     // println!(" {}-{} ", index % 3, index / 3);
        //     grid.attach(image, (index % 3) as i32, (index / 3) as i32, 1, 1)
        // }

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
        app.connect_activate(move |_| window.present());
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
            let gfile = gio::File::for_path(path);
            glib::translate::from_glib_full(gdk_content_provider_new_typed(
                g_file_get_type(),
                gfile,
            ))
        }
    }

    fn build_image_by_url(url: &str, frames: Vec<Frame>) -> Picture {
        // let file = gio::File::for_uri(url)
        //     .read(None::<&gio::Cancellable>)
        //     .unwrap();
        //
        // let pixbuf = gdk_pixbuf::Pixbuf::from_stream(&file, None::<&gio::Cancellable>).unwrap();

        let gif_paintable = GifPaintable::new();

        gif_paintable.load_from_frames(frames);

        let image = Picture::builder()
            .halign(Align::Center)
            .valign(Align::Center)
            .width_request(100)
            .height_request(100)
            .css_classes(["image"])
            .paintable(&gif_paintable)
            .build();

        let gesture_click = gtk::GestureClick::new();
        let url_string = url.to_string();

        // //text/uri-list
        //let file_list_content_provider =ContentProvider::for_bytes("text/uri-list;charset=utf-8", &uri_list);
        // {
        //     let path_buf = PathBuf::from("C:/Users/Geek/Pictures/3.jpg");
        //     // let uri_list=SimpleUI::generate_uri_list(&vec![path_buf]);
        //     Display::default()
        //         .unwrap()
        //         .clipboard()
        //         .set_content(Some(&SimpleUI::generate_content_provider_from_path(
        //             &path_buf,
        //         )))
        //         .unwrap();
        // }

        gesture_click.connect_pressed(
            move |gesture_click: &GestureClick, _: i32, _: f64, _: f64| {
                println!("{}", "1");
                let path_buf = PathBuf::from(Self::gen_res_url(&url_string, ".gif"));
                WinClip::clip(path_buf.to_str().unwrap().to_string())
            },
        );

        image.add_controller(gesture_click);
        // image.set_from_pixbuf(Some(&pixbuf));
        image
    }

    // fn get_image_widgets() -> Vec<Image> {
    //     let urls = [
    //         "https://img2.baidu.com/it/u=4130459079,902939219&fm=253&fmt=auto&app=120&f=JPEG?w=500&h=500",
    //         "https://img0.baidu.com/it/u=2973981695,449639631&fm=253&fmt=auto&app=120&f=JPEG?w=456&h=328",
    //         "https://img0.baidu.com/it/u=1355305794,2447146551&fm=253&fmt=auto&app=120&f=JPEG?w=300&h=300",
    //         "https://img2.baidu.com/it/u=2200213970,3195987466&fm=253&fmt=auto&app=138&f=JPEG?w=440&h=440",
    //         "https://img1.baidu.com/it/u=925909949,2724448435&fm=253&fmt=auto&app=120&f=JPEG?w=500&h=500",
    //         "https://img1.baidu.com/it/u=1434603982,4190874381&fm=253&fmt=auto&app=138&f=JPEG?w=500&h=500",
    //         "https://img0.baidu.com/it/u=3196165980,1615390016&fm=253&fmt=auto&app=138&f=JPEG?w=500&h=500",
    //         "https://img2.baidu.com/it/u=4130459079,902939219&fm=253&fmt=auto&app=120&f=JPEG?w=500&h=500",
    //         "https://img0.baidu.com/it/u=2973981695,449639631&fm=253&fmt=auto&app=120&f=JPEG?w=456&h=328",
    //         "https://img0.baidu.com/it/u=1355305794,2447146551&fm=253&fmt=auto&app=120&f=JPEG?w=300&h=300",
    //         "https://img2.baidu.com/it/u=2200213970,3195987466&fm=253&fmt=auto&app=138&f=JPEG?w=440&h=440",
    //         "https://img1.baidu.com/it/u=925909949,2724448435&fm=253&fmt=auto&app=120&f=JPEG?w=500&h=500",
    //         "https://img1.baidu.com/it/u=1434603982,4190874381&fm=253&fmt=auto&app=138&f=JPEG?w=500&h=500",
    //         "https://img0.baidu.com/it/u=3196165980,1615390016&fm=253&fmt=auto&app=138&f=JPEG?w=500&h=500"
    //     ];
    //     urls.map(SimpleUI::build_image_by_url).to_vec()
    // }
}
