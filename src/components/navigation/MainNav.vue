<script setup>
import NavItem from "./NavItem.vue";
import { BoltIcon } from "@heroicons/vue/24/outline";

const { t } = useI18n();

const navigationItems = [
  {
    label: "Flash Clean",
    route: "/flash-clean",
    icon: BoltIcon,
    iconClass: "text-amber-300",
  },
];
</script>

<template>
  <aside class="w-64 h-screen flex flex-col bg-sidebar border-r border-divider">
    <div class="p-6">
      <h1 class="text-2xl font-bold text-primary">{{ t("APP_TITLE") }}</h1>
    </div>

    <nav class="flex-1 px-3 py-4">
      <NavItem v-for="item in navigationItems" :key="item.route" :item="item" />
    </nav>

    <div class="p-4 border-t border-divider">
      <ThemeToggleButton />
    </div>
  </aside>
</template>
