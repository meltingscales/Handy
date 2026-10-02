//! Mobile stand-in for the recording overlay: Android and iOS apps cannot
//! float a second always-on-top window, so overlay updates are no-ops.

use tauri::AppHandle;

pub fn create_recording_overlay(_app_handle: &AppHandle) {}

pub fn emit_recording_ready(_app_handle: &AppHandle) {}

pub fn show_recording_overlay(_app_handle: &AppHandle) {}

pub fn show_streaming_overlay(_app_handle: &AppHandle) {}

pub fn show_transcribing_overlay(_app_handle: &AppHandle) {}

pub fn show_processing_overlay(_app_handle: &AppHandle) {}

pub fn update_overlay_position(_app_handle: &AppHandle) {}

pub fn hide_recording_overlay(_app_handle: &AppHandle) {}

pub fn update_overlay_enabled_cache(_enabled: bool) {}

pub fn emit_levels(_app_handle: &AppHandle, _levels: &[f32]) {}
