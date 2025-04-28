mod tray;
mod taskbar;
mod registry;

use std::sync::mpsc;
use tray::TrayMessage;
use anyhow::Result;

fn main() -> Result<()> {
    // Initialize tray with channel
    let (tray, receiver) = tray::SystemTray::new()?;

    // Load or create default settings
    let mut settings = registry::load_settings().unwrap_or_default();
    taskbar::apply_transparency(settings.transparency)?;

    // Main event loop
    while let Ok(msg) = receiver.recv() {
        match msg {
            TrayMessage::ShowSettings => {
                if let Some(new_settings) = registry::show_settings_dialog(&settings) {
                    settings = new_settings;
                    taskbar::apply_transparency(settings.transparency)?;
                    registry::save_settings(&settings)?;
                }
            }
            TrayMessage::RestoreDefaults => {
                settings = registry::Settings::default();
                taskbar::restore_defaults()?;
                registry::save_settings(&settings)?;
            }
            TrayMessage::Exit => break,
        }
    }

    Ok(())
}