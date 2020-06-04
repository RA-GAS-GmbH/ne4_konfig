use crate::gui::gtk3::UiCommand;
use futures::channel::mpsc::*;
use futures::prelude::*;
use tokio::time::*;
use tokio_serial::*;

#[derive(Debug)]
pub enum TokioCommand {
    Connect,
    Disconnect,
    Messgas,
    Nullpunkt,
    ReadRegistersLoop,
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
                            // match modbus_address {}
                            match timeout(
                                Duration::from_millis(4000),
                                update_sensor(port.unwrap(), modbus_address),
                            )
                            .await
                            {
                                Ok(values) => ui_event_sender
                                    .clone()
                                    .send(UiCommand::UpdateSensorValues(values))
                                    .await
                                    .expect("update sensor"),
                                Err(_) => ui_event_sender
                                    .clone()
                                    .send(UiCommand::Timeout)
                                    .await
                                    .expect("update sensor"),
                            }
                        }
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

pub(crate) fn list_ports() -> tokio_serial::Result<Vec<String>> {
    match tokio_serial::available_ports() {
        Ok(ports) => Ok(ports.into_iter().map(|x| x.port_name).collect()),
        Err(e) => Err(e),
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

fn _port_scan() -> Vec<String> {
    let mut ports = list_ports().expect("Scanning for ports should never fail");
    ports.sort();
    ports.reverse();
    debug!("Found ports: {:?}", &ports);

    ports
}

async fn update_sensor(_port: String, _modbus_address: u8) -> Result<Vec<u16>> {
    let registers = vec![0u16; 49];
    //
    // let port = Serial::from_path(
    //     port,
    //     &SerialPortSettings {
    //         baud_rate: 9600,
    //         ..Default::default()
    //     },
    // )
    // .unwrap();
    // let mut ctx = rtu::connect_slave(port, modbus_address.into()).await?;
    //
    // for (i, reg) in registers.iter_mut().enumerate() {
    //     let value = ctx.read_input_registers(i as u16, 1).await?;
    //     *reg = value[0];
    // }
    Ok(registers)
}
