<script setup lang="ts">
import { ref, watch } from "vue";
import SectionCard from "./SectionCard.vue";
import { useAppState } from "../composables/useAppState";
import type { TimeGateOption, ScanIntervalOption } from "../types";

const { state, updateSetting, applyKeywords } = useAppState();

const draftKeyword = ref("");
const extraKeywords = ref<string[]>([...state.value.keywords.slice(1)]);
const kwExpanded = ref(false);

// Sync extraKeywords when state.keywords changes from backend
watch(() => state.value.keywords, (newKws) => {
  extraKeywords.value = [...newKws.slice(1)];
});

function confirmPrimary() {
  if (!draftKeyword.value.trim()) return;
  const all = [draftKeyword.value.trim(), ...state.value.keywords.slice(1)];
  applyKeywords(all);
  draftKeyword.value = "";
}

function addExtraKeyword() {
  extraKeywords.value.push("");
}

function confirmAll() {
  const primary = state.value.keywords[0] || "";
  const rest = extraKeywords.value.filter((k) => k.trim());
  applyKeywords([primary, ...rest]);
}

function removeExtraKw(i: number) {
  extraKeywords.value.splice(i, 1);
  confirmAll();
}

const timeGateOpts: { value: TimeGateOption; label: string }[] = [
  { value: "last24h", label: "最近24小时" },
  { value: "last7d", label: "最近7天" },
  { value: "customDate", label: "指定日期之后" },
];

const scanIntervalOpts: { value: ScanIntervalOption; label: string }[] = [
  { value: "1", label: "每1分钟" },
  { value: "5", label: "每5分钟" },
  { value: "15", label: "每15分钟" },
  { value: "60", label: "每60分钟" },
];
</script>
<template>
  <SectionCard title="扫描规则">
    <!-- Primary keyword -->
    <div class="flex items-center justify-between text-sm">
      <span class="text-gray-600 dark:text-gray-300 shrink-0">关键词</span>
      <div class="flex items-center gap-1">
        <input
          v-model="draftKeyword"
          type="text"
          placeholder="带字幕"
          class="text-xs border border-gray-300 dark:border-gray-600 rounded px-2 py-0.5 w-24 bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-200"
          @keyup.enter="confirmPrimary"
        />
        <button class="text-xs text-blue-500 hover:underline" @click="confirmPrimary">确认</button>
      </div>
    </div>

    <!-- Extra keywords -->
    <div class="text-sm">
      <div class="flex items-center justify-between">
        <span class="text-gray-600 dark:text-gray-300">更多关键词 ({{ extraKeywords.length }})</span>
        <div class="flex gap-2">
          <button class="text-xs text-blue-500 hover:underline" @click="kwExpanded = !kwExpanded">
            {{ kwExpanded ? '收起' : '展开' }}
          </button>
          <button
            class="text-xs text-blue-500 hover:underline disabled:opacity-40 disabled:cursor-not-allowed"
            :disabled="extraKeywords.length >= 2"
            @click="addExtraKeyword"
          >添加…</button>
        </div>
      </div>
      <div v-if="kwExpanded && extraKeywords.length > 0" class="mt-1 space-y-1">
        <div
          v-for="(_kw, i) in extraKeywords"
          :key="i"
          class="flex items-center gap-1 bg-white dark:bg-gray-700 rounded px-2 py-1"
        >
          <input
            v-model="extraKeywords[i]"
            class="text-xs flex-1 bg-transparent border-none outline-none text-gray-700 dark:text-gray-200"
          />
          <button class="text-red-500 text-xs font-bold hover:text-red-700" @click="removeExtraKw(i)">✕</button>
        </div>
        <button class="text-xs text-blue-500 hover:underline" @click="confirmAll">确认</button>
      </div>
    </div>

    <!-- Time gate -->
    <div class="flex items-center justify-between text-sm">
      <span class="text-gray-600 dark:text-gray-300">时间门槛</span>
      <div class="inline-flex rounded-lg overflow-hidden border border-gray-300 dark:border-gray-600">
        <button
          v-for="opt in timeGateOpts"
          :key="opt.value"
          class="px-2 py-0.5 text-xs transition-colors"
          :class="state.timeGate === opt.value
            ? 'bg-blue-500 text-white'
            : 'bg-white dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-600'"
          @click="updateSetting('timeGate', opt.value)"
        >{{ opt.label }}</button>
      </div>
    </div>
    <div v-if="state.timeGate === 'customDate'" class="flex items-center justify-end">
      <input
        type="date"
        :value="state.customDate || ''"
        class="text-xs border border-gray-300 dark:border-gray-600 rounded px-2 py-0.5 bg-white dark:bg-gray-700 text-gray-800 dark:text-gray-200"
        @change="updateSetting('customDate', ($event.target as HTMLInputElement).value)"
      />
    </div>

    <!-- Scan interval -->
    <div class="flex items-center justify-between text-sm">
      <span class="text-gray-600 dark:text-gray-300">扫描频率</span>
      <div class="inline-flex rounded-lg overflow-hidden border border-gray-300 dark:border-gray-600">
        <button
          v-for="opt in scanIntervalOpts"
          :key="opt.value"
          class="px-2 py-0.5 text-xs transition-colors"
          :class="state.scanInterval === opt.value
            ? 'bg-blue-500 text-white'
            : 'bg-white dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-600'"
          @click="updateSetting('scanInterval', opt.value)"
        >{{ opt.label }}</button>
      </div>
    </div>

    <!-- Immediate compress -->
    <div class="flex items-center justify-between text-sm">
      <span class="text-gray-600 dark:text-gray-300">检测到新文件立即压缩</span>
      <input
        type="checkbox"
        class="accent-blue-500 w-4 h-4"
        :checked="state.immediateCompress"
        @change="updateSetting('immediateCompress', ($event.target as HTMLInputElement).checked)"
      />
    </div>
  </SectionCard>
</template>
