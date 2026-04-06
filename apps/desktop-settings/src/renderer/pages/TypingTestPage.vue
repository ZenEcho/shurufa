<script setup lang="ts">
import { computed, nextTick, onMounted, ref } from 'vue'
import {
  NAlert,
  NButton,
  NCard,
  NDivider,
  NEmpty,
  NInput,
  NSpace,
  NTag,
  useMessage
} from 'naive-ui'
import type { TypingKeyInput } from '@shurufa/shared-types'
import { useTypingStore } from '../stores/typing'

const typingStore = useTypingStore()
const message = useMessage()
const typingPad = ref<HTMLElement | null>(null)

const response = computed(() => typingStore.response)
const currentModeLabel = computed(() => (response.value?.inputMode === 'English' ? '英文' : '中文'))
const hasActiveCandidates = computed(() => (response.value?.candidates.items.length ?? 0) > 0)

const focusTypingPad = async (): Promise<void> => {
  await nextTick()
  typingPad.value?.focus()
}

const initializePad = async (): Promise<void> => {
  try {
    await typingStore.initialize()
    await focusTypingPad()
  } catch {
    message.error(typingStore.error ?? '初始化输入测试失败')
  }
}

const resetPad = async (): Promise<void> => {
  try {
    await typingStore.reset()
    await focusTypingPad()
    message.success('输入测试已重置')
  } catch {
    message.error(typingStore.error ?? '重置输入测试失败')
  }
}

const toggleInputMode = async (): Promise<void> => {
  try {
    await typingStore.handleKey({ kind: 'toggleInputMode' })
    await focusTypingPad()
  } catch {
    message.error(typingStore.error ?? '切换输入模式失败')
  }
}

const selectCandidate = async (index: number): Promise<void> => {
  try {
    await typingStore.handleKey({ kind: 'number', number: index + 1 })
    await focusTypingPad()
  } catch {
    message.error(typingStore.error ?? '选词失败')
  }
}

const normalizeKey = (event: KeyboardEvent): TypingKeyInput | null => {
  if (event.ctrlKey || event.metaKey || event.altKey) {
    return null
  }

  if (event.key === 'F2') {
    return { kind: 'toggleInputMode' }
  }

  if (event.key === 'Backspace') {
    return { kind: 'backspace' }
  }

  if (event.key === 'Enter') {
    return { kind: 'enter' }
  }

  if (event.key === ' ') {
    return { kind: 'space' }
  }

  if (event.key === 'Escape') {
    return { kind: 'escape' }
  }

  if (/^[1-9]$/.test(event.key) && hasActiveCandidates.value) {
    return { kind: 'number', number: Number(event.key) }
  }

  if (event.key.length === 1) {
    return { kind: 'char', char: event.key }
  }

  return null
}

const handleKeydown = async (event: KeyboardEvent): Promise<void> => {
  const key = normalizeKey(event)

  if (!key) {
    return
  }

  event.preventDefault()

  try {
    await typingStore.handleKey(key)
  } catch {
    message.error(typingStore.error ?? '处理按键失败')
  }
}

onMounted(() => {
  void initializePad()
})
</script>

