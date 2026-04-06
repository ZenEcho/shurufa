import { mount } from '@vue/test-utils'
import { describe, expect, it } from 'vitest'
import SectionPanel from './SectionPanel.vue'

describe('SectionPanel', () => {
  it('renders headings and slot content', () => {
    const wrapper = mount(SectionPanel, {
      props: {
        eyebrow: 'Diagnostics',
        title: 'Recent Logs'
      },
      slots: {
        default: '<p>One recent error</p>'
      }
    })

    expect(wrapper.text()).toContain('Diagnostics')
    expect(wrapper.text()).toContain('Recent Logs')
    expect(wrapper.text()).toContain('One recent error')
  })
})
