<script setup lang="ts">
import { ref } from "vue";

defineEmits<{ (e: "close"): void }>();

const showDetail = ref(false);

const features = [
  "监控指定文件夹，自动发现视频文件",
  "支持关键词过滤（如「带字幕」）",
  "自动使用 FFmpeg 进行压缩",
  "支持 H.264 / H.265 编码",
  "支持硬件加速编码",
  "可设置码率档位：12 / 18 / 25 Mbps",
  "支持覆盖原文件、另存文件夹、添加后缀",
  "支持多目录监听（最多3个）",
  "自动检测外置磁盘挂载状态",
  "支持开机自启动和后台常驻",
];

const details = [
  "本工具用于批量自动压缩视频文件，适用于摄影师、视频创作者日常素材管理。",
  "设置好监听目录和压缩参数后，点击「开始」即可自动扫描和压缩。",
  "支持的视频格式：mp4、mov、mkv、avi、m4v。",
  "压缩时保持原始分辨率和帧率，仅调整码率以减小文件体积。",
  "建议使用 H.265 编码以获得更好的压缩比。",
];
</script>
<template>
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/40" @click.self="$emit('close')">
    <div class="bg-white dark:bg-gray-800 rounded-2xl p-5 w-[340px] max-h-[80vh] overflow-y-auto shadow-xl space-y-3">
      <div class="flex items-center justify-between">
        <h2 class="text-base font-bold text-gray-900 dark:text-white">功能说明</h2>
        <button class="text-gray-400 hover:text-gray-600 text-lg" @click="$emit('close')">✕</button>
      </div>

      <ul class="space-y-1.5">
        <li v-for="(f, i) in features" :key="i" class="text-xs text-gray-700 dark:text-gray-300 flex gap-1.5">
          <span class="text-blue-500">•</span>{{ f }}
        </li>
      </ul>

      <button
        class="text-xs text-blue-500 hover:underline"
        @click="showDetail = !showDetail"
      >{{ showDetail ? '收起详情' : '详细说明' }}</button>

      <div v-if="showDetail" class="space-y-1.5">
        <p v-for="(d, i) in details" :key="i" class="text-xs text-gray-600 dark:text-gray-400">{{ d }}</p>
      </div>
    </div>
  </div>
</template>
