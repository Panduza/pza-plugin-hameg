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
    pub voltages: Vec<f64>,
    pub currents: Vec<f64>,
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

        // let current_str = currents[self.channel as usize];
        // let current = match current_str[..current_str.len() - 1].parse() {
        //     Ok(v) => v,
        //     Err(_e) => return platform_error_result!("Unexpected answer from HM7044."),
        // };

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

    async fn select_channel(&mut self, channel: u32) -> Result<(), Error> {
        let ans = self.device.ask(&format!("SEL {}\r", channel + 1)).await?;

        if ans != format!("channel {} selected\r", channel + 1) {
            return Err(Error::DriverError(format!("Bad answer {:?}", ans)));
        }

        Ok(())
    }

    // ///
    // /// Control current getter
    // ///
    // pub async fn get_iset(&mut self) -> Result<f32, Error> {
    //     let cmd = "ISET1?".to_string();
    //     let response = self.driver.ask(&cmd).await?;

    //     let value = response
    //         .parse::<f32>()
    //         .map_err(|e| Error::Generic(format!("{:?}", e)))?;

    //     Ok(value)
    // }

    // ///
    // /// Control current getter
    // ///
    // pub async fn set_iset(&mut self, value: f32) -> Result<(), Error> {
    //     let cmd = format!("ISET1:{:.3}", value);
    //     self.driver.send(&cmd).await
    // }

    // ///
    // /// Control current getter
    // ///
    // pub async fn get_vset(&mut self) -> Result<f32, Error> {
    //     let cmd = "VSET1?".to_string();
    //     let response = self.driver.ask(&cmd).await?;

    //     let value = response
    //         .parse::<f32>()
    //         .map_err(|e| Error::Generic(format!("{:?}", e)))?;

    //     Ok(value)
    // }

    // ///
    // /// Control current getter
    // ///
    // pub async fn set_vset(&mut self, value: f32) -> Result<(), Error> {
    //     let cmd = format!("VSET1:{:.2}", value);
    //     self.driver.send(&cmd).await
    // }

    // ///
    // ///
    // ///
    // pub async fn get_iout(&mut self) -> Result<f32, Error> {
    //     let cmd = "IOUT1?".to_string();
    //     let response = self.driver.ask(&cmd).await?;
    //     let value = response
    //         .parse::<f32>()
    //         .map_err(|e| Error::Generic(format!("{:?}", e)))?;
    //     Ok(value)
    // }

    // ///
    // ///
    // ///
    // pub async fn get_vout(&mut self) -> Result<f32, Error> {
    //     let cmd = "VOUT1?".to_string();
    //     let response = self.driver.ask(&cmd).await?;
    //     let value = response
    //         .parse::<f32>()
    //         .map_err(|e| Error::Generic(format!("{:?}", e)))?;
    //     Ok(value)
    // }

    // ///
    // ///
    // ///
    // pub async fn set_out(&mut self, value: bool) -> Result<(), Error> {
    //     match value {
    //         true => {
    //             let cmd = "OUT1".to_string();
    //             self.driver.send(&cmd).await
    //         }
    //         false => {
    //             let cmd = "OUT0".to_string();
    //             self.driver.send(&cmd).await
    //         }
    //     }
    // }

    // ///
    // ///
    // ///
    // pub async fn get_out(&mut self) -> Result<bool, Error> {
    //     let cmd = "STATUS?".to_string();
    //     let response = self.driver.ask(&cmd).await?;
    //     let byte = response.as_bytes()[0];
    //     if (byte & (1 << 6)) == 0 {
    //         Ok(false)
    //     } else {
    //         Ok(true)
    //     }
    // }

    // ///
    // ///
    // ///
    // pub async fn set_beep(&mut self, value: bool) -> Result<(), Error> {
    //     match value {
    //         true => {
    //             let cmd = "BEEP1".to_string();
    //             self.driver.send(&cmd).await
    //         }
    //         false => {
    //             let cmd = "BEEP0".to_string();
    //             self.driver.send(&cmd).await
    //         }
    //     }
    // }

    // ///
    // ///
    // ///
    // pub async fn get_beep(&mut self) -> Result<bool, Error> {
    //     let cmd = "STATUS?".to_string();
    //     let response = self.driver.ask(&cmd).await?;
    //     let byte = response.as_bytes()[0];
    //     if (byte & (1 << 4)) == 0 {
    //         Ok(false)
    //     } else {
    //         Ok(true)
    //     }
    // }

    // ///
    // ///
    // ///
    // pub async fn set_ocp(&mut self, value: bool) -> Result<(), Error> {
    //     match value {
    //         true => {
    //             let cmd = "OCP1".to_string();
    //             self.driver.send(&cmd).await
    //         }
    //         false => {
    //             let cmd = "OCP0".to_string();
    //             self.driver.send(&cmd).await
    //         }
    //     }
    // }

    // ///
    // ///
    // ///
    // pub async fn set_ovp(&mut self, value: bool) -> Result<(), Error> {
    //     match value {
    //         true => {
    //             let cmd = "OVP1".to_string();
    //             self.driver.send(&cmd).await
    //         }
    //         false => {
    //             let cmd = "OVP0".to_string();
    //             self.driver.send(&cmd).await
    //         }
    //     }
    // }
}
