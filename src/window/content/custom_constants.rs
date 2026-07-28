use crate::window::window::Window;

use adw;
use adw::prelude::*;
use adw::{ActionRow, ButtonRow, Dialog, DialogPresentationMode, EntryRow, ToolbarView, ToolbarStyle, HeaderBar, Banner, ButtonContent};
use adw::gtk::{ListBox, SelectionMode, ScrolledWindow, Image, Box, Orientation, Button};
use adw::gdk::{Display};

use std::fs;
use std::fs::File;
use std::cell::RefCell;
use std::rc::Rc;
use std::io::Write;
use std::vec::Vec;

use indexmap::map::IndexMap;

pub struct CustomConstantsPage {
    pub scrolled_list: ScrolledWindow,
    pub list: ListBox,

    pub constants: IndexMap<String, (String, String, f64)>, //<latex, (name, display, value)>
}

impl CustomConstantsPage {
    pub fn new() -> Rc<RefCell<Self>> {
        let this = Rc::new(RefCell::new(Self {
            scrolled_list: ScrolledWindow::new(),
            list: ListBox::builder().margin_top(32).margin_end(32).margin_bottom(32).margin_start(32)
                .selection_mode(SelectionMode::None).css_classes(vec![String::from("boxed-list")]).build(),

            constants: IndexMap::new(),
        }));

        return this;
    }
}

const CONSTANTS_SAVE_FILE: &str = "custom_constants.csv";

pub fn init(this: &Rc<RefCell<Window>>) {
    if !fs::exists(CONSTANTS_SAVE_FILE).expect(&format!("Can't check existence of file {CONSTANTS_SAVE_FILE}")) {
        File::create(CONSTANTS_SAVE_FILE).expect(&format!("Can't create file {CONSTANTS_SAVE_FILE}"));
    } else {
       build_list(&fs::read_to_string(CONSTANTS_SAVE_FILE).expect(&format!("Can't create file {CONSTANTS_SAVE_FILE}")), this);
    }

    let add_button = ButtonRow::builder()
        .activatable(true)
        .title("Add constant")
        .start_icon_name("value-increase-symbolic")
        .build();

    let this_copy = Rc::clone(this);
    add_button.connect_activated(move |_| {
        open_const_creation_dialog(&this_copy);
    });

    let window = this.borrow();
    let content = window.content.borrow();
    let custom_constants_page = content.custom_constants_page.borrow();
    custom_constants_page.list.append(&add_button);
    drop(custom_constants_page);
    drop(content);
    drop(window);
}

fn build_list(file: &str, this: &Rc<RefCell<Window>>) {
    let constants = file.lines();

    for constant in constants {
        let window = this.borrow();
        let content = window.content.borrow();
        let mut custom_constants_page = content.custom_constants_page.borrow_mut();

        let mut parts = constant.split(",");
        let utf8 = parts.next().unwrap().to_string();
        let latex = parts.next().unwrap().to_string();
        let value = parts.next().unwrap().to_string().parse::<f64>().unwrap();
        let name = parts.next().unwrap().to_string();

        custom_constants_page.constants.insert(latex.clone(), (name.clone(), utf8.clone(), value.clone()));

        drop(custom_constants_page);
        drop(content);
        drop(window);

        let row = create_const_row(&Rc::clone(&this), &latex, &utf8, &name, &value);
        
        let window = this.borrow();
        let content = window.content.borrow();
        let custom_constants_page = content.custom_constants_page.borrow();

        custom_constants_page.list.append(&row);

        drop(custom_constants_page);
        drop(content);
        drop(window);
    }

    let window = this.borrow();
    let content = window.content.borrow();
    let custom_constants_page = content.custom_constants_page.borrow();

    custom_constants_page.scrolled_list.set_child(Some(&custom_constants_page.list));
    
    drop(custom_constants_page);
    drop(content);
    drop(window);
}

