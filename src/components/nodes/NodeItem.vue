<script setup>
import { ref } from "vue";
import NodesList from "./NodesList.vue";

defineProps({
  node: {
    type: Object,
    required: true,
    properties: {
      path: String,
      size: Number,
      children: Array,
      isFile: Boolean,
    },
  },
});

const isExpanded = ref(false);

const handleToggle = () => {
  isExpanded.value = !isExpanded.value;
};

function formatBytes(bytes) {
  if (!bytes) return "0 B";
  const k = 1024;
  const sizes = ["B", "KB", "MB", "GB", "TB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
}
</script>

<template>
  <div class="select-none">
    <div
      class="node-header flex items-center py-2 px-2 rounded text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-900"
      :class="{ 'cursor-pointer': node.children?.length }"
      @click="handleToggle"
    >
      <!-- Expand/Collapse Icon -->
      <div v-if="node.children?.length" class="shrink-0">
        <svg
          :class="{ 'rotate-90': isExpanded }"
          class="w-4 h-4 transition-transform duration-200"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M9 5l7 7-7 7"
          />
        </svg>
      </div>
      <div v-else class="w-4" />

      <!-- File/Folder Icon -->
      <div class="shrink-0 ml-2">
        <svg
          v-if="node.isFile"
          class="w-4 h-4 text-blue-500"
          fill="currentColor"
          viewBox="0 0 20 20"
        >
          <path
            d="M8 16.5a1.5 1.5 0 11-3 0 1.5 1.5 0 013 0zM15 16.5a1.5 1.5 0 11-3 0 1.5 1.5 0 013 0z"
          />
          <path
            d="M3 4a1 1 0 00-1 1v10a1 1 0 001 1h4a3 3 0 013 3H7a3 3 0 01-3-3V5a1 1 0 00-1-1zm14.854 10.854a1 1 0 001.414-1.414l-3-3a1 1 0 00-1.414 0l-3 3a1 1 0 101.414 1.414l1.793-1.793 1.793 1.793z"
          />
        </svg>
        <svg
          v-else
          class="w-4 h-4 text-yellow-500"
          fill="currentColor"
          viewBox="0 0 20 20"
        >
          <path
            d="M2 6a2 2 0 012-2h12a2 2 0 012 2v8a2 2 0 01-2 2H4a2 2 0 01-2-2V6zm4 2a1 1 0 011-1h4a1 1 0 011 1v4a1 1 0 01-1 1H7a1 1 0 01-1-1V8z"
          />
        </svg>
      </div>

      <!-- Node Name and Size -->
      <div class="flex-1 ml-3 min-w-0">
        <p
          class="text-sm font-medium text-gray-900 dark:text-gray-100 truncate"
        >
          {{ node.path }}
        </p>
      </div>

      <div class="shrink-0 ml-2">
        <span class="text-xs text-gray-500 dark:text-gray-400">
          {{ formatBytes(node.size) }}
        </span>
      </div>
    </div>

    <!-- Children List (Nested) -->
    <div
      v-if="isExpanded && node.children?.length"
      class="ml-4 mt-1 border-l-2 border-gray-300 dark:border-gray-600 pl-2"
    >
      <NodesList :nodes="node.children" />
    </div>
  </div>
</template>

<style scoped></style>
