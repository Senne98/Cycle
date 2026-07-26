use crate::window::sidebar::*;
use crate::window::sidebar;
use crate::window::content::*;
use crate::window::content;

use adw;
use adw::{NavigationSplitView, ApplicationWindow};

use std::cell::RefCell;
use std::rc::Rc;

pub struct Window {
    pub split_view: NavigationSplitView,
    pub sidebar: Rc<RefCell<SidebarWindow>>,
    pub content: Rc<RefCell<ContentWindow>>,
    pub app_window : ApplicationWindow,
}

impl Window {
    pub fn new(app: &adw::Application) -> Rc<RefCell<Self>> {
        let this = Rc::new(RefCell::new(Self {
            split_view: NavigationSplitView::new(),
            sidebar: SidebarWindow::new(),
            content: ContentWindow:: new(),
            app_window: ApplicationWindow::builder().application(app).default_width(980).default_height(640).build(),
        }));

        let this_clone = Rc::clone(&this);

        sidebar::init(&this_clone);
        content::init(&this_clone);

        let window = this.borrow_mut();
        let sidebar = window.sidebar.borrow_mut();
        let content = window.content.borrow_mut();

        window.split_view.set_sidebar(Some(&sidebar.page));
        window.split_view.set_content(Some(&content.page));

        drop(content);
        drop(sidebar);
        drop(window);

        return this;
    }
}

pub fn update(this: &Rc<RefCell<Window>>) { 
    sidebar::update(this);
    content::update(this);
}
