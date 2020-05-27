use crate::{
    serial_thread::{list_ports, SerialResponse, SerialThread},
    tokio_thread::*,
};
use chrono::Utc;
use gio::prelude::*;
use glib::{signal_handler_block, signal_handler_unblock};
use gtk::prelude::*;
use gtk::Application;
use std::cell::RefCell;
use std::collections::HashMap;

#[macro_use]
pub mod macros;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum StatusContext {
    PortOperation,
}

pub struct Ui {
    button_reset: gtk::Button,
    button_nullpunkt: gtk::Button,
    button_messgas: gtk::Button,
    combo_box_text_ports_changed_signal: glib::SignalHandlerId,
    combo_box_text_ports_map: HashMap<String, u32>,
    combo_box_text_ports: gtk::ComboBoxText,
    entry_modbus_address: gtk::Entry,
    label_sensor_type_value: gtk::Label,
    label_sensor_working_mode_value: gtk::Label,
    list_store_sensor: gtk::ListStore,
    statusbar_application: gtk::Statusbar,
    statusbar_contexts: HashMap<StatusContext, u32>,
    toggle_button_connect_toggle_signal: glib::SignalHandlerId,
    toggle_button_connect: gtk::ToggleButton,
    application_window: gtk::ApplicationWindow,
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
    // Create/ start tokio thread
    let tokio_thread = TokioThread::new();
    let mut ui_event_sender = tokio_thread.ui_event_sender;
    let data_event_receiver = tokio_thread.data_event_receiver;

    let glade_str = include_str!("main.ui");
    let builder = gtk::Builder::new_from_string(glade_str);
    let application_window: gtk::ApplicationWindow = build!(builder, "application_window");
    let _combo_box_text_ports: gtk::ComboBoxText = build!(builder, "combo_box_text_ports");
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
    // Modbus Adresse
    let entry_modbus_address = build!(builder, "entry_modbus_address");
    // Reset Button
    let button_reset: gtk::Button = build!(builder, "button_reset");
    // Labels Sensor Werte
    let label_sensor_type_value: gtk::Label = build!(builder, "label_sensor_type_value");
    let label_sensor_working_mode_value: gtk::Label =
        build!(builder, "label_sensor_working_mode_value");
    let button_nullpunkt: gtk::Button = build!(builder, "button_nullpunkt");
    let button_messgas: gtk::Button = build!(builder, "button_messgas");

    // ListStore Sensor Values
    let list_store_sensor: gtk::ListStore = build!(builder, "list_store_sensor");

    // Connect button, disabled if no ports available
    let toggle_button_connect: gtk::ToggleButton = build!(builder, "toggle_button_connect");
    if combo_box_text_ports_map.is_empty() {
        toggle_button_connect.set_sensitive(false);
    }

    application_window.set_application(Some(app));

    // Callbacks
    // let combo_box_text_ports_changed_signal = combo_box_text_ports.connect_changed(move |s| {
    //     if let Some(port_name) = s.get_active_text() {
    //         GLOBAL.with(|global| {
    //             if let Some((_, ref serial_thread, _)) = *global.borrow() {
    //                 match serial_thread.send_port_change_port_cmd(port_name.to_string()) {
    //                     Err(_) => {}
    //                     Ok(_) => {}
    //                 }
    //             }
    //         });
    //     }
    // });

    let ui_event_sender1 = ui_event_sender.clone();
    let combo_box_text_ports_changed_signal = combo_box_text_ports.connect_changed(move |s| {
        if let Some(port_name) = s.get_active_text() {
            ui_event_sender1.clone().try_send(TokioCommand::ChangePort("".into())).expect("Send UI event");
        }
    });

    let toggle_button_connect_toggle_signal = toggle_button_connect.connect_clicked(move |s| {
        if s.get_active() {
            GLOBAL.with(|global| {
                if let Some((ref ui, _, _)) = *global.borrow() {
                    ui.combo_box_text_ports.set_sensitive(false);
                    ui.entry_modbus_address.set_sensitive(false);
                    ui.button_reset.set_sensitive(false);
                    ui.label_sensor_type_value
                        .set_text("RA-GAS GmbH - NE4-MOD-BUS");
                    ui.label_sensor_working_mode_value.set_text("10 CO 1000ppm");
                }
            });
        } else {
            GLOBAL.with(|global| {
                if let Some((ref ui, _, _)) = *global.borrow() {
                    ui.combo_box_text_ports.set_sensitive(true);
                    ui.entry_modbus_address.set_sensitive(true);
                    ui.button_reset.set_sensitive(true);
                    ui.label_sensor_type_value.set_text("");
                    ui.label_sensor_working_mode_value.set_text("");
                }
            });
        }
    });

    let ui_event_sender2 = ui_event_sender.clone();
    button_nullpunkt.connect_clicked(move |_| {
        ui_event_sender2.clone().try_send(TokioCommand::Connect).expect("send UI event from Nullpunkt button");
    });

