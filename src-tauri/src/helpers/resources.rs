//! Bundled resources as paths that native code can open.

use std::path::PathBuf;
use tauri::{AppHandle, Manager};

/// Resolve a bundled resource (e.g. `resources/models/silero_vad_v4.onnx`) to
/// a real file path.
///
/// Android keeps resources inside the APK, where native libraries such as
/// ONNX Runtime cannot open them, so there the file is copied once into the
/// app cache directory and that copy's path is returned.
pub fn resource_file_path(app: &AppHandle, relative: &str) -> Result<PathBuf, String> {
    let resolved = app
        .path()
        .resolve(relative, tauri::path::BaseDirectory::Resource)
        .map_err(|e| format!("Failed to resolve resource {relative}: {e}"))?;

    #[cfg(target_os = "android")]
    return extract_android_asset(app, relative, resolved);

    #[cfg(not(target_os = "android"))]
    Ok(resolved)
}

#[cfg(target_os = "android")]
fn extract_android_asset(
    app: &AppHandle,
    relative: &str,
    asset: PathBuf,
) -> Result<PathBuf, String> {
    use tauri_plugin_fs::FsExt;

    let bytes = app
        .fs()
        .read(asset)
        .map_err(|e| format!("Failed to read bundled {relative}: {e}"))?;

    let target = app
        .path()
        .app_cache_dir()
        .map_err(|e| format!("Failed to resolve cache dir: {e}"))?
        .join(relative);

    // Re-extract when an app update ships different contents.
    if std::fs::read(&target).ok().as_deref() == Some(bytes.as_slice()) {
        return Ok(target);
    }

    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create {}: {e}", parent.display()))?;
    }
    // Write then rename so an interrupted copy never leaves a truncated file
    // behind that a later run would load.
    let partial = target.with_extension("partial");
    std::fs::write(&partial, &bytes)
        .map_err(|e| format!("Failed to write {}: {e}", partial.display()))?;
    std::fs::rename(&partial, &target)
        .map_err(|e| format!("Failed to move {} into place: {e}", target.display()))?;
    Ok(target)
}
