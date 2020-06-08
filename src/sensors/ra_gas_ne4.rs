use std::future::*;
use std::{
    cell::RefCell,
    io::{Error, ErrorKind, Result},
    rc::Rc,
    time::Duration,
};
use tokio::prelude::*;
use tokio::time::timeout;
use tokio_modbus::{
    client::util::{reconnect_shared_context, SharedContext},
    prelude::*,
};

pub const VALUE_RREG_START: u16 = 0x0000;
pub const VALUE_RREG_COUNT: u16 = 0x0001;

pub async fn read_value(context: &mut client::Context) -> Result<Vec<u16>> {
    context
        .read_input_registers(VALUE_RREG_START, VALUE_RREG_COUNT)
        .await
}

pub async fn read_value_with_timeout(context: &mut client::Context, timeout: Duration) -> u16 {
    // timeout(timeout, read_value(context))
    //     .await
    0u16
}
