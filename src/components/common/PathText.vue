<script setup>
const props = defineProps({
  text: {
    type: String,
    required: true,
  },
});

const name = computed(() => {
  const parts = props.text.split(/[/\\]/);
  return `/${parts[parts.length - 1]}`;
});

const restParts = computed(() => {
  const parts = props.text.split(/[/\\]/);
  parts.pop();
  return parts.join("/");
});
</script>

<template>
  <div class="flex flex-nowrap">
    <div class="truncate">{{ restParts }}</div>
    <div class="text-nowrap whitespace-nowrap">{{ name }}</div>
  </div>
</template>
