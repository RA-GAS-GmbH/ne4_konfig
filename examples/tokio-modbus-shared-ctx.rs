/// 04-06-2020 12:29
// This example shows the Modbus CLient Module
// The Modbus Client holds the TokioModbus Shared Context and acts on the Modbus RTU

pub mod modbus_client {
    use std::{cell::RefCell, future::Future, io::Error, pin::Pin, rc::Rc};
    use tokio_modbus::client::{
        rtu,
        util::{NewContext, SharedContext},
        Context,
    };
    use tokio_serial::{Serial, SerialPortSettings};

    pub struct ModbusClient {
        path: String,
        settings: SerialPortSettings,
    }
    impl ModbusClient {
        pub fn new() -> Self {
            ModbusClient {
                path: "/dev/ttyUSB0".into(),
                settings: SerialPortSettings {
                    baud_rate: 9600,
                    ..Default::default()
                },
            }
        }

        pub fn ctx(self) -> Rc<RefCell<SharedContext>> {
            Rc::new(RefCell::new(SharedContext::new(
                None, // no initial context, i.e. not connected
                Box::new(self),
            )))
        }
    }

    impl NewContext for ModbusClient {
        fn new_context(&self) -> Pin<Box<dyn Future<Output = Result<Context, Error>>>> {
            let serial = Serial::from_path(&self.path, &self.settings);
            Box::pin(async {
                let port = serial?;
                rtu::connect(port).await
            })
        }
    }
}

use crate::modbus_client::*;
use tokio_modbus::client::util::reconnect_shared_context;
use tokio_modbus::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let modbus_client = ModbusClient::new();
    let modbus_ctx = modbus_client.ctx();

    reconnect_shared_context(&modbus_ctx).await?;
    assert!(modbus_ctx.borrow().is_connected());

    let context = modbus_ctx.borrow().share_context().unwrap();
    let mut context = context.borrow_mut();
    context.set_slave(1.into());

    let response = context.read_input_registers(0, 5).await?;
    println!("response: {:?}", &response);

    Ok(())
}
