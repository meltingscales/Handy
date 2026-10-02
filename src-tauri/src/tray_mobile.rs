//! Mobile stand-in for the system tray: Android and iOS have none, so tray
//! updates driven by recording state and settings are no-ops.

use tauri::AppHandle;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrayIconState {
    Idle,
    Recording,
    Transcribing,
}

pub fn set_tray_state(_app: &AppHandle, _state: TrayIconState) {}

pub fn refresh_tray_icon(_app: &AppHandle) {}

pub fn update_tray_menu(_app: &AppHandle) {}

pub fn set_tray_visibility(_app: &AppHandle, _visible: bool) {}
