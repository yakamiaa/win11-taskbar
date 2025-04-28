use tray_item::TrayItem;
use std::sync::mpsc::{Sender, channel};
use anyhow::Result;

pub enum TrayMessage {
    ShowSettings,
    RestoreDefaults,
    Exit,
}

pub struct SystemTray {
    tray_item: TrayItem,
    sender: Sender<TrayMessage>,
}

impl SystemTray {
    pub fn new() -> Result<(Self, std::sync::mpsc::Receiver<TrayMessage>)> {
        let (sender, receiver) = channel();
        
        let mut tray_item = TrayItem::new("Taskbar Customizer", "")?;
        
        tray_item.add_menu_item("Settings", {
            let sender = sender.clone();
            move || sender.send(TrayMessage::ShowSettings).unwrap()
        })?;

        tray_item.add_menu_item("Restore Defaults", {
            let sender = sender.clone();
            move || sender.send(TrayMessage::RestoreDefaults).unwrap()
        })?;

        tray_item.add_menu_item("Exit", {
            move || sender.send(TrayMessage::Exit).unwrap()
        })?;

        Ok((Self { tray_item, sender }, receiver))
    }
}