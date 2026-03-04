<script setup lang="ts">
import { ref, computed } from "vue";
import SectionCard from "./SectionCard.vue";
import { useAppState } from "../composables/useAppState";
import { openUrl } from "@tauri-apps/plugin-opener";

const { state, pickFfmpeg, updateSetting, refreshFfmpeg, clearQueue } = useAppState();
const APP_AUTHOR_LINK = "https://space.bilibili.com/17519822";

const ffmpegPopover = ref(false);

async function chooseFfmpeg() {
  const p = await pickFfmpeg();
  if (p) {
    await updateSetting("ffmpegPath", p);
    await refreshFfmpeg();
  }
}

function formatBytes(b: number): string {
  if (b < 1024) return b + " B";
  if (b < 1048576) return (b / 1024).toFixed(1) + " KB";
  if (b < 1073741824) return (b / 1048576).toFixed(1) + " MB";
  return (b / 1073741824).toFixed(2) + " GB";
}

function formatTime(secs: number): string {
  if (secs <= 0) return "--:--";
  const m = Math.floor(secs / 60);
  const s = Math.floor(secs % 60);
  return `${m}:${s.toString().padStart(2, "0")}`;
}

const hwColor = computed(() =>
  state.value.hardwareStatusText.includes("已开启")
    ? "text-green-500"
    : "text-gray-500 dark:text-gray-400"
);

const displayQueue = computed(() => state.value.queueItems.slice(0, 6));
</script>
<template>
  <SectionCard title="运行状态">
    <!-- Status row -->
    <div class="flex items-center justify-between text-sm">
      <span class="text-gray-600 dark:text-gray-300">状态：<span class="text-gray-800 dark:text-gray-200">{{ state.statusDisplay }}</span></span>
    </div>

    <!-- FFmpeg info -->
    <div class="flex items-center justify-between text-sm">
      <div class="flex items-center gap-2">
        <span class="text-gray-600 dark:text-gray-300">FFmpeg {{ state.ffmpegVersionShort }}</span>
        <span :class="hwColor" class="text-xs">{{ state.hardwareStatusText }}</span>
        <button
          class="text-gray-400 hover:text-blue-500 text-xs"
          @click="ffmpegPopover = !ffmpegPopover"
        >ⓘ</button>
      </div>
    </div>

    <!-- FFmpeg popover -->
    <div v-if="ffmpegPopover" class="text-[11px] bg-white dark:bg-gray-700 rounded-lg p-2 space-y-1 border border-gray-200 dark:border-gray-600">
      <p>版本：{{ state.ffmpegVersionFull || state.ffmpegVersionShort }}</p>
      <p>编码器：{{ state.ffmpegEncoderList.join(', ') || '未检测' }}</p>
      <p>当前编码器：{{ state.currentEncoderText || '—' }}</p>
      <p v-if="state.hardwareStatusHint">提示：{{ state.hardwareStatusHint }}</p>
      <p v-if="state.ffmpegEncoderError" class="text-red-500">错误：{{ state.ffmpegEncoderError }}</p>
    </div>

    <!-- Current task -->
    <div v-if="state.currentFileName" class="space-y-1">
      <div class="flex items-center justify-between text-xs text-gray-700 dark:text-gray-300">
        <span class="truncate max-w-[200px]">{{ state.currentFileName }}</span>
        <span>{{ (state.currentFileProgress * 100).toFixed(1) }}%</span>
      </div>
      <div class="w-full h-1.5 bg-gray-200 dark:bg-gray-600 rounded-full overflow-hidden">
        <div class="h-full bg-blue-500 rounded-full transition-all" :style="{ width: (state.currentFileProgress * 100) + '%' }"></div>
      </div>
      <div class="flex justify-between text-[10px] text-gray-400">
        <span>已用 {{ formatTime(state.currentFileElapsed) }}</span>
        <span>剩余 {{ formatTime(state.currentFileRemaining) }}</span>
      </div>
    </div>
    <div v-else class="text-xs text-gray-400">
      当前空闲<span v-if="state.lastCompletedName"> · 上次完成：{{ state.lastCompletedName }}</span>
    </div>

    <!-- Queue progress -->
    <div v-if="state.batchTotal > 0" class="space-y-1">
      <div class="flex items-center justify-between text-xs">
        <span class="text-gray-600 dark:text-gray-300">队列进度 {{ state.queueProgress }}</span>
        <button class="text-red-500 hover:underline text-[11px]" @click="clearQueue">清空</button>
      </div>
      <div class="w-full h-1.5 bg-gray-200 dark:bg-gray-600 rounded-full overflow-hidden">
        <div
          class="h-full bg-green-500 rounded-full transition-all"
          :style="{ width: (state.batchTotal > 0 ? state.batchCompleted / state.batchTotal * 100 : 0) + '%' }"
        ></div>
      </div>
    </div>

    <!-- Pending queue list -->
    <div v-if="state.queueItems.length" class="space-y-1">
      <p class="text-xs text-gray-500">待处理列表</p>
      <div
        v-for="item in displayQueue"
        :key="item.id"
        class="flex items-center justify-between text-[11px] text-gray-600 dark:text-gray-300"
      >
        <span class="truncate max-w-[180px]">{{ item.name }}</span>
        <span class="text-gray-400">{{ formatBytes(item.sizeBytes) }}</span>
      </div>
      <p v-if="state.queueItems.length > 6" class="text-[10px] text-gray-400">
        …还有 {{ state.queueItems.length - 6 }} 项
      </p>
    </div>

    <!-- FFmpeg path -->
    <div class="flex items-center justify-between text-sm">
      <span class="text-gray-600 dark:text-gray-300 text-xs truncate max-w-[200px]">{{ state.ffmpegPath || 'FFmpeg 未配置' }}</span>
      <button class="text-xs text-blue-500 hover:underline shrink-0" @click="chooseFfmpeg">选择 ffmpeg</button>
    </div>

    <!-- Author -->
    <div class="text-[11px] text-gray-400 flex items-center gap-1">
      <span>制作者：摄影师云飞</span>
      <button class="text-blue-500 hover:underline" @click="openUrl(APP_AUTHOR_LINK)">主页</button>
    </div>
  </SectionCard>
</template>
