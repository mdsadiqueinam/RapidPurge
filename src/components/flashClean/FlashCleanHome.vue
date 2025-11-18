<script setup>
import { ArrowPathIcon } from "@heroicons/vue/24/outline";

const { t } = useI18n();

const { startScan, currentPathStr, junkFound, scanning, finished } =
  useFlashClean();

const scanningText = computed(() => `${t("Scanning")}...`);
</script>

<template>
  <div class="flex flex-col justify-center items-center h-full">
    <h1 class="text-4xl font-semibold">{{ t("Flash Clean") }}</h1>
    <p class="my-2">
      {{ t("Find and clean junk files quickly and efficiently.") }}
    </p>

    <!-- Show Scan button when not scanning and not finished -->
    <div v-if="!scanning && !finished" class="my-8">
      <BaseButton size="2xl" round class="px-24" @click="startScan">
        {{ t("Scan") }}
      </BaseButton>
    </div>

    <!-- Show PathText and junkFound during scanning -->
    <template v-if="scanning">
      <!-- Loader -->
      <div class="my-8 flex flex-col items-center gap-6">
        <ArrowPathIcon class="animate-spin h-16 w-16 text-primary" />
        <p class="text-lg font-medium">{{ scanningText }}</p>
      </div>

      <PathText
        :text="currentPathStr"
        class="mb-4 max-w-[300px] text-sm text-sidebar-selected"
      />

      <div class="text-xl font-medium">
        {{ junkFound }}
      </div>
    </template>

    <!-- Show scan details when finished -->
    <FlashCleanScanDetails v-if="finished" />
  </div>
</template>
