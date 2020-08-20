mod tokio_runtime {
    use futures::channel::mpsc::*;
    use futures::prelude::*;
    use tokio::time::{timeout, Duration};
    use tokio_modbus::client::{
        util::{reconnect_shared_context, NewContext, SharedContext},
        Context,
    };
    use tokio_modbus::prelude::*;
    // use tokio_serial::{Serial, SerialPortSettings};
    use std::{
        cell::RefCell,
        future::Future,
        io::Error,
        pin::Pin,
        rc::Rc,
        sync::{Arc, Mutex},
    };
    use tokio_serial::*;

    // get rid of `#[tokio::main]`, implement TokioRuntime
    //

    pub enum TokioRuntimeCommand {
        ReadRegisters,
        ReadRegisters1,
    }

    pub struct TokioRuntime {
        pub tx: Sender<TokioRuntimeCommand>,
        pub available_ports: Vec<String>,
    }

    impl TokioRuntime {
        pub fn new() -> Self {
            let (tx, mut rx) = futures::channel::mpsc::channel(0);
            // second tx for 2nd serial_interface-check-thread
            let tx2 = tx.clone();

            let available_ports = TokioRuntime::get_ports();

            std::thread::spawn(move || {
                // Tokio Thread
                let mut rt = tokio::runtime::Runtime::new().expect("create tokio runtime");

                rt.block_on(async {
                    while let Some(event) = rx.next().await {
                        match event {
                            TokioRuntimeCommand::ReadRegisters => {
                                println!("Read registers");
                            }
                            _ => unimplemented!(),
                        }
                    }
                });
            });

            // Another Thread to periodical check the serial Intefaces.
            std::thread::spawn(move || {
                // Tokio Thread
                let mut rt = tokio::runtime::Runtime::new().expect("create tokio runtime");

                rt.block_on(async {
                    let mut ports: Vec<String> = vec![];
                    let mut interval = tokio::time::interval(Duration::from_millis(100));

                    loop {
                        let available_ports = TokioRuntime::get_ports();
                        if available_ports.len() != ports.len() {
                            println!("available serialports have changed: {:?}", available_ports);
                        };
                        ports = available_ports;
                        interval.tick().await;
                    }
                });
            });

            Self {
                tx,
                available_ports,
            }
        }

        pub fn run(&mut self) {
            loop {
                self.tx.send(TokioRuntimeCommand::ReadRegisters1);
            }
        }

        /// List available serial ports
        fn list_ports() -> tokio_serial::Result<Vec<String>> {
            match tokio_serial::available_ports() {
                Ok(ports) => Ok(ports.into_iter().map(|x| x.port_name).collect()),
                Err(e) => Err(e),
            }
        }

        /// Get and filter available serial ports
        ///
        /// This function is called from the gui thread.
        fn get_ports() -> Vec<String> {
            let mut ports =
                TokioRuntime::list_ports().expect("Scanning for ports should never fail");
            ports.sort();
            // Remove unwanted ports under linux
            ports.retain(|p| p != "/dev/ttyS0");

            ports
        }
    }
}

use tokio_runtime::{TokioRuntime, TokioRuntimeCommand};

fn main() {
    let mut rt = TokioRuntime::new();
}

pub async fn oldmain() -> std::result::Result<(), Box<dyn std::error::Error>> {
    use std::{cell::RefCell, future::Future, io::Error, pin::Pin, rc::Rc};

    use tokio_modbus::client::{
        rtu,
        util::{reconnect_shared_context, NewContext, SharedContext},
        Context,
    };
    use tokio_modbus::prelude::*;
    use tokio_serial::{Serial, SerialPortSettings};

    const SLAVE_1: Slave = Slave(247);
    const SLAVE_2: Slave = Slave(247);

    #[derive(Debug)]
    struct SerialConfig {
        path: String,
        settings: SerialPortSettings,
    }

    impl NewContext for SerialConfig {
        fn new_context(
            &self,
        ) -> Pin<Box<dyn Future<Output = std::result::Result<Context, Error>>>> {
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

    let context = shared_context.borrow().share_context().unwrap();
    let mut context = context.borrow_mut();

    println!("Reading a sensor value from {:?}", SLAVE_1);
    context.set_slave(SLAVE_1);
    let response = context.read_input_registers(0u16, 10u16).await?;
    println!("Sensor value for device {:?} is: {:?}", SLAVE_1, response);

    println!("Reading a sensor value from {:?}", SLAVE_2);
    context.set_slave(SLAVE_2);
    let response = context.read_holding_registers(0u16, 10u16).await?;
    println!("Sensor value for device {:?} is: {:?}", SLAVE_2, response);

    Ok(())
}
