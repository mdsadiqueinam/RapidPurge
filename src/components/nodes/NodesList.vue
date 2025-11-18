<script setup>
const props = defineProps({
  nodes: {
    type: Array,
    required: true,
    default: () => [],
  },
});

const sortedNodes = computed(() => {
  return props.nodes.slice().sort((a, b) => b.size - a.size);
});
</script>

<template>
  <div class="space-y-2 overflow-auto">
    <NodeItem v-for="node in sortedNodes" :key="node.path" :node="node" />
  </div>
</template>

<style scoped></style>
