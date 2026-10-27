import { useEffect, useRef, useState } from "react";
import { applyDisplay, followSystemTheme, loadDisplay, saveDisplay, type DisplaySettings } from "../lib/display";

/** Display settings held at the app root; changing one saves it, applies it and re-renders. */
export function useDisplay() {
  const [settings, setSettings] = useState<DisplaySettings>(loadDisplay);
  const current = useRef(settings);
  current.current = settings;
  useEffect(() => followSystemTheme(() => current.current), []);

  const update = (patch: Partial<DisplaySettings>) => {
    const next = { ...current.current, ...patch };
    // Updated before the re-render so a second change in the same event builds on this one.
    current.current = next;
    saveDisplay(next);
    applyDisplay(next);
    setSettings(next);
  };
  return { settings, update };
}
