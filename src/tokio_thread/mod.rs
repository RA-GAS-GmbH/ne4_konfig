use futures::channel::mpsc::*;
use futures::future::TryFutureExt;
use futures::sink::SinkExt;
use futures::stream::StreamExt;
use std::thread;
use tokio::runtime::Runtime;
use tokio::time::*;
use tokio_serial::*;

#[derive(Debug)]
pub enum TokioCommand {
    Connect,
    ChangePort(String),
    GeneralError,
}

#[derive(Debug)]
pub enum TokioResponse {
    Connect(()),
    /// A sorted list of ports found during a port scan. Guaranteed to contain the currently-active
    /// port if there is one.
    PortsFound(Vec<String>),
    /// A port error has occurred that is likely the result of a serial device disconnected. This
    /// also returns a list of all still-attached serial devices.
    UnexpectedDisconnection(Vec<String>),
}

#[derive(Debug)]
pub enum GeneralError {
    Send(TokioCommand),
}

pub struct TokioThread {
    pub data_event_receiver: Receiver<TokioResponse>,
    pub ui_event_sender: Sender<TokioCommand>,
}

impl TokioThread {
    pub fn new() -> Self {
        let (ui_event_sender, mut ui_event_receiver) = futures::channel::mpsc::channel(0);
        let (mut data_event_sender, data_event_receiver) = futures::channel::mpsc::channel(0);

        thread::spawn(move || {
            let port: Option<Box<dyn tokio_serial::SerialPort>> = None;
            let settings: SerialPortSettings = Default::default();

            let mut rt = Runtime::new().expect("create tokio runtime");
            rt.block_on(async {
                let mut data_event_sender2 = data_event_sender.clone();
                tokio::spawn(async move {
                    let mut port: Option<Box<dyn tokio_serial::SerialPort>> = None;
                    let mut interval = interval(Duration::from_millis(1000));
                    loop {
                        interval.tick().await;

                        let ports = port_scan().await;

                        let message = {
                            if let Some(ref mut p) = port {
                                if let Some(name) = p.name() {
                                    if ports.binary_search(&name).is_err() {
                                        TokioResponse::UnexpectedDisconnection(ports)
                                    } else {
                                        TokioResponse::PortsFound(ports)
                                    }
                                } else {
                                    TokioResponse::PortsFound(ports)
                                }
                            } else {
                                TokioResponse::PortsFound(ports)
                            }
                        };
                        if let TokioResponse::UnexpectedDisconnection(_) = message {
                            port = None;
                        }
                        data_event_sender2
                            .send(message)
                            .await
                            .expect("data_event_sender send message");
                    }
                });

                while let Some(event) = ui_event_receiver.next().await {
                    info!("Got event: {:?}", event);
                    match event {
                        TokioCommand::Connect => data_event_sender
                            .send(TokioResponse::Connect(connect().await))
                            .await
                            .expect("send connect event"),
                        TokioCommand::ChangePort(name) => {
                            if port.is_some() {
                                info!("Change port to '{}' using settings {:?}", &name, &settings);
                            }
                        }
                        TokioCommand::GeneralError => {}
                    }
                }
            })
        });

        TokioThread {
            data_event_receiver,
            ui_event_sender,
        }
    }

    pub fn send_port_change_port_cmd(
        &self,
        port_name: String,
    ) -> std::result::Result<(), GeneralError> {
        let mut tx = self.ui_event_sender.clone();
        tx.send(TokioCommand::ChangePort(port_name))
            .map_err(|e| GeneralError::Send(TokioCommand::GeneralError));
        Ok(())
    }
}

pub(crate) fn list_ports() -> tokio_serial::Result<Vec<String>> {
    match tokio_serial::available_ports() {
        Ok(ports) => Ok(ports.into_iter().map(|x| x.port_name).collect()),
        Err(e) => Err(e),
    }
}

async fn port_scan() -> Vec<String> {
    let mut ports = list_ports().expect("Scanning for ports should never fail");
    ports.sort();
    debug!("Found ports: {:?}", &ports);

    ports
}

async fn connect() -> () {
    println!("Function connect()");

    ()
}
