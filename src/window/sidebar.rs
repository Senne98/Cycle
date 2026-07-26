use crate::window::window::{Window};
use crate::window::window;
use crate::window::about;

use adw;
use adw::prelude::*;
use adw::{HeaderBar, SidebarItem, SidebarSection, Sidebar, ToolbarView, ToolbarStyle, NavigationPage, ButtonContent};
use adw::gtk::{Button};

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u32)]
pub enum Tab {
    Calculator = 0,
    DefaultConstants = 1,
    CustomConstants = 2,
}

#[allow(unreachable_patterns)]
impl Tab {
    pub fn from_u32(v: u32) -> Option<Self> {
        match v {
            0 => Some(Tab::Calculator),
            1 => Some(Tab::DefaultConstants),
            2 => Some(Tab::CustomConstants),
            _ => None,
        }
    }

    pub fn to_str(&self) -> Option<&str> {
        match self {
            Tab::Calculator => Some("Calculator"),
            Tab::DefaultConstants => Some("Default Constants"),
            Tab::CustomConstants => Some("Custom Constants"),
            _ => None,
        }
    }
}

pub struct SidebarWindow {
    pub page: NavigationPage,
    pub header: HeaderBar,
    pub sidebar_list: Sidebar,

    pub active_tab: Tab,
}

impl SidebarWindow {
    pub fn new() -> Rc<RefCell<Self>> {
        let this = Rc::new(RefCell::new(Self {
            page: NavigationPage::builder().title("Cycle").build(),
            header: HeaderBar::builder().show_title(true).build(),
            sidebar_list: Sidebar::new(),
            active_tab: Tab::Calculator,
        }));

        return this;
    }
}

pub fn init(this: &Rc<RefCell<Window>>) {
    build_header(this);

    let item_calculator = SidebarItem::new(Tab::Calculator.to_str().unwrap());
    item_calculator.set_icon_name(Some("accessories-calculator-symbolic"));
    let item_default_constants = SidebarItem::new(Tab::DefaultConstants.to_str().unwrap());
    item_default_constants.set_icon_name(Some("applications-science-symbolic"));
    let item_custom_constants = SidebarItem::new(Tab::CustomConstants.to_str().unwrap());
    item_custom_constants.set_icon_name(Some("view-list-ordered-symbolic"));

    let section = SidebarSection::new();
    section.append(item_calculator);
    section.append(item_default_constants);
    section.append(item_custom_constants);

    let this_clone = Rc::clone(this);

    let sidebar_list = Sidebar::new();
    sidebar_list.append(section);
    sidebar_list.connect_notify_local(Some("selected"), 
        move |_sidebar, _| {
            let window = this_clone.borrow();
            let mut sidebar = window.sidebar.borrow_mut();

            let idx: u32 = sidebar.sidebar_list.selected();
            sidebar.active_tab = Tab::from_u32(idx).expect("REASON");

            drop(sidebar);
            drop(window);

            window::update(&this_clone);
        },
    );

    let window = this.borrow();
    let mut sidebar = window.sidebar.borrow_mut();

    sidebar.sidebar_list = sidebar_list;

    let sidebar_toolbar = ToolbarView::new();
    sidebar_toolbar.add_top_bar(&sidebar.header);
    sidebar_toolbar.set_content(Some(&sidebar.sidebar_list));
    sidebar_toolbar.set_top_bar_style(ToolbarStyle::Flat);

    sidebar.page = NavigationPage::builder()
        .title("Cycle")
        .child(&sidebar_toolbar)
        .build();

    drop(sidebar);
    drop(window);
}

fn build_header(this: &Rc<RefCell<Window>>) {
    let about_button_content = ButtonContent::builder()
        .icon_name("help-about-symbolic")
        .build();

    let about_button = Button::builder()
        .child(&about_button_content)
        .tooltip_text("About")
        .build();

    let this_clone = Rc::clone(this);

    about_button.connect_clicked(move |_| {

        let this = this_clone.borrow();

        let about_dialog = about::get();
        about_dialog.borrow().present(Some(&this.app_window));
        drop(this);
    });

    let window = this.borrow();
    let sidebar = window.sidebar.borrow();
    
    sidebar.header.pack_end(&about_button);

    drop(sidebar);
    drop(window);
}


pub fn update(_this: &Rc<RefCell<Window>>) {
}

