<script setup>
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
        <svg
          class="animate-spin h-16 w-16 text-primary"
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
        >
          <circle
            class="opacity-25"
            cx="12"
            cy="12"
            r="10"
            stroke="currentColor"
            stroke-width="4"
          ></circle>
          <path
            class="opacity-75"
            fill="currentColor"
            d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
          ></path>
        </svg>
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
