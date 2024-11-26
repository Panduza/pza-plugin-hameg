mod driver;
mod identity;

use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;

use driver::Hm7044Driver;
// use crate::common::driver::KoradDriver;
use panduza_platform_core::drivers::serial::eol::Driver as SerialEolDriver;
use panduza_platform_core::drivers::serial::Settings as SerialSettings;
use panduza_platform_core::drivers::usb::Settings as UsbSettings;
use panduza_platform_core::{DeviceLogger, Instance};
use panduza_platform_core::{DriverOperations, Error};
use serde_json::json;
use tokio::sync::Mutex;
use tokio::time::sleep;

static DEVICE_VENDOR_ID: u16 = 0x0416;
static DEVICE_PRODUCT_ID: u16 = 0x5011;
static DEVICE_SERIAL_BAUDRATE: u32 = 9600; // We do not care... it is USB serial

///
/// Device to control PicoHA Dio Board
///
pub struct Hm7044Device {
    ///
    /// Device logger
    logger: Option<DeviceLogger>,
    ///
    /// Serial settings to connect to the pico
    serial_settings: Option<SerialSettings>,
    // ///
    // /// Connector to communicate with the pico
    // connector: Option<Connector>,

    // ///
    // ///
    // pico_connector: Option<PicoHaDioConnector>,
}

impl Hm7044Device {
    ///
    /// Constructor
    ///
    pub fn new() -> Self {
        Hm7044Device {
            logger: None,
            serial_settings: None,
            // connector: None,
            // pico_connector: None,
        }
    }

    ///
    /// Prepare settings of the device
    ///
    pub async fn prepare_settings(&mut self, instance: Instance) -> Result<(), Error> {
        // Get the device logger
        let logger = instance.logger.clone();

        // Get the device settings
        let json_settings = instance.settings().await.or(Some(json!({}))).unwrap();

        // Log debug info
        logger.info(format!("JSON settings: {:?}", json_settings));

        // Usb settings
        let usb_settings = UsbSettings::new()
            .set_vendor(DEVICE_VENDOR_ID)
            .set_model(DEVICE_PRODUCT_ID)
            .optional_set_serial_from_json_settings(&json_settings);
        logger.info(format!("USB settings: {}", usb_settings));

        // Serial settings
        self.serial_settings = Some(
            SerialSettings::new()
                .set_port_name_from_json_or_usb_settings(&json_settings, &usb_settings)
                .map_err(|e| Error::Generic(e.to_string()))?
                .set_baudrate(DEVICE_SERIAL_BAUDRATE), // .set_time_lock_duration(Duration::from_millis(5000)), // require delay between 2 commands
        );

        Ok(())
    }

    ///
    ///
    ///
    pub fn mount_driver(&mut self) -> Result<Arc<Mutex<Hm7044Driver<SerialEolDriver>>>, Error> {
        //
        // Recover settings
        let settings = self.serial_settings.as_ref().ok_or(Error::BadSettings(
            "Serial Settings not provided".to_string(),
        ))?;

        let driver = SerialEolDriver::open(settings, vec![b'\r'])?;

        let kdriver = Hm7044Driver::new(driver);

        Ok(Arc::new(Mutex::new(kdriver)))
    }
}

#[async_trait]
impl DriverOperations for Hm7044Device {
    ///
    ///
    ///
    async fn mount(&mut self, mut instance: Instance) -> Result<(), Error> {
        //
        // Init logger
        self.logger = Some(instance.logger.clone());

        //
        //
        let logger = instance.logger.clone();

        //
        //
        self.prepare_settings(instance.clone()).await?;

        let driver = self.mount_driver()?;

        identity::mount(instance.clone(), driver.clone()).await?;

        //
        //
        let class_channel = instance.create_class("channel").finish();

        //
        //
        for channel_num in 0..4 {}
        // channel
        //    0
        //    1
        //    2
        //    3

        // crate::common::real::identity::mount(instance.clone(), driver.clone()).await?;
        // crate::common::real::control::mount(instance.clone(), driver.clone()).await?;
        // crate::common::real::measure::mount(instance.clone(), driver.clone()).await?;

        //
        //
        logger.info("Mount Successful !!!");

        Ok(())
    }
    ///
    /// Easiest way to implement the reboot event
    ///
    async fn wait_reboot_event(&mut self, _instance: Instance) {
        sleep(Duration::from_secs(5)).await;
    }
}
