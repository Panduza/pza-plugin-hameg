mod control;

use super::driver::Hm7044Driver;
use panduza_platform_core::protocol::AsciiCmdRespProtocol;
use panduza_platform_core::{Class, Error, Instance};
use std::sync::Arc;
use tokio::sync::Mutex;

///
///
///
pub async fn mount<SD: AsciiCmdRespProtocol + 'static>(
    instance: Instance,
    mut parent_class: Class,
    channel_id: usize,
    driver: Arc<Mutex<Hm7044Driver<SD>>>,
) -> Result<(), Error> {
    //
    //
    let class_channel = parent_class
        .create_class(format!("{}", channel_id))
        .finish();

    //
    //
    control::mount(
        instance.clone(),
        class_channel.clone(),
        channel_id,
        driver.clone(),
    )
    .await?;

    Ok(())
}
