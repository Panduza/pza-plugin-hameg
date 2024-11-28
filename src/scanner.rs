use panduza_platform_core::drivers::serial::Settings;
use panduza_platform_core::ProductionOrder;
use panduza_platform_core::Scanner;

#[derive(Default)]
pub struct HamegScanner {}

impl HamegScanner {
    ///
    ///
    ///
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

impl Scanner for HamegScanner {
    ///
    ///
    ///
    fn name(&self) -> String {
        "Hameg".to_string()
    }

    ///
    ///
    ///
    fn scan(&self) -> Vec<ProductionOrder> {
        let mut orders = Vec::new();

        if let Ok(ports) = Settings::available_usb_serial_ports() {
            for (port_name, vid, pid, usb_number) in ports {
                orders.push(
                    ProductionOrder::new("hameg.HM7044", "TBD")
                        .add_string_setting("serial_port_name", port_name)
                        .add_u16_setting("usb_vid", vid)
                        .add_u16_setting("usb_pid", pid)
                        .add_string_setting("usb_serial", usb_number),
                );
            }
        }

        orders
    }
}
