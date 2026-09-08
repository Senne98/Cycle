//Copyright (C) 2026  Senne98
//Lisence: https://github.com/Senne98/Cycle/blob/main/LICENSE

use crate::parser::constants::*;

use adw;
use adw::prelude::*;
use adw::{ActionRow};
use adw::gtk::{ListBox, SelectionMode, ScrolledWindow, Image, Box, Orientation, Label, Align};
use adw::gdk::{Display};

use std::sync::LazyLock;

thread_local! {
    static SCROLLED_LIST: LazyLock<ScrolledWindow> = LazyLock::new(|| ScrolledWindow::new());
}

pub fn create_default_const_page() {
    let constants = get_default_constants();
    let list_box: ListBox = ListBox::builder()
        .margin_top(32)
        .margin_end(32)
        .margin_bottom(32)
        .margin_start(32)
        .selection_mode(SelectionMode::None)
        .css_classes(vec![String::from("boxed-list")])
        .build();

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

        list_box.append(&row);
    }

    let credits = Label::builder()
        .label("All constants come from NIST (or wikipedia if not available on NIST)".to_string())
        .wrap(true)
        .halign(Align::Center)
        .margin_top(0)
        .margin_bottom(32)
        .build();
    credits.add_css_class("dim-label");

    let scroll_box = Box::new(Orientation::Vertical, 0);
    scroll_box.append(&list_box);
    scroll_box.append(&credits);

    SCROLLED_LIST.with(|s| {
        s.set_child(Some(&scroll_box));
    });
}

pub fn get_scrolled_list() -> ScrolledWindow {
    SCROLLED_LIST.with(|s| (**s).clone())
}


