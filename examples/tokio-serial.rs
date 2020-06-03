use futures::prelude::*;
use mio_serial::Serial;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let (master, slave) = Serial::pair()?;

    let future = tokio::spawn(async move {
        let mut buffer = [0u8; 1];
        // master.read(&mut buffer).await?;

        // let [command] = buffer;
        // assert_eq!(command, 1);
        //
        // master.write_all(&buffer).await?;
        // master.shutdown().await?;
        //
        std::io::Result::<()>::Ok(())
    });

    // slave.write_all(&[1]).await?;
    //
    // let mut buffer = [0u8; 1];
    // slave.read_exact(&mut buffer).await?;
    //
    // let [response] = buffer;
    // assert_eq!(response, 1);
    //
    // future.await??;

    Ok(())
}
