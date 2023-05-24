mod common_error;

use std::borrow::Cow;
use std::cell::{Cell, RefCell};
use std::error::Error;
use std::fmt::Debug;
use std::ops::ControlFlow::Continue;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::thread::spawn;

use gtk::glib::{clone, MainContext, Receiver, WeakRef};
use gtk::subclass::prelude::ObjectSubclassIsExt;
use gtk::ConstraintStrength::Weak;
use gtk::{glib, Application};
use serde_json::to_string;
use windows::s;

use crate::emotions_app::app::FindEmotion;
use crate::learn_test::common_error::{CommonError, ErrorType};

fn test_clone() {
    let a = Application::builder().build();
    let main_context = MainContext::default();

    main_context.spawn_local(glib::clone!(@strong a => async move {
        println!("{}",a.to_string())
    }));
}

pub trait ELearnTrait {
    fn clone_me(&self) -> Self;
}

#[derive(Clone, Copy)]
pub struct ELearner {}

pub enum CommonRc<T> {
    RustRc(Rc<T>),
    RustArc(Arc<T>),
}

impl<T> CommonRc<T> {
    pub fn new_rc(value: T) -> CommonRc<T> {
        CommonRc::RustRc(Rc::new(value))
    }

    pub fn new_arc(value: T) -> CommonRc<T> {
        CommonRc::RustArc(Arc::new(value))
    }

    pub fn require_arc(self) -> Result<Arc<T>, CommonError> {
        match self {
            CommonRc::RustArc(e) => Ok(e),
            CommonRc::RustRc(_) => Err(CommonError::new(ErrorType::ParingError, None, None)),
        }
    }
}

impl<T> Clone for CommonRc<T> {
    fn clone(&self) -> Self {
        match self {
            CommonRc::RustRc(value) => CommonRc::RustRc(value.clone()),
            CommonRc::RustArc(value) => CommonRc::RustArc(value.clone()),
        }
    }
}

impl<T> Deref for CommonRc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            CommonRc::RustRc(value) => value,
            CommonRc::RustArc(value) => value,
        }
    }
}

pub struct DemoInner {
    value: String,
}

pub struct Demo {
    // el: Box<dyn ELearnTrait + Send + Sync>,
    pub receiver: RefCell<Option<Receiver<Box<u8>>>>,
    pub inner: Arc<DemoInner>,
}

// impl Clone for Demo {
//     fn clone(&self) -> Self {
//         Demo {
//             // el: Box::new(),
//             receiver: RefCell::new(None),
//             inner: DemoInner {
//                 value:"".to_string()
//             },
//         }
//     }
// }
// el: Box<dyn ELearnTrait + Send + Sync>

#[derive(Clone)]
struct ABC {
    value: String,
}

impl ABC {
    fn new(str: String) -> Self {
        ABC { value: str }
    }

    fn print(&self) {
        println!("{}", self.value.clone())
    }
}

impl Demo {
    pub fn new(atom: bool) -> CommonRc<Demo> {
        let (sender, r) = MainContext::channel::<Box<u8>>(glib::PRIORITY_DEFAULT);
        let mut receiver = RefCell::new(Some(r));
        let demo = Demo {
            receiver,
            inner: Arc::new(DemoInner {
                value: "Demo".to_string(),
            }),
        };

        if atom {
            CommonRc::new_arc(demo)
        } else {
            CommonRc::new_rc(demo)
        }
    }

    pub fn start(&self, demo_rc: CommonRc<Demo>) {
        let demo_rc2 = demo_rc.clone();
    }

    fn setup_channel(&self) {
        let (sender, r) = MainContext::channel::<Box<u8>>(glib::PRIORITY_DEFAULT);
        let mut receiver = RefCell::new(Some(r));

        let (tx, rx) = channel::<Box<u8>>();

        let inner = Arc::downgrade(&self.inner);

        let abc = ABC::new("".to_string());

        let ac = Arc::new(abc);
        // let ac2 = ac.clone();
        spawn(move || {
            ac.as_ref().print();
        });
    }

    fn process_action(demoInner: &DemoInner) -> glib::Continue {
        glib::Continue(true)
    }

    pub fn do_sth(&self) {}
}

fn app() {
    // let demo = Demo::new(Box::new(ELearner {}));
    // demo.start(demo.clone());
}

// impl<T: ELearnTrait> LearnTest<T> {
//     fn copy_shallow(&self) -> LearnTest<T> {
//         return LearnTest { text: self.text };
//     }
//
//     fn do_sth(&self) {
//         let s = self.clone();
//         let s2 = s.clone();
//         //
//         spawn(move || {
//             s.clone().do_sth2();
//         });
//         // println!("{:?}", t)
//     }
//
//     fn do_sth2(&self) {}
// }

#[cfg(test)]
mod tests {
    // 注意这个惯用法：在 tests 模块中，从外部作用域导入所有名字。
    use super::*;

    #[test]
    fn test() {
        let a1 = Arc::new("123");
        let a2 = Arc::downgrade(&a1);
        println!("{}", Arc::strong_count(&a1));
        {
            let a3 = a2.upgrade().unwrap();
        }
        println!("{}", Arc::strong_count(&a1));
    }
}
