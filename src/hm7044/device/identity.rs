use super::driver::Hm7044Driver;
use panduza_platform_core::protocol::AsciiCmdRespProtocol;
use panduza_platform_core::{Error, Instance};
use std::sync::Arc;
use tokio::sync::Mutex;

///
///
///
pub async fn mount<SD: AsciiCmdRespProtocol>(
    mut instance: Instance,
    driver: Arc<Mutex<Hm7044Driver<SD>>>,
) -> Result<(), Error> {
    //
    // Create attribute
    let att_identity = instance
        .create_attribute("identity")
        .with_ro()
        .with_info("Identity string of the power supply")
        .finish_as_string()
        .await?;

    //
    // Just init
    let idn = driver.lock().await.get_idn().await?;
    att_identity.set(idn).await?;

    Ok(())
}
