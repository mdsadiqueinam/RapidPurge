<script setup>
const { t } = useI18n();
</script>

<template>
  <div class="flex flex-col justify-center items-center h-full">
    <h1 class="text-4xl font-semibold">{{ t("Flash Clean") }}</h1>
    <p class="my-2">
      {{ t("Find and clean junk files quickly and efficiently.") }}
    </p>

    <div class="my-8">
      <BaseButton size="2xl" round class="px-24">{{ t("Scan") }}</BaseButton>
    </div>
  </div>
</template>
