<template>
  <form v-bind="$attrs">
    <slot />
  </form>
</template>

<script setup lang="ts">
defineOptions({ inheritAttrs: false })

defineProps<{
  labelWidth?: string
}>()
</script>
