<script setup lang="ts">
import { ref, computed } from "vue";
import SectionCard from "./SectionCard.vue";
import { useAppState } from "../composables/useAppState";
import { openUrl } from "@tauri-apps/plugin-opener";

const { state, pickFfmpeg, updateSetting, refreshFfmpeg } = useAppState();
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
  if (b < 0) return "-";
  if (b < 1024) return b + " B";
  if (b < 1048576) return (b / 1024).toFixed(1) + " KB";
  if (b < 1073741824) return (b / 1048576).toFixed(1) + " MB";
  return (b / 1073741824).toFixed(2) + " GB";
}

function formatTime(secs: number): string {
  if (secs <= 0) return "--:--:--";
  const h = Math.floor(secs / 3600);
  const m = Math.floor((secs % 3600) / 60);
  const s = Math.floor(secs % 60);
  return `${h.toString().padStart(2, "0")}:${m.toString().padStart(2, "0")}:${s.toString().padStart(2, "0")}`;
}

const hwColor = computed(() =>
  (state.value.hardwareAvailableH264 || state.value.hardwareAvailableH265)
    ? "text-green-500"
    : "text-gray-500 dark:text-gray-400"
);

const displayQueue = computed(() => state.value.queueItems.slice(0, 6));
</script>
<template>
  <SectionCard title="运行状态">
    <!-- Status row with last scan -->
    <div class="flex items-center justify-between text-sm">
      <span class="text-gray-600 dark:text-gray-300">状态：<span class="text-gray-800 dark:text-gray-200">{{ state.statusDisplay }}</span></span>
      <span class="text-xs text-gray-400">上次扫描：{{ state.lastScan || '无' }}</span>
    </div>

    <!-- FFmpeg info -->
    <div class="flex items-center gap-2 text-sm">
      <span class="text-gray-600 dark:text-gray-300 text-xs">FFmpeg {{ state.ffmpegVersionShort }} | <span :class="hwColor">{{ state.hardwareStatusText }}</span></span>
      <button
        class="text-gray-400 hover:text-blue-500 text-xs"
        @click="ffmpegPopover = !ffmpegPopover"
      >ⓘ</button>
    </div>

    <!-- FFmpeg popover -->
    <div v-if="ffmpegPopover" class="text-[11px] bg-white dark:bg-gray-700 rounded-lg p-2 space-y-1 border border-gray-200 dark:border-gray-600">
      <p><b>FFmpeg 版本</b></p>
      <p class="text-gray-500">{{ state.ffmpegVersionFull || '未配置' }}</p>
      <p><b>检测到的硬件编码器</b></p>
      <template v-if="state.ffmpegEncoderList.length">
        <p v-for="(enc, i) in state.ffmpegEncoderList" :key="i" class="text-gray-500">{{ enc }}</p>
      </template>
      <p v-else class="text-gray-500">无</p>
      <p class="text-gray-500">当前实际使用：{{ state.currentEncoderText || '未知' }}</p>
      <p v-if="state.hardwareStatusHint" class="text-gray-500">提示：{{ state.hardwareStatusHint }}</p>
      <p v-if="state.ffmpegEncoderError" class="text-red-500">错误：{{ state.ffmpegEncoderError }}</p>
    </div>

    <!-- Current task -->
    <div v-if="state.currentFileName" class="space-y-1">
      <div class="flex items-center justify-between text-xs text-gray-700 dark:text-gray-300">
        <span>当前任务：<span class="truncate max-w-[200px] inline-block align-bottom">{{ state.currentFileName }}</span></span>
        <span>{{ Math.floor(state.currentFileProgress * 100) }}%</span>
      </div>
      <div class="w-full h-1.5 bg-gray-200 dark:bg-gray-600 rounded-full overflow-hidden">
        <div class="h-full bg-blue-500 rounded-full transition-all" :style="{ width: (state.currentFileProgress * 100) + '%' }"></div>
      </div>
      <div class="flex justify-between text-[10px] text-gray-400">
        <span>预估时长：{{ formatTime(state.currentFileEstimatedTotal) }}</span>
        <span>预计剩余：{{ formatTime(state.currentFileRemaining) }}</span>
      </div>
    </div>
    <div v-else class="text-xs text-gray-400">
      当前空闲<span v-if="state.lastCompletedName" class="text-green-500"> · 已完成：{{ state.lastCompletedName }}</span>
    </div>

    <!-- Queue progress -->
    <div v-if="state.batchTotal > 0" class="space-y-1">
      <div class="flex items-center justify-between text-xs">
        <span class="text-gray-600 dark:text-gray-300">队列进度：{{ state.batchCompleted }}/{{ state.batchTotal }}</span>
        <span class="text-gray-400">{{ state.batchTotal > 0 ? Math.floor(state.batchCompleted / state.batchTotal * 100) : 0 }}%</span>
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
      <p class="text-xs text-gray-500">待处理列表：</p>
      <div
        v-for="item in displayQueue"
        :key="item.id"
        class="text-[11px] text-gray-600 dark:text-gray-300"
      >
        <span>• {{ item.name }}  原始 {{ formatBytes(item.sizeBytes) }}  预估 {{ item.estimatedBytes != null ? formatBytes(item.estimatedBytes) : '-' }}</span>
      </div>
      <p v-if="state.queueItems.length > 6" class="text-[10px] text-gray-400">
        …还有 {{ state.queueItems.length - 6 }} 项
      </p>
    </div>

    <!-- FFmpeg path -->
    <div class="flex items-center justify-between text-sm">
      <span class="text-gray-600 dark:text-gray-300 text-xs truncate max-w-[200px]">ffmpeg：{{ state.ffmpegPath || '未找到' }}</span>
      <button class="text-xs text-blue-500 hover:underline shrink-0" @click="chooseFfmpeg">选择 ffmpeg</button>
    </div>

    <!-- Author -->
    <div class="text-[11px] text-gray-400 flex items-center gap-1">
      <span>制作者：摄影师云飞</span>
      <button class="text-blue-500 hover:underline" @click="openUrl(APP_AUTHOR_LINK)">主页</button>
    </div>
  </SectionCard>
</template>
