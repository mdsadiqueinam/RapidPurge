<script setup>
defineProps({
  item: {
    type: Object,
    required: true,
  },
});
</script>

<template>
  <router-link
    :to="item.route"
    class="flex items-center gap-3 px-4 py-3 rounded-lg transition-colors text-sidebar-text hover:bg-sidebar-hover hover:text-sidebar-text-hover [&.router-link-active]:bg-sidebar-selected [&.router-link-active]:text-sidebar-text-selected"
  >
    <span v-if="item.icon" class="text-xl">
      <component :is="item.icon" class="size-8" :class="item.iconClass" />
    </span>
    <span class="font-medium">{{ item.label }}</span>
  </router-link>
</template>
