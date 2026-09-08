//Copyright (C) 2026  Senne98
//Lisence: https://github.com/Senne98/Cycle/blob/main/LICENSE

mod application;
mod parser;

use application::window::*;

use parser::constants::*;

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

fn build_ui(app: &Application) {
    load_constants();

    create_application_window(app);

    get_app_window().set_content(Some(&get_split_view()));

    update_window();

    present_window()
}
