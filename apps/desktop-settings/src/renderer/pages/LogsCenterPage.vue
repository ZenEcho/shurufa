<script setup lang="ts">
import { computed, onMounted } from 'vue'
import type { DataTableColumns } from 'naive-ui'
import { NAlert, NButton, NCard, NDataTable, NTag } from 'naive-ui'
import type { ErrorLogEntry } from '@shurufa/shared-types'
import { useLogsStore } from '../stores/logs'

const logsStore = useLogsStore()

const columns = computed<DataTableColumns<ErrorLogEntry>>(() => [
  { title: '级别', key: 'level' },
  { title: '模块', key: 'module' },
  { title: '消息', key: 'message' },
  {
    title: '上下文',
    key: 'contextJson',
    render: (row) => row.contextJson ?? '-'
  },
  {
    title: '时间',
    key: 'createdAt',
    render: (row) => row.createdAt
  }
])

onMounted(() => {
  void logsStore.load()
})
</script>

<template>
  <div class="space-y-6">
    <n-card embedded>
      <div class="flex flex-wrap items-center justify-between gap-4">
        <div>
          <div class="text-lg font-600 text-slate-900">日志中心</div>
          <p class="mt-2 text-sm leading-6 text-slate-500">
            当前页面读取的是 SQLite 中持久化的运行日志；导出和清理将在下一轮继续补齐。
          </p>
        </div>
        <n-button secondary :disabled="logsStore.loading" @click="logsStore.load()">重新加载</n-button>
      </div>
    </n-card>

    <n-alert v-if="logsStore.error" type="error" :bordered="false">{{ logsStore.error }}</n-alert>

    <n-card embedded>
      <div class="mb-4 flex items-center gap-2">
        <n-tag type="info">{{ logsStore.entries.length }} 条记录</n-tag>
      </div>
      <n-data-table :columns="columns" :data="logsStore.entries" :loading="logsStore.loading" />
    </n-card>
  </div>
</template>
