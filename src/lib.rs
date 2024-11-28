use panduza_platform_core::{Producer, Scanner};

#[cfg(feature = "plugin")]
panduza_platform_core::plugin_interface!("hameg");

mod hm7044;
mod scanner;

// Export the producers of the plugin
//
pub fn plugin_producers() -> Vec<Box<dyn Producer>> {
    let mut producers: Vec<Box<dyn Producer>> = vec![];
    producers.push(hm7044::producer::Hm7044::new());
    return producers;
}

//
//
pub fn plugin_scanners() -> Vec<Box<dyn Scanner>> {
    let mut scanners: Vec<Box<dyn Scanner>> = vec![];
    scanners.push(scanner::HamegScanner::default().boxed());
    return scanners;
}
