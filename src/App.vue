<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useAppState } from "./composables/useAppState";
import HeaderSection from "./components/HeaderSection.vue";
import MonitorSection from "./components/MonitorSection.vue";
import ScanSection from "./components/ScanSection.vue";
import CompressSection from "./components/CompressSection.vue";
import OutputSection from "./components/OutputSection.vue";
import RuntimeSection from "./components/RuntimeSection.vue";
import LogSection from "./components/LogSection.vue";
import HelpDialog from "./components/HelpDialog.vue";
import DonateDialog from "./components/DonateDialog.vue";

const { init } = useAppState();
const showHelp = ref(false);
const showDonate = ref(false);

onMounted(() => {
  init();
});
</script>
<template>
  <div class="min-h-screen bg-white dark:bg-gray-900 text-gray-900 dark:text-gray-100 p-4 space-y-3 max-w-lg mx-auto select-none">
    <HeaderSection @open-help="showHelp = true" />
    <MonitorSection />
    <ScanSection />
    <CompressSection />
    <OutputSection />
    <RuntimeSection />
    <LogSection @open-donate="showDonate = true" />
    <HelpDialog v-if="showHelp" @close="showHelp = false" />
    <DonateDialog v-if="showDonate" @close="showDonate = false" />
  </div>
</template>
