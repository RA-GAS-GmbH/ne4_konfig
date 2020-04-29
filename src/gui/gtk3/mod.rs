use gio::prelude::*;
use gtk::prelude::*;

use gtk::Application;

pub fn launch() {
    let application = Application::new(
        Some("com.gaswarnanlagen.ne4-mod-bus.ne4_konfig"),
        Default::default(),
    )
    .expect("failed to initalize GTK application");

    application.connect_activate(|app| {
        let glade_src = include_str!("main.ui");
        let builder = gtk::Builder::new_from_string(glade_src);
        let window_main: gtk::ApplicationWindow = builder
            .get_object("window_main")
            .expect("Couldn't get window");
        window_main.set_application(Some(app));

        window_main.show_all();
    });

    application.run(&[]);
}
