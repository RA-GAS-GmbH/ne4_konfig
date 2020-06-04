use crate::gui::gtk3::UiCommand;
use futures::channel::mpsc::*;
use futures::prelude::*;
use tokio::time::*;
use tokio_modbus::prelude::*;
use tokio_serial::*;

#[derive(Debug)]
pub enum TokioCommand {
    Connect,
    Disconnect,
    Messgas,
    Nullpunkt,
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
                        TokioCommand::Nullpunkt => {
                            info!("Execute event TokioCommand::Nullpunkt");
                            ui_event_sender
                                .clone()
                                .send(UiCommand::UpdateSensorType("Nullpunkt".into()))
                                .await
                                .expect("Failed to send Ui command")
                        }
                        TokioCommand::Messgas => {
                            info!("Execute event TokioCommand::Messgas");
                            ui_event_sender
                                .clone()
                                .send(UiCommand::UpdateSensorType("Messgas".into()))
                                .await
                                .expect("Failed to send Ui command")
                        }
                    }
                }
            })
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

async fn read_registers(
    port: Option<String>,
    modbus_address: u8,
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

            let tty_path = port.clone().unwrap_or("".into());
            let slave = Slave(modbus_address);
            let mut settings = SerialPortSettings::default();
            settings.baud_rate = 9600;
            let port = Serial::from_path(tty_path, &settings).unwrap();
            let mut registers = vec![0u16; 49];

            let mut ctx = rtu::connect_slave(port, slave).await.unwrap();
            for (i, reg) in registers.iter_mut().enumerate() {
                match tokio::time::timeout(
                    Duration::from_millis(500),
                    ctx.read_input_registers(i as u16, 1),
                )
                .await
                {
                    Ok(value) => match value {
                        Ok(value) => *reg = value[0],
                        Err(e) => {
                            ui_event_sender
                                .clone()
                                .send(UiCommand::Error(format!(
                                    "Error while read_register {}: {}",
                                    i,
                                    e.to_string()
                                )))
                                .await
                                .expect("Failed to send Ui command");
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
                                "Error while read_input_registers: {}",
                                e.to_string()
                            )))
                            .await
                            .expect("Failed to send Ui command");
                        break;
                    }
                }
            }

            ui_event_sender
                .clone()
                .send(UiCommand::UpdateSensorValues(Ok(registers)))
                .await
                .expect("Failed to send Ui command");

            delay_for(Duration::from_millis(100)).await;
        }
    });
    Ok(())
}

fn _port_scan() -> Vec<String> {
    let mut ports = list_ports().expect("Scanning for ports should never fail");
    ports.sort();
    ports.reverse();
    debug!("Found ports: {:?}", &ports);

    ports
}
