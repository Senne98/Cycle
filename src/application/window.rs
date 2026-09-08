use crate::application::sidebar::*;
use crate::application::content::*;
use crate::application::content::custom_constants::*;
use crate::application::content::default_constants::*;
use crate::application::content::equations::*;

use adw;
use adw::prelude::*;
use adw::{NavigationSplitView, ApplicationWindow, Application};

use std::cell::OnceCell;
use std::cell::LazyCell;

thread_local! {
    static SPLIT_VIEW: LazyCell<NavigationSplitView> = LazyCell::new(|| NavigationSplitView::new());
    static APP_WINDOW: OnceCell<ApplicationWindow> = OnceCell::new();
}

pub fn create_application_window(app: &Application) {
    APP_WINDOW.with(|w| {
        w.set(ApplicationWindow::builder()
            .application(app)
            .default_width(980)
            .default_height(640)
            .build()).expect("APP_WINDOW could not be initialized!");
    });

    create_custom_const_page();
    create_default_const_page();
    create_equations();

    let sidebar = create_sidebar();
    let content = create_content();

    SPLIT_VIEW.with(|s| {
        s.set_sidebar(Some(&sidebar));
        s.set_content(Some(&content));
    });
}

pub fn update_window() {
    update_content();
}

pub fn present_window() {
    APP_WINDOW.with(|w| {
        if let Some(window) = w.get() {
            window.present();
        } else {
            panic!("APP_WINDOW should be initialized!");
        };
    });
}

pub fn get_app_window() -> ApplicationWindow {
    APP_WINDOW.with(|w| {
        if let Some(window) = w.get() {
            return window.clone();
        } else {
            panic!("app_window should be initialized!");
        };
    })
}

pub fn get_split_view() -> NavigationSplitView {
    SPLIT_VIEW.with(|s| (**s).clone())
}
