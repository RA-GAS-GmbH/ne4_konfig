use crate::tokio_thread;
use crate::tokio_thread::{TokioCommand, TokioThread};
use chrono::Utc;
use gio::prelude::*;
use glib::clone;
use glib::{signal_handler_block, signal_handler_unblock};
use gtk::prelude::*;
use gtk::Application;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[macro_use]
pub mod macros;
pub mod treestore_values;

const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &'static str = env!("CARGO_PKG_NAME");
const PKG_DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

pub struct Ui {
    // application_window: gtk::ApplicationWindow,
    // combo_box_text_sensor_working_mode_map: HashMap<String, u16>,
    // toggle_button_connect_toggle_signal: glib::SignalHandlerId,
    button_messgas: gtk::Button,
    button_new_modbus_address: gtk::Button,
    button_nullpunkt: gtk::Button,
    button_reset: gtk::Button,
    button_sensor_working_mode: gtk::Button,
    combo_box_text_ports_changed_signal: glib::SignalHandlerId,
    combo_box_text_ports_map: Rc<RefCell<HashMap<String, u32>>>,
    combo_box_text_ports: gtk::ComboBoxText,
    combo_box_text_sensor_working_mode: gtk::ComboBoxText,
    entry_modbus_address: gtk::Entry,
    label_sensor_ma_value: gtk::Label,
    label_sensor_type_value: gtk::Label,
    label_sensor_value_value: gtk::Label,
    list_store_sensor: gtk::ListStore,
    statusbar_application: gtk::Statusbar,
    statusbar_contexts: HashMap<StatusContext, u32>,
    toggle_button_connect: gtk::ToggleButton,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum StatusContext {
    PortOperation,
    Error,
}

#[derive(Debug)]
pub enum UiCommand {
    DisableConnectUiElements,
    Error(String),
    Disconnect,
    EnableConnectUiElements,
    UpdatePorts(Vec<String>),
    UpdateSensorType(String),
    UpdateSensorValue(u16),
    UpdateSensorValues(Result<Vec<u16>, mio_serial::Error>),
    NewWorkingMode(tokio::io::Result<()>),
    NewModbusAddress(tokio::io::Result<()>),
    Nullpunkt(tokio::io::Result<()>),
    Messgas(tokio::io::Result<()>),
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
    // The tokio thread new function gets the `ui_event_sender` for communication with the
    // UI thread. TokioThread::new() returns the `tokio_thread_sender` to communicate with the
    // TokioThread.
    let (ui_event_sender, mut ui_event_receiver) = futures::channel::mpsc::channel(0);
    let tokio_thread = TokioThread::new(ui_event_sender);
    let tokio_thread_sender = tokio_thread.tokio_thread_sender;

    // Now build the UI
    let glade_str = include_str!("main.ui");
    let builder = gtk::Builder::new_from_string(glade_str);
    let application_window: gtk::ApplicationWindow = build!(builder, "application_window");
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
    let mut combo_box_text_ports_map = HashMap::<String, u32>::new();
    scan_ports(&combo_box_text_ports, &mut combo_box_text_ports_map);

    // Sensor Working Mode selector
    let combo_box_text_sensor_working_mode: gtk::ComboBoxText =
        build!(builder, "combo_box_text_sensor_working_mode");
    combo_box_text_sensor_working_mode.set_sensitive(false);
    #[allow(unused_assignments)]
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
    for (name, id) in combo_box_text_sensor_working_mode_map.clone() {
        combo_box_text_sensor_working_mode.append(Some(&id.to_string()), &name);
    }

    // Modbus Adresse
    let entry_modbus_address: gtk::Entry = build!(builder, "entry_modbus_address");
    entry_modbus_address.set_sensitive(false);
    let entry_new_modbus_address: gtk::Entry = build!(builder, "entry_new_modbus_address");
    entry_new_modbus_address.set_sensitive(false);

    // Reset Button
    let button_reset: gtk::Button = build!(builder, "button_reset");
    // Labels Sensor Werte
    let label_sensor_type_value: gtk::Label = build!(builder, "label_sensor_type_value");
    let button_nullpunkt: gtk::Button = build!(builder, "button_nullpunkt");
    button_nullpunkt.set_sensitive(false);
    let button_messgas: gtk::Button = build!(builder, "button_messgas");
    button_messgas.set_sensitive(false);
    let button_new_modbus_address: gtk::Button = build!(builder, "button_new_modbus_address");
    button_new_modbus_address.set_sensitive(false);
    let button_sensor_working_mode: gtk::Button = build!(builder, "button_sensor_working_mode");
    button_sensor_working_mode.set_sensitive(false);

