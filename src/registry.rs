use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Settings {
    pub transparency: u8,
    pub small_icons: bool,
    pub win10_style: bool,
}

#[derive(Error, Debug)]
pub enum RegistryError {
    #[error("Registry operation failed")]
    OperationFailed,
    #[error("Serialization error")]
    SerializationError(#[from] serde_json::Error),
}

pub fn load_settings() -> Result<Settings, RegistryError> {
    Ok(Settings::default())
}

pub fn save_settings(settings: &Settings) -> Result<(), RegistryError> {
    Ok(())
}

pub fn show_settings_dialog(current: &Settings) -> Option<Settings> {
    println!("Current settings: {:?}", current);
    println!("Enter new transparency (0-255):");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).ok()?;
    let transparency = input.trim().parse().unwrap_or(150);
    
    Some(Settings {
        transparency,
        ..*current
    })
}