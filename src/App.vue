<script setup>
import MainNav from "./components/MainNav.vue";
</script>

<template>
  <div class="flex min-h-screen min-w-screen bg-main text-main-text">
    <MainNav />

    <main class="flex-1 p-6">
      <router-view />
    </main>
  </div>
</template>
