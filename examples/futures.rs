use futures;
use std::thread;
use tokio::runtime::Runtime;

mod ui {
    pub mod gtk3 {}
}

fn main() {
    // let (ui_event_sender, mut ui_event_receiver) = futures::channel::mpsc::channel(0);
    // let (mut data_event_sender, data_event_receiver) = futures::channel::mpsc::channel(0);

    thread::spawn(move || {
        let mut runtime = Runtime::new().expect("Runtime");
        runtime.block_on(async { loop {} })
    });
}
