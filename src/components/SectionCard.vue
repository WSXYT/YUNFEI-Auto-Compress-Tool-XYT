<script setup lang="ts">
defineProps<{ title: string }>();
</script>
<template>
  <section class="rounded-xl bg-gray-50 dark:bg-gray-800/50 p-3 space-y-2">
    <h3 class="text-sm font-semibold text-gray-500 dark:text-gray-400">{{ title }}</h3>
    <slot />
  </section>
</template>
