// Tokio Thread
mod tokio_thread {
    use crate::gui::gtk3::UiCommand;
    use futures::channel::mpsc::*;
    use futures::prelude::*;
    use tokio::prelude::*;

    #[derive(Debug)]
    pub enum TokioCommand {
        Nullpunkt,
        Messgas,
    }
    #[derive(Debug)]
    pub enum TokioResponse {}

    pub struct TokioThread {
        pub tokio_thread_sender: Sender<TokioCommand>,
    }

    impl TokioThread {
        pub fn new(ui_event_sender: Sender<UiCommand>) -> Self {
            let (tokio_thread_sender, mut tokio_thread_receiver) =
                futures::channel::mpsc::channel(0);

            std::thread::spawn(move || {
                let mut rt = tokio::runtime::Runtime::new().expect("create tokio runtime");
                rt.block_on(async {
                    while let Some(event) = tokio_thread_receiver.next().await {
                        debug!("Tokio Thread got event: TokioCommand::{:?}", event);
                        match event {
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
        }
        #[derive(Debug)]
        pub enum UiResponse {}

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

            application_window.set_application(Some(application));

            button_nullpunkt.connect_clicked(clone!(
                @strong tokio_thread_sender => move |_| {
                    tokio_thread_sender
                        .clone()
                        .try_send(TokioCommand::Nullpunkt)
                        .expect("Faild to send tokio command");
            }));

            button_messgas.connect_clicked(clone!(
                @strong tokio_thread_sender => move |_| {
                    tokio_thread_sender
                        .clone()
                        .try_send(TokioCommand::Messgas)
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
#[macro_use]
extern crate log;

fn main() {
    env_logger::init();

    info!("Launch GUI");
    gui::gtk3::launch();
}
