<script setup lang="ts">
defineEmits<{ (e: "close"): void }>();
</script>
<template>
  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/40" @click.self="$emit('close')">
    <div class="bg-white dark:bg-gray-800 rounded-2xl p-5 w-[340px] shadow-xl space-y-4">
      <div class="flex items-center justify-between">
        <h2 class="text-base font-bold text-gray-900 dark:text-white">☕ 请作者喝奶茶</h2>
        <button class="text-gray-400 hover:text-gray-600 text-lg" @click="$emit('close')">✕</button>
      </div>
      <p class="text-xs text-gray-600 dark:text-gray-400">如果觉得好用，可以请作者喝杯奶茶 🧋</p>
      <div class="grid grid-cols-2 gap-3">
        <div class="flex flex-col items-center gap-1">
          <div class="w-28 h-28 bg-gray-100 dark:bg-gray-700 rounded-lg flex items-center justify-center text-gray-400 text-xs">
            微信收款码
          </div>
          <span class="text-[11px] text-gray-500">微信</span>
        </div>
        <div class="flex flex-col items-center gap-1">
          <div class="w-28 h-28 bg-gray-100 dark:bg-gray-700 rounded-lg flex items-center justify-center text-gray-400 text-xs">
            支付宝收款码
          </div>
          <span class="text-[11px] text-gray-500">支付宝</span>
        </div>
      </div>
    </div>
  </div>
</template>
