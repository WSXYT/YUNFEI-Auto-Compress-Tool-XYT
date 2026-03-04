<script setup lang="ts">
import { ref } from "vue";
import SectionCard from "./SectionCard.vue";
import { useAppState } from "../composables/useAppState";
import type { OutputMode } from "../types";

const { state, updateSetting, applySuffix, pickFolder } = useAppState();

const draftSuffix = ref(state.value.suffixText);

const outputModeOpts: { value: OutputMode; label: string }[] = [
  { value: "overwrite", label: "覆盖原文件" },
  { value: "outputFolder", label: "另存到指定文件夹" },
  { value: "suffix", label: "添加后缀" },
];

async function chooseOutput() {
  const p = await pickFolder();
  if (p) await updateSetting("outputPath", p);
}

function confirmSuffix() {
  applySuffix(draftSuffix.value);
}
</script>
<template>
  <SectionCard title="输出设置">
    <!-- Output mode -->
    <div class="flex items-center justify-between text-sm">
      <span class="text-gray-600 dark:text-gray-300">输出方式</span>
      <div class="inline-flex rounded-lg overflow-hidden border border-gray-300 dark:border-gray-600">
        <button
          v-for="opt in outputModeOpts"
          :key="opt.value"
          class="px-2 py-0.5 text-xs transition-colors"
          :class="state.outputMode === opt.value
            ? 'bg-blue-500 text-white'
            : 'bg-white dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-600'"
          @click="updateSetting('outputMode', opt.value)"
        >{{ opt.label }}</button>
      </div>
    </div>

    <!-- Output folder (only when outputFolder mode) -->
    <div v-if="state.outputMode === 'outputFolder'" class="flex items-center justify-between text-sm">
      <span class="text-gray-600 dark:text-gray-300 shrink-0">输出文件夹</span>
      <div class="flex items-center gap-2 min-w-0">
        <span class="truncate text-gray-800 dark:text-gray-200 text-xs max-w-[200px]">{{ state.outputPath || '未选择' }}</span>
        <button class="text-xs text-blue-500 hover:underline shrink-0" @click="chooseOutput">选择…</button>
      </div>
    </div>

    <!-- Suffix (always visible, matching original Swift UI) -->
    <div class="flex items-center justify-between text-sm">
      <span class="text-gray-600 dark:text-gray-300 shrink-0">后缀</span>
      <div class="flex items-center gap-1">
        <input
          v-model="draftSuffix"
          type="text"
          class="text-xs border border-gray-300 dark:border-gray-600 rounded px-2 py-0.5 w-24 bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-200"
          @keyup.enter="confirmSuffix"
        />
        <button class="text-xs text-blue-500 hover:underline" @click="confirmSuffix">确定</button>
      </div>
    </div>

    <!-- Overwrite warning -->
    <p v-if="state.outputMode === 'overwrite'" class="text-[11px] text-orange-500">
      覆盖模式会直接替换原文件
    </p>

    <!-- Launch at login + Keep alive on same row -->
    <div class="flex items-center justify-between text-sm">
      <label class="flex items-center gap-1.5 cursor-pointer">
        <input
          type="checkbox"
          class="accent-blue-500 w-4 h-4"
          :checked="state.launchAtLogin"
          @change="updateSetting('launchAtLogin', ($event.target as HTMLInputElement).checked)"
        />
        <span class="text-gray-600 dark:text-gray-300">开机自启动</span>
      </label>
      <label class="flex items-center gap-1.5 cursor-pointer">
        <input
          type="checkbox"
          class="accent-blue-500 w-4 h-4"
          :checked="state.keepAlive"
          @change="updateSetting('keepAlive', ($event.target as HTMLInputElement).checked)"
        />
        <span class="text-gray-600 dark:text-gray-300">后台常驻（关闭窗口不退出）</span>
      </label>
    </div>
  </SectionCard>
</template>
