import { platform } from "@tauri-apps/plugin-os";

/**
 * Android and iOS builds have no global shortcuts, tray, recording overlay,
 * autostart, updater or synthetic typing, so their settings are hidden there.
 */
export const isMobile = platform() === "android" || platform() === "ios";
