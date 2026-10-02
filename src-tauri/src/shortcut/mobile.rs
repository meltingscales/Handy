//! Mobile stand-ins for the global-shortcut backends. Android and iOS give
//! apps no system-wide key capture, so registration succeeds without doing
//! anything and recording is started from the app UI instead.

use tauri::AppHandle;

use crate::settings::ShortcutBinding;

pub mod tauri_impl {
    use super::*;

    pub fn init_shortcuts(_app: &AppHandle) {}

    pub fn validate_shortcut(_raw: &str) -> Result<(), String> {
        Ok(())
    }

    pub fn register_shortcut(_app: &AppHandle, _binding: ShortcutBinding) -> Result<(), String> {
        Ok(())
    }

    pub fn unregister_shortcut(_app: &AppHandle, _binding: ShortcutBinding) -> Result<(), String> {
        Ok(())
    }

    pub fn register_cancel_shortcut(_app: &AppHandle) {}

    pub fn unregister_cancel_shortcut(_app: &AppHandle) {}
}

pub mod handy_keys {
    use super::*;

    /// Never managed on mobile; exists so state lookups type-check.
    pub struct HandyKeysState;

    const UNSUPPORTED: &str = "handy-keys is not available on mobile";

    pub fn init_shortcuts(_app: &AppHandle) -> Result<(), String> {
        Err(UNSUPPORTED.into())
    }

    pub fn validate_shortcut(_raw: &str) -> Result<(), String> {
        Ok(())
    }

    pub fn register_shortcut(_app: &AppHandle, _binding: ShortcutBinding) -> Result<(), String> {
        Ok(())
    }

    pub fn unregister_shortcut(_app: &AppHandle, _binding: ShortcutBinding) -> Result<(), String> {
        Ok(())
    }

    pub fn register_cancel_shortcut(_app: &AppHandle) {}

    pub fn unregister_cancel_shortcut(_app: &AppHandle) {}

    #[tauri::command]
    #[specta::specta]
    pub fn start_handy_keys_recording(_app: AppHandle, _binding_id: String) -> Result<(), String> {
        Err(UNSUPPORTED.into())
    }

    #[tauri::command]
    #[specta::specta]
    pub fn stop_handy_keys_recording(_app: AppHandle) -> Result<(), String> {
        Err(UNSUPPORTED.into())
    }
}
