import { flushPromises } from '@vue/test-utils'
import type { TypingSnapshot } from '@shurufa/shared-types'
import { mount } from '@vue/test-utils'
import { createPinia } from 'pinia'
import TypingTestPage from './TypingTestPage.vue'
import { mountPage } from '../../test/page-test-utils'
import { typingSessionFixture } from '../../test/ime-api'
import { useTypingStore } from '../stores/typing'
import { messageApi } from '../../test/naive-ui-stub'

describe('TypingTestPage', () => {
  it('initializes the typing pad on mount and forwards character keydown events', async () => {
    const keySnapshot: TypingSnapshot = {
      session: {
        ...typingSessionFixture,
        rawKeys: 'a'
      },
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

    const { wrapper, api } = await mountPage(TypingTestPage, {
      imeApiOverrides: {
        processTypingKey: vi.fn(async () => keySnapshot)
      }
    })

    expect(api.createTypingSession).toHaveBeenCalledTimes(1)

    await wrapper.find('[tabindex="0"]').trigger('keydown', { key: 'a' })
    await flushPromises()

    expect(api.processTypingKey).toHaveBeenCalledWith(expect.any(Object), {
      kind: 'char',
      char: 'a'
    })
  })

  it('selects candidates and clears committed text from toolbar actions', async () => {
    const { wrapper, api, pinia } = await mountPage(TypingTestPage)
    const typingStore = useTypingStore(pinia)

    typingStore.session = typingSessionFixture
    typingStore.snapshot = {
      session: typingSessionFixture,
      response: {
        consumed: true,
        commitText: null,
        preedit: {
          compositionText: 'ni',
          cursor: 2
        },
        candidates: {
          items: [{ id: '1', text: 'candidate', annotation: null, hotkey: '1' }],
          pageIndex: 0,
          hasNextPage: false
        },
        inputMode: 'Chinese'
      }
    }
    typingStore.committedText = 'already committed'
    await flushPromises()

    const buttons = wrapper.findAll('button')
    await buttons[2]!.trigger('click')
    await flushPromises()
    expect(wrapper.text()).not.toContain('already committed')

    const candidateButton = wrapper
      .findAll('button')
      .find((button) => button.text().trim().startsWith('1.'))

    expect(candidateButton).toBeTruthy()

    await candidateButton!.trigger('click')
    await flushPromises()

    expect(api.processTypingKey).toHaveBeenCalledWith(expect.any(Object), {
      kind: 'number',
      number: 1
    })
  })

  it('shows an error message instead of crashing when preload api is missing', async () => {
    Object.defineProperty(window, 'imeApi', {
      configurable: true,
      writable: true,
      value: undefined
    })

    mount(TypingTestPage, {
      global: {
        plugins: [createPinia()]
      }
    })

    await flushPromises()

    expect(messageApi.error).toHaveBeenCalledWith(expect.stringContaining('IME API unavailable'))
  })
})
