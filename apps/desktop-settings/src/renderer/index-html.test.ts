/**
 * @vitest-environment node
 */

import { readFileSync } from 'node:fs'
import { resolve } from 'node:path'

describe('renderer html security policy', () => {
  it('declares a content security policy without unsafe-eval', () => {
    const html = readFileSync(resolve(__dirname, 'index.html'), 'utf8')

    expect(html).toContain('Content-Security-Policy')
    expect(html).not.toContain('unsafe-eval')
    expect(html).not.toContain('frame-ancestors')
  })
})
