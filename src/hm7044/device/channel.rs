mod control;

use super::driver::Hm7044Driver;
use panduza_platform_core::protocol::AsciiCmdRespProtocol;
use panduza_platform_core::{Class, Error, Instance};
use std::sync::Arc;
use tokio::sync::Mutex;

///
///
///
pub async fn mount<SD: AsciiCmdRespProtocol>(
    mut instance: Instance,
    mut class: Class,
    channel_id: u32,
    driver: Arc<Mutex<Hm7044Driver<SD>>>,
) -> Result<(), Error> {
    //
    //
    let class_channel = class.create_class(format!("{}", channel_id)).finish();

    //
    //
    control::mount(instance.clone(), class.clone(), channel_id, driver.clone()).await?;

    Ok(())
}
