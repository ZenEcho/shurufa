<script setup lang="ts">
import { computed, onMounted } from 'vue'
import type { DataTableColumns } from 'naive-ui'
import { NAlert, NButton, NCard, NDataTable } from 'naive-ui'
import type { InputHistoryEntry } from '@shurufa/shared-types'
import { useHistoryStore } from '../stores/history'

const historyStore = useHistoryStore()

const columns = computed<DataTableColumns<InputHistoryEntry>>(() => [
  { title: '方案', key: 'schemaId' },
  { title: '输入编码', key: 'inputCode' },
  { title: '上屏结果', key: 'committedText' },
  { title: '使用次数', key: 'usageCount' },
  { title: '最近使用', key: 'lastUsedAt' }
])

onMounted(() => {
  void historyStore.load()
})
</script>

<template>
  <div class="space-y-6">
    <n-card embedded>
      <div class="flex flex-wrap items-center justify-between gap-4">
        <div>
          <div class="text-lg font-600 text-slate-900">输入历史</div>
          <p class="mt-2 text-sm leading-6 text-slate-500">
            历史记录来自 SQLite；等运行时开始持续写入后，这里会显示真实学习数据。
          </p>
        </div>
        <n-button secondary :disabled="historyStore.loading" @click="historyStore.load()">重新加载</n-button>
      </div>
    </n-card>

    <n-alert v-if="historyStore.error" type="error" :bordered="false">{{ historyStore.error }}</n-alert>

    <n-card embedded>
      <n-data-table :columns="columns" :data="historyStore.entries" :loading="historyStore.loading" />
    </n-card>
  </div>
</template>
