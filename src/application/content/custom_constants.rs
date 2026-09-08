//Copyright (C) 2026  Senne98
//Lisence: https://github.com/Senne98/Cycle/blob/main/LICENSE

use crate::application::window::*;

use crate::parser::constants::*;

use adw;
use adw::prelude::*;
use adw::{ActionRow, ButtonRow, Dialog, DialogPresentationMode, EntryRow, ToolbarView, ToolbarStyle, HeaderBar, Banner, ButtonContent};
use adw::gtk::{ListBox, SelectionMode, ScrolledWindow, Image, Box, Orientation, Button};
use adw::gdk::{Display};

use std::vec::Vec;
use std::sync::LazyLock;

thread_local! {
    static SCROLLED_LIST: LazyLock<ScrolledWindow> =  LazyLock::new(|| ScrolledWindow::new());
    static LIST_BOX: LazyLock<ListBox> =  LazyLock::new(|| ListBox::builder().margin_top(32).margin_end(32).margin_bottom(32).margin_start(32)
                .selection_mode(SelectionMode::None).css_classes(vec![String::from("boxed-list")]).build());
}

pub fn create_custom_const_page() {
    build_list();
    
    let scroll_box = Box::new(Orientation::Vertical, 0);
    scroll_box.append(&LIST_BOX.with(|l| (**l).clone()));

    SCROLLED_LIST.with(|s| {
        s.set_child(Some(&scroll_box));
    });
}

pub fn get_scrolled_list() -> ScrolledWindow {
    return SCROLLED_LIST.with(|s| (**s).clone());
}

fn build_list() {
    LIST_BOX.with(|l| {
        while let Some(child) = l.first_child() {
            l.remove(&child);
        }
    });

    let rows: Vec<ActionRow> = get_custom_constants().into_iter()
        .map(|(latex, name, display, value)| create_const_row(&latex, &name, &display, &value))
        .collect();

    LIST_BOX.with(|l| {
        for row in rows {
            l.append(&row);
        }
    });


    create_add_button();
 }

/*
* CONSTANT CREATION DIALOG
*/

fn open_const_creation_dialog() {
    let header = HeaderBar::builder()
        .show_title(true)
        .build();

    let form = ListBox::builder()
        .margin_top(16)
        .margin_end(32)
        .margin_bottom(32)
        .margin_start(32)
        .width_request(354)
        .selection_mode(SelectionMode::None)
        .css_classes(vec![String::from("boxed-list")])
        .build();

    let name_field = EntryRow::builder()
        .title("Name")
        .build();
    form.append(&name_field);

    let latex_field = EntryRow::builder()
        .title("Latex symbol")
        .build();
    form.append(&latex_field);

    let display_field = EntryRow::builder()
        .title("Display symbol")
        .build();
    form.append(&display_field);

    let value_field = EntryRow::builder()
        .title("Value")
        .build();
    form.append(&value_field);

    let confirm = Button::builder()
        .label("Confirm")
        .build();

    let cancel = Button::builder()
        .label("Cancel")
        .build();

    let action_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(16)
        .margin_start(32)
        .margin_end(32)
        .margin_bottom(32)
        .homogeneous(true)
        .build();
    action_box.append(&cancel);
    action_box.append(&confirm);

    let banner = Banner::new("");
    banner.set_button_label(Some("Accept"));

    let banner_clone = banner.clone();

    banner.connect_button_clicked(move |_| {
        banner_clone.set_revealed(false);
    });

    let content_box = Box::builder()
        .orientation(Orientation::Vertical)
        .build();
    content_box.append(&banner);
    content_box.append(&form);
    content_box.append(&action_box);

    let toolbar = ToolbarView::new();
    toolbar.add_top_bar(&header);
    toolbar.set_content(Some(&content_box));
    toolbar.set_top_bar_style(ToolbarStyle::Flat);

    let dialog = Dialog::builder()
        .can_close(true)
        .child(&toolbar)
        .follows_content_size(true)
        .presentation_mode(DialogPresentationMode::Floating)
        .title("Add constant")
        .build();

    let dialog_clone = dialog.clone();
    cancel.connect_clicked(move |_| {
        dialog_clone.close();
    });

    let dialog_clone = dialog.clone();
    confirm.connect_clicked(move |_| {
        confirm_action(&dialog_clone, &latex_field, &name_field, &display_field, &value_field, &banner);
    });

    dialog.present(Some(&get_app_window()));
}


