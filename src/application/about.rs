use adw;
use adw::{AboutDialog};

#[warn(dead_code)]

const NAME: &str = "Cycle";
const DEVELOPER: &str = "Senne";

const VERSION: &str = "0.0.1";

pub fn get_about_dialog() -> AboutDialog {
    AboutDialog::builder()
        .application_name(NAME)
        .developer_name(DEVELOPER)

        .version(VERSION)
        .build()
}
