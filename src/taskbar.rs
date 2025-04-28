use windows::Win32::{
    Foundation::{HWND, COLORREF},
    UI::WindowsAndMessaging::{
        FindWindowW, SetWindowLongW, GetWindowLongW, GWL_EXSTYLE, 
        SetLayeredWindowAttributes, LWA_ALPHA, WS_EX_LAYERED
    },
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
        let style = GetWindowLongW(hwnd, GWL_EXSTYLE) as i32 | WS_EX_LAYERED as i32;
        SetWindowLongW(hwnd, GWL_EXSTYLE, style as _);
        SetLayeredWindowAttributes(hwnd, COLORREF(0), alpha, LWA_ALPHA)?;
        Ok(())
    }
}

pub fn restore_defaults() -> Result<(), TaskbarError> {
    unsafe {
        let hwnd = find_taskbar()?;
        let style = GetWindowLongW(hwnd, GWL_EXSTYLE) as i32 & !(WS_EX_LAYERED as i32);
        SetWindowLongW(hwnd, GWL_EXSTYLE, style as _);
        Ok(())
    }
}

fn find_taskbar() -> Result<HWND, TaskbarError> {
    unsafe {
        let hwnd = FindWindowW(windows::core::w!("Shell_TrayWnd"), None);
        if hwnd.is_null() {
            Err(TaskbarError::HandleNotFound)
        } else {
            Ok(hwnd)
        }
    }
}