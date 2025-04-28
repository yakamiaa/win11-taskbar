mod tray;
mod taskbar;
mod registry;

use std::sync::mpsc;
use tray::TrayMessage;
use anyhow::Result;

fn main() -> Result<()> {
    // Initialize tray with channel
    let (tray, receiver) = tray::SystemTray::new()?;
    tray.set_tooltip("Taskbar Customizer")?;
    tray.show_notification("Initialized", "Running in background")?;

    // Load or create default settings
    let mut settings = registry::load_settings().unwrap_or_default();
    taskbar::apply_settings(&settings)?;

    // Main event loop
    while let Ok(msg) = receiver.recv() {
        match msg {
            TrayMessage::ShowSettings => {
                if let Some(new_settings) = registry::show_settings_dialog(&settings) {
                    settings = new_settings;
                    taskbar::apply_settings(&settings)?;
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