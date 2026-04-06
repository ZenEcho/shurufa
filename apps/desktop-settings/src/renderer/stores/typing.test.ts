import { createPinia, setActivePinia } from 'pinia'
import type { TypingSnapshot } from '@shurufa/shared-types'
import { beforeEach, describe, expect, it, vi } from 'vitest'
import { createImeApiMock, typingSessionFixture } from '../../test/ime-api'
import { useTypingStore } from './typing'

describe('useTypingStore', () => {
  beforeEach(() => {
    setActivePinia(createPinia())
  })

  it('initializes a session and prepares an empty snapshot', async () => {
    const api = createImeApiMock()
    const store = useTypingStore()

    await store.initialize()

    expect(api.createTypingSession).toHaveBeenCalledOnce()
    expect(store.session).toEqual(typingSessionFixture)
    expect(store.snapshot?.response.candidates.items).toEqual([])
    expect(store.eventLog).toEqual([])
    expect(store.loading).toBe(false)
  })

  it('processes a key, appends committed text, and records the event', async () => {
    const committedSnapshot: TypingSnapshot = {
      session: {
        ...typingSessionFixture,
        rawKeys: 'ni'
      },
      response: {
        consumed: true,
        commitText: '你',
        preedit: {
          compositionText: '',
          cursor: 0
        },
        candidates: {
          items: [],
          pageIndex: 0,
          hasNextPage: false
        },
        inputMode: 'Chinese'
      }
    }

    const api = createImeApiMock({
      processTypingKey: vi.fn(async () => committedSnapshot)
    })
    const store = useTypingStore()

    await store.handleKey({ kind: 'char', char: 'n' })

    expect(api.createTypingSession).toHaveBeenCalledOnce()
    expect(api.processTypingKey).toHaveBeenCalledOnce()
    expect(store.committedText).toBe('你')
    expect(store.eventLog).toHaveLength(1)
    expect(store.eventLog[0]?.label).toBe('n')
    expect(store.eventLog[0]?.commitText).toBe('你')
  })

  it('falls back to local text handling when a key is not consumed', async () => {
    const fallbackSnapshot: TypingSnapshot = {
      session: typingSessionFixture,
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
        inputMode: 'Chinese'
      }
    }

    createImeApiMock({
      processTypingKey: vi.fn(async () => fallbackSnapshot)
    })
    const store = useTypingStore()

    await store.handleKey({ kind: 'char', char: 'a' })
    await store.handleKey({ kind: 'space' })
    await store.handleKey({ kind: 'backspace' })

    expect(store.committedText).toBe('a')
  })

  it('resets the session and clears committed text', async () => {
    const fallbackSnapshot: TypingSnapshot = {
      session: typingSessionFixture,
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
        inputMode: 'Chinese'
      }
    }

    const api = createImeApiMock({
      processTypingKey: vi.fn(async () => fallbackSnapshot)
    })
    const store = useTypingStore()

    await store.handleKey({ kind: 'char', char: 'a' })
    expect(store.committedText).not.toBe('')

    await store.reset()

    expect(api.createTypingSession).toHaveBeenCalledTimes(2)
    expect(store.committedText).toBe('')
  })

  it('stores the error message when processing fails', async () => {
    createImeApiMock({
      processTypingKey: vi.fn(async () => {
        throw new Error('typing failed')
      })
    })
    const store = useTypingStore()

    await expect(store.handleKey({ kind: 'enter' })).rejects.toThrow('typing failed')

    expect(store.error).toBe('typing failed')
    expect(store.processing).toBe(false)
  })

  it('sends plain cloneable payloads to the preload api', async () => {
    const fallbackSnapshot: TypingSnapshot = {
      session: typingSessionFixture,
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
        inputMode: 'Chinese'
      }
    }

    const api = createImeApiMock({
      processTypingKey: vi.fn(async (session, key) => {
        structuredClone(session)
        structuredClone(key)

        return fallbackSnapshot
      })
    })
    const store = useTypingStore()

    await store.initialize()
    await store.handleKey({ kind: 'char', char: 'a' })

    expect(api.processTypingKey).toHaveBeenCalledOnce()
  })

  it('surfaces a clear error when the preload api is unavailable', async () => {
    Object.defineProperty(window, 'imeApi', {
      configurable: true,
      writable: true,
      value: undefined
    })
    const store = useTypingStore()

    await expect(store.initialize()).rejects.toThrow('IME API unavailable')

    expect(store.error).toContain('IME API unavailable')
    expect(store.loading).toBe(false)
  })
})