    // ListStore Sensor Values
    let list_store_sensor: gtk::ListStore = build!(builder, "list_store_sensor");

    // Connect button, disabled if no ports available
    let toggle_button_connect: gtk::ToggleButton = build!(builder, "toggle_button_connect");
    if combo_box_text_ports_map.is_empty() {
        toggle_button_connect.set_sensitive(false);
    }

    let label_sensor_value_value: gtk::Label = build!(builder, "label_sensor_value_value");

    let label_sensor_ma_value: gtk::Label = build!(builder, "label_sensor_ma_value");

    let menu_item_quit: gtk::MenuItem = build!(builder, "menu_item_quit");
    let menu_item_about: gtk::MenuItem = build!(builder, "menu_item_about");

    let header_bar: gtk::HeaderBar = build!(builder, "header_bar");
    let about_dialog: gtk::AboutDialog = build!(builder, "about_dialog");
    let about_dialog_button_ok: gtk::Button = build!(builder, "about_dialog_button_ok");

    header_bar.set_title(Some(PKG_NAME));
    #[cfg(feature = "ra-gas")]
    header_bar.set_title(Some(&format!("{} - RA-GAS intern!", PKG_NAME)));
    header_bar.set_subtitle(Some(PKG_VERSION));

    about_dialog.set_program_name(PKG_NAME);
    #[cfg(feature = "ra-gas")]
    about_dialog.set_program_name(&format!("{} - RA-GAS intern!", PKG_NAME));
    about_dialog.set_version(Some(PKG_VERSION));
    about_dialog.set_comments(Some(PKG_DESCRIPTION));

    // #[cfg(feature = "ra-gas")]
    if cfg!(feature = "ra-gas") {
        let hbox_new_modbus_address: gtk::Box = build!(builder, "hbox_new_modbus_address");
        let check_button_mcs = gtk::CheckButton::new_with_label("MCS");
        hbox_new_modbus_address.pack_end(&check_button_mcs, false, false, 0);
        hbox_new_modbus_address.reorder_child(&check_button_mcs, 1);
    }

    application_window.set_application(Some(app));

    //
    // CSS
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
    #[cfg(feature = "ra-gas")]
    css_provider
        .load_from_path("resources/ra-gas.css")
        .expect("Failed to load CSS stylesheet (ra-gas features)");
    //
    // Callbacks
    //

    // let combo_box_text_ports_changed_signal = combo_box_text_ports.connect_changed(clone!(
    // @strong combo_box_text_ports,
    // @strong tokio_thread_sender
    // => move |_| {
    //
    // }));
    let combo_box_text_ports_changed_signal = combo_box_text_ports.connect_changed(move |_| {});

    toggle_button_connect.connect_clicked(clone!(
            @strong combo_box_text_ports_map,
            @strong combo_box_text_ports,
            @strong entry_modbus_address,
            @strong tokio_thread_sender
            => move |s| {
                if s.get_active() {
                    // get port
                    let active_port = combo_box_text_ports.get_active().unwrap_or(0);
                    info!("active_port: {:?}", &active_port);

                    let mut port = None;
                    for (p, i) in &combo_box_text_ports_map {
                        if *i == active_port {
                            port = Some(p.to_owned());
                            break;
                        }
                    }
                    info!("port: {:?}", &port);

                    // get modbus_address
                    let modbus_address = entry_modbus_address.get_text().unwrap_or("247".into());
                    info!("port: {:?}, modbus_address: {:?}", &port, &modbus_address);

                    tokio_thread_sender
                        .clone()
                        .try_send(TokioCommand::Connect)
                        .expect("Faild to send tokio command");

                    tokio_thread_sender
                        .clone()
                        .try_send(TokioCommand::UpdateSensor(port, modbus_address.parse().unwrap_or(247)))
                        .expect("Faild to send tokio command");

                } else {
            tokio_thread_sender
                .clone()
                .try_send(TokioCommand::Disconnect)
                .expect("Faild to send tokio command");
        }
            }
    ));

