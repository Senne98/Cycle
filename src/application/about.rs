use adw;
use adw::{AboutDialog};

use adw::gtk::IconTheme;
use adw::gdk::Display;

#[warn(dead_code)]

const NAME: &str = "Cycle";
const ICON: &str = "com.domain.Cycle";
const DEVELOPER: &str = "Senne";

const VERSION: &str = env!("CARGO_PKG_VERSION");

const ICON_SVG: &[u8] = include_bytes!("../../rescources/icons/hicolor/scalable/apps/com.domain.Cycle.svg");

pub fn generate_icon() {
    //AI generated, because I couldn't be bothered. I hate file thingies
    if let Some(display) = Display::default() {
        let icon_theme = IconTheme::for_display(&display);
        let dir = std::env::temp_dir().join("cycle-icons/hicolor/scalable/apps");
        let _ = std::fs::create_dir_all(&dir);
        let _ = std::fs::write(dir.join("com.domain.Cycle.svg"), ICON_SVG);
        icon_theme.add_search_path(std::env::temp_dir().join("cycle-icons"));
    }
    //end of slop, well AI slop at least, the rest is my slop
}

pub fn get_about_dialog() -> AboutDialog {
    AboutDialog::builder()
        .application_icon(ICON)
        .application_name(NAME)
        .developer_name(DEVELOPER)

        .version(VERSION)
        .build()
}
