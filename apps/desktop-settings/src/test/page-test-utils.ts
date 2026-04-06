import { flushPromises, mount } from '@vue/test-utils'
import { createPinia } from 'pinia'
import type { Component } from 'vue'
import { createImeApiMock } from './ime-api'
import { messageApi } from './naive-ui-stub'

export const mountPage = async (
  component: Component,
  options?: {
    imeApiOverrides?: Partial<Window['imeApi']>
    beforeMount?: (pinia: ReturnType<typeof createPinia>) => void
  }
) => {
  const pinia = createPinia()
  options?.beforeMount?.(pinia)
  const api = createImeApiMock(options?.imeApiOverrides)

  const wrapper = mount(component, {
    global: {
      plugins: [pinia]
    }
  })

  await flushPromises()

  return {
    wrapper,
    pinia,
    api,
    messageApi
  }
}
