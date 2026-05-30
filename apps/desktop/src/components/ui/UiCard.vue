<template>
  <section v-bind="$attrs" class="rounded-2xl border border-stone-200 bg-white shadow-sm">
    <slot />
  </section>
</template>

<script setup lang="ts">
defineOptions({ inheritAttrs: false })

defineProps<{
  shadow?: string
}>()
</script>
