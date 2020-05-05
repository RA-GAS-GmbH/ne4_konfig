use crate::serial_thread::{list_ports, SerialThread};
use chrono::Utc;
use gio::prelude::*;
use glib;
use gtk::prelude::*;
use gtk::Application;
use std::cell::RefCell;
use std::collections::HashMap;


/// Build some with builder ><
/// ```
/// let window_main: gtk::Window = builder.get_object("window_main").expect("Could not find 'window_main' in glade file");
/// let window_main: gtk::Window = build!(builder, "window_main");
/// ```
#[macro_export] macro_rules! build {
    ($builder:ident, $e:expr) => { $builder.get_object($e).expect(&format!("Could not find '{}' in glade file", $e)) };
}


#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum StatusContext {
    PortOperation,
}

pub struct Ui {
    window_application: gtk::ApplicationWindow,
    combo_box_text_ports: gtk::ComboBoxText,
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
    let window_application: gtk::ApplicationWindow = build!(builder, "window_application");
    let combo_box_text_ports: gtk::ComboBoxText = build!(builder, "combo_box_text_ports");
    // Statusbar
    let statusbar_application: gtk::Statusbar = build!(builder, "statusbar_application");
    let context_id_port_ops = statusbar_application.get_context_id("port operations");
    let context_map: HashMap<StatusContext, u32> =
        [(StatusContext::PortOperation, context_id_port_ops)]
            .iter()
            .cloned()
            .collect();
    // Serial port selector
    let combo_box_text_ports: gtk::ComboBoxText = build!(builder, "combo_box_text_ports");
    let mut combo_box_text_ports_map = HashMap::new();
    if let Ok(mut ports) = list_ports() {
        ports.sort();
        if !ports.is_empty() {
            for (i, p) in (0u32..).zip(ports.into_iter()) {
                combo_box_text_ports.append(None, &p);
                combo_box_text_ports_map.insert(p, i);
            }
            combo_box_text_ports.set_active(Some(0));
        } else {
            combo_box_text_ports.append(None, "Keine Schnittstelle gefunden");
            combo_box_text_ports.set_active(Some(0));
            combo_box_text_ports.set_sensitive(false);
        }
    } else {
        combo_box_text_ports.append(None, "Keine Schnittstelle gefunden");
        combo_box_text_ports.set_active(Some(0));
        combo_box_text_ports.set_sensitive(false);
    }


    window_application.set_application(Some(app));

    let ui = Ui {
        window_application: window_application.clone(),
        combo_box_text_ports: combo_box_text_ports.clone(),
        statusbar_application: statusbar_application.clone(),
        statusbar_contexts: context_map,
    };

    let state = State {
        connected_port: None,
    };

    // Start SerialThread
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

// Die `receive` Funktion handelt "events" vom SerialThread
fn receive() -> glib::Continue {
    GLOBAL.with(|global| {
        if let Some((ref mut ui, ref serial_thread, ref mut state)) = *global.borrow_mut() {
            match serial_thread.from_port_chan_rx.try_recv() {
                Ok(_) => {
                    info!("Unhandled Event in GUI!");
                    log_status(&ui, StatusContext::PortOperation, "OK Event");
                }
                Err(e) => {
                    log_status(
                        &ui,
                        StatusContext::PortOperation,
                        &format!("Error: {:?}", e),
                    );
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
