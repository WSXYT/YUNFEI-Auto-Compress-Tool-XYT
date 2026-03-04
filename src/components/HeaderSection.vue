<script setup lang="ts">
import { useAppState } from "../composables/useAppState";
import { openUrl } from "@tauri-apps/plugin-opener";
import { APP_TITLE } from "../types";

const { state, startScan, stopScan } = useAppState();
const APP_AUTHOR_LINK = "https://space.bilibili.com/17519822";

defineEmits<{ (e: "openHelp"): void }>();

function openHomepage() {
  openUrl(APP_AUTHOR_LINK);
}

async function toggleScan() {
  if (state.value.scanEnabled) {
    await stopScan();
  } else {
    await startScan();
  }
}

const keywordsDisplay = (kw: string[]) =>
  kw.length > 0 ? kw.join("、") : "全部";
</script>
<template>
  <div class="flex items-center justify-between">
    <div>
      <h1 class="text-lg font-bold text-gray-900 dark:text-white">{{ APP_TITLE }}</h1>
      <p class="text-xs text-gray-500 dark:text-gray-400">
        监控文件夹 → 发现<span
          :class="state.keywords.length > 0 ? 'text-blue-500 font-medium' : ''"
        >{{ keywordsDisplay(state.keywords) }}</span>视频 → 自动压缩
      </p>
    </div>
    <div class="flex items-center gap-2">
      <button
        class="text-xs text-blue-500 hover:underline"
        @click="openHomepage"
      >主页</button>
      <button
        class="text-xs px-3 py-1 rounded-lg bg-gray-200 dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-300 dark:hover:bg-gray-600"
        @click="$emit('openHelp')"
      >说明</button>
      <button
        class="text-xs px-4 py-1 rounded-lg font-medium text-white"
        :class="state.scanEnabled ? 'bg-red-500 hover:bg-red-600' : 'bg-blue-500 hover:bg-blue-600'"
        @click="toggleScan"
      >{{ state.scanEnabled ? '停止' : '开始' }}</button>
    </div>
  </div>
</template>
