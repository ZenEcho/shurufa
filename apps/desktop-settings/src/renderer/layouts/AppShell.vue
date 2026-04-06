<script setup lang="ts">
import { computed } from 'vue'
import type { MenuOption } from 'naive-ui'
import {
  NLayout,
  NLayoutContent,
  NLayoutFooter,
  NLayoutHeader,
  NLayoutSider,
  NMenu,
  NScrollbar,
  NTag
} from 'naive-ui'
import type { NavSection, RuntimeStatus, SettingsSectionId } from '@shurufa/shared-types'

const props = defineProps<{
  title: string
  subtitle: string
  sections: NavSection[]
  activeSection: SettingsSectionId
  runtime: RuntimeStatus | null
}>()

defineEmits<{
  selectSection: [sectionId: SettingsSectionId]
}>()

const menuOptions = computed<MenuOption[]>(() =>
  props.sections.map((section) => ({
    key: section.id,
    label: section.label
  }))
)
</script>

<template>
  <n-layout has-sider class="min-h-screen bg-transparent">
    <n-layout-sider
      bordered
      collapse-mode="width"
      :collapsed-width="72"
      :width="280"
      content-class="px-4 py-6"
      class="border-r border-white/60 bg-white/86 shadow-[0_20px_60px_rgba(15,23,42,0.08)] backdrop-blur"
    >
      <div class="px-2">
        <div class="text-xs uppercase tracking-[0.28em] text-slate-500">Windows 输入法实验室</div>
        <h1 class="mt-3 text-2xl font-700 text-slate-900">{{ title }}</h1>
        <p class="mt-2 text-sm leading-6 text-slate-500">{{ subtitle }}</p>
      </div>

      <n-menu
        class="mt-6"
        :value="activeSection"
        :options="menuOptions"
        @update:value="$emit('selectSection', $event as SettingsSectionId)"
      />
    </n-layout-sider>

    <n-layout class="bg-transparent">
      <n-layout-header class="border-b border-white/55 bg-white/70 px-8 py-5 backdrop-blur">
        <div class="flex items-center justify-between gap-4">
          <div>
            <div class="text-xs uppercase tracking-[0.24em] text-slate-500">设置中心</div>
            <div class="mt-2 text-2xl font-700 text-slate-900">书入法桌面控制台</div>
          </div>

          <div class="flex flex-wrap items-center gap-2">
            <n-tag size="small" type="info">{{ runtime?.activePlatform ?? '加载中' }}</n-tag>
            <n-tag size="small" type="success">{{ runtime?.serviceStatus ?? '启动中' }}</n-tag>
            <n-tag size="small" :bordered="false">方案 {{ runtime?.defaultSchema ?? '...' }}</n-tag>
          </div>
        </div>
      </n-layout-header>

      <n-layout-content embedded content-class="p-8">
        <n-scrollbar>
          <slot />
        </n-scrollbar>
      </n-layout-content>

      <n-layout-footer class="border-t border-white/55 bg-white/70 px-8 py-4 text-sm text-slate-500 backdrop-blur">
        <div class="flex flex-wrap items-center justify-between gap-3">
          <span>Electron 设置中心界面，配置状态由 Rust 运行时驱动</span>
          <span>SQLite 持久化是当前唯一可信数据源</span>
        </div>
      </n-layout-footer>
    </n-layout>
  </n-layout>
</template>
