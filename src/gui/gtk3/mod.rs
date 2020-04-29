use gio::prelude::*;
use gtk::prelude::*;

use gtk::{Application, ApplicationWindow};

pub fn launch(app: &gtk::Application) {
    let glade_src = include_str!("main.ui");
    let builder = gtk::Builder::new_from_string(glade_src);
    let window_main: gtk::ApplicationWindow = builder
        .get_object("window_main")
        .expect("Couldn't get window");
    window_main.set_application(Some(app));

    window_main.show_all();
}
