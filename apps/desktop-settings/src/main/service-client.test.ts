/**
 * @vitest-environment node
 */

import { join, resolve } from 'node:path'
import type { AppConfig, TypingSessionState } from '@shurufa/shared-types'
import { beforeEach, describe, expect, it, vi } from 'vitest'

const execFileAsyncMock = vi.hoisted(() => vi.fn())

vi.mock('node:child_process', () => ({
  execFile: vi.fn()
}))

vi.mock('node:util', async () => {
  const actual = await vi.importActual<typeof import('node:util')>('node:util')

  return {
    ...actual,
    promisify: vi.fn(() => execFileAsyncMock)
  }
})

const workspaceRoot = resolve(__dirname, '../../../../')
const manifestPath = join(workspaceRoot, 'rust', 'Cargo.toml')
const desktopSettingsRoot = resolve(__dirname, '../..')

const loadModule = async () => import('./service-client')

describe('service-client', () => {
  const originalCwd = process.cwd()

  beforeEach(() => {
    execFileAsyncMock.mockReset()
    process.chdir(originalCwd)
    vi.resetModules()
  })

  it('maps config responses to the shared camelCase shape', async () => {
    execFileAsyncMock.mockResolvedValueOnce({
      stdout: JSON.stringify({
        general: { startup_with_system: true, locale: 'zh-CN' },
        input: {
          default_schema: 'pinyin',
          english_mode_by_default: false,
          candidate_page_size: 9
        },
        appearance: {
          theme: 'system',
          font_size: 16,
          candidate_layout: 'vertical'
        },
        logging: {
          level: 'info',
          redact_input_content: true
        }
      })
    })

    const { getConfig } = await loadModule()

    await expect(getConfig()).resolves.toEqual({
      general: { startupWithSystem: true, locale: 'zh-CN' },
      input: {
        defaultSchema: 'pinyin',
        englishModeByDefault: false,
        candidatePageSize: 9
      },
      appearance: {
        theme: 'system',
        fontSize: 16,
        candidateLayout: 'vertical'
      },
      logging: {
        level: 'info',
        redactInputContent: true
      }
    })

    expect(execFileAsyncMock).toHaveBeenCalledWith(
      'cargo.exe',
      ['run', '--quiet', '--manifest-path', manifestPath, '-p', 'ime-service', '--', '--json', 'get-config'],
      expect.objectContaining({
        cwd: workspaceRoot,
        windowsHide: true
      })
    )
  })

  it('resolves cargo manifest and cwd from the workspace root instead of process.cwd()', async () => {
    process.chdir(desktopSettingsRoot)
    execFileAsyncMock.mockResolvedValueOnce({
      stdout: JSON.stringify({
        general: { startup_with_system: true, locale: 'zh-CN' },
        input: {
          default_schema: 'pinyin',
          english_mode_by_default: false,
          candidate_page_size: 9
        },
        appearance: {
          theme: 'system',
          font_size: 16,
          candidate_layout: 'vertical'
        },
        logging: {
          level: 'info',
          redact_input_content: true
        }
      })
    })

    const { getConfig } = await loadModule()
    await getConfig()

    expect(execFileAsyncMock).toHaveBeenCalledWith(
      'cargo.exe',
      ['run', '--quiet', '--manifest-path', manifestPath, '-p', 'ime-service', '--', '--json', 'get-config'],
      expect.objectContaining({
        cwd: workspaceRoot
      })
    )
  })

  it('serializes config updates with snake_case payload keys', async () => {
    execFileAsyncMock.mockResolvedValueOnce({
      stdout: JSON.stringify({
        service_status: 'running',
        active_platform: 'windows-tsf',
        default_schema: 'double-pinyin'
      })
    })

    const { updateConfig } = await loadModule()
    const config = {
      general: { startupWithSystem: false, locale: 'zh-CN' },
      input: {
        defaultSchema: 'double-pinyin',
        englishModeByDefault: true,
        candidatePageSize: 5
      },
      appearance: {
        theme: 'dark',
        fontSize: 18,
        candidateLayout: 'horizontal'
      },
      logging: {
        level: 'debug',
        redactInputContent: false
      }
    } satisfies AppConfig

    await expect(updateConfig(config)).resolves.toEqual({
      serviceStatus: 'running',
      activePlatform: 'windows-tsf',
      defaultSchema: 'double-pinyin'
    })

    expect(execFileAsyncMock).toHaveBeenCalledWith(
      'cargo.exe',
      [
        'run',
        '--quiet',
        '--manifest-path',
        manifestPath,
        '-p',
        'ime-service',
        '--',
        '--json',
        'set-config',
        JSON.stringify({
          general: {
            startup_with_system: false,
            locale: 'zh-CN'
          },
          input: {
            default_schema: 'double-pinyin',
            english_mode_by_default: true,
            candidate_page_size: 5
          },
          appearance: {
            theme: 'dark',
            font_size: 18,
            candidate_layout: 'horizontal'
          },
          logging: {
            level: 'debug',
            redact_input_content: false
          }
        })
      ],
      expect.any(Object)
    )
  })

  it('builds the dashboard snapshot from config and runtime requests', async () => {
    execFileAsyncMock
      .mockResolvedValueOnce({
        stdout: JSON.stringify({
          general: { startup_with_system: false, locale: 'zh-CN' },
          input: {
            default_schema: 'wubi',
            english_mode_by_default: false,
            candidate_page_size: 7
          },
          appearance: {
            theme: 'system',
            font_size: 16,
            candidate_layout: 'vertical'
          },
          logging: {
            level: 'info',
            redact_input_content: true
          }
        })
      })
      .mockResolvedValueOnce({
        stdout: JSON.stringify({
          service_status: 'running',
          active_platform: 'windows-tsf',
          default_schema: 'wubi'
        })
      })

    const { getDashboard } = await loadModule()
    const dashboard = await getDashboard()

    expect(dashboard.metrics).toHaveLength(4)
    expect(dashboard.metrics.map((metric) => metric.value)).toEqual([
      'windows-tsf',
      'running',
      'wubi',
      '7'
    ])
  })

  it('maps list responses into shared dictionary entries', async () => {
    execFileAsyncMock.mockResolvedValueOnce({
      stdout: JSON.stringify([
        {
          id: 1,
          schema_id: 'pinyin',
          code: 'nihao',
          word: '你好',
          weight: 42,
          source: 'user',
          created_at: 10,
          updated_at: 11
        }
      ])
    })

    const { listUserDictionary } = await loadModule()

    await expect(listUserDictionary()).resolves.toEqual([
      {
        id: 1,
        schemaId: 'pinyin',
        code: 'nihao',
        word: '你好',
        weight: 42,
        source: 'user',
        createdAt: 10,
        updatedAt: 11
      }
    ])
  })

  it('serializes typing payloads and maps the typing snapshot response', async () => {
    execFileAsyncMock.mockResolvedValueOnce({
      stdout: JSON.stringify({
        session: {
          raw_keys: 'ni',
          composition_text: '你',
          selected_index: 0,
          candidates: [{ id: '1', text: '你', annotation: null, hotkey: '1' }],
          input_mode: 'Chinese'
        },
        response: {
          consumed: true,
          commit_text: null,
          preedit: {
            composition_text: '你',
            cursor: 1
          },
          candidates: {
            items: [{ id: '1', text: '你', annotation: null, hotkey: '1' }],
            page_index: 0,
            has_next_page: false
          },
          input_mode: 'Chinese'
        }
      })
    })

    const { processTypingKey } = await loadModule()
    const session = {
      rawKeys: 'n',
      compositionText: '你',
      selectedIndex: 0,
      candidates: [{ id: '1', text: '你', annotation: null, hotkey: '1' }],
      inputMode: 'Chinese'
    } satisfies TypingSessionState

    await expect(processTypingKey(session, { kind: 'toggleInputMode' })).resolves.toEqual({
      session: {
        rawKeys: 'ni',
        compositionText: '你',
        selectedIndex: 0,
        candidates: [{ id: '1', text: '你', annotation: null, hotkey: '1' }],
        inputMode: 'Chinese'
      },
      response: {
        consumed: true,
        commitText: null,
        preedit: {
          compositionText: '你',
          cursor: 1
        },
        candidates: {
          items: [{ id: '1', text: '你', annotation: null, hotkey: '1' }],
          pageIndex: 0,
          hasNextPage: false
        },
        inputMode: 'Chinese'
      }
    })

    expect(execFileAsyncMock).toHaveBeenCalledWith(
      'cargo.exe',
      [
        'run',
        '--quiet',
        '--manifest-path',
        manifestPath,
        '-p',
        'ime-service',
        '--',
        '--json',
        'process-typing-key',
        JSON.stringify({
          session: {
            raw_keys: 'n',
            composition_text: '你',
            selected_index: 0,
            candidates: [{ id: '1', text: '你', annotation: null, hotkey: '1' }],
            input_mode: 'Chinese'
          },
          event: 'ToggleInputMode'
        })
      ],
      expect.any(Object)
    )
  })
})
