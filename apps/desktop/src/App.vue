<template>
  <div class="app-root">
    <RouterView />
    <ConfirmHost />
    <ToastHost />
  </div>
</template>

<script setup lang="ts">
import ConfirmHost from './ui/ConfirmHost.vue'
import ToastHost from './ui/ToastHost.vue'
</script>
