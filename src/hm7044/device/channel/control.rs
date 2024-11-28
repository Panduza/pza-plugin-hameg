use crate::hm7044::device::driver::Hm7044Driver;
use panduza_platform_core::protocol::AsciiCmdRespProtocol;
use panduza_platform_core::{spawn_on_command, Class, Error, Instance};
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
    // Start logging
    let logger = instance.logger.clone();
    logger.info("Mounting 'control' class...");

    //
    // Create attribute
    let mut itf_control = instance.create_class("control").finish();

    // current::mount(instance.clone(), itf_control.clone(), driver.clone()).await?;
    // voltage::mount(instance.clone(), itf_control.clone(), driver.clone()).await?;
    // options::mount(instance.clone(), itf_control.clone(), driver.clone()).await?;

    //
    //
    let att_oe = itf_control
        .create_attribute("output_enable")
        .with_rw()
        .finish_as_boolean()
        .await?;

    // let v = driver.lock().await.get_out().await?;
    // att_oe.set(v).await.unwrap();

    //
    // Execute action on each command received
    // let logger_2 = instance.logger.clone();
    // let att_oe_2 = att_oe.clone();
    // spawn_on_command!(
    //     instance,
    //     att_oe_2,
    //     on_command(logger_2.clone(), att_oe_2.clone(), driver.clone())
    // );

    //
    // End of mount
    logger.info("Mounting 'control' class -> OK");
    Ok(())
}
