//
// Copyright © 2020 Haim Gelfenbeyn
// This code is licensed under MIT license (see LICENSE.txt for details)
//

use anyhow::{anyhow, Result};
use config;
use dirs;
use serde::Deserialize;
use std::collections::HashSet;

use crate::display_control;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub usb_device: String,
    pub monitor_input_when_plugged_in: display_control::InputSource,
    pub monitor_input_when_unplugged: display_control::InputSource,
    pub which_monitors_to_switch: HashSet<usize>,
}

impl Configuration {
    pub fn load() -> Result<Self> {
        let config_file_name = Self::config_file_name()?;
        let mut settings = config::Config::default();
        settings
            .merge(config::File::from(config_file_name.to_path_buf()))?
            .merge(config::Environment::with_prefix("DISPLAY_SWITCH"))?;
        let config = settings.try_into::<Self>()?;
        info!(
            "Configuration loaded ({:?}): {:?}",
            config_file_name, config
        );
        Ok(config)
    }

    pub fn config_file_name() -> Result<std::path::PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or(anyhow!("Config directory not found"))?
            .join("display-switch");
        std::fs::create_dir_all(&config_dir)?;
        Ok(config_dir.join("display-switch.json"))
    }

    pub fn log_file_name() -> Result<std::path::PathBuf> {
        let log_dir = dirs::data_local_dir()
            .ok_or(anyhow!("Data-local directory not found"))?
            .join("display-switch");
        std::fs::create_dir_all(&log_dir)?;
        Ok(log_dir.join("display-switch.log"))
    }
}
