use crate::tokio_thread::*;
use chrono::Utc;
use gio::prelude::*;
use glib::clone;
use glib::{signal_handler_block, signal_handler_unblock};
use gtk::prelude::*;
use gtk::Application;
use std::collections::HashMap;

#[macro_use]
pub mod macros;

pub struct Ui {
    // application_window: gtk::ApplicationWindow,
    // button_messgas: gtk::Button,
    // button_nullpunkt: gtk::Button,
    // button_reset: gtk::Button,
    combo_box_text_ports_changed_signal: glib::SignalHandlerId,
    combo_box_text_ports_map: HashMap<String, u32>,
    combo_box_text_ports: gtk::ComboBoxText,
    // combo_box_text_sensor_working_mode_map: HashMap<String, u16>,
    combo_box_text_sensor_working_mode: gtk::ComboBoxText,
    // entry_modbus_address: gtk::Entry,
    label_sensor_type_value_value: gtk::Label,
    label_sensor_type_value: gtk::Label,
    // list_store_sensor: gtk::ListStore,
    statusbar_application: gtk::Statusbar,
    statusbar_contexts: HashMap<StatusContext, u32>,
    // toggle_button_connect_toggle_signal: glib::SignalHandlerId,
    toggle_button_connect: gtk::ToggleButton,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum StatusContext {
    PortOperation,
}

#[derive(Debug)]
pub enum UiCommand {
    PortsFound(Vec<String>),
    Timeout,
    UpdateSensorType(String),
    UpdateSensorValue(u16),
    UpdateSensorValues(Result<Vec<u16>, mio_serial::Error>),
}

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
    // Create and start the tokio thread
    // communication erfolgt via the tokio_thread_sender
    let (ui_event_sender, mut ui_event_receiver) = futures::channel::mpsc::channel(0);
    let tokio_thread = TokioThread::new(ui_event_sender);
    let tokio_thread_sender = tokio_thread.tokio_thread_sender;

    // Now build the UI
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
        ports.reverse();
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

    // Sensor Working Mode selector
    let combo_box_text_sensor_working_mode: gtk::ComboBoxText =
        build!(builder, "combo_box_text_sensor_working_mode");
    let mut combo_box_text_sensor_working_mode_map = HashMap::new();
    combo_box_text_sensor_working_mode_map = [
        ("Unkonfiguriert".to_string(), 0),
        ("CO 1000 ppm".to_string(), 10),
        ("CO 300 ppm".to_string(), 12),
        ("NO 250 ppm".to_string(), 20),
        ("NO2 20 ppm".to_string(), 30),
        ("NH3 1000 ppm".to_string(), 40),
        ("NH3 100 ppm".to_string(), 42),
        ("CL2 10 ppm".to_string(), 50),
        ("H2S 25 ppm".to_string(), 60),
    ]
    .iter()
    .cloned()
    .collect();
    for (name, id) in combo_box_text_sensor_working_mode_map {
        combo_box_text_sensor_working_mode.append(Some(&id.to_string()), &name);
    }
    // Modbus Adresse
    let entry_modbus_address: gtk::Entry = build!(builder, "entry_modbus_address");
    // Reset Button
    let button_reset: gtk::Button = build!(builder, "button_reset");
    // Labels Sensor Werte
    let label_sensor_type_value: gtk::Label = build!(builder, "label_sensor_type_value");
    let button_nullpunkt: gtk::Button = build!(builder, "button_nullpunkt");
    let button_messgas: gtk::Button = build!(builder, "button_messgas");

    // ListStore Sensor Values
    let _list_store_sensor: gtk::ListStore = build!(builder, "list_store_sensor");

    // Connect button, disabled if no ports available
    let toggle_button_connect: gtk::ToggleButton = build!(builder, "toggle_button_connect");
    if combo_box_text_ports_map.is_empty() {
        toggle_button_connect.set_sensitive(false);
    }

    let label_sensor_type_value_value: gtk::Label =
        build!(builder, "label_sensor_type_value_value");

    application_window.set_application(Some(app));

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

    // Callbacks
    let combo_box_text_ports_changed_signal = combo_box_text_ports.connect_changed(move |_| {});

