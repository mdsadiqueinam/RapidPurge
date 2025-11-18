<script setup>
const { t } = useI18n();
</script>

<template>
  <div
    class="min-h-screen min-w-screen bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100"
  >
    <main class="p-4">
      {{ t("APP_TITLE") }}
    </main>
  </div>
</template>