    button_reset.connect_clicked(move |_| {
        GLOBAL.with(|global| {
            if let Some((ref ui, _, _)) = *global.borrow() {
                ui.entry_modbus_address.set_text("247");
            }
        });
    });

    let ui = Ui {
        button_reset,
        button_nullpunkt,
        button_messgas,
        combo_box_text_ports_changed_signal,
        combo_box_text_ports_map,
        combo_box_text_ports,
        entry_modbus_address,
        label_sensor_type_value,
        label_sensor_working_mode_value,
        list_store_sensor,
        statusbar_application,
        statusbar_contexts: context_map,
        toggle_button_connect_toggle_signal,
        toggle_button_connect,
        application_window: application_window.clone(),
    };

    let state = State {
        connected_port: None,
    };

    // Set CSS styles for the entire application.
    let css_provider = gtk::CssProvider::new();
    let display = gdk::Display::get_default().expect("Couldn't open default GDK display");
    let screen = display.get_default_screen();
    gtk::StyleContext::add_provider_for_screen(
        &screen,
        &css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
    css_provider
        .load_from_path("resources/style.css")
        .expect("Failed to load CSS stylesheet");

    // // Start SerialThread
    // GLOBAL.with(move |global| {
    //     *global.borrow_mut() = Some((
    //         ui,
    //         SerialThread::new(|| {
    //             glib::idle_add(receive);
    //         }),
    //         state,
    //     ));
    // });

    application_window.show_all();

    // future on main thread has access to UI
    let future = {
        let button_nullpunkt = &ui.button_nullpunkt.clone();
        let mut data_event_receiver = data_event_receiver;
        async move {
            use futures::stream::StreamExt;

            while let Some(event) = data_event_receiver.next().await {
                println!("Got some data_event: {:?}", event);

            }

        }
    };

    let c = glib::MainContext::default();
    c.spawn_local(future);
}

// Die `receive` Funktion handelt "events" vom SerialThread
fn receive() -> glib::Continue {
    GLOBAL.with(|global| {
        if let Some((ref mut ui, ref serial_thread, ref mut _state)) = *global.borrow_mut() {
            match serial_thread.from_port_chan_rx.try_recv() {
                Ok(SerialResponse::PortsFound(ports)) => {
                    info!("Found some ports!");
                    // Determine if the new found port match existing ones
                    let replace = {
                        if ports.len() != ui.combo_box_text_ports_map.len() {
                            true
                        } else {
                            ports
                                .iter()
                                .enumerate()
                                .map(|t| ui.combo_box_text_ports_map[t.1] != t.0 as u32)
                                .all(|x| x)
                        }
                    };

                    if replace {
                        // First save whichever the currently-selected port is
                        let current_port = {
                            let active_port = ui.combo_box_text_ports.get_active().unwrap_or(0);
                            let mut n = None;
                            for (p, i) in &ui.combo_box_text_ports_map {
                                if *i == active_port {
                                    n = Some(p.to_owned());
                                    break;
                                }
                            }
                            n
                        };

                        ui.combo_box_text_ports.remove_all();
                        ui.combo_box_text_ports_map.clear();
                        if ports.is_empty() {
                            ui.combo_box_text_ports
                                .append(None, "Keine Schnittstelle gefunden");
                            ui.combo_box_text_ports.set_sensitive(false);
                            &ui.toggle_button_connect.set_sensitive(false);
                        } else {
                            for (i, p) in (0u32..).zip(ports.into_iter()) {
                                ui.combo_box_text_ports.append(None, &p);
                                ui.combo_box_text_ports_map.insert(p, i);
                            }
                            ui.combo_box_text_ports.set_sensitive(true);
                            ui.toggle_button_connect.set_sensitive(true);
                        }
                        signal_handler_block(
                            &ui.combo_box_text_ports,
                            &ui.combo_box_text_ports_changed_signal,
                        );
                        if let Some(p) = current_port {
                            ui.combo_box_text_ports.set_active(Some(
                                *ui.combo_box_text_ports_map.get(&p).unwrap_or(&0),
                            ));
                        } else {
                            ui.combo_box_text_ports.set_active(Some(0));
                        }
                        signal_handler_unblock(
                            &ui.combo_box_text_ports,
                            &ui.combo_box_text_ports_changed_signal,
                        );
                    }
                }
                Ok(e) => {
                    info!("Unhandled Event in GUI!: {:?}", &e);
                    log_status(
                        &ui,
                        StatusContext::PortOperation,
                        &format!("Unhandled Event in GUI!: {:?}", &e),
                    );
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
    let _context_id = ui.statusbar_contexts.get(&context).unwrap();
    let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S");
    let formatted_message = format!("[{}]: {}", timestamp, message);
    ui.statusbar_application.push(0, &formatted_message);
}
