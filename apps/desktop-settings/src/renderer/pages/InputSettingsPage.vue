<script setup lang="ts">
import { computed } from 'vue'
import { storeToRefs } from 'pinia'
import {
  NAlert,
  NButton,
  NCard,
  NForm,
  NFormItem,
  NInputNumber,
  NSelect,
  NSpace,
  NSwitch,
  NTag,
  useMessage
} from 'naive-ui'
import { useConfigStore } from '../stores/config'
import { useRuntimeStore } from '../stores/runtime'

const configStore = useConfigStore()
const runtimeStore = useRuntimeStore()
const message = useMessage()

const { error, inputConfig, isDirty, lastSavedAt, loading, saving } = storeToRefs(configStore)

const schemaOptions = [
  { label: '拼音', value: 'pinyin' },
  { label: '五笔', value: 'wubi' }
]

const saveSettings = async (): Promise<void> => {
  try {
    const runtime = await configStore.save()

    if (runtime) {
      runtimeStore.setRuntime(runtime)
    }

    await runtimeStore.load()
    message.success('输入设置已保存到 Rust 服务与 SQLite')
  } catch {
    message.error(error.value ?? '保存输入设置失败')
  }
}

const resetSettings = async (): Promise<void> => {
  try {
    await configStore.resetToDefaults()
    await runtimeStore.load()
    message.success('已恢复默认设置')
  } catch {
    message.error(error.value ?? '重置设置失败')
  }
}

const savedAtLabel = computed(() =>
  lastSavedAt.value ? new Date(lastSavedAt.value).toLocaleString() : '本次会话尚未保存'
)
</script>

<template>
  <div class="space-y-6">
    <n-card embedded>
      <div class="flex flex-wrap items-center justify-between gap-3">
        <div>
          <div class="text-lg font-600 text-slate-900">输入设置</div>
          <p class="mt-2 text-sm leading-6 text-slate-500">
            该页面会通过 Electron 写入 Rust 服务，并将结果持久化到 SQLite。
          </p>
        </div>

        <div class="flex items-center gap-2">
          <n-tag :type="isDirty ? 'warning' : 'success'">{{ isDirty ? '未保存' : '已同步' }}</n-tag>
          <span class="text-sm text-slate-500">{{ savedAtLabel }}</span>
        </div>
      </div>
    </n-card>

    <n-alert v-if="error" type="error" :bordered="false">
      {{ error }}
    </n-alert>

    <n-card title="输入行为" embedded>
      <n-form label-placement="left" label-width="220">
        <n-form-item label="默认输入方案">
          <n-select
            :value="inputConfig?.defaultSchema ?? 'pinyin'"
            :options="schemaOptions"
            :disabled="loading || saving"
            @update:value="configStore.patchInputConfig({ defaultSchema: String($event) })"
          />
        </n-form-item>

        <n-form-item label="默认英文模式">
          <n-switch
            :value="inputConfig?.englishModeByDefault ?? false"
            :disabled="loading || saving"
            @update:value="configStore.patchInputConfig({ englishModeByDefault: $event })"
          />
        </n-form-item>

        <n-form-item label="候选每页数量">
          <n-input-number
            :value="inputConfig?.candidatePageSize ?? 9"
            :min="5"
            :max="9"
            :disabled="loading || saving"
            @update:value="configStore.patchInputConfig({ candidatePageSize: Number($event ?? 9) })"
          />
        </n-form-item>
      </n-form>
    </n-card>

    <n-space justify="end">
      <n-button secondary :disabled="loading || saving" @click="configStore.load()">
        重新加载
      </n-button>
      <n-button tertiary :disabled="loading || saving" @click="resetSettings">
        恢复默认值
      </n-button>
      <n-button type="primary" :loading="saving" :disabled="loading || !isDirty" @click="saveSettings">
        保存修改
      </n-button>
    </n-space>
  </div>
</template>
