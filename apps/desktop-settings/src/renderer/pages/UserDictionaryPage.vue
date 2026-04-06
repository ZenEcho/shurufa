<script setup lang="ts">
import { computed, onMounted, reactive, h } from 'vue'
import { storeToRefs } from 'pinia'
import type { DataTableColumns } from 'naive-ui'
import {
  NAlert,
  NButton,
  NCard,
  NDataTable,
  NForm,
  NFormItem,
  NInput,
  NInputNumber,
  NModal,
  NSpace,
  useMessage
} from 'naive-ui'
import type { UserDictionaryEntry } from '@shurufa/shared-types'
import { useDictionaryStore } from '../stores/dictionary'

const dictionaryStore = useDictionaryStore()
const message = useMessage()

const { entries, error, loading, saving } = storeToRefs(dictionaryStore)

const modalState = reactive({
  show: false,
  schemaId: 'pinyin',
  code: '',
  word: '',
  weight: 1,
  source: 'manual'
})

const openCreateModal = (): void => {
  modalState.show = true
}

const closeCreateModal = (): void => {
  modalState.show = false
}

const createEntry = async (): Promise<void> => {
  try {
    await dictionaryStore.createEntry({
      schemaId: modalState.schemaId,
      code: modalState.code,
      word: modalState.word,
      weight: Number(modalState.weight),
      source: modalState.source
    })
    closeCreateModal()
    modalState.code = ''
    modalState.word = ''
    modalState.weight = 1
    message.success('词条创建成功')
  } catch {
    message.error(error.value ?? '创建词条失败')
  }
}

const removeEntry = async (id: number): Promise<void> => {
  try {
    await dictionaryStore.deleteEntry(id)
    message.success('词条删除成功')
  } catch {
    message.error(error.value ?? '删除词条失败')
  }
}

const columns = computed<DataTableColumns<UserDictionaryEntry>>(() => [
  { title: '方案', key: 'schemaId' },
  { title: '编码', key: 'code' },
  { title: '词条', key: 'word' },
  { title: '权重', key: 'weight' },
  { title: '来源', key: 'source' },
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
  void dictionaryStore.load()
})
</script>

<template>
  <div class="space-y-6">
    <n-card embedded>
      <div class="flex flex-wrap items-center justify-between gap-4">
        <div>
          <div class="text-lg font-600 text-slate-900">用户词典</div>
          <p class="mt-2 text-sm leading-6 text-slate-500">
            这里管理的手动词条已持久化到 SQLite，并通过 Rust 服务读写。
          </p>
        </div>
        <n-space>
          <n-button secondary :disabled="loading || saving" @click="dictionaryStore.load()">重新加载</n-button>
          <n-button type="primary" @click="openCreateModal">新增词条</n-button>
        </n-space>
      </div>
    </n-card>

    <n-alert v-if="error" type="error" :bordered="false">{{ error }}</n-alert>

    <n-card embedded>
      <n-data-table :columns="columns" :data="entries" :loading="loading" />
    </n-card>

    <n-modal
      v-model:show="modalState.show"
      preset="card"
      title="新增用户词条"
      class="max-w-[560px]"
    >
      <n-form label-placement="left" label-width="120">
        <n-form-item label="方案 ID">
          <n-input v-model:value="modalState.schemaId" />
        </n-form-item>
        <n-form-item label="编码">
          <n-input v-model:value="modalState.code" />
        </n-form-item>
        <n-form-item label="词条">
          <n-input v-model:value="modalState.word" />
        </n-form-item>
        <n-form-item label="权重">
          <n-input-number v-model:value="modalState.weight" :min="1" :max="99" />
        </n-form-item>
      </n-form>

      <template #footer>
        <n-space justify="end">
          <n-button @click="closeCreateModal">取消</n-button>
          <n-button type="primary" :loading="saving" @click="createEntry">创建</n-button>
        </n-space>
      </template>
    </n-modal>
  </div>
</template>
