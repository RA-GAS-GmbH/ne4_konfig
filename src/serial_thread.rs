use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};
use tokio_serial::*;

#[derive(Debug)]
pub enum SerialCommand {
    ConnectToPort { name: String, baud: u32 },
    Disconnect,
}

#[derive(Debug)]
pub enum SerialResponse {
    Data(Vec<u8>),
    DisconnectSuccess,
    OpenPortSuccess(String),
    OpenPortError(std::io::Error),
}

#[derive(Debug)]
enum GeneralError {
    Send(SerialCommand),
}

fn list_ports() -> tokio_serial::Result<Vec<String>> {
    match tokio_serial::available_ports() {
        Ok(ports) => Ok(ports.into_iter().map(|x| x.port_name).collect()),
        Err(e) => Err(e),
    }
}

pub struct SerialThread {
    pub from_port_chan_rx: Receiver<SerialResponse>,
    pub to_port_chan_tx: Sender<SerialCommand>,
}

impl SerialThread {
    pub fn new<F: Fn() + Send + 'static>(callback: F) -> Self {
        let (from_port_chan_tx, from_port_chan_rx) = channel();
        let (to_port_chan_tx, to_port_chan_rx) = channel();

        thread::spawn(move || {
            let mut port: Option<Box<dyn tokio_serial::SerialPort>> = None;

            let mut settings: SerialPortSettings = Default::default();

            let loop_time = 10usize; // ms
            let port_scan_time = Duration::from_secs(5);
            let mut last_port_scan_time = Instant::now();

            loop {
                // Check for incoming commands
                match to_port_chan_rx.try_recv() {
                    Ok(SerialCommand::ConnectToPort { name, baud }) => {
                        settings.baud_rate = baud;
                        info!(
                            "Connecting to {} at {} with settings: {:?}",
                            &name, &baud, &settings
                        );
                        match Serial::from_path(&name, &settings) {
                            Ok(p) => {
                                port = Some(Box::new(p));
                                from_port_chan_tx
                                    .send(SerialResponse::OpenPortSuccess(name))
                                    .unwrap();
                            }
                            Err(e) => {
                                from_port_chan_tx
                                    .send(SerialResponse::OpenPortError(e))
                                    .unwrap();
                            }
                        }
                        callback();
                    }
                    Ok(SerialCommand::Disconnect) => {
                        info!("Disconnecting");
                        port = None;
                        from_port_chan_tx
                            .send(SerialResponse::DisconnectSuccess)
                            .unwrap();
                        callback();
                    }
                    Err(TryRecvError::Empty) | Err(TryRecvError::Disconnected) => (),
                }

                // Scan for ports every so often
                if last_port_scan_time.elapsed() > port_scan_time {
                    last_port_scan_time = Instant::now();
                    let mut ports = list_ports().expect("Scanning for ports should never fail");
                    ports.sort();
                    debug!("Found ports: {:?}", &ports);

                    // check if our port was disconnected

                    callback();
                }

                thread::sleep(Duration::from_millis(loop_time as u64));
            }
        });

        SerialThread {
            from_port_chan_rx: from_port_chan_rx,
            to_port_chan_tx: to_port_chan_tx,
        }
    }
}
