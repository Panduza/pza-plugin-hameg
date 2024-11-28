use panduza_platform_core::protocol::AsciiCmdRespProtocol;
use panduza_platform_core::{format_driver_error, Error};
use regex::Regex;

///
///
///
pub struct Hm7044Driver<SD> {
    ///
    /// Device Communication Driver
    ///
    device: SD,

    pub channel_number: usize,
    pub voltages: Vec<f32>,
    pub currents: Vec<f32>,
    pub enables: Vec<bool>,
}

impl<SD: AsciiCmdRespProtocol> Hm7044Driver<SD> {
    ///
    ///
    ///
    pub fn new(driver: SD, channel_number: usize) -> Self {
        Self {
            device: driver,
            channel_number: channel_number,
            voltages: vec![0.0; channel_number],
            currents: vec![0.0; channel_number],
            enables: vec![false; channel_number],
        }
    }

    ///
    /// Get identity string
    ///
    pub async fn get_idn(&mut self) -> Result<String, Error> {
        let cmd = "IDN?".to_string(); // Wierd here it is only IDN not *IDN
        let response = self.device.ask(&cmd).await?;
        Ok(response)
    }

    ///
    ///
    ///
    pub async fn refresh_status(&mut self) -> Result<(), Error> {
        //
        //
        let read_result = self.device.ask(&"READ".to_string()).await?;
        let v_a_e: Vec<&str> = read_result.split(';').collect();
        if v_a_e.len() != 3 {
            return Err(format_driver_error!(
                "{:?} Unexpected answer from HM7044 '{:?}'",
                line!(),
                read_result
            ));
        }
        //
        //
        let voltages: Vec<&str> = v_a_e[0][..v_a_e[0].len() - 1].split(' ').collect();
        let currents: Vec<&str> = v_a_e[1][..v_a_e[1].len() - 1].split(' ').collect();
        if voltages.len() != 4 || currents.len() != 4 {
            return Err(format_driver_error!(
                "{:?} Unexpected answer from HM7044 '{:?}'",
                line!(),
                read_result
            ));
        }

        //
        //
        for c in 0..self.channel_number {
            let voltage_str = voltages[c];
            match voltage_str[..voltage_str.len() - 1].parse() {
                Ok(v) => self.voltages[c] = v,
                Err(e) => {
                    return Err(format_driver_error!(
                        "Cannot convert voltage value into f64 from HM7044 '{:?}' ({:?})",
                        voltage_str,
                        e
                    ));
                }
            };
        }

        //
        //
        for c in 0..self.channel_number {
            let current_str = currents[c];
            match current_str[..current_str.len() - 1].parse() {
                Ok(v) => self.currents[c] = v,
                Err(e) => {
                    return Err(format_driver_error!(
                        "Cannot convert current value into f64 from HM7044 '{:?}' ({:?})",
                        current_str,
                        e
                    ));
                }
            };
        }

        let reg = Regex::new(r"(?i:CV|CC|OFF)");
        let extract: Vec<String> = reg
            .unwrap()
            .captures_iter(v_a_e[2])
            .map(|cap| cap[0].to_string())
            .collect();
        for c in 0..self.channel_number {
            let enable = match extract[c].as_str() {
                "CV" | "CC" => true,
                "OFF" => false,
                e => {
                    return Err(format_driver_error!(
                        "Cannot convert enable value into boolean from HM7044 ({:?})",
                        e
                    ));
                }
            };
            self.enables[c] = enable;
        }

        Ok(())
    }

    pub async fn select_channel(&mut self, channel: usize) -> Result<(), Error> {
        let ans = self.device.ask(&format!("SEL {}\r", channel + 1)).await?;

        if ans != format!("channel {} selected", channel + 1) {
            return Err(Error::DriverError(format!(
                "Bad answer {:?} / for channel {:?}",
                ans, channel
            )));
        }

        Ok(())
    }

    ///
    ///
    ///
    pub async fn set_channel_output_enable(
        &mut self,
        channel: usize,
        on: bool,
    ) -> Result<(), Error> {
        //
        //
        self.select_channel(channel).await?;

        //
        //
        let cmd = if on {
            "ON".to_string()
        } else {
            "OFF".to_string()
        };

        let ans = self.device.ask(&cmd).await?;

        Ok(())
    }

    ///
    ///
    ///
    pub async fn set_global_output_enable(&mut self, enable: bool) -> Result<(), Error> {
        //
        //
        let cmd = if enable {
            "EN".to_string()
        } else {
            "DIS".to_string()
        };

        let ans = self.device.ask(&cmd).await?;

        // format!("output {}\r", if enable { "enabled" } else { "disabled" }),

        Ok(())
    }

    pub async fn set_voltage(&mut self, channel: usize, voltage: f32) -> Result<(), Error> {
        //
        //
        self.select_channel(channel).await?;

        //
        //
        let _ans = self.device.ask(&format!("SET {:.2} V\r", voltage)).await?;

        Ok(())
    }

    pub async fn set_current(&mut self, channel: usize, current: f32) -> Result<(), Error> {
        //
        //
        self.select_channel(channel).await?;

        //
        //
        let _ans = self.device.ask(&format!("SET {:.3} A\r", current)).await?;

        Ok(())
    }
}