fn create_const_row(this: &Rc<RefCell<Window>>, latex: &str, utf8: &str, name: &str, value: &f64) -> ActionRow {

    let row = ActionRow::builder()
        .activatable(true)
        .title(format!("{} = {}", utf8, value))
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
    let this_clone = Rc::clone(this);
    remove_button.connect_clicked(move |_| {
        let window = this_clone.borrow();
        let content = window.content.borrow();
        let mut custom_constants_page = content.custom_constants_page.borrow_mut();

        custom_constants_page.constants.shift_remove(&latex_clone.to_string());

        drop(custom_constants_page);

        save_constants(&Rc::clone(&content.custom_constants_page));

        drop(content);
        drop(window);

        rebuild_list(&this_clone);
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

fn open_const_creation_dialog(this: &Rc<RefCell<Window>>) {
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

    let this_copy = Rc::clone(this);

    let dialog_clone = dialog.clone();
    confirm.connect_clicked(move |_| {
        confirm_action(&dialog_clone, &name_field, &latex_field, &display_field, &value_field, &banner, &this_copy);
    });

    let window = this.borrow();
    dialog.present(Some(&window.app_window));
    drop(window);
}

fn confirm_action(dialog: &Dialog, name_field: &EntryRow, latex_field: &EntryRow, display_field: &EntryRow, value_field: &EntryRow, banner: &Banner, this: &Rc<RefCell<Window>>) {
    let name = name_field.text().to_string();
    let latex = latex_field.text().to_string();
    let display = display_field.text().to_string();
    let value = value_field.text().to_string();

    let window = this.borrow();
    let content = window.content.borrow();
    let default_constant_page = content.default_constants_page.borrow();
    let mut custom_constants_page = content.custom_constants_page.borrow_mut();

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
    } else if default_constant_page.constants.contains_key(&latex) || custom_constants_page.constants.contains_key(&latex) {
        banner.set_title("A constant with this Latex symbol already exists!");
        banner.set_revealed(true);
    } else {

        let value = value.parse::<f64>().unwrap();

        custom_constants_page.constants.insert(latex.clone(), (name.clone(), display.clone(), value.clone()));

        let custom_constants_page_clone = Rc::clone(&content.custom_constants_page);

        drop(default_constant_page);
        drop(custom_constants_page);
        drop(content);
        drop(window);

        rebuild_list(&Rc::clone(&this));
        save_constants(&custom_constants_page_clone);

        dialog.close();
    } 
}

fn rebuild_list(this: &Rc<RefCell<Window>>) {

    let constants: Vec<(String, String, String, f64)> = {
        let window = this.borrow();
        let content = window.content.borrow();
        let custom_constants_page = content.custom_constants_page.borrow();

        while let Some(child) = custom_constants_page.list.first_child() {
            custom_constants_page.list.remove(&child);
        }

        custom_constants_page.constants.iter()
            .map(|(latex, (name, display, value))| (latex.clone(), name.clone(), display.clone(), value.clone()))
            .collect()
    };

    let rows: Vec<ActionRow> = constants.into_iter()
        .map(|(latex, name, display, value)| create_const_row(&Rc::clone(&this), &latex, &display, &name, &value))
        .collect();

    {
        let window = this.borrow();
        let content = window.content.borrow();
        let custom_constants_page = content.custom_constants_page.borrow();

        for row in rows {
            custom_constants_page.list.append(&row);
        }
    }
 
    let add_button = ButtonRow::builder()
        .activatable(true)
        .title("Add constant")
        .start_icon_name("value-increase-symbolic")
        .build();

    let this_clone = Rc::clone(this);

    add_button.connect_activated(move |_| {
        open_const_creation_dialog(&this_clone);
    });

    let window = this.borrow();
    let content = window.content.borrow();
    let custom_constants_page = content.custom_constants_page.borrow();

    custom_constants_page.list.append(&add_button);

    drop(custom_constants_page);
    drop(content);
    drop(window);
}

fn save_constants(this: &Rc<RefCell<CustomConstantsPage>>) {
    let custom_constants_page = this.borrow();

    let mut file_content = String::from("");

    let latex_symbols = custom_constants_page.constants.keys();

    for latex in latex_symbols {
        let (name, utf8, value) = custom_constants_page.constants.get(latex).unwrap();
        let latex = latex.clone();

        file_content.push_str(&format!("{},{},{},{}\n", utf8, latex, value, name));
    }

    let _ = file_content.trim_end_matches("\n");

    let mut file = File::create(CONSTANTS_SAVE_FILE).expect(&format!("Can't create file {CONSTANTS_SAVE_FILE}"));
    let _ = file.write_all(file_content.as_bytes());
}
