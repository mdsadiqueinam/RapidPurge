<script setup>
import NavItem from "./NavItem.vue";

const { t } = useI18n();

const navigationItems = [
  {
    label: "Flash Clean",
    route: "/flash-clean",
    icon: "⚡",
  },
];
</script>

<template>
  <aside class="w-64 h-screen flex flex-col bg-sidebar border-r border-divider">
    <div class="p-6">
      <h1 class="text-2xl font-bold text-primary">{{ t("APP_TITLE") }}</h1>
    </div>

    <nav class="flex-1 px-3 py-4">
      <NavItem
        v-for="item in navigationItems"
        :key="item.route"
        :label="item.label"
        :route="item.route"
        :icon="item.icon"
      />
    </nav>
  </aside>
</template>
