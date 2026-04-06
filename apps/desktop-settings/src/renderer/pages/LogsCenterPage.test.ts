import LogsCenterPage from './LogsCenterPage.vue'
import { errorLogFixture } from '../../test/ime-api'
import { mountPage } from '../../test/page-test-utils'

describe('LogsCenterPage', () => {
  it('loads logs on mount and renders the log payload', async () => {
    const { wrapper, api } = await mountPage(LogsCenterPage)

    expect(api.listErrorLogs).toHaveBeenCalledTimes(1)
    expect(wrapper.text()).toContain(errorLogFixture.message)
    expect(wrapper.text()).toContain(String(1))
  })
})
