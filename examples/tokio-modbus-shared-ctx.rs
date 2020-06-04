/// Tokio Modbus
/// ## Shared Context (NewContext)
/// A SharedContext have to been attachted to the SerialConfig via `impl NewContext for SerialConfig`
use std::{cell::RefCell, future::Future, io::Error, pin::Pin, rc::Rc};
use tokio_modbus::client::{
    rtu,
    util::{reconnect_shared_context, NewContext, SharedContext},
    Context,
};
use tokio_modbus::prelude::*;
use tokio_serial::{Serial, SerialPortSettings};

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let serial_config = SerialConfig {
        path: "/dev/ttyUSB0".into(),
        settings: SerialPortSettings {
            baud_rate: 9600,
            ..Default::default()
        },
    };

    let shared_context = Rc::new(RefCell::new(SharedContext::new(
        None, // no initial context, i.e. not connected
        Box::new(serial_config),
    )));

    println!(
        "Before reconnect_shared_context(): &shared_context.borrow().is_connected(): {}",
        &shared_context.borrow().is_connected()
    );
    reconnect_shared_context(&shared_context).await?;
    println!(
        "After reconnect_shared_context(): &shared_context.borrow().is_connected(): {}",
        &shared_context.borrow().is_connected()
    );

    Ok(())
}
