use egui::{Ui, Window};
use crate::taskbar::TaskbarSettings;

pub fn show_settings_window(settings: &TaskbarSettings) {
    let mut modified_settings = settings.clone();
    
    egui::Window::new("Taskbar Customizer")
        .resizable(false)
        .show(egui::context(), |ui| {
            ui.heading("Taskbar Style");
            ui.checkbox(&mut modified_settings.win10_style, "Windows 10 Style");
            
            ui.separator();
            
            ui.heading("Appearance");
            ui.horizontal(|ui| {
                ui.label("Transparency:");
                ui.add(egui::Slider::new(&mut modified_settings.transparency, 0..=255));
            });
            
            ui.checkbox(&mut modified_settings.small_icons, "Small Icons");
            ui.checkbox(&mut modified_settings.acrylic_blur, "Acrylic Blur Effect");
            
            if ui.button("Apply").clicked() {
                // Save and apply settings
            }
            
            if ui.button("Restore Defaults").clicked() {
                // Restore original settings
            }
        });
}