<script setup lang="ts">
import { computed, onMounted, ref } from 'vue'
import type { DashboardSnapshot, RuntimeStatus, SettingsSectionId } from '@shurufa/shared-types'
import { sectionMeta } from '@shurufa/shared-types'
import { AppShell, MetricCard, SectionPanel } from '@shurufa/shared-ui'

const dashboard = ref<DashboardSnapshot | null>(null)
const runtime = ref<RuntimeStatus | null>(null)
const activeSection = ref<SettingsSectionId>('overview')

const metrics = computed(() => dashboard.value?.metrics ?? [])
const sections = computed(() => sectionMeta)

onMounted(async () => {
  ;[dashboard.value, runtime.value] = await Promise.all([
    window.imeApi.getDashboard(),
    window.imeApi.getRuntimeStatus()
  ])
})
</script>

<template>
  <AppShell
    title="Shurufa"
    subtitle="跨平台输入法设置中心"
    :sections="sections"
    :active-section="activeSection"
    @select-section="activeSection = $event"
  >
    <template #hero>
      <div class="rounded-7 border border-white/70 bg-white/80 p-6 shadow-[0_20px_70px_rgba(20,33,61,0.08)] backdrop-blur">
        <div class="flex flex-col gap-4 lg:flex-row lg:items-end lg:justify-between">
          <div class="space-y-2">
            <p class="m-0 text-sm uppercase tracking-[0.28em] text-slate">
              System IME Control Center
            </p>
            <h1 class="m-0 text-4xl font-700 text-ink">
              把输入主链路留给 Rust，把体验交给桌面设置中心。
            </h1>
            <p class="m-0 max-w-3xl text-base leading-7 text-slate">
              当前骨架已经拆分出设置中心、核心服务、平台宿主与数据层，后续可以直接接入 Windows TSF、macOS IMK 和 Linux IBus。
            </p>
          </div>

          <div class="min-w-70 rounded-6 bg-ink px-5 py-4 text-white shadow-[0_16px_40px_rgba(20,33,61,0.25)]">
            <div class="text-xs uppercase tracking-[0.24em] text-white/70">
              Runtime
            </div>
            <div class="mt-3 text-2xl font-700">
              {{ runtime?.serviceStatus ?? 'Loading...' }}
            </div>
            <div class="mt-2 text-sm text-white/70">
              {{ runtime?.activePlatform ?? 'desktop-settings' }} · schema {{ runtime?.defaultSchema ?? 'pinyin' }}
            </div>
          </div>
        </div>
      </div>
    </template>

    <template #default>
      <section class="grid gap-4 md:grid-cols-2 xl:grid-cols-4">
        <MetricCard
          v-for="metric in metrics"
          :key="metric.label"
          :label="metric.label"
          :value="metric.value"
          :hint="metric.hint"
        />
      </section>

      <div class="grid gap-4 xl:grid-cols-[1.3fr_0.7fr]">
        <SectionPanel title="当前阶段" eyebrow="Roadmap">
          <ul class="m-0 space-y-3 pl-5 text-sm leading-7 text-slate">
            <li>Electron 设置中心已成为跨平台 UI 宿主，后续通过 IPC 对接 Rust service。</li>
            <li>Rust workspace 已拆分核心、配置、词库、数据库、日志与服务模块。</li>
            <li>平台原生宿主将分别对接 Windows TSF、macOS IMK、Linux IBus。</li>
          </ul>
        </SectionPanel>

        <SectionPanel title="下一步建议" eyebrow="Build Path">
          <div class="space-y-3 text-sm leading-7 text-slate">
            <p class="m-0">先打通 Windows TSF 到 Rust Core 的最小闭环：按键输入、候选生成、确认上屏。</p>
            <p class="m-0">随后把 SQLite 配置、用户词库和日志流接到设置中心里，形成完整开发调试回路。</p>
          </div>
        </SectionPanel>
      </div>
    </template>
  </AppShell>
</template>

