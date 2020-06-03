mod gui {
    pub mod gtk3 {
        #[macro_use]
        pub mod macros {
            #[macro_export]
            macro_rules! build {
                ($builder:ident, $e:expr) => {
                    $builder
                        .get_object($e)
                        .expect(&format!("Couldn't find '{}' in glade ui file", $e))
                };
            }
        }
        use gio::prelude::*;
        use gtk::prelude::*;

        fn build_ui(application: &gtk::Application) {
            let glade_str = include_str!("../src/gui/gtk3/main.ui");
            let builder = gtk::Builder::new_from_string(glade_str);
            let application_window: gtk::ApplicationWindow = build!(builder, "application_window");

            let button_nullpunkt: gtk::Button = build!(builder, "button_nullpunkt");
            let button_messgas: gtk::Button = build!(builder, "button_messgas");

            button_nullpunkt.connect_clicked(move |_| {});

            button_messgas.connect_clicked(move |_| {});

            application_window.set_application(Some(application));

            application_window.show_all();
        }

        pub fn launch() {
            let application = gtk::Application::new(
                Some("com.gaswarnanlagen.ne4-mod-bus.ne4_konfig"),
                Default::default(),
            )
            .expect("failed to initalize GTK application");

            application.connect_activate(|app| {
                build_ui(app);
            });

            application.run(&[]);
        }
    }
}

#[macro_use]
extern crate log;

fn main() {
    env_logger::init();

    info!("Launch GUI");
    gui::gtk3::launch();
}
