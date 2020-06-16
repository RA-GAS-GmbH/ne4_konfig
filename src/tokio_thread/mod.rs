use crate::gui::gtk3::UiCommand;
use futures::channel::mpsc::*;
use futures::prelude::*;
use tokio::time::{timeout, Duration};
use tokio_modbus::prelude::*;
use tokio_serial::*;

#[derive(Debug)]
pub enum TokioCommand {
    Connect,
    Disconnect,
    Messgas(Option<String>, u8),
    NewWorkingMode(Option<String>, u8, u16),
    NewModbusAddress(Option<String>, u8, u8),
    Nullpunkt(Option<String>, u8),
    UpdateSensor(Option<String>, u8),
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
        let (tokio_thread_sender, mut tokio_thread_receiver) = futures::channel::mpsc::channel(0);
        // Clone the ui_event_sender. This is used in a second thread, see below.
        let ui_event_sender2 = ui_event_sender.clone();

        std::thread::spawn(move || {
            // Tokio Thread
            let mut rt = tokio::runtime::Runtime::new().expect("create tokio runtime");
            // Shared State
            let state = std::sync::Arc::new(tokio::sync::Mutex::new(TokioState::Disconnected));

            rt.block_on(async {
                while let Some(event) = tokio_thread_receiver.next().await {
                    debug!("Tokio Thread got event: TokioCommand::{:?}", event);
                    match event {
                        TokioCommand::UpdateSensor(port, modbus_address) => {
                            info!("Execute event TokioCommand::UpdateSensor");
                            read_registers(
                                port,
                                modbus_address,
                                ui_event_sender.clone(),
                                state.clone(),
                            )
                            .await
                            .expect("Could not start read registers loop");
                        }
                        TokioCommand::Connect => {
                            info!("Execute event TokioCommand::Connect");
                            // Adjust shared state
                            let mut state = state.lock().await;
                            *state = TokioState::Connected;

                            ui_event_sender
                                .clone()
                                .send(UiCommand::DisableConnectUiElements)
                                .await
                                .expect("Failed to send Ui command");
                        }
                        TokioCommand::Disconnect => {
                            info!("Execute event TokioCommand::Disconnect");
                            // Adjust shared state
                            let mut state = state.lock().await;
                            *state = TokioState::Disconnected;

                            ui_event_sender
                                .clone()
                                .send(UiCommand::EnableConnectUiElements)
                                .await
                                .expect("Failed to send Ui command");
                        }
                        TokioCommand::Nullpunkt(port, modbus_address) => {
                            info!("Execute event TokioCommand::Nullpunkt");
                            ui_event_sender
                                .clone()
                                .send(UiCommand::Nullpunkt(nullpunkt(port, modbus_address).await))
                                .await
                                .expect("Failed to send Ui command")
                        }
                        TokioCommand::Messgas(port, modbus_address) => {
                            info!("Execute event TokioCommand::Messgas");
                            ui_event_sender
                                .clone()
                                .send(UiCommand::Messgas(messgas(port, modbus_address).await))
                                .await
                                .expect("Failed to send Ui command")
                        }
                        TokioCommand::NewWorkingMode(port, modbus_address, working_mode) => {
                            info!("Execute event TokioCommand::Messgas");
                            ui_event_sender
                                .clone()
                                .send(UiCommand::NewWorkingMode(
                                    new_working_mode(port, modbus_address, working_mode).await,
                                ))
                                .await
                                .expect("Failed to send Ui command")
                        }
                        TokioCommand::NewModbusAddress(port, modbus_address, new_modbus) => {
                            info!("Execute event TokioCommand::Messgas");
                            ui_event_sender
                                .clone()
                                .send(UiCommand::NewModbusAddress(
                                    new_modbus_address(port, modbus_address, new_modbus).await,
                                ))
                                .await
                                .expect("Failed to send Ui command")
                        }
                    }
                }
            })
        });

        // Another Thread to check the serial Intefaces.
        std::thread::spawn(move || {
            // Tokio Thread
            let mut rt = tokio::runtime::Runtime::new().expect("create tokio runtime");

            rt.block_on(async {
                let mut ports: Vec<String> = vec![];
                let mut interval = tokio::time::interval(Duration::from_millis(100));

                // Initial send one update ports for program start
                let available_ports = scan_ports();
                let _ = ui_event_sender2
                    .clone()
                    .send(UiCommand::UpdatePorts(available_ports.clone()))
                    .await;

                loop {
                    let available_ports = scan_ports();
                    if available_ports.len() > ports.len() {
                        let _ = ui_event_sender2
                            .clone()
                            .send(UiCommand::UpdatePorts(available_ports.clone()))
                            .await;
                    } else if available_ports.len() < ports.len() {
                        let _ = ui_event_sender2.clone().send(UiCommand::Disconnect).await;

                        let _ = ui_event_sender2
                            .clone()
                            .send(UiCommand::UpdatePorts(available_ports.clone()))
                            .await;
                    };
                    ports = available_ports;
                    interval.tick().await;
                }
            });
        });

        TokioThread {
            tokio_thread_sender,
        }
    }
}

