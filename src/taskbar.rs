use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{
        FindWindowW, SetWindowLongW, GetWindowLongW, GWL_EXSTYLE, 
        SetLayeredWindowAttributes, LWA_ALPHA, WS_EX_LAYERED
    },
    System::SystemServices::COLORREF,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaskbarError {
    #[error("Window handle not found")]
    HandleNotFound,
    #[error("API call failed")]
    ApiError(#[from] windows::core::Error),
}

pub fn apply_transparency(alpha: u8) -> Result<(), TaskbarError> {
    unsafe {
        let hwnd = find_taskbar()?;
        let style = GetWindowLongW(hwnd, GWL_EXSTYLE) | WS_EX_LAYERED;
        SetWindowLongW(hwnd, GWL_EXSTYLE, style);
        SetLayeredWindowAttributes(hwnd, COLORREF(0), alpha, LWA_ALPHA)?;
        Ok(())
    }
}

pub fn restore_defaults() -> Result<(), TaskbarError> {
    unsafe {
        let hwnd = find_taskbar()?;
        let style = GetWindowLongW(hwnd, GWL_EXSTYLE) & !WS_EX_LAYERED;
        SetWindowLongW(hwnd, GWL_EXSTYLE, style);
        Ok(())
    }
}

fn find_taskbar() -> Result<HWND, TaskbarError> {
    unsafe {
        FindWindowW("Shell_TrayWnd", None)
            .ok_or(TaskbarError::HandleNotFound)
    }
}