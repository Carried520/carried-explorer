<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const diskInfo = ref("");
setInterval(async () => {
 diskInfo.value = await invoke("get_drive_info");
} , 1000);
</script>

<template>
  <p>{{ diskInfo }}</p>
</template>
