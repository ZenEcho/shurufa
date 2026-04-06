<script setup lang="ts">
import { computed } from 'vue'
import { NAlert, NCard, NEmpty, NGrid, NGridItem, NStatistic, NTag } from 'naive-ui'
import { useRuntimeStore } from '../stores/runtime'

const runtimeStore = useRuntimeStore()

const metrics = computed(() => runtimeStore.dashboard?.metrics ?? [])
const runtime = computed(() => runtimeStore.runtime)
</script>

<template>
  <div class="space-y-6">
    <n-alert type="info" show-icon :bordered="false">
      总览页现在读取的是 Rust 服务真实数据，不再是静态示例数据。
    </n-alert>

    <n-grid cols="1 s:2 xl:4" responsive="screen" :x-gap="16" :y-gap="16">
      <n-grid-item v-for="metric in metrics" :key="metric.label">
        <n-card embedded class="h-full">
          <n-statistic :label="metric.label" :value="metric.value" />
          <p class="mt-4 text-sm leading-6 text-slate-500">{{ metric.hint }}</p>
        </n-card>
      </n-grid-item>
    </n-grid>

    <n-card title="运行时快照" embedded>
      <div v-if="runtime" class="grid gap-4 lg:grid-cols-3">
        <div class="rounded-4 bg-slate-50 p-4">
          <div class="text-xs uppercase tracking-[0.24em] text-slate-500">平台</div>
          <div class="mt-3 text-lg font-600 text-slate-900">{{ runtime.activePlatform }}</div>
        </div>
        <div class="rounded-4 bg-slate-50 p-4">
          <div class="text-xs uppercase tracking-[0.24em] text-slate-500">服务</div>
          <div class="mt-3">
            <n-tag type="success">{{ runtime.serviceStatus }}</n-tag>
          </div>
        </div>
        <div class="rounded-4 bg-slate-50 p-4">
          <div class="text-xs uppercase tracking-[0.24em] text-slate-500">默认方案</div>
          <div class="mt-3 text-lg font-600 text-slate-900">{{ runtime.defaultSchema }}</div>
        </div>
      </div>

      <n-empty v-else description="暂时还没有可用的运行时状态" />
    </n-card>
  </div>
</template>
