pub mod default_constants;
pub mod custom_constants;
pub mod equations;

use crate::application::sidebar::*;

use adw;
use adw::prelude::*;
use adw::{HeaderBar, ToolbarView, ToolbarStyle, NavigationPage, ActionRow};
use adw::gtk::{ListBox, SelectionMode};

use std::sync::LazyLock;

thread_local! {
    static PAGE: LazyLock<NavigationPage> = LazyLock::new(|| NavigationPage::builder().title(Tab::Calculator.to_str().unwrap()).build());
    static HEADER: LazyLock<HeaderBar> = LazyLock::new(|| HeaderBar::builder().show_title(true).build());
    static CONTENT_TOOLBAR: LazyLock<ToolbarView> = LazyLock::new(|| ToolbarView::new());
}

pub fn create_content() -> NavigationPage {
    HEADER.with(|h| {
        h.set_decoration_layout(Some(":close"));
    });

    /*let row = ActionRow::builder()
        .activatable(true)
        .title("Click me")
        .build();
    row.connect_activated(|_| {
        eprintln!("Click");
    });

    let list = ListBox::builder()
        .margin_top(32)
        .margin_end(32)
        .margin_bottom(32)
        .margin_start(32)
        .selection_mode(SelectionMode::None)
        .css_classes(vec![String::from("boxed-list")])
        .build();
    list.append(&row);*/

    CONTENT_TOOLBAR.with(|c| {
        c.add_top_bar(&HEADER.with(|h| (**h).clone()));
        //c.set_content(Some(&list));
        c.set_top_bar_style(ToolbarStyle::Flat);
    });

    PAGE.with(|p| {
        p.set_title(&get_active_tab_name());
        p.set_child(Some(&CONTENT_TOOLBAR.with(|c| (**c).clone())));
    });

    update_content();
    PAGE.with(|p| {
        return (**p).clone();
    })
}

pub fn update_content() {
    PAGE.with(|p| {
        p.set_title(&get_active_tab_name());
    });

    CONTENT_TOOLBAR.with(|c| { 
        match get_active_tab() {
            Tab::Calculator => {
                c.set_content(Some(&equations::get_scrolled_list()))
            },
            Tab::DefaultConstants => {
                c.set_content(Some(&default_constants::get_scrolled_list()));
            },
            Tab::CustomConstants => {
                c.set_content(Some(&custom_constants::get_scrolled_list()));
            },
        };
    });
}
