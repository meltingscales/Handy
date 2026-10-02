import React, { useEffect, useState } from "react";
import { useTranslation } from "react-i18next";
import { listen } from "@tauri-apps/api/event";
import { commands } from "@/bindings";
import { Button } from "./ui/Button";

type RecordingState =
  | "idle"
  | "recording"
  | "streaming"
  | "transcribing"
  | "processing";

/**
 * Starts and stops a transcription from inside the app. Mobile has no global
 * shortcuts or floating overlay, so the backend sends the overlay's
 * show/hide events to the main window and this button mirrors them.
 */
const RecordButton: React.FC = () => {
  const { t } = useTranslation();
  const [state, setState] = useState<RecordingState>("idle");

  useEffect(() => {
    const unlistenShow = listen<RecordingState>("show-overlay", (event) =>
      setState(event.payload),
    );
    const unlistenHide = listen("hide-overlay", () => setState("idle"));
    return () => {
      unlistenShow.then((fn) => fn());
      unlistenHide.then((fn) => fn());
    };
  }, []);

  const busy = state === "transcribing" || state === "processing";
  const recording = state === "recording" || state === "streaming";

  const label = busy
    ? t(`overlay.${state}`)
    : recording
      ? t("overlay.stop")
      : t("overlay.record");

  return (
    <div className="w-full px-4 pb-3">
      <Button
        variant={recording ? "danger" : "primary"}
        size="lg"
        className="w-full py-4"
        disabled={busy}
        onClick={() => commands.toggleTranscription()}
      >
        {label}
      </Button>
    </div>
  );
};

export default RecordButton;
