// Tokio Thread
mod tokio_thread {
    use crate::gui::gtk3::UiCommand;
    use futures::channel::mpsc::*;
    use futures::prelude::*;
    use tokio::time::*;

    #[derive(Debug)]
    pub enum TokioCommand {
        Nullpunkt,
        Messgas,
        ReadRegistersLoop,
        Connect,
        Disconnect,
    }

    #[derive(Debug, PartialEq)]
    enum TokioState {
        Connected,
        Disconnected,
    }

    pub struct TokioThread {
        pub tokio_thread_sender: Sender<TokioCommand>,
    }

    impl TokioThread {
        pub fn new(ui_event_sender: Sender<UiCommand>) -> Self {
            let (tokio_thread_sender, mut tokio_thread_receiver) =
                futures::channel::mpsc::channel(0);

            std::thread::spawn(move || {
                // Tokio Thread
                let mut rt = tokio::runtime::Runtime::new().expect("create tokio runtime");
                // Shared State
                let state = std::sync::Arc::new(tokio::sync::Mutex::new(TokioState::Disconnected));

                rt.block_on(async {
                    while let Some(event) = tokio_thread_receiver.next().await {
                        debug!("Tokio Thread got event: TokioCommand::{:?}", event);
                        match event {
                            TokioCommand::ReadRegistersLoop => {
                                read_registers(ui_event_sender.clone(), state.clone())
                                    .await
                                    .expect("Could not start read registers loop")
                            }
                            TokioCommand::Connect => {
                                let mut state = state.lock().await;
                                *state = TokioState::Connected;
                            }
                            TokioCommand::Disconnect => {
                                let mut state = state.lock().await;
                                *state = TokioState::Disconnected;
                            }
                            TokioCommand::Nullpunkt => ui_event_sender
                                .clone()
                                .send(UiCommand::UpdateSensorType("Nullpunkt".into()))
                                .await
                                .expect("Failed to send Ui command"),
                            TokioCommand::Messgas => ui_event_sender
                                .clone()
                                .send(UiCommand::UpdateSensorType("Messgas".into()))
                                .await
                                .expect("Failed to send Ui command"),
                        }
                    }
                })
            });

            TokioThread {
                tokio_thread_sender,
            }
        }
    }

    async fn read_registers(
        ui_event_sender: Sender<UiCommand>,
        state: std::sync::Arc<tokio::sync::Mutex<TokioState>>,
    ) -> tokio::io::Result<()> {
        // TODO: Check if thread was alreaddy started

        tokio::task::spawn(async move {
            loop {
                let state = state.lock().await;
                if *state == TokioState::Disconnected {
                    break;
                }
                use tokio_modbus::prelude::*;
                use tokio_serial::{Serial, SerialPortSettings};

                let tty_path = "/dev/ttyUSB0";
                let slave = Slave(0x1);

                let mut settings = SerialPortSettings::default();
                settings.baud_rate = 9600;
                let port = Serial::from_path(tty_path, &settings).unwrap();

                let mut ctx = rtu::connect_slave(port, slave).await.unwrap();
                let rsp = ctx.read_input_registers(0x2, 1).await.unwrap();

                ui_event_sender
                    .clone()
                    .send(UiCommand::UpdateSensorValue(rsp[0]))
                    .await
                    .expect("Failed to send Ui command");
                delay_for(Duration::from_millis(100)).await;
            }
        });
        Ok(())
    }
}

// GUI with the GUI Thread
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

        use crate::tokio_thread::*;
        use gio::prelude::*;
        use glib::clone;
        use gtk::prelude::*;

        #[derive(Debug)]
        pub enum UiCommand {
            UpdateSensorType(String),
            UpdateSensorValue(u16),
        }

        // Build the main gtk thread
        fn build_ui(application: &gtk::Application) {
            // Create and start the tokio thread
            // communication erfolgt via the tokio_thread_sender
            let (ui_event_sender, mut ui_event_receiver) = futures::channel::mpsc::channel(0);
            let tokio_thread = TokioThread::new(ui_event_sender);
            let tokio_thread_sender = tokio_thread.tokio_thread_sender;

            // Now build the UI
            let glade_str = include_str!("../src/gui/gtk3/main.ui");
            let builder = gtk::Builder::new_from_string(glade_str);
            let application_window: gtk::ApplicationWindow = build!(builder, "application_window");

            let button_nullpunkt: gtk::Button = build!(builder, "button_nullpunkt");
            let button_messgas: gtk::Button = build!(builder, "button_messgas");
            let label_sensor_type_value: gtk::Label = build!(builder, "label_sensor_type_value");
            let label_sensor_type_value_value: gtk::Label =
                build!(builder, "label_sensor_type_value_value");

            application_window.set_application(Some(application));

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

            application_window.show_all();

            // future on main thread has access to UI
            let future = {
                use futures::stream::StreamExt;

                async move {
                    while let Some(event) = ui_event_receiver.next().await {
                        match event {
                            UiCommand::UpdateSensorType(text) => {
                                label_sensor_type_value.set_text(&text)
                            }
                            UiCommand::UpdateSensorValue(value) => {
                                let value = format!("{}", value);
                                label_sensor_type_value_value.set_text(&value)
                            }
                        }
                    }
                }
            };

            let c = glib::MainContext::default();
            c.spawn_local(future);
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

// Modbus Client functions
pub mod modbus_client {
    use std::{cell::RefCell, future::Future, io::Error, pin::Pin, rc::Rc};
    use tokio_modbus::client::{
        rtu,
        util::{NewContext, SharedContext},
        Context,
    };
    use tokio_serial::{Serial, SerialPortSettings};

    pub struct ModbusClient {
        path: String,
        settings: SerialPortSettings,
    }
    impl ModbusClient {
        pub fn new() -> Self {
            ModbusClient {
                path: "/dev/ttyUSB0".into(),
                settings: SerialPortSettings {
                    baud_rate: 9600,
                    ..Default::default()
                },
            }
        }

        pub fn ctx(self) -> Rc<RefCell<SharedContext>> {
            Rc::new(RefCell::new(SharedContext::new(
                None, // no initial context, i.e. not connected
                Box::new(self),
            )))
        }
    }

    impl NewContext for ModbusClient {
        fn new_context(&self) -> Pin<Box<dyn Future<Output = Result<Context, Error>>>> {
            let serial = Serial::from_path(&self.path, &self.settings);
            Box::pin(async {
                let port = serial?;
                rtu::connect(port).await
            })
        }
    }
}

#[macro_use]
extern crate log;

fn main() {
    pretty_env_logger::init();

    info!("Launch GUI");
    gui::gtk3::launch();
}
