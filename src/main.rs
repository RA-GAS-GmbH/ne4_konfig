use gio::prelude::*;
use gtk::prelude::*;

use gtk::{Application, ApplicationWindow};

mod gui {
    pub mod gtk3;
}

fn main() {
    let application = Application::new(
        Some("com.gaswarnanlagen.ne4-mod-bus.ne4_konfig"),
        Default::default(),
    )
    .expect("failed to initalize GTK application");

    application.connect_activate(|app| {
        gui::gtk3::launch(app);
    });

    application.run(&[]);
}
