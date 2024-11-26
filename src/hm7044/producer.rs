use super::device::Hm7044Device;
use panduza_platform_core::{DriverOperations, Producer};

pub struct Hm7044 {}

impl Hm7044 {
    pub fn new() -> Box<Hm7044> {
        Box::new(Hm7044 {})
    }
}

impl Producer for Hm7044 {
    fn manufacturer(&self) -> String {
        "hameg".to_string()
    }

    fn model(&self) -> String {
        "HM7044".to_string()
    }

    fn description(&self) -> String {
        "Driver for HM7044 Power Supply".to_string()
    }

    fn props(&self) -> panduza_platform_core::Props {
        panduza_platform_core::Props::default()
    }

    fn produce(&self) -> Result<Box<dyn DriverOperations>, panduza_platform_core::Error> {
        return Ok(Box::new(Hm7044Device::new()));
    }
}
