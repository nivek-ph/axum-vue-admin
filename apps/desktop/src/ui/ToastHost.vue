<template>
  <Teleport to="body">
    <div class="app-toast-host" aria-live="polite">
      <button
        v-for="toast in toastItems"
        :key="toast.id"
        type="button"
        class="app-toast"
        :class="`app-toast--${toast.type}`"
        @click="removeToast(toast.id)"
      >
        {{ toast.message }}
      </button>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { removeToast, toastItems } from './toast'
</script>
