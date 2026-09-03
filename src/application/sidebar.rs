use crate::application::window::*;
use crate::application::about::*;

use adw;
use adw::prelude::*;
use adw::{HeaderBar, SidebarItem, SidebarSection, Sidebar, ToolbarView, ToolbarStyle, NavigationPage, ButtonContent};
use adw::gtk::{Button};

use std::cell::LazyCell;
use std::sync::Mutex;

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

    pub fn to_str(&self) -> Option<String> {
        match self {
            Tab::Calculator => Some("Calculator".to_string()),
            Tab::DefaultConstants => Some("Default Constants".to_string()),
            Tab::CustomConstants => Some("Custom Constants".to_string()),
            _ => None,
        }
    }
}

thread_local! {
    static PAGE: LazyCell<NavigationPage> = LazyCell::new(|| NavigationPage::builder().title("Cycle").build());
    static HEADER: LazyCell<HeaderBar> = LazyCell::new(|| HeaderBar::builder().show_title(true).build());
    static SIDEBAR_LIST: LazyCell<Sidebar> = LazyCell::new(|| Sidebar::new());
}
static ACTIVE_TAB: Mutex<Tab> = Mutex::new(Tab::Calculator);

pub fn get_active_tab() -> Tab {
    return ACTIVE_TAB.lock().unwrap().clone();
}

pub fn create_sidebar() -> NavigationPage {
    generate_icon();
    let item_calculator = SidebarItem::new(&Tab::Calculator.to_str().unwrap());
    item_calculator.set_icon_name(Some("accessories-calculator-symbolic"));

    let item_default_constants = SidebarItem::new(&Tab::DefaultConstants.to_str().unwrap());
    item_default_constants.set_icon_name(Some("applications-science-symbolic"));

    let item_custom_constants = SidebarItem::new(&Tab::CustomConstants.to_str().unwrap());
    item_custom_constants.set_icon_name(Some("view-list-ordered-symbolic"));

    let section = SidebarSection::new();
    section.append(item_calculator);
    section.append(item_default_constants);
    section.append(item_custom_constants);

    build_header();
    
    let sidebar_toolbar = ToolbarView::new();
    HEADER.with(|h| {
         sidebar_toolbar.add_top_bar(&**h);
    });

    SIDEBAR_LIST.with(|s| {
        s.append(section);

        let s_clone = (**s).clone();

        s.connect_notify_local(Some("selected"), 
            move |_sidebar, _| {
                let mut tab = ACTIVE_TAB.lock().unwrap();
                *tab = Tab::from_u32(s_clone.selected()).expect("Index is out of bounds!");
                drop(tab);
                update_window();
            },
        );
 
        sidebar_toolbar.set_content(Some(&**s)); 
    });

    sidebar_toolbar.set_top_bar_style(ToolbarStyle::Flat);
    
    PAGE.with(|p| {
        p.set_child(Some(&sidebar_toolbar));
        return (**p).clone();
    })
}

fn build_header() {
    let about_button_content = ButtonContent::builder()
        .icon_name("help-about-symbolic")
        .build();

    let about_button = Button::builder()
        .child(&about_button_content)
        .tooltip_text("About")
        .build();

    about_button.connect_clicked(move |_| {
        let about_dialog = get_about_dialog();
        about_dialog.present(Some(&get_app_window()));
    });

    HEADER.with(|h| {
        h.pack_end(&about_button);
    });
}

pub fn get_active_tab_name() -> String {
    ACTIVE_TAB.lock().unwrap().to_str().unwrap()
}
