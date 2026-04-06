import { computed, defineComponent, h } from 'vue'
import type { PropType, VNode } from 'vue'
import { vi } from 'vitest'

type Column<Row = Record<string, unknown>> = {
  title?: string
  key?: string
  render?: (row: Row) => VNode | string | number | null | undefined
}

export const messageApi = {
  success: vi.fn(),
  error: vi.fn(),
  warning: vi.fn(),
  info: vi.fn()
}

export const resetMessageApi = (): void => {
  messageApi.success.mockReset()
  messageApi.error.mockReset()
  messageApi.warning.mockReset()
  messageApi.info.mockReset()
}

const renderSlot = (slots: Record<string, (() => VNode[] | VNode | string | null | undefined) | undefined>, name = 'default') =>
  slots[name]?.()

const normalizeChildren = (
  value: VNode[] | VNode | string | number | null | undefined
): Array<VNode | string | number> => {
  if (value == null) {
    return []
  }

  return Array.isArray(value) ? value : [value]
}

const createPassThrough = (name: string, tag = 'div', extra?: (props: Record<string, unknown>, slots: Record<string, () => VNode[] | VNode | string | null | undefined>) => VNode[]) =>
  defineComponent({
    name,
    inheritAttrs: false,
    props: {
      title: String,
      description: String,
      bordered: Boolean,
      embedded: Boolean,
      showIcon: Boolean,
      type: String,
      preset: String,
      label: String
    },
    setup(props, { slots, attrs }) {
      return () =>
        h(
          tag,
          {
            ...attrs,
            'data-ui': name
          },
          [
            props.title ? h('div', { 'data-prop': 'title' }, props.title) : null,
            props.label ? h('label', { 'data-prop': 'label' }, props.label) : null,
            props.description ? h('div', { 'data-prop': 'description' }, props.description) : null,
            ...(extra ? extra(props as Record<string, unknown>, slots as Record<string, () => VNode[] | VNode | string | null | undefined>) : []),
            ...normalizeChildren(renderSlot(slots)),
            ...normalizeChildren(renderSlot(slots, 'footer'))
          ]
        )
    }
  })

export const NAlert = createPassThrough('NAlert', 'section')
export const NCard = createPassThrough('NCard', 'section')
export const NDivider = createPassThrough('NDivider', 'hr')
export const NEmpty = createPassThrough('NEmpty', 'section')
export const NForm = createPassThrough('NForm', 'form')
export const NFormItem = createPassThrough('NFormItem', 'div')
export const NGrid = createPassThrough('NGrid', 'div')
export const NGridItem = createPassThrough('NGridItem', 'div')
export const NSpace = createPassThrough('NSpace', 'div')
export const NTag = createPassThrough('NTag', 'span')

export const NStatistic = defineComponent({
  name: 'NStatistic',
  props: {
    label: String,
    value: [String, Number]
  },
  setup(props) {
    return () =>
      h('div', { 'data-ui': 'NStatistic' }, [
        h('div', props.label ?? ''),
        h('div', String(props.value ?? ''))
      ])
  }
})

export const NButton = defineComponent({
  name: 'NButton',
  inheritAttrs: false,
  props: {
    disabled: Boolean,
    loading: Boolean,
    type: String,
    secondary: Boolean,
    tertiary: Boolean,
    size: String
  },
  emits: ['click'],
  setup(props, { slots, emit, attrs }) {
    return () =>
      h(
        'button',
        {
          ...attrs,
          disabled: props.disabled || props.loading,
          'data-ui': 'NButton',
          onClick: (event: MouseEvent) => emit('click', event)
        },
        normalizeChildren(renderSlot(slots))
      )
  }
})

