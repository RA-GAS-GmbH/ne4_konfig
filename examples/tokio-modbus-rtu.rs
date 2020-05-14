use futures::channel::mpsc::{Receiver, Sender};
use gio::prelude::*;
use gtk::{ApplicationWindow, Builder, Button, Label};
use gtk::prelude::*;
use serde::Deserialize;
use std::cell::RefCell;
use std::env::args;
use std::rc::Rc;
use tokio;

#[macro_use]
extern crate ne4_konfig;

#[derive(Debug)]
enum UiEvent {
    NullpunktClicked,
}

#[derive(Debug)]
enum DataEvent {
    Nullpunkt(()),
}

// FIXME: rename to Ui
#[derive(Clone)]
struct UiElements {
    info_label: Label,
}


fn main() {
    use std::thread;

    // FIXME: rename ui_event_sender receiver to tx rx
    // FIXME: rename data_event_sender receiver to tx rx
    // thread-to-thread communication
    let (ui_event_sender, mut ui_event_receiver) = futures::channel::mpsc::channel(0);
    let (mut data_event_sender, data_event_receiver) = futures::channel::mpsc::channel(0);


    // spawn data/ network thread
    thread::spawn(move || {
        use tokio::runtime::Runtime;

        let mut rt = Runtime::new().expect("couln't create tokio runtime");
        rt.block_on(async {
            use futures::sink::SinkExt;
            use futures::stream::StreamExt;

            while let Some(event) = ui_event_receiver.next().await {
                println!("got event: {:?}", event);
                match event {
                    UiEvent::NullpunktClicked => data_event_sender
                        .send(DataEvent::Nullpunkt(nullpunkt_abgleich().await))
                        .await
                        .expect("couln't dio nullpunkt abgleich"),
                }
            }
        })
    });

    // main thread is ui thread
    let application = gtk::Application::new(
        Some("com.gaswarnanlagen.ne4-mod-bus.ne4_konfig"),
        Default::default(),
    )
    .expect("Couldn't create Gtk Application");

    // this type is a hack to make the closure for `connect_activate` `Clone`
    // https://git.sr.ht/~azdle/nex-trip-gtk/tree/c8f04fcd933d9ddeeec47176e59db8e48efd8abd/src/main.rs#L135
    let data_event_receiver = Rc::new(RefCell::new(Some(data_event_receiver)));
    application.connect_activate(move |app| {
        build_ui(app, ui_event_sender.clone(), data_event_receiver.clone());
    });

    application.run(&[]);
}

// FIXME: rename application to app
fn build_ui(
    application: &gtk::Application,
    ui_event_sender: Sender<UiEvent>,
    data_event_receiver: Rc<RefCell<Option<Receiver<DataEvent>>>>,
) {
    let glade_str = include_str!("../src/gui/gtk3/main.ui");
    let builder = gtk::Builder::new_from_string(glade_str);

    let application_window: gtk::ApplicationWindow = build!(builder, "application_window");
    application_window.set_application(Some(application));

    let button_nullpunkt: gtk::Button = build!(builder, "button_nullpunkt");

    {
        button_nullpunkt.connect_clicked(move |_| {
            ui_event_sender
                .clone()
                .try_send(UiEvent::NullpunktClicked)
                .expect("couln't send NullpunktClicked");
        });
    }

    application_window.show_all();

    // future on main thread that has access to UI
    let future = {
        let mut data_event_receiver = data_event_receiver
            .replace(None)
            .take()
            .expect("Couldn't replace data_receiver");
        async move {
            use futures::stream::StreamExt;

            while let Some(event) = data_event_receiver.next().await {
                println!("data_event: {:?}", event);
                // match event {
                //
                // }
            }
        }
    };

    let c = glib::MainContext::default();
    c.spawn_local(future);

    // c.pop_thread_default();
}

async fn nullpunkt_abgleich() -> () {
    foo().await;
    ()
}

async fn foo() -> Result<(), Box<dyn std::error::Error>> {
    use std::{cell::RefCell, future::Future, io::Error, pin::Pin, rc::Rc};

    use tokio_modbus::client::{
        rtu,
        util::{reconnect_shared_context, NewContext, SharedContext},
        Context,
    };
    use tokio_modbus::prelude::*;
    use tokio_serial::{Serial, SerialPortSettings};

    const SLAVE_1: Slave = Slave(247);

    #[derive(Debug)]
    struct SerialConfig {
        path: String,
        settings: SerialPortSettings,
    }

    impl NewContext for SerialConfig {
        fn new_context(&self) -> Pin<Box<dyn Future<Output = Result<Context, Error>>>> {
            let serial = Serial::from_path(&self.path, &self.settings);
            Box::pin(async {
                let port = serial?;
                rtu::connect(port).await
            })
        }
    }

    let serial_config = SerialConfig {
        path: "/dev/ttyUSB0".into(),
        settings: SerialPortSettings {
            baud_rate: 9600,
            ..Default::default()
        },
    };
    println!("Configuration: {:?}", serial_config);

    // A shared, reconnectable context is not actually needed in this
    // simple example. Nevertheless we use it here to demonstrate how
    // it works.
    let shared_context = Rc::new(RefCell::new(SharedContext::new(
        None, // no initial context, i.e. not connected
        Box::new(serial_config),
    )));

    // Reconnect for connecting an initial context
    reconnect_shared_context(&shared_context).await?;

    assert!(shared_context.borrow().is_connected());
    println!("Connected");

    println!("Reading a sensor value from {:?}", SLAVE_1);
    let context = shared_context.borrow().share_context().unwrap();
    let mut context = context.borrow_mut();
    context.set_slave(SLAVE_1);
    let response = context.read_holding_registers(0, 5).await?;
    println!("Sensor value for device {:?} is: {:?}", SLAVE_1, response);

    Ok(())
}
