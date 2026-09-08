use adw;
use adw::{AboutDialog};

use adw::gtk::IconTheme;
use adw::gdk::Display;

use std::fs::*;
use std::env::*;

#[warn(dead_code)]

const NAME: &str = "Cycle";
const ICON: &str = "com.domain.Cycle";
const DEVELOPER: &str = "Senne";

const VERSION: &str = "0.0.1";

const ICON_SVG: &[u8] = include_bytes!("../../rescources/icons/hicolor/scalable/apps/com.domain.Cycle.svg");

pub fn generate_icon() {
    if let Some(display) = Display::default() {
        let icon_theme = IconTheme::for_display(&display);
        let dir = temp_dir().join("cycle-icons/hicolor/scalable/apps");
        let _ = create_dir_all(&dir);
        let _ = write(dir.join("com.domain.Cycle.svg"), ICON_SVG);
        icon_theme.add_search_path(temp_dir().join("cycle-icons"));
    }
}

pub fn get_about_dialog() -> AboutDialog {
    AboutDialog::builder()
        .application_icon(ICON)
        .application_name(NAME)
        .developer_name(DEVELOPER)

        .version(VERSION)
        .build()
}