<template>
  <div class="space-y-6">
    <n-alert type="info" :bordered="false" show-icon>
      这里是输入法 MVP 打字测试页。键盘事件会通过 Electron 调到 Rust `ime-core`，你可以直接在测试区敲字母、空格、回车、退格、数字选词；按 <code>F2</code> 可切换中英文。
    </n-alert>

    <n-card embedded>
      <div class="flex flex-wrap items-center justify-between gap-4">
        <div>
          <div class="text-lg font-600 text-slate-900">输入测试</div>
          <p class="mt-2 text-sm leading-6 text-slate-500">
            当前是项目内可直接手动验证的最小打字闭环，还没有接入系统级 TSF 注册，但已经走真实 Rust 输入状态机。
          </p>
        </div>

        <n-space>
          <n-tag type="success">当前模式：{{ currentModeLabel }}</n-tag>
          <n-tag type="info">原始编码：{{ typingStore.session?.rawKeys || '无' }}</n-tag>
          <n-button secondary @click="focusTypingPad">聚焦测试区</n-button>
          <n-button tertiary @click="toggleInputMode">切换中英文</n-button>
          <n-button tertiary @click="typingStore.clearCommittedText()">清空已上屏文本</n-button>
          <n-button type="primary" :loading="typingStore.loading || typingStore.processing" @click="resetPad">
            重置测试
          </n-button>
        </n-space>
      </div>
    </n-card>

    <n-alert v-if="typingStore.error" type="error" :bordered="false">
      {{ typingStore.error }}
    </n-alert>

    <div class="grid gap-6 xl:grid-cols-[minmax(0,2fr)_360px]">
      <n-card embedded title="打字测试区">
        <div
          ref="typingPad"
          tabindex="0"
          class="min-h-[220px] rounded-4 border border-dashed border-slate-300 bg-slate-50 p-4 outline-none transition focus:border-sky-400 focus:bg-white focus:shadow-[0_0_0_4px_rgba(14,165,233,0.12)]"
          @keydown="handleKeydown"
        >
          <div class="text-xs tracking-[0.2em] text-slate-500">点击这里后直接输入</div>
          <n-divider class="!my-3" />
          <div class="min-h-[120px] whitespace-pre-wrap break-all text-16 leading-8 text-slate-900">
            <span>{{ typingStore.committedText }}</span>
            <span
              v-if="response?.preedit.compositionText"
              class="rounded-2 bg-amber-100 px-1 text-amber-900 ring-1 ring-amber-300"
            >
              {{ response.preedit.compositionText }}
            </span>
            <span v-if="!typingStore.committedText && !response?.preedit.compositionText" class="text-slate-400">
              在这里敲键盘开始测试
            </span>
          </div>
        </div>

        <div class="mt-4">
          <div class="mb-2 text-sm font-600 text-slate-700">已上屏文本</div>
          <n-input :value="typingStore.committedText" type="textarea" :autosize="{ minRows: 4, maxRows: 8 }" readonly />
        </div>
      </n-card>

      <div class="space-y-6">
        <n-card embedded title="候选列表">
          <div v-if="response?.candidates.items.length" class="grid gap-2">
            <n-button
              v-for="(candidate, index) in response.candidates.items"
              :key="candidate.id"
              tertiary
              class="!justify-start"
              @click="selectCandidate(index)"
            >
              {{ index + 1 }}. {{ candidate.text }}
              <span v-if="candidate.annotation" class="ml-2 text-slate-400">{{ candidate.annotation }}</span>
            </n-button>
          </div>
          <n-empty v-else description="当前没有候选词" />
        </n-card>

        <n-card embedded title="当前状态">
          <div class="grid gap-3 text-sm text-slate-600">
            <div class="rounded-4 bg-slate-50 p-3">Preedit：{{ response?.preedit.compositionText || '无' }}</div>
            <div class="rounded-4 bg-slate-50 p-3">光标位置：{{ response?.preedit.cursor ?? 0 }}</div>
            <div class="rounded-4 bg-slate-50 p-3">最近提交：{{ response?.commitText || '无' }}</div>
            <div class="rounded-4 bg-slate-50 p-3">最近是否被输入法消费：{{ response?.consumed ? '是' : '否' }}</div>
          </div>
        </n-card>
      </div>
    </div>

    <n-card embedded title="按键事件记录">
      <div v-if="typingStore.eventLog.length" class="space-y-3">
        <div
          v-for="eventItem in typingStore.eventLog"
          :key="eventItem.id"
          class="flex flex-wrap items-center justify-between gap-3 rounded-4 bg-slate-50 p-3 text-sm"
        >
          <div class="flex flex-wrap items-center gap-2">
            <n-tag size="small" :type="eventItem.consumed ? 'success' : 'warning'">
              {{ eventItem.consumed ? '已消费' : '透传' }}
            </n-tag>
            <span class="font-600 text-slate-800">{{ eventItem.label }}</span>
            <span class="text-slate-500">preedit：{{ eventItem.compositionText || '无' }}</span>
            <span class="text-slate-500">commit：{{ eventItem.commitText || '无' }}</span>
          </div>
          <span class="text-slate-400">{{ eventItem.createdAt }}</span>
        </div>
      </div>
      <n-empty v-else description="还没有按键记录" />
    </n-card>
  </div>
</template>
