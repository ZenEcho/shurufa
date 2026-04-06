import { computed, ref, shallowRef } from 'vue'
import { defineStore } from 'pinia'
import type { TypingKeyInput, TypingSessionState, TypingSnapshot } from '@shurufa/shared-types'

interface TypingEventRecord {
  id: number
  label: string
  consumed: boolean
  commitText: string | null
  compositionText: string
  createdAt: string
}

const buildEmptySnapshot = (session: TypingSessionState): TypingSnapshot => ({
  session,
  response: {
    consumed: false,
    commitText: null,
    preedit: {
      compositionText: '',
      cursor: 0
    },
    candidates: {
      items: [],
      pageIndex: 0,
      hasNextPage: false
    },
    inputMode: session.inputMode
  }
})

const cloneForIpc = <T>(value: T): T => JSON.parse(JSON.stringify(value)) as T

const getImeApi = (): Window['imeApi'] => {
  if (!window.imeApi) {
    throw new Error('IME API unavailable')
  }

  return window.imeApi
}

const describeKey = (key: TypingKeyInput): string => {
  switch (key.kind) {
    case 'char':
      return key.char ?? ''
    case 'number':
      return String(key.number ?? '')
    case 'backspace':
      return 'Backspace'
    case 'enter':
      return 'Enter'
    case 'space':
      return 'Space'
    case 'escape':
      return 'Escape'
    case 'toggleInputMode':
      return '切换中英文'
  }
}

export const useTypingStore = defineStore('typing', () => {
  const session = ref<TypingSessionState | null>(null)
  const snapshot = ref<TypingSnapshot | null>(null)
  const committedText = ref('')
  const eventLog = ref<TypingEventRecord[]>([])
  const loading = shallowRef(false)
  const processing = shallowRef(false)
  const error = shallowRef<string | null>(null)

  const response = computed(() => snapshot.value?.response ?? null)

  const initialize = async (): Promise<void> => {
    loading.value = true
    error.value = null

    try {
      const nextSession = await getImeApi().createTypingSession()
      session.value = nextSession
      snapshot.value = buildEmptySnapshot(nextSession)
      eventLog.value = []
    } catch (loadError) {
      error.value = loadError instanceof Error ? loadError.message : '初始化输入测试会话失败'
      throw loadError
    } finally {
      loading.value = false
    }
  }

  const reset = async (): Promise<void> => {
    committedText.value = ''
    await initialize()
  }

  const clearCommittedText = (): void => {
    committedText.value = ''
  }

  const applyFallbackInput = (key: TypingKeyInput): void => {
    switch (key.kind) {
      case 'char':
        committedText.value += key.char ?? ''
        break
      case 'space':
        committedText.value += ' '
        break
      case 'enter':
        committedText.value += '\n'
        break
      case 'backspace':
        committedText.value = committedText.value.slice(0, -1)
        break
      default:
        break
    }
  }

  const handleKey = async (key: TypingKeyInput): Promise<TypingSnapshot> => {
    if (!session.value) {
      await initialize()
    }

    processing.value = true
    error.value = null

    try {
      const nextSnapshot = await getImeApi().processTypingKey(
        cloneForIpc(session.value as TypingSessionState),
        cloneForIpc(key)
      )
      session.value = nextSnapshot.session
      snapshot.value = nextSnapshot

      if (nextSnapshot.response.commitText) {
        committedText.value += nextSnapshot.response.commitText
      } else if (!nextSnapshot.response.consumed) {
        applyFallbackInput(key)
      }

      eventLog.value = [
        {
          id: Date.now(),
          label: describeKey(key),
          consumed: nextSnapshot.response.consumed,
          commitText: nextSnapshot.response.commitText,
          compositionText: nextSnapshot.response.preedit.compositionText,
          createdAt: new Date().toLocaleTimeString()
        },
        ...eventLog.value
      ].slice(0, 20)

      return nextSnapshot
    } catch (processError) {
      error.value = processError instanceof Error ? processError.message : '处理按键失败'
      throw processError
    } finally {
      processing.value = false
    }
  }

  return {
    session,
    snapshot,
    response,
    committedText,
    eventLog,
    loading,
    processing,
    error,
    initialize,
    reset,
    clearCommittedText,
    handleKey
  }
})
