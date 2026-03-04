<script setup lang="ts">
import SectionCard from "./SectionCard.vue";
import { useAppState } from "../composables/useAppState";
import { computed } from "vue";
import type { CodecOption, QualityPreset } from "../types";

const { state, updateSetting } = useAppState();

const codecOpts: { value: CodecOption; label: string }[] = [
  { value: "h264", label: "H.264" },
  { value: "h265", label: "H.265" },
];

const presetOpts: { value: QualityPreset; label: string; mbps: string }[] = [
  { value: "spaceSaving", label: "12Mbps", mbps: "12" },
  { value: "balanced", label: "18Mbps 推荐", mbps: "18" },
  { value: "highQuality", label: "25Mbps", mbps: "25" },
];

const presetTitle = computed(() => {
  const map: Record<QualityPreset, string> = {
    spaceSaving: "省空间 (12Mbps)",
    balanced: "均衡 (18Mbps)",
    highQuality: "高画质 (25Mbps)",
  };
  return map[state.value.qualityPreset];
});

const bitrateNum = computed(() => {
  const map: Record<QualityPreset, string> = {
    spaceSaving: "12",
    balanced: "18",
    highQuality: "25",
  };
  return map[state.value.qualityPreset];
});
</script>
<template>
  <SectionCard title="压缩设置">
    <!-- Codec -->
    <div class="flex items-center justify-between text-sm">
      <span class="text-gray-600 dark:text-gray-300">编码器</span>
      <div class="inline-flex rounded-lg overflow-hidden border border-gray-300 dark:border-gray-600">
        <button
          v-for="opt in codecOpts"
          :key="opt.value"
          class="px-3 py-0.5 text-xs transition-colors"
          :class="state.codec === opt.value
            ? 'bg-blue-500 text-white'
            : 'bg-white dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-600'"
          @click="updateSetting('codec', opt.value)"
        >{{ opt.label }}</button>
      </div>
    </div>

    <!-- Quality preset -->
    <div class="flex items-center justify-between text-sm">
      <span class="text-gray-600 dark:text-gray-300">码率档位</span>
      <div class="inline-flex rounded-lg overflow-hidden border border-gray-300 dark:border-gray-600">
        <button
          v-for="opt in presetOpts"
          :key="opt.value"
          class="px-2 py-0.5 text-xs transition-colors flex items-center gap-1"
          :class="state.qualityPreset === opt.value
            ? 'bg-blue-500 text-white'
            : 'bg-white dark:bg-gray-700 text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-600'"
          @click="updateSetting('qualityPreset', opt.value)"
        >
          <span v-if="state.qualityPreset === opt.value">✓</span>
          {{ opt.label }}
        </button>
      </div>
    </div>

    <p class="text-[11px] text-gray-400">当前：{{ presetTitle }}</p>
    <p class="text-[11px] text-gray-400">自动压缩参数：保持分辨率/帧率，4K 码率 {{ bitrateNum }} Mbps</p>
  </SectionCard>
</template>