    let _toggle_button_connect_toggle_signal = toggle_button_connect.connect_clicked(clone!(
            @strong button_reset,
            @strong combo_box_text_ports_map,
            @strong combo_box_text_sensor_working_mode,
            @strong entry_modbus_address,
            @strong label_sensor_type_value,
        @strong combo_box_text_ports,
        @strong tokio_thread_sender
            => move |s| {
        if s.get_active() {
            combo_box_text_ports.set_sensitive(false);
            combo_box_text_sensor_working_mode.set_sensitive(false);
            entry_modbus_address.set_sensitive(false);
            button_reset.set_sensitive(false);
            // label_sensor_type_value.set_text("RA-GAS GmbH - NE4-MOD-BUS");
            // get port
            let active_port = combo_box_text_ports.get_active().unwrap_or(0);
            let mut port = None;
            for (p, i) in &combo_box_text_ports_map {
                if *i == active_port {
                    port = Some(p.to_owned());
                    break;
                }
            }
            // get modbus_address
            let modbus_address = entry_modbus_address.get_text().unwrap_or("0".into());
            tokio_thread_sender
                .clone()
                .try_send(TokioCommand::UpdateSensor(port, modbus_address.parse().unwrap()))
                .expect("send UI event from Nullpunkt button");
        } else {
            combo_box_text_ports.set_sensitive(true);
            combo_box_text_sensor_working_mode.set_sensitive(true);
            entry_modbus_address.set_sensitive(true);
            button_reset.set_sensitive(true);
            label_sensor_type_value.set_text("");
        }
    }));

    button_nullpunkt.connect_clicked(clone!(
        @strong tokio_thread_sender => move |_| {
            tokio_thread_sender
                .clone()
                .try_send(TokioCommand::Nullpunkt)
                .expect("Faild to send tokio command");

                tokio_thread_sender
                .clone()
                .try_send(TokioCommand::Connect)
                .expect("Faild to send tokio command");

                tokio_thread_sender
                .clone()
                .try_send(TokioCommand::ReadRegistersLoop)
                .expect("Faild to send tokio command");
    }));

    button_messgas.connect_clicked(clone!(
        @strong tokio_thread_sender => move |_| {
            tokio_thread_sender
                .clone()
                .try_send(TokioCommand::Messgas)
                .expect("Faild to send tokio command");

            tokio_thread_sender
                .clone()
                .try_send(TokioCommand::Disconnect)
                .expect("Faild to send tokio command");
    }));

    button_reset.connect_clicked(clone!(@strong entry_modbus_address => move |_| {
        entry_modbus_address.set_text("247");
    }));

    // Zugriff auf die Elemente der UI
    let mut ui = Ui {
        // application_window: application_window.clone(),
        // button_messgas,
        // button_nullpunkt,
        // button_reset,
        combo_box_text_ports_changed_signal,
        combo_box_text_ports_map,
        combo_box_text_ports,
        // combo_box_text_sensor_working_mode_map,
        combo_box_text_sensor_working_mode,
        // entry_modbus_address,
        label_sensor_type_value_value,
        label_sensor_type_value,
        // list_store_sensor,
        statusbar_application,
        statusbar_contexts: context_map,
        // toggle_button_connect_toggle_signal,
        toggle_button_connect,
    };

    application_window.show_all();

    // future on main thread has access to UI
    let future = {
        use futures::stream::StreamExt;

        async move {
            while let Some(event) = ui_event_receiver.next().await {
                match event {
                    UiCommand::UpdateSensorType(text) => {
                        log_status(
                            &ui,
                            StatusContext::PortOperation,
                            &format!("Connect!: {:?}", &text),
                        );
                        &ui.label_sensor_type_value.set_text(&text);
                    }
                    UiCommand::Timeout => {
                        info!("Timeout!");
                        log_status(&ui, StatusContext::PortOperation, &format!("Timeout!"));
                    }
                    UiCommand::PortsFound(ports) => {
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
                    UiCommand::UpdateSensorValue(value) => {
                        let value = format!("{}", value);
                        &ui.label_sensor_type_value_value.set_text(&value);
                    }
                    UiCommand::UpdateSensorValues(values) => {
                        let values = values.unwrap();
                        info!("Update Sensor: {:?}", values[0]);
                        &ui.combo_box_text_sensor_working_mode.set_active(Some(0));
                        &ui.label_sensor_type_value_value
                            .set_text(&values[2].to_string());
                        // log_status(
                        //     &ui,
                        //     StatusContext::PortOperation,
                        //     &format!("Update Sensor: {:?}", &values),
                        // );
                    }
                }
            }
        }
    };

    let c = glib::MainContext::default();
    c.spawn_local(future);
}

/// Log messages to the status bar using the specific status context.
fn log_status(ui: &Ui, context: StatusContext, message: &str) {
    let _context_id = ui.statusbar_contexts.get(&context).unwrap();
    let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S");
    let formatted_message = format!("[{}]: {}", timestamp, message);
    ui.statusbar_application.push(0, &formatted_message);
}
