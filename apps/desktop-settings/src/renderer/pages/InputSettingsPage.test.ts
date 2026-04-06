import { flushPromises } from '@vue/test-utils'
import InputSettingsPage from './InputSettingsPage.vue'
import { configFixture, runtimeStatusFixture, updatedConfigFixture } from '../../test/ime-api'
import { mountPage } from '../../test/page-test-utils'
import { useConfigStore } from '../stores/config'
import { useRuntimeStore } from '../stores/runtime'

describe('InputSettingsPage', () => {
  it('saves changed settings and refreshes runtime state', async () => {
    const { wrapper, api, messageApi } = await mountPage(InputSettingsPage, {
      beforeMount: (pinia) => {
        const configStore = useConfigStore(pinia)
        const runtimeStore = useRuntimeStore(pinia)

        configStore.persistedConfig = structuredClone(configFixture)
        configStore.draftConfig = structuredClone(configFixture)
        runtimeStore.runtime = runtimeStatusFixture
      },
      imeApiOverrides: {
        updateConfig: vi.fn(async () => runtimeStatusFixture),
        getRuntimeStatus: vi.fn(async () => runtimeStatusFixture),
        getDashboard: vi.fn(async () => ({ metrics: [] }))
      }
    })

    await wrapper.find('select').setValue('wubi')
    await flushPromises()
    await wrapper.findAll('button').at(-1)!.trigger('click')
    await flushPromises()

    expect(api.updateConfig).toHaveBeenCalledWith(
      expect.objectContaining({
        input: expect.objectContaining({
          defaultSchema: 'wubi'
        })
      })
    )
    expect(api.getRuntimeStatus).toHaveBeenCalled()
    expect(messageApi.success).toHaveBeenCalled()
  })

  it('resets settings through the defaults action', async () => {
    const { wrapper, api, messageApi } = await mountPage(InputSettingsPage, {
      beforeMount: (pinia) => {
        const configStore = useConfigStore(pinia)
        configStore.persistedConfig = structuredClone(configFixture)
        configStore.draftConfig = structuredClone(updatedConfigFixture)
      }
    })

    await wrapper.findAll('button')[1]!.trigger('click')
    await flushPromises()

    expect(api.resetConfig).toHaveBeenCalled()
    expect(messageApi.success).toHaveBeenCalled()
  })
})
