use crate::parser::constants::*;

use adw;
use adw::prelude::*;
use adw::{ActionRow};
use adw::gtk::{ListBox, SelectionMode, ScrolledWindow, Image};
use adw::gdk::{Display};

use std::sync::LazyLock;

thread_local! {
    static SCROLLED_LIST: LazyLock<ScrolledWindow> = LazyLock::new(|| ScrolledWindow::new());
    static LIST_BOX: LazyLock<ListBox> = LazyLock::new(|| ListBox::builder().margin_top(32).margin_end(32).margin_bottom(32).margin_start(32)
                .selection_mode(SelectionMode::None).css_classes(vec![String::from("boxed-list")]).build());
}

pub fn create_default_const_page() {
    let constants = get_default_constants();

    LIST_BOX.with(|l| {
        while let Some(child) = l.first_child() {
            l.remove(&child);
        }
    });

    for constant in constants {
        let (latex, name, display, value) = constant;
        
        let row = ActionRow::builder()
            .activatable(true)
            .title(format!("{display} = {value}"))
            .subtitle(name)
            .build();

        let copy_symbol = Image::from_icon_name("edit-copy-symbolic");
        row.add_suffix(&copy_symbol);

        row.connect_activated(move |_| {
            let display = Display::default().expect("no display found");
            let clipboard = display.clipboard();
            clipboard.set_text(&latex);
        });

        LIST_BOX.with(|l| {
            l.append(&row);
        });
    }

    SCROLLED_LIST.with(|s| {
        s.set_child(Some(&LIST_BOX.with(|l| (**l).clone())));
    });
}

pub fn get_scrolled_list() -> ScrolledWindow {
    SCROLLED_LIST.with(|s| (**s).clone())
}


