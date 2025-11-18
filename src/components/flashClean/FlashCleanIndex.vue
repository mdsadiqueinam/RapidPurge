<script setup>
const { finished } = useFlashClean();
</script>

<template>
  <FlashCleanHome v-if="!finished" />
  <FlashCleanScanDetails v-if="finished" />
</template>
