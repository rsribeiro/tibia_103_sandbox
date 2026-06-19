/********************************************************************************
 * 
 * For ease of use, the game server can be configured in the "server.toml" file
 * 
 ********************************************************************************/
use anyhow::{anyhow, Result};
use serde::Deserialize;
use std::{net::Ipv4Addr, path::Path, sync::OnceLock};


/********************************************************************************
* 
* Configurable settings
* 
********************************************************************************/
pub static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server: Server,
    pub world: World,
}

#[derive(Deserialize, Debug)]
pub struct Server {
    pub ip: Ipv4Addr,
    pub port: u16,
    pub debug_messages: bool,
    pub status_message: String,
    pub message_of_the_day: String,
}

#[derive(Deserialize, Debug)]
pub struct World {
    pub map_file: String,
}


/********************************************************************************
 * 
 * Initialize config
 * 
 ********************************************************************************/
pub fn init(path: &Path) -> Result<()> {
    if !path.exists() {
        return Err(anyhow!("Config file {:?} not found", path));
    }
    
    let contents = std::fs::read_to_string(path)?;
    let config = toml::from_str::<Config>(&contents)?;
    CONFIG.set(config).map_err(|_| anyhow!("Config already initialised"))?;
    Ok(())
}


/********************************************************************************
 * 
 * Enable debug logging
 * 
 ********************************************************************************/
pub fn is_debug() -> bool {
    CONFIG.get().map(|c| c.server.debug_messages).unwrap_or(false)
}


/********************************************************************************
 * 
 * Print a datetime formatted log message if "debug_messages = true"
 * 
 ********************************************************************************/
#[macro_export]
macro_rules! debug_log {
    () => {
        if $crate::config::is_debug() {
            println!(
                "[{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                ""
            );
        }
    };
    ($($arg:tt)+) => {
        if $crate::config::is_debug() {
            println!(
                "[{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                format!($($arg)*)
            );
        }
    };
}