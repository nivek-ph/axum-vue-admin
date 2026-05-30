<template>
  <div class="app-root">
    <RouterView />
    <ToastHost />
  </div>
</template>

<script setup lang="ts">
import ToastHost from './ui/ToastHost.vue'
</script>
