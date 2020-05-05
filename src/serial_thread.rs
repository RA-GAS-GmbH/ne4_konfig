use core::num;
use std::sync::mpsc::{channel, Receiver, Sender, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};
use tokio_serial::*;

#[derive(Debug)]
pub enum SerialCommand {
    ChangePort(String),
    ConnectToPort { name: String, baud: u32 },
    Disconnect,
}

#[derive(Debug)]
pub enum SerialResponse {
    Data(Vec<u8>),
    DisconnectSuccess,
    OpenPortSuccess(String),
    OpenPortError(std::io::Error),
    /// A port error has occurred that is likely the result of a serial device disconnected. This
    /// also returns a list of all still-attached serial devices.
    UnexpectedDisconnection(Vec<String>),
    /// A sorted list of ports found during a port scan. Guaranteed to contain the currently-active
    /// port if there is one.
    PortsFound(Vec<String>),
}

#[derive(Debug)]
pub enum GeneralError {
    Parse(num::ParseIntError),
    Send(SerialCommand),
}

pub(crate) fn list_ports() -> Result<Vec<String>> {
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
                // First check if we have any incoming commands
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
                    Ok(SerialCommand::ChangePort(name)) => {
                        if port.is_some() {
                            info!("Change port to '{}' using settings {:?}", &name, &settings);
                        }
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

                // If a port is active, handle reading and writing to it
                // TODO: implement port handling

                // Scan for ports every so often
                if last_port_scan_time.elapsed() > port_scan_time {
                    last_port_scan_time = Instant::now();
                    let mut ports = list_ports().expect("Scanning for ports should never fail");
                    ports.sort();
                    debug!("Found ports: {:?}", &ports);

                    // check if our port was disconnected
                    let message = {
                        if let Some(ref mut p) = port {
                            if let Some(name) = p.name() {
                                if ports.binary_search(&name).is_err() {
                                    SerialResponse::UnexpectedDisconnection(ports)
                                } else {
                                    SerialResponse::PortsFound(ports)
                                }
                            } else {
                                SerialResponse::PortsFound(ports)
                            }
                        } else {
                            SerialResponse::PortsFound(ports)
                        }
                    };
                    if let SerialResponse::UnexpectedDisconnection(_) = message {
                        port = None;
                    }
                    from_port_chan_tx
                        .send(message)
                        .expect("Sending port_scan message failed");
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

    pub fn send_port_change_port_cmd(
        &self,
        port_name: String,
    ) -> std::result::Result<(), GeneralError> {
        let tx = &self.to_port_chan_tx;
        tx.send(SerialCommand::ChangePort(port_name))
            .map_err(|e| GeneralError::Send(e.0))?;
        Ok(())
    }
}
