use std::thread;
use tokio::runtime::Runtime;
use tokio_modbus::prelude::*;
use tokio_serial::{Serial, SerialPortSettings};


#[tokio::main]
async fn main() {
    let registers = read_all_registers().await;
    println!("{:#?}", registers);
}


async fn read_all_registers() -> Result<Vec<u16>, futures::io::Error> {
    const TTY: &str = "/dev/ttyUSB0";
    const CLIENT_ID: u8 = 247;

    let mut registers = vec![0u16; 49];

    let port = Serial::from_path(
        TTY,
        &SerialPortSettings {
            baud_rate: 9600,
            ..Default::default()
        },
    )
    .unwrap();
    let mut ctx = rtu::connect_slave(port, CLIENT_ID.into()).await?;

    for (i, reg) in registers.iter_mut().enumerate() {
        let value = ctx.read_holding_registers(i as u16, 1).await?;
        *reg = value[0];
    }
    Ok(registers)
}
