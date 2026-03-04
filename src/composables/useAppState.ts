import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { FrontendState } from "../types";
import { defaultState } from "../types";

const state = ref<FrontendState>({ ...defaultState });

let initialized = false;

function apply(fs: FrontendState) {
  state.value = fs;
}

export function useAppState() {
  async function init() {
    if (initialized) return;
    initialized = true;
    try {
      const fs = await invoke<FrontendState>("get_state");
      apply(fs);
    } catch (_e) {
      /* backend not ready yet */
    }
    await listen<FrontendState>("state-updated", (ev) => {
      apply(ev.payload);
    });
  }

  async function updateSetting(key: string, value: unknown) {
    const fs = await invoke<FrontendState>("update_settings", { key, value });
    apply(fs);
  }

  async function startScan() {
    const fs = await invoke<FrontendState>("start_scan");
    apply(fs);
  }

  async function stopScan() {
    const fs = await invoke<FrontendState>("stop_scan");
    apply(fs);
  }

  async function scanOnce() {
    const fs = await invoke<FrontendState>("scan_once");
    apply(fs);
  }

  async function applyKeywords(keywords: string[]) {
    const fs = await invoke<FrontendState>("apply_keywords", { keywords });
    apply(fs);
  }

  async function applySuffix(suffix: string) {
    const fs = await invoke<FrontendState>("apply_suffix", { suffix });
    apply(fs);
  }

  async function addInputPath(path: string) {
    const fs = await invoke<FrontendState>("add_input_path", { path });
    apply(fs);
  }

  async function removeInputPath(path: string) {
    const fs = await invoke<FrontendState>("remove_input_path", { path });
    apply(fs);
  }

  async function clearQueue() {
    const fs = await invoke<FrontendState>("clear_queue");
    apply(fs);
  }

  async function refreshFfmpeg() {
    const fs = await invoke<FrontendState>("refresh_ffmpeg_status");
    apply(fs);
  }

  async function pickFolder(): Promise<string | null> {
    return await invoke<string | null>("pick_folder");
  }

  async function pickFfmpeg(): Promise<string | null> {
    return await invoke<string | null>("pick_ffmpeg");
  }

  async function downloadFfmpeg() {
    const fs = await invoke<FrontendState>("download_ffmpeg_cmd");
    apply(fs);
  }

  return {
    state,
    init,
    updateSetting,
    startScan,
    stopScan,
    scanOnce,
    applyKeywords,
    applySuffix,
    addInputPath,
    removeInputPath,
    clearQueue,
    refreshFfmpeg,
    pickFolder,
    pickFfmpeg,
    downloadFfmpeg,
  };
}
