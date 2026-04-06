import { mount } from '@vue/test-utils'
import PlaceholderPage from './PlaceholderPage.vue'

describe('PlaceholderPage', () => {
  it('renders the provided title and description', () => {
    const wrapper = mount(PlaceholderPage, {
      props: {
        title: 'Candidates',
        description: 'Work in progress'
      }
    })

    expect(wrapper.text()).toContain('Candidates')
    expect(wrapper.text()).toContain('Work in progress')
  })
})
