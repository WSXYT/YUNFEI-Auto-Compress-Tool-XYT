<script setup lang="ts">
import { ref } from "vue";
import SectionCard from "./SectionCard.vue";
import { useAppState } from "../composables/useAppState";
import type { MountCheckIntervalOption } from "../types";

const { state, updateSetting, pickFolder, addInputPath, removeInputPath } = useAppState();

const extraExpanded = ref(false);

async function choosePrimary() {
  const p = await pickFolder();
  if (p) await updateSetting("inputPath", p);
}

async function addExtra() {
  const p = await pickFolder();
  if (p) await addInputPath(p);
}

function mountBadge(s: string): { text: string; cls: string } {
  if (s.includes("已挂载")) return { text: s, cls: "bg-green-100 text-green-700 dark:bg-green-900/40 dark:text-green-400" };
  if (s.includes("未挂载")) return { text: s, cls: "bg-red-100 text-red-700 dark:bg-red-900/40 dark:text-red-400" };
  if (s.includes("检测关闭")) return { text: s, cls: "bg-gray-100 text-gray-600 dark:bg-gray-700 dark:text-gray-400" };
  return { text: s, cls: "bg-orange-100 text-orange-700 dark:bg-orange-900/40 dark:text-orange-400" };
}

const mountIntervalOpts: { value: MountCheckIntervalOption; label: string }[] = [
  { value: "3", label: "每3分钟" },
  { value: "5", label: "每5分钟" },
  { value: "10", label: "每10分钟" },
];
</script>
<template>
  <SectionCard title="监听设置">
    <!-- Primary path -->
    <div class="flex items-center justify-between text-sm">
      <span class="text-gray-600 dark:text-gray-300 shrink-0">监听目录</span>
      <div class="flex items-center gap-2 min-w-0">
        <span class="truncate text-gray-800 dark:text-gray-200 text-xs max-w-[200px]">{{ state.inputPath || '未选择' }}</span>
        <button class="text-xs text-blue-500 hover:underline shrink-0" @click="choosePrimary">选择…</button>
      </div>
    </div>

    <!-- Extra paths -->
    <div class="text-sm">
      <div class="flex items-center justify-between">
        <span class="text-gray-600 dark:text-gray-300">更多目录 ({{ state.extraInputPaths.length }})</span>
        <div class="flex gap-2">
          <button class="text-xs text-blue-500 hover:underline" @click="extraExpanded = !extraExpanded">
            {{ extraExpanded ? '收起' : '展开' }}
          </button>
          <button
            class="text-xs text-blue-500 hover:underline disabled:opacity-40 disabled:cursor-not-allowed"
            :disabled="state.extraInputPaths.length >= 2"
            @click="addExtra"
          >添加…</button>
        </div>
      </div>
      <div v-if="extraExpanded && state.extraInputPaths.length" class="mt-1 space-y-1">
        <div
          v-for="(p, _i) in state.extraInputPaths"
          :key="p"
          class="flex items-center justify-between bg-white dark:bg-gray-700 rounded px-2 py-1"
        >
          <span class="truncate text-xs text-gray-700 dark:text-gray-300 max-w-[220px]">{{ p }}</span>
          <button class="text-red-500 text-xs font-bold hover:text-red-700" @click="removeInputPath(p)">✕</button>
        </div>
      </div>
    </div>

    <!-- Mount status -->
    <div class="flex items-center justify-between text-sm">
      <div class="flex items-center gap-2">
        <span class="text-gray-600 dark:text-gray-300">挂载状态</span>
        <span class="text-[10px] px-1.5 py-0.5 rounded-full font-medium" :class="mountBadge(state.mountStatus).cls">
          {{ mountBadge(state.mountStatus).text }}
        </span>
      </div>
      <label class="flex items-center gap-1.5 cursor-pointer">
        <span class="text-xs text-gray-500">自动检测挂载</span>
        <input
          type="checkbox"
          class="accent-blue-500 w-4 h-4"
          :checked="state.mountMonitorEnabled"
          @change="updateSetting('mountMonitorEnabled', ($event.target as HTMLInputElement).checked)"
        />
      </label>
    </div>

    <!-- Mount interval -->
    <div class="flex items-center justify-between text-sm">
      <span class="text-gray-600 dark:text-gray-300">检测间隔</span>
      <div class="inline-flex rounded-lg overflow-hidden border border-gray-300 dark:border-gray-600">
        <button
          v-for="opt in mountIntervalOpts"
          :key="opt.value"
          class="px-2 py-0.5 text-xs transition-colors"
          :class="[
            state.mountInterval === opt.value
              ? 'bg-blue-500 text-white'
              : 'bg-white dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-600',
            !state.mountMonitorEnabled ? 'opacity-40 cursor-not-allowed' : ''
          ]"
          :disabled="!state.mountMonitorEnabled"
          @click="updateSetting('mountInterval', opt.value)"
        >{{ opt.label }}</button>
      </div>
    </div>

    <!-- Include subfolders -->
    <div class="flex items-center justify-between text-sm">
      <span class="text-gray-600 dark:text-gray-300">包含子文件夹</span>
      <input
        type="checkbox"
        class="accent-blue-500 w-4 h-4"
        :checked="state.includeSubfolders"
        @change="updateSetting('includeSubfolders', ($event.target as HTMLInputElement).checked)"
      />
    </div>
  </SectionCard>
</template>
