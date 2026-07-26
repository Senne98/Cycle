pub mod default_constants;
pub mod custom_constants;

use crate::window::window::{Window};
use crate::window::sidebar::*;
use crate::window::content::default_constants::DefaultConstantsPage;
use crate::window::content::custom_constants::CustomConstantsPage;

use adw;
use adw::prelude::*;
use adw::{HeaderBar, ToolbarView, ToolbarStyle, NavigationPage, ActionRow};
use adw::gtk::{ListBox, SelectionMode};

use std::cell::RefCell;
use std::rc::Rc;

pub struct ContentWindow {
    pub page: NavigationPage,
    pub header: HeaderBar,
    pub content_toolbar: ToolbarView,

    pub default_constants_page: Rc<RefCell<DefaultConstantsPage>>,
    pub custom_constants_page: Rc<RefCell<CustomConstantsPage>>,
}

impl ContentWindow {
    pub fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            page: NavigationPage::builder().title(Tab::Calculator.to_str().unwrap()).build(),
            header: HeaderBar::builder().show_title(true).build(),
            content_toolbar: ToolbarView::new(),

            default_constants_page: DefaultConstantsPage::new(),
            custom_constants_page: CustomConstantsPage::new(),
        }))
    }
}


pub fn init(this: &Rc<RefCell<Window>>) {
    custom_constants::init(this);

    let window = this.borrow();
    let mut content = window.content.borrow_mut();
    let sidebar = window.sidebar.borrow();

    default_constants::init(&content.default_constants_page); 

    content.header.set_decoration_layout(Some(":close"));

    let row = ActionRow::builder()
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
    list.append(&row);

    content.content_toolbar.add_top_bar(&content.header);
    content.content_toolbar.set_content(Some(&list));
    content.content_toolbar.set_top_bar_style(ToolbarStyle::Flat);

    content.page = NavigationPage::builder()
        .title(sidebar.active_tab.to_str().unwrap())
        .child(&content.content_toolbar)
        .build();

    drop(sidebar);
    drop(content);
    drop(window);
}

#[allow(unreachable_patterns)]
pub fn update(this: &Rc<RefCell<Window>>) { 
    let window = this.borrow();
    let content = window.content.borrow();
    let sidebar = window.sidebar.borrow();

    content.page.set_title(sidebar.active_tab.to_str().unwrap());

    let active_tab = sidebar.active_tab;

    match active_tab {
        Tab::Calculator => {},
        Tab::DefaultConstants => {
            let default_constants_page = content.default_constants_page.borrow();
            content.content_toolbar.set_content(Some(&default_constants_page.scrolled_list));
            drop(default_constants_page);
        },
        Tab::CustomConstants => {
            let custom_constants_page = content.custom_constants_page.borrow();
            content.content_toolbar.set_content(Some(&custom_constants_page.scrolled_list));
            drop(custom_constants_page);
        },
        _ => {},
    };


    drop(sidebar);
    drop(content);
    drop(window);
}
