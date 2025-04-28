use tray_item::{TrayItem, IconSource};
use std::sync::mpsc::{Sender, channel};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TrayError {
    #[error("Tray creation failed")]
    CreationFailed,
    #[error("Notification failed")]
    NotificationFailed,
}

pub enum TrayMessage {
    ShowSettings,
    RestoreDefaults,
    Exit,
}

pub struct SystemTray {
    item: TrayItem,
    sender: Sender<TrayMessage>,
}

impl SystemTray {
    pub fn new() -> Result<(Self, std::sync::mpsc::Receiver<TrayMessage>), TrayError> {
        let (sender, receiver) = channel();
        
        let mut item = TrayItem::new("Taskbar Customizer", IconSource::Resource(""))
            .map_err(|_| TrayError::CreationFailed)?;

        item.add_menu_item("Settings", {
            let sender = sender.clone();
            move || sender.send(TrayMessage::ShowSettings).unwrap()
        }).map_err(|_| TrayError::CreationFailed)?;

        item.add_menu_item("Restore Defaults", {
            let sender = sender.clone();
            move || sender.send(TrayMessage::RestoreDefaults).unwrap()
        }).map_err(|_| TrayError::CreationFailed)?;

        item.add_menu_item("Exit", {
            move || sender.send(TrayMessage::Exit).unwrap()
        }).map_err(|_| TrayError::CreationFailed)?;

        Ok((Self { item, sender }, receiver))
    }

    pub fn set_tooltip(&self, tooltip: &str) -> Result<(), TrayError> {
        self.item.set_tooltip(tooltip)
            .map_err(|_| TrayError::CreationFailed)
    }

    pub fn show_notification(&self, title: &str, msg: &str) -> Result<(), TrayError> {
        self.item.show_notification(title, msg, None)
            .map_err(|_| TrayError::NotificationFailed)
    }
}