<script setup>
import { ref } from "vue";
import { invoke, Channel } from "@tauri-apps/api/core";
import vueSvg from "@svgs/vue.svg";

const data = ref({});
const greetMsg = computed(() => data.value.lastPathStr);
const size = computed(() => formatBytes(data.value.junkFound));
const pathInfos = ref([]);
const name = ref("");
const event = new Channel();

event.onmessage = (msg) => {
  if (msg.event === "progress") {
    data.value = msg.data;
  } else if (msg.event === "finished") {
    pathInfos.value = msg.data.rootNodes;
  }
  // console.log("Received message from Rust:", msg);
};

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  invoke("flash_scan", { event });
}

function formatBytes(bytes) {
  if (!bytes) return "0 Bytes";
  const k = 1024;
  const sizes = ["Bytes", "KB", "MB", "GB", "TB"];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
}
</script>

<template>
  <main class="flex flex-col items-center justify-center p-4 w-full">
    <h1>Welcome to Tauri + Vue</h1>

    <div class="flex items-center justify-center space-x-4 my-4">
      <a href="https://vite.dev" target="_blank">
        <img src="/vite.svg" class="size-24" alt="Vite logo" />
      </a>
      <a href="https://tauri.app" target="_blank">
        <img src="/tauri.svg" class="size-24" alt="Tauri logo" />
      </a>
      <a href="https://vuejs.org/" target="_blank">
        <vueSvg class="size-24" />
      </a>
    </div>
    <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>

    <form class="row" @submit.prevent="greet">
      <input id="greet-input" v-model="name" placeholder="Enter a name..." />
      <button type="submit">Greet</button>
    </form>
    <p>{{ greetMsg }}</p>
    <p>{{ size }}</p>

    <NodesList :nodes="pathInfos" />
  </main>
</template>
