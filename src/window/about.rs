use adw;
use adw::{AboutDialog};

use std::cell::RefCell;
use std::rc::Rc;

const NAME: &str = "Cycle";
const DEVELOPER: &str = "Senne";

const VERSION: &str = "0.0.1";

pub fn get() -> Rc<RefCell<AboutDialog>> {
    let about_dialog = Rc::new(RefCell::new(AboutDialog::builder()
        .application_name(NAME)
        .developer_name(DEVELOPER)
        .version(VERSION)
        .build()));
    return about_dialog;
}
