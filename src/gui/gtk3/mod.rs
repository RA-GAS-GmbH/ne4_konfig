use chrono::Utc;
use crate::serial_thread::SerialThread;
use gio::prelude::*;
use glib;
use gtk::Application;
use gtk::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;


#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum StatusContext {
    PortOperation,
}

pub struct Ui {
    window_application: gtk::ApplicationWindow,
    combo_box_ports: gtk::ComboBox,
    statusbar_application: gtk::Statusbar,
    statusbar_contexts: HashMap<StatusContext, u32>,
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
    let combo_box_ports: gtk::ComboBox = builder
        .get_object("combo_box_ports")
        .expect("Couldn't get ports ComboBox");
    // Statusbar related setup
    let statusbar_application: gtk::Statusbar = builder
        .get_object("statusbar_application")
        .expect("Couldn't get Statusbar");
    let context_id_port_ops = statusbar_application.get_context_id("port operations");
    let context_map: HashMap<StatusContext, u32> =
        [(StatusContext::PortOperation, context_id_port_ops)]
            .iter()
            .cloned()
            .collect();

    window_application.set_application(Some(app));

    let ui = Ui {
        window_application: window_application.clone(),
        combo_box_ports: combo_box_ports.clone(),
        statusbar_application: statusbar_application.clone(),
        statusbar_contexts: context_map,
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
                    log_status(&ui, StatusContext::PortOperation, "OK Event");
                }
                Err(e) => {
                    log_status(&ui, StatusContext::PortOperation, &format!("Error: {:?}", e));
                }
            }
        }
    });
    glib::Continue(false)
}

/// Log messages to the status bar using the specific status context.
fn log_status(ui: &Ui, context: StatusContext, message: &str) {
    let context_id = ui.statusbar_contexts.get(&context).unwrap();
    let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S");
    let formatted_message = format!("[{}]: {}", timestamp, message);
    ui.statusbar_application.push(0, &formatted_message);
}