pub(crate) fn list_ports() -> tokio_serial::Result<Vec<String>> {
    match tokio_serial::available_ports() {
        Ok(ports) => Ok(ports.into_iter().map(|x| x.port_name).collect()),
        Err(e) => Err(e),
    }
}

pub fn scan_ports() -> Vec<String> {
    let mut ports = list_ports().expect("Scanning for ports should never fail");
    ports.sort();
    // Remove unwanted ports
    ports.retain(|p| p != "/dev/ttyS0");

    ports
}

async fn nullpunkt(port: Option<String>, modbus_address: u8) -> tokio::io::Result<()> {
    let tty_path = port.clone().unwrap_or("".into());
    let slave = Slave(modbus_address);
    let mut settings = SerialPortSettings::default();
    settings.baud_rate = 9600;
    let port = Serial::from_path(tty_path, &settings).unwrap();
    let mut ctx = rtu::connect_slave(port, slave).await.unwrap();

    ctx.write_single_register(10, 11111).await
}

async fn new_working_mode(
    port: Option<String>,
    modbus_address: u8,
    working_mode: u16,
) -> tokio::io::Result<()> {
    let tty_path = port.clone().unwrap_or("".into());
    let slave = Slave(modbus_address);
    let mut settings = SerialPortSettings::default();
    settings.baud_rate = 9600;
    let port = Serial::from_path(tty_path, &settings).unwrap();
    let mut ctx = rtu::connect_slave(port, slave).await.unwrap();

    // Entsperren
    let _ = ctx.write_single_register(49, 9876).await;
    // Arbeitsmode umstellen
    ctx.write_single_register(99, working_mode).await
}

async fn new_modbus_address(
    port: Option<String>,
    modbus_address: u8,
    new_modbus_address: u8,
) -> tokio::io::Result<()> {
    let tty_path = port.clone().unwrap_or("".into());
    let slave = Slave(modbus_address);
    let mut settings = SerialPortSettings::default();
    settings.baud_rate = 9600;
    let port = Serial::from_path(tty_path, &settings).unwrap();
    let mut ctx = rtu::connect_slave(port, slave).await.unwrap();

    // Entsperren
    let _ = ctx.write_single_register(49, 9876).await;
    // Arbeitsmode umstellen
    ctx.write_single_register(50, new_modbus_address.into())
        .await
}

async fn messgas(port: Option<String>, modbus_address: u8) -> tokio::io::Result<()> {
    let tty_path = port.clone().unwrap_or("".into());
    let slave = Slave(modbus_address);
    let mut settings = SerialPortSettings::default();
    settings.baud_rate = 9600;
    let port = Serial::from_path(tty_path, &settings).unwrap();
    let mut ctx = rtu::connect_slave(port, slave).await.unwrap();

    ctx.write_single_register(12, 11111).await
}

async fn read_registers(
    port: Option<String>,
    modbus_address: u8,
    ui_event_sender: Sender<UiCommand>,
    state: std::sync::Arc<tokio::sync::Mutex<TokioState>>,
) -> tokio::io::Result<()> {
    // TODO: Check if thread was alreaddy started

    let tty_path = port.clone().unwrap_or("".into());
    let slave = Slave(modbus_address);
    let mut settings = SerialPortSettings::default();
    settings.baud_rate = 9600;
    let port = Serial::from_path(tty_path, &settings).unwrap();
    let mut ctx = rtu::connect_slave(port, slave).await.unwrap();

    tokio::task::spawn(async move {
        'update: loop {
            let state = state.lock().await;
            if *state == TokioState::Disconnected {
                break;
            }

            let mut registers = vec![0u16; 50];

            for (i, reg) in registers.iter_mut().enumerate() {
                match timeout(
                    Duration::from_millis(2000),
                    ctx.read_input_registers(i as u16, 1),
                )
                .await
                {
                    Ok(value) => match value {
                        Ok(value) => *reg = value[0],
                        Err(e) => {
                            ui_event_sender
                                .clone()
                                .send(UiCommand::Disconnect)
                                .await
                                .expect("Failed to send Ui command");

                            ui_event_sender
                                .clone()
                                .send(UiCommand::Error(format!(
                                    "Register {} konnte nicht gelesen werden: {}",
                                    i,
                                    e.to_string()
                                )))
                                .await
                                .expect("Failed to send Ui command");
                            break 'update;
                        }
                    },
                    Err(e) => {
                        ui_event_sender
                            .clone()
                            .send(UiCommand::Disconnect)
                            .await
                            .expect("Failed to send Ui command");

                        ui_event_sender
                            .clone()
                            .send(UiCommand::Error(format!(
                                "Timeout beim lesen aller Register des Sensors: {}",
                                e.to_string()
                            )))
                            .await
                            .expect("Failed to send Ui command");
                        break 'update;
                    }
                };
            }
            ui_event_sender
                .clone()
                .send(UiCommand::UpdateSensorValues(Ok(registers)))
                .await
                .expect("Failed to send Ui command");

            // delay_for(Duration::from_millis(1000)).await;
        }
    });
    Ok(())
}
