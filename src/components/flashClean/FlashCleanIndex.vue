<script setup>
const { finished } = useFlashClean();
</script>

<template>
  <FlashCleanHome v-if="!finished" />
  <FlashCleanScanPreview v-if="finished" />
</template>
