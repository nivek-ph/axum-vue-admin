<template>
  <UApp>
    <RouterView />
    <ToastHost />
  </UApp>
</template>

<script setup lang="ts">
import ToastHost from './ui/ToastHost.vue'
</script>
