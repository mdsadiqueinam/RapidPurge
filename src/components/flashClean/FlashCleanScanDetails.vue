<script setup>
const { t } = useI18n();

const { junkFound, resetState } = useFlashClean();
</script>

<template>
  <div class="flex flex-col items-center justify-center h-full gap-6 my-8">
    <!-- Scan Complete Header -->
    <div class="text-center">
      <h2 class="text-3xl font-semibold mb-2">
        {{ t("Scan Complete") }}
      </h2>
      <p class="text-sidebar-selected">
        {{ t("Review the junk files found on your system") }}
      </p>
    </div>

    <!-- Junk Found Display -->
    <div class="bg-sidebar/30 rounded-xl px-12 py-8 text-center">
      <p class="text-sm text-sidebar-selected mb-2">
        {{ t("Total Junk Found") }}
      </p>
      <p class="text-4xl font-bold text-primary">
        {{ junkFound }}
      </p>
    </div>

    <!-- Action Buttons -->
    <div class="flex gap-4 mt-4">
      <BaseButton
        size="lg"
        variant="secondary"
        round
        class="px-8"
        @click="resetState"
      >
        {{ t("Scan Again") }}
      </BaseButton>
      <BaseButton size="lg" round class="px-8">
        {{ t("Clean Now") }}
      </BaseButton>
    </div>
  </div>
</template>