/*
* HELPER FUNTCIONS
*/

fn create_const_row(latex: &str, name: &str, display: &str, value: &str) -> ActionRow {

    let row = ActionRow::builder()
        .activatable(true)
        .title(format!("{display} = {value}"))
        .subtitle(name)
        .build();

    let copy_symbol = Image::from_icon_name("edit-copy-symbolic");

    let remove_button_content = ButtonContent::builder()
        .icon_name("user-trash-symbolic")
        .build();
    let remove_button = Button::builder()
        .child(&remove_button_content)
        .build();
    let remove_button_style = remove_button.style_context();
    remove_button_style.add_class("destructive-action");

    let latex_clone = latex.to_string().clone();
    remove_button.connect_clicked(move |_| {
        remove_custom_constant(latex_clone.clone());
        build_list();
    });

    let end_buttons = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(16)
        .margin_top(8)
        .margin_bottom(8)
        .build();
    end_buttons.append(&copy_symbol);
    end_buttons.append(&remove_button);

    row.add_suffix(&end_buttons);

    let latex_clone = latex.to_string().clone();
    row.connect_activated(move |_| {
        let display = Display::default().expect("no display found");
        let clipboard = display.clipboard();
        clipboard.set_text(&latex_clone);
    });

    return row;
}

fn create_add_button() {
     let add_button = ButtonRow::builder()
        .activatable(true)
        .title("Add constant")
        .start_icon_name("value-increase-symbolic")
        .build();

    add_button.connect_activated(move |_| {
        open_const_creation_dialog();
    });

    LIST_BOX.with(|l| {
        l.append(&add_button);
    });
}


fn confirm_action(dialog: &Dialog, latex_field: &EntryRow, name_field: &EntryRow, display_field: &EntryRow, value_field: &EntryRow, banner: &Banner) {
    let name = name_field.text().to_string();
    let latex = latex_field.text().to_string();
    let display = display_field.text().to_string();
    let value = value_field.text().to_string();

    if name == "" || latex == "" || display == "" || value == "" {
        banner.set_title("All fields must be filled!");
        banner.set_revealed(true);
    } else if value.contains(" ") {
        banner.set_title("Name can not contain spaces!");
        banner.set_revealed(true);
    } else if latex.contains(" ") {
        banner.set_title("Latex symbol can not contain spaces!");
        banner.set_revealed(true);
    } else if display.contains(" ") {
        banner.set_title("Display symbol can not contain spaces!");
        banner.set_revealed(true);
    } else if !latex.starts_with("\\") {
        banner.set_title("Latex symbol should start with \"\\\" !");
        println!("{}", latex);
        banner.set_revealed(true);
    } else if !value.parse::<f64>().is_ok() {
        banner.set_title("Value should be a number!");
        banner.set_revealed(true);
     }else if latex.strip_prefix("\\").unwrap() == "" {
        banner.set_title("Latex symbol can not be \"\\\"!");
        banner.set_revealed(true);
    } else if latex.strip_prefix("\\").unwrap().contains("\\") {
        banner.set_title("Latex symbol should only contain \"\\\" at the start!");
        banner.set_revealed(true);
    } else if latex.strip_prefix("\\").unwrap().contains(&['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']) {
        banner.set_title("Latex symbol can not contain numbers!");
        banner.set_revealed(true);
    } else if latex.strip_prefix("\\").unwrap().contains(&['.', '*', '/', '+', '-', '\"', '\'', '!']) {
        banner.set_title("Latex symbol can not contain . , * , \" , \' , / , + , - or !");
        banner.set_revealed(true);
    } else if is_constant(&latex) {
        banner.set_title("A constant with this Latex symbol already exists!");
        banner.set_revealed(true);
    } else {
        add_custom_constant(&latex.clone(), &name.clone(), &display.clone(), &value.clone());
 
        build_list();
        dialog.close();
    } 
}