    button_new_modbus_address.connect_clicked(clone!(
        @strong combo_box_text_ports,
        @strong combo_box_text_ports_map,
        @strong entry_modbus_address,
        @strong entry_new_modbus_address,
        @strong tokio_thread_sender
        => move |_| {
            let active_port = combo_box_text_ports.get_active().unwrap_or(0);
            let mut port = None;
            for (p, i) in &combo_box_text_ports_map {
                if *i == active_port {
                    port = Some(p.to_owned());
                    break;
                }
            }
            let modbus_address = entry_modbus_address.get_text().unwrap_or("0".into());
            let new_modbus_address = entry_new_modbus_address.get_text().unwrap_or("0".into());

            tokio_thread_sender
                .clone()
                .try_send(TokioCommand::NewModbusAddress(port, modbus_address.to_owned().parse().unwrap_or(0), new_modbus_address.to_owned().parse().unwrap_or(0)))
                .expect("Faild to send tokio command");
        }
    ));

    button_nullpunkt.connect_clicked(clone!(
        @strong combo_box_text_ports,
        @strong combo_box_text_ports_map,
        @strong entry_modbus_address,
        @strong tokio_thread_sender
        => move |_| {
            let active_port = combo_box_text_ports.get_active().unwrap_or(0);
            let mut port = None;
            for (p, i) in &combo_box_text_ports_map {
                if *i == active_port {
                    port = Some(p.to_owned());
                    break;
                }
            }
            let modbus_address = entry_modbus_address.get_text().unwrap_or("0".into());

            tokio_thread_sender
                .clone()
                .try_send(TokioCommand::Nullpunkt(port, modbus_address.to_owned().parse().unwrap_or(0)))
                .expect("Faild to send tokio command");
    }));

    button_messgas.connect_clicked(clone!(
        @strong combo_box_text_ports,
        @strong combo_box_text_ports_map,
        @strong entry_modbus_address,
        @strong tokio_thread_sender
        => move |_| {
            let active_port = combo_box_text_ports.get_active().unwrap_or(0);
            let mut port = None;
            for (p, i) in &combo_box_text_ports_map {
                if *i == active_port {
                    port = Some(p.to_owned());
                    break;
                }
            }
            let modbus_address = entry_modbus_address.get_text().unwrap_or("0".into());

            tokio_thread_sender
                .clone()
                .try_send(TokioCommand::Messgas(port, modbus_address.to_owned().parse().unwrap_or(0)))
                .expect("Faild to send tokio command");
    }));

    button_reset.connect_clicked(clone!(
        @strong entry_modbus_address => move |_| {
        entry_modbus_address.set_text("247");
    }));

    button_sensor_working_mode.connect_clicked(clone!(
        @strong entry_modbus_address,
        @strong combo_box_text_ports_map,
        @strong combo_box_text_ports,
        @strong combo_box_text_sensor_working_mode,
        @strong tokio_thread_sender => move |_| {
            let active_port = combo_box_text_ports.get_active().unwrap_or(0);
            let mut port = None;
            for (p, i) in &combo_box_text_ports_map {
                if *i == active_port {
                    port = Some(p.to_owned());
                    break;
                }
            }
            let modbus_address = entry_modbus_address.get_text().unwrap_or("247".into());
            let working_mode = combo_box_text_sensor_working_mode.get_active_id().unwrap_or("0".into());

            tokio_thread_sender
                .clone()
                .try_send(TokioCommand::NewWorkingMode(port, modbus_address.to_owned().parse().unwrap_or(0), working_mode.to_owned().parse().unwrap_or(0)))
                .expect("Faild to send tokio command");
    }));

    menu_item_quit.connect_activate(clone!(
        @weak application_window => move |_| {
            application_window.destroy()
        }
    ));

    menu_item_about.connect_activate(clone!(
        @strong about_dialog => move |_| {
            about_dialog.show()
        }
    ));

    about_dialog_button_ok.connect_clicked(clone!(
        @strong about_dialog => move |_| {
            about_dialog.hide()
        }
    ));

    // Zugriff auf die Elemente der UI
    let ui = Ui {
        // application_window: application_window.clone(),
        // combo_box_text_sensor_working_mode_map,
        // toggle_button_connect_toggle_signal,
        button_messgas,
        button_new_modbus_address,
        button_nullpunkt,
        button_reset,
        button_sensor_working_mode,
        combo_box_text_ports_changed_signal,
        combo_box_text_ports_map: Rc::new(RefCell::new(combo_box_text_ports_map)),
        combo_box_text_ports,
        combo_box_text_sensor_working_mode,
        entry_modbus_address,
        label_sensor_ma_value,
        label_sensor_type_value,
        label_sensor_value_value,
        list_store_sensor,
        statusbar_application,
        statusbar_contexts: context_map,
        toggle_button_connect,
    };

    application_window.show_all();

