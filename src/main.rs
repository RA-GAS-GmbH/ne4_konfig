extern crate gtk;
extern crate gio;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{Application, ApplicationWindow, Button};

fn main() {
    let application = Application::new(
        Some("com.gaswarnanlagen.ne4-mod-bus.configuration_gui"),
        Default::default(),
    ).expect("failed to initalize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("NE4-MOD-BUS - Konfiguration");
        window.set_default_size(1024, 600);

        let button = Button::new_with_label("Einstellungen");
        button.connect_clicked(|_| {
            println!("Einstellungen wurden gewählt!");
        });
        window.add(&button);

        window.show_all();
    });

    application.run(&[]);
}
