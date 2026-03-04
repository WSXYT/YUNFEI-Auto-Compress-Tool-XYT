<script setup lang="ts">
import { ref, computed } from "vue";
import { useAppState } from "../composables/useAppState";

const { state } = useAppState();

defineEmits<{ (e: "openDonate"): void }>();

const showUpdateLog = ref(false);

const updateLogEntries = [
  "v1.0.2 — 新增多目录监听、挂载检测",
  "v1.0.1 — 新增关键词过滤、后缀输出",
  "v1.0.0 — 首个版本，支持自动扫描和压缩",
];

const recentLogs = computed(() => state.value.logs.slice(-12).reverse());
</script>
<template>
  <div class="rounded-xl bg-gray-100 dark:bg-gray-900/50 p-3 space-y-2">
    <div class="flex items-center justify-between">
      <h3 class="text-sm font-semibold text-gray-500 dark:text-gray-400">日志</h3>
      <button
        class="text-xs px-2 py-0.5 rounded"
        :class="showUpdateLog
          ? 'bg-blue-500 text-white'
          : 'bg-gray-200 dark:bg-gray-700 text-gray-600 dark:text-gray-300'"
        @click="showUpdateLog = !showUpdateLog"
      >更新日志</button>
    </div>

    <div v-if="showUpdateLog" class="space-y-0.5">
      <p v-for="(entry, i) in updateLogEntries" :key="i" class="text-[11px] text-gray-500 dark:text-gray-400">
        {{ entry }}
      </p>
    </div>

    <div class="max-h-48 overflow-y-auto space-y-0.5">
      <p
        v-for="(log, i) in recentLogs"
        :key="i"
        class="text-[11px] text-gray-600 dark:text-gray-400 font-mono"
      >{{ log }}</p>
      <p v-if="!recentLogs.length" class="text-[11px] text-gray-400">暂无日志</p>
    </div>

    <div class="flex justify-center pt-1">
      <button
        class="text-xs px-3 py-1 rounded-lg bg-orange-400 hover:bg-orange-500 text-white"
        @click="$emit('openDonate')"
      >☕ 请作者喝奶茶</button>
    </div>
  </div>
</template>
