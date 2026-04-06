<script setup lang="ts">
defineProps<{
  label: string
  value: string
  hint: string
}>()
</script>

<template>
  <article class="rounded-6 border border-white/70 bg-white/82 p-5 shadow-[0_12px_32px_rgba(20,33,61,0.08)] backdrop-blur">
    <div class="text-xs uppercase tracking-[0.24em] text-slate">{{ label }}</div>
    <div class="mt-3 text-2xl font-700 text-ink">{{ value }}</div>
    <p class="mt-3 m-0 text-sm leading-6 text-slate">{{ hint }}</p>
  </article>
</template>

