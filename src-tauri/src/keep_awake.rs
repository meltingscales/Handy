//! Keep the screen on during long-running work.
//!
//! Android suspends an app once the screen locks, which stalls model
//! downloads, so while a [`KeepAwake`] guard is alive the main activity sets
//! `FLAG_KEEP_SCREEN_ON`. Desktop systems keep background work running, so
//! there the guard does nothing.

use tauri::AppHandle;

/// Holds the screen on until dropped. Guards nest: the flag clears when the
/// last one goes away.
pub struct KeepAwake {
    #[cfg(target_os = "android")]
    app: AppHandle,
}

impl KeepAwake {
    #[cfg_attr(not(target_os = "android"), allow(unused_variables))]
    pub fn acquire(app: &AppHandle) -> Self {
        #[cfg(target_os = "android")]
        {
            android::adjust(app, 1);
            Self { app: app.clone() }
        }
        #[cfg(not(target_os = "android"))]
        Self {}
    }
}

#[cfg(target_os = "android")]
impl Drop for KeepAwake {
    fn drop(&mut self) {
        android::adjust(&self.app, -1);
    }
}

#[cfg(target_os = "android")]
mod android {
    use std::sync::Mutex;
    use tauri::{AppHandle, Manager};

    /// WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON
    const FLAG_KEEP_SCREEN_ON: i32 = 0x80;

    /// Live guard count. The lock is held while queueing the flag change so
    /// concurrent acquire/release cannot reach the UI thread out of order.
    static HOLDERS: Mutex<usize> = Mutex::new(0);

    pub fn adjust(app: &AppHandle, delta: isize) {
        let mut holders = HOLDERS.lock().unwrap_or_else(|p| p.into_inner());
        let before = *holders;
        *holders = before.saturating_add_signed(delta);
        match (before, *holders) {
            (0, 1) => set_flag(app, true),
            (1, 0) => set_flag(app, false),
            _ => {}
        }
    }

    fn set_flag(app: &AppHandle, on: bool) {
        let Some(window) = app.get_webview_window("main") else {
            return;
        };
        let result = window.with_webview(move |webview| {
            // Runs on the UI thread, which window flag changes require.
            webview.jni_handle().exec(move |env, activity, _webview| {
                let method = if on { "addFlags" } else { "clearFlags" };
                let result = env
                    .call_method(activity, "getWindow", "()Landroid/view/Window;", &[])
                    .and_then(|window| window.l())
                    .and_then(|window| {
                        env.call_method(&window, method, "(I)V", &[FLAG_KEEP_SCREEN_ON.into()])
                    });
                if let Err(e) = result {
                    log::warn!("Failed to {method} FLAG_KEEP_SCREEN_ON: {e}");
                }
            });
        });
        if let Err(e) = result {
            log::warn!("Failed to reach the main webview to keep the screen on: {e}");
        }
    }
}
