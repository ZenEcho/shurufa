import { afterEach, vi } from 'vitest'
import { resetMessageApi } from './naive-ui-stub'

vi.mock('naive-ui', async () => import('./naive-ui-stub'))

afterEach(() => {
  resetMessageApi()
  vi.restoreAllMocks()
  vi.unstubAllGlobals()
})
