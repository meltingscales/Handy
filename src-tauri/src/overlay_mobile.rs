//! Mobile stand-in for the recording overlay. Android and iOS apps cannot
//! float a second always-on-top window, so the overlay's show/hide events go
//! to the main window instead, where the record button reflects them.

use tauri::{AppHandle, Emitter};

fn show_overlay_state(app_handle: &AppHandle, state: &str) {
    let _ = app_handle.emit_to("main", "show-overlay", state);
}

pub fn create_recording_overlay(_app_handle: &AppHandle) {}

pub fn emit_recording_ready(_app_handle: &AppHandle) {}

pub fn show_recording_overlay(app_handle: &AppHandle) {
    show_overlay_state(app_handle, "recording");
}

pub fn show_streaming_overlay(app_handle: &AppHandle) {
    show_overlay_state(app_handle, "streaming");
}

pub fn show_transcribing_overlay(app_handle: &AppHandle) {
    show_overlay_state(app_handle, "transcribing");
}

pub fn show_processing_overlay(app_handle: &AppHandle) {
    show_overlay_state(app_handle, "processing");
}

pub fn update_overlay_position(_app_handle: &AppHandle) {}

pub fn hide_recording_overlay(app_handle: &AppHandle) {
    let _ = app_handle.emit_to("main", "hide-overlay", ());
}

pub fn update_overlay_enabled_cache(_enabled: bool) {}

pub fn emit_levels(_app_handle: &AppHandle, _levels: &[f32]) {}