export const NInput = defineComponent({
  name: 'NInput',
  inheritAttrs: false,
  props: {
    value: String,
    type: String,
    readonly: Boolean
  },
  emits: ['update:value'],
  setup(props, { emit, attrs }) {
    return () =>
      props.type === 'textarea'
        ? h('textarea', {
            ...attrs,
            value: props.value ?? '',
            readOnly: props.readonly,
            'data-ui': 'NInput',
            onInput: (event: Event) =>
              emit('update:value', (event.target as HTMLTextAreaElement).value)
          })
        : h('input', {
            ...attrs,
            value: props.value ?? '',
            readOnly: props.readonly,
            'data-ui': 'NInput',
            onInput: (event: Event) =>
              emit('update:value', (event.target as HTMLInputElement).value)
          })
  }
})

export const NInputNumber = defineComponent({
  name: 'NInputNumber',
  inheritAttrs: false,
  props: {
    value: Number,
    min: Number,
    max: Number,
    disabled: Boolean
  },
  emits: ['update:value'],
  setup(props, { emit, attrs }) {
    return () =>
      h('input', {
        ...attrs,
        type: 'number',
        value: props.value ?? 0,
        min: props.min,
        max: props.max,
        disabled: props.disabled,
        'data-ui': 'NInputNumber',
        onInput: (event: Event) => {
          const value = Number((event.target as HTMLInputElement).value)
          emit('update:value', Number.isNaN(value) ? null : value)
        }
      })
  }
})

export const NSelect = defineComponent({
  name: 'NSelect',
  inheritAttrs: false,
  props: {
    value: [String, Number],
    disabled: Boolean,
    options: {
      type: Array as PropType<Array<{ label: string; value: string | number }>>,
      default: () => []
    }
  },
  emits: ['update:value'],
  setup(props, { emit, attrs }) {
    return () =>
      h(
        'select',
        {
          ...attrs,
          value: props.value,
          disabled: props.disabled,
          'data-ui': 'NSelect',
          onChange: (event: Event) => emit('update:value', (event.target as HTMLSelectElement).value)
        },
        props.options.map((option) =>
          h('option', { key: option.value, value: option.value }, option.label)
        )
      )
  }
})

export const NSwitch = defineComponent({
  name: 'NSwitch',
  inheritAttrs: false,
  props: {
    value: Boolean,
    disabled: Boolean
  },
  emits: ['update:value'],
  setup(props, { emit, attrs }) {
    return () =>
      h('input', {
        ...attrs,
        type: 'checkbox',
        checked: props.value,
        disabled: props.disabled,
        'data-ui': 'NSwitch',
        onChange: (event: Event) =>
          emit('update:value', (event.target as HTMLInputElement).checked)
      })
  }
})

export const NModal = defineComponent({
  name: 'NModal',
  props: {
    show: Boolean,
    title: String
  },
  setup(props, { slots }) {
    return () =>
      props.show
        ? h('section', { 'data-ui': 'NModal' }, [
            props.title ? h('div', props.title) : null,
            ...normalizeChildren(renderSlot(slots)),
            ...normalizeChildren(renderSlot(slots, 'footer'))
          ])
        : null
  }
})

export const NDataTable = defineComponent({
  name: 'NDataTable',
  props: {
    columns: {
      type: Array as PropType<Column[]>,
      default: () => []
    },
    data: {
      type: Array as PropType<Record<string, unknown>[]>,
      default: () => []
    },
    loading: Boolean
  },
  setup(props) {
    const normalizedRows = computed(() => props.data ?? [])

    return () =>
      h('div', { 'data-ui': 'NDataTable', 'data-loading': String(props.loading ?? false) }, [
        ...normalizedRows.value.map((row, rowIndex) =>
          h(
            'div',
            { key: rowIndex, 'data-row': rowIndex },
            (props.columns ?? []).map((column, columnIndex) =>
              h(
                'div',
                { key: `${rowIndex}-${columnIndex}`, 'data-column-key': column.key ?? '' },
                normalizeChildren(
                  column.render
                    ? column.render(row)
                    : column.key
                      ? String(row[column.key] ?? '')
                      : ''
                )
              )
            )
          )
        )
      ])
  }
})

export const useMessage = () => messageApi
