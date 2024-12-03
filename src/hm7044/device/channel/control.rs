mod current;
mod voltage;

use crate::hm7044::device::driver::Hm7044Driver;
use panduza_platform_core::protocol::AsciiCmdRespProtocol;
use panduza_platform_core::{
    spawn_on_command, BooleanAttServer, Class, Error, Instance, InstanceLogger,
};
use std::sync::Arc;
use tokio::sync::Mutex;

///
///
///
pub async fn mount<SD: AsciiCmdRespProtocol + 'static>(
    mut instance: Instance,
    mut parent_class: Class,
    channel_id: usize,
    driver: Arc<Mutex<Hm7044Driver<SD>>>,
) -> Result<(), Error> {
    //
    // Start logging
    let logger = instance.logger.clone();
    logger.info("Mounting 'control' class...");

    //
    // Create attribute
    let mut itf_control = parent_class.create_class("control").finish();

    current::mount(
        instance.clone(),
        itf_control.clone(),
        channel_id,
        driver.clone(),
    )
    .await?;
    voltage::mount(
        instance.clone(),
        itf_control.clone(),
        channel_id,
        driver.clone(),
    )
    .await?;
    // options::mount(instance.clone(), itf_control.clone(), driver.clone()).await?;

    //
    //
    let att_oe = itf_control
        .create_attribute("output_enable")
        .with_rw()
        .finish_as_boolean()
        .await?;

    let v: bool = driver.lock().await.enables[channel_id];
    att_oe.set(v).await.unwrap();

    //
    // Execute action on each command received
    let logger_2 = instance.logger.clone();
    let att_oe_2 = att_oe.clone();
    spawn_on_command!(
        instance,
        att_oe_2,
        on_command(
            logger_2.clone(),
            att_oe_2.clone(),
            channel_id,
            driver.clone()
        )
    );

    //
    // End of mount
    logger.info("Mounting 'control' class -> OK");
    Ok(())
}

///
///
///
async fn on_command<SD: AsciiCmdRespProtocol + 'static>(
    logger: InstanceLogger,
    mut att_oe: BooleanAttServer,
    channel_id: usize,
    driver: Arc<Mutex<Hm7044Driver<SD>>>,
) -> Result<(), Error> {
    while let Some(command) = att_oe.pop_cmd().await {
        //
        // Log
        logger.debug(format!("command received '{:?}'", command));

        driver
            .lock()
            .await
            .set_channel_output_enable(channel_id, command)
            .await?;

        driver.lock().await.refresh_status().await?;
        att_oe.set(driver.lock().await.enables[channel_id]).await?;
    }
    Ok(())
}
