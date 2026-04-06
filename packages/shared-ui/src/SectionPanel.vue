<script setup lang="ts">
defineProps<{
  eyebrow: string
  title: string
}>()
</script>

<template>
  <section class="rounded-6 border border-white/70 bg-white/82 p-5 shadow-[0_12px_32px_rgba(20,33,61,0.08)] backdrop-blur">
    <div class="text-xs uppercase tracking-[0.24em] text-slate">{{ eyebrow }}</div>
    <h3 class="m-0 mt-2 text-xl font-700 text-ink">{{ title }}</h3>
    <div class="mt-4">
      <slot />
    </div>
  </section>
</template>

