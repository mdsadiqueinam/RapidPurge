<script setup>
defineProps({
  label: {
    type: String,
    required: true,
  },
  route: {
    type: String,
    required: true,
  },
  icon: {
    type: String,
    default: "",
  },
});
</script>

<template>
  <router-link
    :to="route"
    class="flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-sidebar-text hover:bg-sidebar-hover hover:text-sidebar-text-hover [&.router-link-active]:bg-sidebar-selected [&.router-link-active]:text-sidebar-text-selected"
  >
    <span v-if="icon" class="text-xl">{{ icon }}</span>
    <span class="font-medium">{{ label }}</span>
  </router-link>
</template>
