import { mountPage } from '../../test/page-test-utils'
import { dashboardFixture, runtimeStatusFixture } from '../../test/ime-api'
import DashboardPage from './DashboardPage.vue'
import { useRuntimeStore } from '../stores/runtime'

describe('DashboardPage', () => {
  it('renders metrics and runtime summary from the runtime store', async () => {
    const { wrapper } = await mountPage(DashboardPage, {
      beforeMount: (pinia) => {
        const runtimeStore = useRuntimeStore(pinia)
        runtimeStore.dashboard = dashboardFixture
        runtimeStore.runtime = runtimeStatusFixture
      }
    })

    expect(wrapper.text()).toContain(dashboardFixture.metrics[0]!.value)
    expect(wrapper.text()).toContain(runtimeStatusFixture.activePlatform)
    expect(wrapper.text()).toContain(runtimeStatusFixture.defaultSchema)
  })

  it('shows the empty state when runtime data is missing', async () => {
    const { wrapper } = await mountPage(DashboardPage, {
      beforeMount: (pinia) => {
        const runtimeStore = useRuntimeStore(pinia)
        runtimeStore.dashboard = null
        runtimeStore.runtime = null
      }
    })

    expect(wrapper.find('[data-ui="NEmpty"]').exists()).toBe(true)
  })
})
