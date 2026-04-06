import { mount } from '@vue/test-utils'
import { describe, expect, it } from 'vitest'
import type { NavSection } from '@shurufa/shared-types'
import AppShell from './AppShell.vue'

const sections: NavSection[] = [
  { id: 'overview', label: 'Overview', description: 'Summary information' },
  { id: 'typing', label: 'Typing', description: 'Typing diagnostics' }
]

describe('AppShell', () => {
  it('renders navigation content and emits the selected section', async () => {
    const wrapper = mount(AppShell, {
      props: {
        title: 'IME Studio',
        subtitle: 'Control center',
        sections,
        activeSection: 'overview'
      },
      slots: {
        hero: '<div>Hero panel</div>',
        default: '<div>Main content</div>'
      }
    })

    const buttons = wrapper.findAll('button')

    expect(wrapper.text()).toContain('IME Studio')
    expect(wrapper.text()).toContain('Control center')
    expect(wrapper.text()).toContain('Hero panel')
    expect(wrapper.text()).toContain('Main content')

    await buttons[1].trigger('click')

    expect(wrapper.emitted('selectSection')).toEqual([['typing']])
  })
})
