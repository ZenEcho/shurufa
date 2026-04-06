import { mount } from '@vue/test-utils'
import { describe, expect, it } from 'vitest'
import MetricCard from './MetricCard.vue'

describe('MetricCard', () => {
  it('renders the metric label, value, and hint', () => {
    const wrapper = mount(MetricCard, {
      props: {
        label: 'Service Status',
        value: 'running',
        hint: 'Background daemon is healthy'
      }
    })

    expect(wrapper.text()).toContain('Service Status')
    expect(wrapper.text()).toContain('running')
    expect(wrapper.text()).toContain('Background daemon is healthy')
  })
})