    // future on main thread has access to UI
    let future = {
        use futures::stream::StreamExt;

        async move {
            while let Some(event) = ui_event_receiver.next().await {
                match event {
                    UiCommand::DisableConnectUiElements => {
                        info!("Execute event UiCommand::DisableConnectUiElements");
                        disable_ui_elements(&ui);
                        // log_status(
                        //     &ui,
                        //     StatusContext::PortOperation,
                        //     &format!(""),
                        // );
                    }
                    UiCommand::EnableConnectUiElements => {
                        info!("Execute event UiCommand::EnableConnectUiElements");
                        enable_ui_elements(&ui);
                        // log_status(
                        //     &ui,
                        //     StatusContext::PortOperation,
                        //     &format!(""),
                        // );
                    }
                    UiCommand::Disconnect => {
                        info!("Execute event UiCommand::Disconnect");
                        &ui.toggle_button_connect.set_active(false);
                        &ui.combo_box_text_ports.set_sensitive(true);
                        &ui.combo_box_text_sensor_working_mode.set_sensitive(true);
                        &ui.entry_modbus_address.set_sensitive(true);
                        &ui.button_reset.set_sensitive(true);

                        tokio_thread_sender
                            .clone()
                            .try_send(TokioCommand::Disconnect)
                            .expect("Faild to send tokio command");
                        // log_status(
                        //     &ui,
                        //     StatusContext::PortOperation,
                        //     &format!("Disconnect"),
                        // );
                    }
                    // FIXME: kann weg
                    UiCommand::UpdateSensorType(text) => {
                        info!("Execute event UiCommand::UpdateSensorType");
                        &ui.label_sensor_type_value.set_text(&text);
                        // log_status(
                        //     &ui,
                        //     StatusContext::PortOperation,
                        //     &format!("Update Sensor Value: {:?}", &text),
                        // );
                    }
                    UiCommand::Error(e) => {
                        info!("Execute event UiCommand::Error");
                        log_status(
                            &ui,
                            StatusContext::PortOperation,
                            &format!("Error: {:?}", e),
                        );
                    }
                    UiCommand::UpdatePorts(ports) => {
                        info!("Execute event UiCommand::UpdatePorts: {:?}", ports);
                        // Update the port listing and other UI elements
                        ui.combo_box_text_ports.remove_all();
                        ui.combo_box_text_ports_map.borrow_mut().clear();
                        if ports.is_empty() {
                            disable_ui_elements(&ui);
                            ui.combo_box_text_ports
                                .append(None, "Keine Schnittstelle gefunden");
                            ui.combo_box_text_ports.set_active(Some(0));
                            ui.combo_box_text_ports.set_sensitive(false);
                            ui.toggle_button_connect.set_sensitive(false);
                        } else {
                            enable_ui_elements(&ui);
                            for (i, p) in (0u32..).zip(ports.clone().into_iter()) {
                                ui.combo_box_text_ports.append(None, &p);
                                ui.combo_box_text_ports_map.borrow_mut().insert(p, i);
                            }
                            signal_handler_block(
                                &ui.combo_box_text_ports,
                                &ui.combo_box_text_ports_changed_signal,
                            );
                            ui.combo_box_text_ports.set_active(Some(0));
                            signal_handler_unblock(
                                &ui.combo_box_text_ports,
                                &ui.combo_box_text_ports_changed_signal,
                            );
                            ui.combo_box_text_ports.set_sensitive(true);
                            ui.toggle_button_connect.set_sensitive(true);
                        }

                        log_status(
                            &ui,
                            StatusContext::PortOperation,
                            &format!(
                                "Ports found: {:?}; ports_map: {:?}",
                                ports,
                                &ui.combo_box_text_ports_map.borrow()
                            ),
                        );
                    }
                    // FIXME: kann weg
                    UiCommand::UpdateSensorValue(value) => {
                        info!("Execute event UiCommand::UpdateSensorValue");
                        let value = format!("{}", value);
                        &ui.label_sensor_value_value.set_text(&value);
                        // log_status(
                        //     &ui,
                        //     StatusContext::PortOperation,
                        //     &format!("Update Sensor Value: {:?}", &value),
                        // );
                    }
                    UiCommand::UpdateSensorValues(values) => {
                        info!("Execute event UiCommand::UpdateSensorValues");
                        debug!("{:?}", values);
                        match values {
                            Ok(values) => {
                                &ui.combo_box_text_sensor_working_mode
                                    .set_active_id(Some(&values[1].to_string()));
                                &ui.label_sensor_value_value.set_text(&values[2].to_string());
                                let sensor_ma: f32 = values[3] as f32 / 100.0;
                                let sensor_ma = format!("{:.02}", sensor_ma);
                                &ui.label_sensor_ma_value.set_text(&sensor_ma);
                                if let Some(iter) = &ui.list_store_sensor.get_iter_first() {
                                    let _: Vec<u32> = values
                                        .iter()
                                        .enumerate()
                                        .map(|(i, val)| {
                                            let reg = &ui
                                                .list_store_sensor
                                                .get_value(&iter, 0)
                                                .get::<u32>()
                                                .unwrap_or(Some(0))
                                                .unwrap_or(0);
                                            // create the glib::value::Value from a u16 this is complicated (see supported types: https://gtk-rs.org/docs/glib/value/index.html)
                                            let val = (*val as u32).to_value();
                                            if i as u32 == *reg {
                                                &ui.list_store_sensor.set_value(&iter, 1, &val);
                                                &ui.list_store_sensor.iter_next(&iter);
                                            }
                                            0
                                        })
                                        .collect();
                                    // Status log
                                    log_status(
                                        &ui,
                                        StatusContext::PortOperation,
                                        &format!("Sensor Update OK"),
                                    );
                                } else {
                                    log_status(
                                        &ui,
                                        StatusContext::Error,
                                        &format!("Error while iterating Sensor list"),
                                    );
                                }
                            }
                            Err(err) => {
                                // Status log
                                log_status(
                                    &ui,
                                    StatusContext::Error,
                                    &format!("Error while Sensor Update: {}", err),
                                );
                            }
                        }
                    }
                    UiCommand::NewModbusAddress(value) => {
                        log_status(
                            &ui,
                            StatusContext::PortOperation,
                            &format!("Neue Modbus Adresse: {:?}", &value),
                        );
                    }
                    UiCommand::NewWorkingMode(value) => {
                        log_status(
                            &ui,
                            StatusContext::PortOperation,
                            &format!("Arbeitsweise: {:?}", &value),
                        );
                    }
                    UiCommand::Nullpunkt(value) => {
                        log_status(
                            &ui,
                            StatusContext::PortOperation,
                            &format!("Nullpunkt: {:?}", &value),
                        );
                    }
                    UiCommand::Messgas(value) => {
                        log_status(
                            &ui,
                            StatusContext::PortOperation,
                            &format!("Messgas: {:?}", &value),
                        );
                    }
                }
            }
        }
    };

