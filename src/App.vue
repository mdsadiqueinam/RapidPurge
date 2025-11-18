<script setup>
const { t } = useI18n();
</script>

<template>
  <div
    class="min-h-screen bg-white dark:bg-gray-800 text-gray-900 dark:text-gray-100"
  >
    <header class="p-4 bg-blue-600 text-white">
      <h1 class="text-2xl font-bold">{{ t("APP_TITLE") }}</h1>
    </header>
    <main class="p-4">
      <!-- Main content goes here -->
    </main>
  </div>
</template>
