<script setup lang="ts">
import { computed, onMounted, reactive, h } from 'vue'
import type { DataTableColumns } from 'naive-ui'
import {
  NAlert,
  NButton,
  NCard,
  NDataTable,
  NForm,
  NFormItem,
  NInput,
  NModal,
  NSpace,
  NSwitch,
  useMessage
} from 'naive-ui'
import type { HotkeyEntry } from '@shurufa/shared-types'
import { useHotkeysStore } from '../stores/hotkeys'

const hotkeysStore = useHotkeysStore()
const message = useMessage()

const modalState = reactive<HotkeyEntry & { show: boolean }>({
  show: false,
  id: '',
  action: '',
  accelerator: '',
  scope: 'global',
  enabled: true,
  updatedAt: 0
})

const openCreateModal = (): void => {
  modalState.show = true
  modalState.id = ''
  modalState.action = ''
  modalState.accelerator = ''
  modalState.scope = 'global'
  modalState.enabled = true
  modalState.updatedAt = 0
}

const saveEntry = async (): Promise<void> => {
  try {
    await hotkeysStore.saveEntry({
      id: modalState.id,
      action: modalState.action,
      accelerator: modalState.accelerator,
      scope: modalState.scope,
      enabled: modalState.enabled,
      updatedAt: modalState.updatedAt
    })
    modalState.show = false
    message.success('快捷键已保存')
  } catch {
    message.error(hotkeysStore.error ?? '保存快捷键失败')
  }
}

const removeEntry = async (id: string): Promise<void> => {
  try {
    await hotkeysStore.deleteEntry(id)
    message.success('快捷键已删除')
  } catch {
    message.error(hotkeysStore.error ?? '删除快捷键失败')
  }
}

const columns = computed<DataTableColumns<HotkeyEntry>>(() => [
  { title: 'ID', key: 'id' },
  { title: '动作', key: 'action' },
  { title: '按键组合', key: 'accelerator' },
  { title: '作用域', key: 'scope' },
  { title: '启用', key: 'enabled' },
  {
    title: '操作',
    key: 'actions',
    render: (row) =>
      h(
        NButton,
        {
          size: 'small',
          tertiary: true,
          type: 'error',
          onClick: () => {
            void removeEntry(row.id)
          }
        },
        { default: () => '删除' }
      )
  }
])

onMounted(() => {
  void hotkeysStore.load()
})
</script>

<template>
  <div class="space-y-6">
    <n-card embedded>
      <div class="flex flex-wrap items-center justify-between gap-4">
        <div>
          <div class="text-lg font-600 text-slate-900">快捷键</div>
          <p class="mt-2 text-sm leading-6 text-slate-500">
            快捷键数据已存入 SQLite，并由 Rust 运行时提供。冲突检测会在下一轮继续补齐。
          </p>
        </div>
        <n-space>
          <n-button secondary :disabled="hotkeysStore.loading || hotkeysStore.saving" @click="hotkeysStore.load()">
            重新加载
          </n-button>
          <n-button type="primary" @click="openCreateModal">新增快捷键</n-button>
        </n-space>
      </div>
    </n-card>

    <n-alert v-if="hotkeysStore.error" type="error" :bordered="false">{{ hotkeysStore.error }}</n-alert>

    <n-card embedded>
      <n-data-table :columns="columns" :data="hotkeysStore.entries" :loading="hotkeysStore.loading" />
    </n-card>

    <n-modal v-model:show="modalState.show" preset="card" title="保存快捷键" class="max-w-[560px]">
      <n-form label-placement="left" label-width="120">
        <n-form-item label="ID">
          <n-input v-model:value="modalState.id" />
        </n-form-item>
        <n-form-item label="动作">
          <n-input v-model:value="modalState.action" />
        </n-form-item>
        <n-form-item label="按键组合">
          <n-input v-model:value="modalState.accelerator" />
        </n-form-item>
        <n-form-item label="作用域">
          <n-input v-model:value="modalState.scope" />
        </n-form-item>
        <n-form-item label="启用">
          <n-switch v-model:value="modalState.enabled" />
        </n-form-item>
      </n-form>

      <template #footer>
        <n-space justify="end">
          <n-button @click="modalState.show = false">取消</n-button>
          <n-button type="primary" :loading="hotkeysStore.saving" @click="saveEntry">保存</n-button>
        </n-space>
      </template>
    </n-modal>
  </div>
</template>
