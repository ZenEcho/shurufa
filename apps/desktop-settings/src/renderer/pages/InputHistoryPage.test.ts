import InputHistoryPage from './InputHistoryPage.vue'
import { historyEntryFixture } from '../../test/ime-api'
import { mountPage } from '../../test/page-test-utils'

describe('InputHistoryPage', () => {
  it('loads history entries on mount and reloads on demand', async () => {
    const { wrapper, api } = await mountPage(InputHistoryPage)

    expect(api.listHistory).toHaveBeenCalledTimes(1)
    expect(wrapper.text()).toContain(historyEntryFixture.committedText)

    await wrapper.find('button').trigger('click')

    expect(api.listHistory).toHaveBeenCalledTimes(2)
  })
})
