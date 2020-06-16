use tokio::io::Result;
use tokio::time::Duration;
use tokio_modbus::prelude::*;

pub const VALUE_RREG_START: u16 = 0x0000;
pub const VALUE_RREG_COUNT: u16 = 0x0001;

pub async fn read_value(context: &mut client::Context) -> Result<Vec<u16>> {
    context
        .read_input_registers(VALUE_RREG_START, VALUE_RREG_COUNT)
        .await
}

pub async fn read_value_with_timeout(_context: &mut client::Context, _timeout: Duration) -> u16 {
    // timeout(timeout, read_value(context))
    //     .await
    0u16
}
