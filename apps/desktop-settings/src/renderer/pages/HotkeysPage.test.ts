import { flushPromises } from '@vue/test-utils'
import HotkeysPage from './HotkeysPage.vue'
import { hotkeyEntryFixture } from '../../test/ime-api'
import { mountPage } from '../../test/page-test-utils'

describe('HotkeysPage', () => {
  it('loads hotkeys on mount and saves a hotkey from the modal form', async () => {
    const { wrapper, api, messageApi } = await mountPage(HotkeysPage)

    expect(api.listHotkeys).toHaveBeenCalledTimes(1)
    expect(wrapper.text()).toContain(hotkeyEntryFixture.accelerator)

    await wrapper.findAll('button')[1]!.trigger('click')

    const inputs = wrapper.findAll('input')
    await inputs[0]!.setValue('new-hotkey')
    await inputs[1]!.setValue('toggle-schema')
    await inputs[2]!.setValue('Ctrl+Shift+1')
    await inputs[3]!.setValue('global')
    await wrapper.findAll('button').at(-1)!.trigger('click')
    await flushPromises()

    expect(api.saveHotkey).toHaveBeenCalledWith(
      expect.objectContaining({
        id: 'new-hotkey',
        action: 'toggle-schema',
        accelerator: 'Ctrl+Shift+1',
        scope: 'global',
        enabled: true
      })
    )
    expect(messageApi.success).toHaveBeenCalled()
  })

  it('removes a hotkey from the table action', async () => {
    const { wrapper, api, messageApi } = await mountPage(HotkeysPage)

    await wrapper.findAll('button').at(-1)!.trigger('click')
    await flushPromises()

    expect(api.deleteHotkey).toHaveBeenCalledWith(hotkeyEntryFixture.id)
    expect(messageApi.success).toHaveBeenCalled()
  })
})
