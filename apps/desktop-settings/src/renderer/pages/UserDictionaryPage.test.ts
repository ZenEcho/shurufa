import { flushPromises } from '@vue/test-utils'
import UserDictionaryPage from './UserDictionaryPage.vue'
import { dictionaryEntryFixture, newDictionaryEntryFixture } from '../../test/ime-api'
import { mountPage } from '../../test/page-test-utils'

describe('UserDictionaryPage', () => {
  it('loads entries on mount and creates a new entry from the modal form', async () => {
    const { wrapper, api, messageApi } = await mountPage(UserDictionaryPage)

    expect(api.listUserDictionary).toHaveBeenCalledTimes(1)
    expect(wrapper.text()).toContain(dictionaryEntryFixture.word)

    const buttons = wrapper.findAll('button')
    await buttons[1]!.trigger('click')

    const inputs = wrapper.findAll('input')
    await inputs[0]!.setValue(newDictionaryEntryFixture.schemaId)
    await inputs[1]!.setValue(newDictionaryEntryFixture.code)
    await inputs[2]!.setValue(newDictionaryEntryFixture.word)
    await inputs[3]!.setValue(String(newDictionaryEntryFixture.weight))
    await wrapper.findAll('button').at(-1)!.trigger('click')
    await flushPromises()

    expect(api.createUserDictionaryEntry).toHaveBeenCalledWith({
      ...newDictionaryEntryFixture,
      weight: newDictionaryEntryFixture.weight
    })
    expect(messageApi.success).toHaveBeenCalled()
  })

  it('deletes an existing entry from the table action', async () => {
    const { wrapper, api, messageApi } = await mountPage(UserDictionaryPage)

    await wrapper.findAll('button').at(-1)!.trigger('click')
    await flushPromises()

    expect(api.deleteUserDictionaryEntry).toHaveBeenCalledWith(dictionaryEntryFixture.id)
    expect(messageApi.success).toHaveBeenCalled()
  })
})
