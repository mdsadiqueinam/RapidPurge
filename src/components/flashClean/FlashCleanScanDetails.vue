<script setup>
import { formatBytes } from "@root/utils/bytes";

const { t } = useI18n();

const { junkFound, resetState, scanData } = useFlashClean();

const categoryList = computed(() => {
  const nodes = scanData.value?.nodes ?? [];
  return nodes.map((node) => ({
    id: node.categoryId,
    label: t(node.categoryId),
    size: formatBytes(node.size),
  }));
});
</script>

<template>
  <div
    class="flex flex-col items-center justify-center h-full gap-6 overflow-auto"
  >
    <!-- Scan Complete Header -->
    <div class="text-center">
      <h2 class="text-3xl font-semibold mb-2">
        {{ t("Scan Complete") }}
      </h2>
      <p class="text-main-selected-text">
        {{ t("Review the junk files found on your system") }}
      </p>
    </div>

    <!-- Junk Found Display -->
    <div
      class="bg-sidebar/30 rounded-xl px-12 py-8 text-center flex flex-col gap-2"
    >
      <p class="text-sm text-main-selected-text">
        {{ t("Total Junk Found") }}
      </p>
      <p class="text-4xl font-bold text-primary">
        {{ junkFound }}
      </p>

      <!-- Category Breakdown -->
      <div v-if="categoryList.length" class="opacity-60">
        <ul class="list-disc">
          <li v-for="category in categoryList" :key="category.id">
            <div class="flex items-center justify-between gap-2">
              <span>{{ category.label }}</span>
              <span>{{ category.size }}</span>
            </div>
          </li>
        </ul>
      </div>
    </div>

    <!-- Action Buttons -->
    <div class="flex gap-4">
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
