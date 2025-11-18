<script setup>
const { t } = useI18n();

const { startScan, currentPathStr, junkFound, scanning, finished } =
  useFlashClean();
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
      <PathText
        :text="currentPathStr"
        class="my-8 max-w-[300px] text-sm text-sidebar-selected"
      />

      <div class="my-8 text-2xl font-medium">
        {{ junkFound }}
      </div>
    </template>
  </div>
</template>
