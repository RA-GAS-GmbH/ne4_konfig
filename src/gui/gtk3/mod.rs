use crate::serial_thread::SerialThread;
use gio::prelude::*;
use glib;
use gtk::prelude::*;
use std::cell::RefCell;

use gtk::Application;

pub struct Ui {
    window_application: gtk::ApplicationWindow,
    window_settings: gtk::Window,
}

pub struct State {
    connected_port: Option<String>,
}

// Thread local storage
thread_local!(
    pub static GLOBAL: RefCell<Option<(Ui, SerialThread, State)>> = RefCell::new(None)
);

pub fn launch() {
    let application = Application::new(
        Some("com.gaswarnanlagen.ne4-mod-bus.ne4_konfig"),
        Default::default(),
    )
    .expect("failed to initalize GTK application");

    application.connect_activate(|app| {
        ui_init(app);
    });

    application.run(&[]);
}

fn ui_init(app: &gtk::Application) {
    let glade_src = include_str!("main.ui");
    let builder = gtk::Builder::new_from_string(glade_src);
    let window_application: gtk::ApplicationWindow = builder
        .get_object("window_application")
        .expect("Couldn't get application window");
    let window_settings: gtk::Window = builder
        .get_object("window_settings")
        .expect("Couldn't get settings window");
    window_application.set_application(Some(app));

    let ui = Ui {
        window_application: window_application.clone(),
        window_settings: window_settings.clone(),
    };

    let state = State {
        connected_port: None,
    };

    GLOBAL.with(move |global| {
        *global.borrow_mut() = Some((
            ui,
            SerialThread::new(|| {
                glib::idle_add(receive);
            }),
            state,
        ));
    });

    window_application.show_all();
}

// Die `receive` Funktion handelt "events"
fn receive() -> glib::Continue {
    GLOBAL.with(|global| {
        if let Some((ref mut ui, ref serial_thread, ref mut state)) = *global.borrow_mut() {
            match serial_thread.from_port_chan_rx.try_recv() {
                Ok(_) => {
                    info!("Unhandled Event in GUI!");
                }
                Err(_) => (),
            }
        }
    });
    glib::Continue(false)
}
