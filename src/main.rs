mod window;

use window::window::Window;
use window::window as wnd;

use adw;
use adw::prelude::*;
use adw::{glib, Application};

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(build_ui);

    return app.run();
}

fn build_ui(app: &adw::Application) {
    let window_ref = Window::new(app);
    let window = window_ref.borrow();

    window.app_window.set_content(Some(&window.split_view));

    drop(window);

    wnd::update(&window_ref);

    let window = window_ref.borrow();
    window.app_window.present();
    drop(window);
}
