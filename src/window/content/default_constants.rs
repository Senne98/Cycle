use adw;
use adw::prelude::*;
use adw::{ActionRow};
use adw::gtk::{ListBox, SelectionMode, ScrolledWindow, Image};
use adw::gdk::{Display};

use std::cell::RefCell;
use std::rc::Rc;

use indexmap::map::IndexMap;

pub struct DefaultConstantsPage {
    pub scrolled_list: ScrolledWindow,
    pub list: ListBox,

    pub constants: IndexMap<String, (String, String, f64)>, //<latex, (name, display, value)>
}

impl DefaultConstantsPage {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            scrolled_list: ScrolledWindow::new(),
            list: ListBox::builder().margin_top(32).margin_end(32).margin_bottom(32).margin_start(32)
                .selection_mode(SelectionMode::None).css_classes(vec![String::from("boxed-list")]).build(),

            constants: IndexMap::new(),
        }))
    }
}

pub fn init(this: &Rc<RefCell<DefaultConstantsPage>>) {
    let cst_file = include_str!("default_constants.csv"); 
    let mut constants = cst_file.lines();

    let mut default_constants_page = this.borrow_mut();

    constants.next();

    for constant in constants {
        let mut parts = constant.split(",");
        let utf8 = parts.next().unwrap().to_string();
        let latex = parts.next().unwrap().to_string();
        let value = parts.next().unwrap().to_string().parse::<f64>().unwrap();
        let name = parts.next().unwrap().to_string();

        default_constants_page.constants.insert(latex.clone(), (name.clone(), utf8.clone(), value.clone()));

        let row = ActionRow::builder()
            .activatable(true)
            .title(format!("{utf8} = {value}"))
            .subtitle(name)
            .build();

        let copy_symbol = Image::from_icon_name("edit-copy-symbolic");
        row.add_suffix(&copy_symbol);

        row.connect_activated(move |_| {
            let display = Display::default().expect("no display found");
            let clipboard = display.clipboard();
            clipboard.set_text(&latex);
        });

        default_constants_page.list.append(&row);
    }

    default_constants_page.scrolled_list.set_child(Some(&default_constants_page.list));
    drop(default_constants_page);
}

