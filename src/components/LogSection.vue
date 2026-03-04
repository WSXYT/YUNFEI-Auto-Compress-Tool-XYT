<script setup lang="ts">
import { ref, computed } from "vue";
import { useAppState } from "../composables/useAppState";

const { state } = useAppState();

defineEmits<{ (e: "openDonate"): void }>();

const showUpdateLog = ref(false);

const updateLogEntries = [
  "1.0.2.51 捐赠二维码内置打包",
  "1.0.2.50 修复启动校验误判（输出路径/压缩器）",
  "1.0.2.48 关键词确认后取消高亮",
  "1.0.2.47 关键词确认高亮与日志完善",
  "1.0.2.46 更多关键词增加确认按钮",
  "1.0.2.45 挂载检测恢复/监听与关键词UI优化",
  "1.0.2.44 恢复多目录/多关键词与功能详情增强",
  "1.0.1.33 修复关闭窗口后状态栏无法唤醒",
  "1.0.1.32 新增挂载自动检测开关与时间间隔设置",
  "1.0.1.31 挂载状态显示与未挂载提醒",
  "1.0.1.29 新增说明按钮与更新日志入口",
  '1.0.1.28 队列完成显示\u201C当前空闲\u201D，恢复 ffmpeg 自动下载',
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
      >{{ showUpdateLog ? '收起更新日志' : '更新日志' }}</button>
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

    <div class="flex justify-end pt-1">
      <button
        class="text-xs px-3 py-1 rounded-lg bg-blue-500 hover:bg-blue-600 text-white"
        @click="$emit('openDonate')"
      >请作者喝奶茶</button>
    </div>
  </div>
</template>
