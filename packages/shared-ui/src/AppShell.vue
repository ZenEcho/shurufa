<script setup lang="ts">
import type { NavSection, SettingsSectionId } from '@shurufa/shared-types'

defineProps<{
  title: string
  subtitle: string
  sections: NavSection[]
  activeSection: SettingsSectionId
}>()

defineEmits<{
  selectSection: [sectionId: SettingsSectionId]
}>()
</script>

<template>
  <div class="min-h-screen p-5 text-ink lg:p-7">
    <div class="mx-auto grid max-w-7xl gap-5 xl:grid-cols-[280px_1fr]">
      <aside class="rounded-8 border border-white/65 bg-white/84 p-5 shadow-[0_16px_50px_rgba(20,33,61,0.10)] backdrop-blur">
        <div class="space-y-2">
          <div class="text-xs uppercase tracking-[0.3em] text-slate">
            Input Method Studio
          </div>
          <h2 class="m-0 text-3xl font-700">{{ title }}</h2>
          <p class="m-0 text-sm leading-6 text-slate">{{ subtitle }}</p>
        </div>

        <nav class="mt-8 space-y-2">
          <button
            v-for="section in sections"
            :key="section.id"
            type="button"
            class="w-full rounded-5 border px-4 py-3 text-left transition"
            :class="section.id === activeSection
              ? 'border-amber bg-amber/10 text-ink shadow-[0_10px_24px_rgba(252,163,17,0.18)]'
              : 'border-transparent bg-mist/55 text-slate hover:border-white hover:bg-white'"
            @click="$emit('selectSection', section.id)"
          >
            <div class="text-sm font-600">{{ section.label }}</div>
            <div class="mt-1 text-xs leading-5 opacity-80">{{ section.description }}</div>
          </button>
        </nav>
      </aside>

      <main class="space-y-5">
        <slot name="hero" />
        <slot />
      </main>
    </div>
  </div>
</template>