    let c = glib::MainContext::default();
    c.spawn_local(future);
}

fn enable_ui_elements(ui: &Ui) {
    ui.toggle_button_connect.set_active(false);
    ui.combo_box_text_ports.set_sensitive(true);
    ui.combo_box_text_sensor_working_mode.set_sensitive(true);
    ui.entry_modbus_address.set_sensitive(true);
    ui.button_reset.set_sensitive(true);
    // FIXME: Remove this hardcoded value
    ui.label_sensor_type_value
        .set_text("RA-GAS GmbH - NE4_MOD_BUS");
    ui.button_nullpunkt.set_sensitive(true);
    ui.button_messgas.set_sensitive(true);
    ui.button_new_modbus_address.set_sensitive(true);
    ui.button_sensor_working_mode.set_sensitive(true);
}

fn disable_ui_elements(ui: &Ui) {
    // ui.toggle_button_connect.set_active(true);
    ui.combo_box_text_ports.set_sensitive(false);
    ui.combo_box_text_sensor_working_mode.set_sensitive(false);
    ui.entry_modbus_address.set_sensitive(false);
    ui.button_reset.set_sensitive(false);
    // FIXME: Remove this hardcoded value
    ui.label_sensor_type_value
        .set_text("RA-GAS GmbH - NE4_MOD_BUS");
    ui.button_nullpunkt.set_sensitive(false);
    ui.button_messgas.set_sensitive(false);
    ui.button_new_modbus_address.set_sensitive(false);
    ui.button_sensor_working_mode.set_sensitive(false);
}

/// Log messages to the status bar using the specific status context.
fn log_status(ui: &Ui, context: StatusContext, message: &str) {
    if let Some(context_id) = ui.statusbar_contexts.get(&context) {
        let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S");
        let formatted_message = format!("[{}]: {}", timestamp, message);
        ui.statusbar_application
            .push(*context_id, &formatted_message);
    }
}

fn scan_ports(
    combo_box_text_ports: &gtk::ComboBoxText,
    combo_box_text_ports_map: &mut HashMap<String, u32>,
) {
    let ports = tokio_thread::scan_ports();
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
}
