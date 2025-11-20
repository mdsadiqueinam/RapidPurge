<script setup>
provideFlashClean();
const route = useRoute();

const id = computed(() => route.params.id);
</script>

<template>
  <FlashCleanIndex :id="id" />
</template>
