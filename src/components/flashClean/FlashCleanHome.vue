<script setup>
const { t } = useI18n();

const { startScan, currentPathStr, junkFound } = useFlashClean();
</script>

<template>
  <div class="flex flex-col justify-center items-center h-full">
    <h1 class="text-4xl font-semibold">{{ t("Flash Clean") }}</h1>
    <p class="my-2">
      {{ t("Find and clean junk files quickly and efficiently.") }}
    </p>

    <div class="my-8">
      <BaseButton size="2xl" round class="px-24" @click="startScan">
        {{ t("Scan") }}
      </BaseButton>
    </div>

    {{ currentPathStr }}
    {{ junkFound }}
  </div>
</template>
