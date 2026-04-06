<script setup lang="ts">
import { computed, onMounted, shallowRef } from 'vue'
import {
  NAlert,
  NConfigProvider,
  NDialogProvider,
  NGlobalStyle,
  NMessageProvider,
  NNotificationProvider,
  NSpin
} from 'naive-ui'
import { sectionMeta, type SettingsSectionId } from '@shurufa/shared-types'
import AppShell from './layouts/AppShell.vue'
import DashboardPage from './pages/DashboardPage.vue'
import HotkeysPage from './pages/HotkeysPage.vue'
import InputHistoryPage from './pages/InputHistoryPage.vue'
import InputSettingsPage from './pages/InputSettingsPage.vue'
import LogsCenterPage from './pages/LogsCenterPage.vue'
import PlaceholderPage from './pages/PlaceholderPage.vue'
import TypingTestPage from './pages/TypingTestPage.vue'
import UserDictionaryPage from './pages/UserDictionaryPage.vue'
import { useConfigStore } from './stores/config'
import { useRuntimeStore } from './stores/runtime'

const activeSection = shallowRef<SettingsSectionId>('overview')
const bootstrapError = shallowRef<string | null>(null)

const configStore = useConfigStore()
const runtimeStore = useRuntimeStore()

const isBootstrapping = computed(() => configStore.loading || runtimeStore.loading)

const currentPlaceholder = computed(() => {
  const section = sectionMeta.find((item) => item.id === activeSection.value)

  return {
    title: section?.label ?? '页面',
    description: section?.description ?? '页面正在建设中'
  }
})

onMounted(async () => {
  try {
    await Promise.all([configStore.load(), runtimeStore.load()])
  } catch (error) {
    bootstrapError.value = error instanceof Error ? error.message : '桌面设置中心初始化失败'
  }
})
</script>

<template>
  <n-config-provider>
    <n-dialog-provider>
      <n-notification-provider>
        <n-message-provider>
          <n-global-style />
          <AppShell
            title="书入法"
            subtitle="基于 Rust 运行时的 Windows 优先输入法设置中心"
            :sections="sectionMeta"
            :active-section="activeSection"
            :runtime="runtimeStore.runtime"
            @select-section="activeSection = $event"
          >
            <div class="space-y-6">
              <n-alert v-if="bootstrapError" type="error" :bordered="false">
                {{ bootstrapError }}
              </n-alert>

              <div v-if="isBootstrapping" class="flex min-h-[320px] items-center justify-center">
                <n-spin size="large" description="正在从 Rust 服务加载设置..." />
              </div>

              <DashboardPage v-else-if="activeSection === 'overview'" />
              <TypingTestPage v-else-if="activeSection === 'typing'" />
              <InputSettingsPage v-else-if="activeSection === 'input'" />
              <UserDictionaryPage v-else-if="activeSection === 'dictionary'" />
              <InputHistoryPage v-else-if="activeSection === 'history'" />
              <HotkeysPage v-else-if="activeSection === 'hotkeys'" />
              <LogsCenterPage v-else-if="activeSection === 'logs'" />
              <PlaceholderPage
                v-else
                :title="currentPlaceholder.title"
                :description="`${currentPlaceholder.description}，该模块已排入下一轮开发。`"
              />
            </div>
          </AppShell>
        </n-message-provider>
      </n-notification-provider>
    </n-dialog-provider>
  </n-config-provider>
</template>
