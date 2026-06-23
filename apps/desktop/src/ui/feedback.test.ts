import { mount } from '@vue/test-utils'
import { nextTick } from 'vue'
import { describe, expect, it, vi } from 'vitest'

import ConfirmHost from './ConfirmHost.vue'
import { ElMessageBox, confirmDialogCancel } from './feedback'

describe('feedback adapters', () => {
  it('does not use the native browser confirm dialog', async () => {
    const nativeConfirm = vi.spyOn(window, 'confirm').mockReturnValue(true)

    const confirmPromise = ElMessageBox.confirm('Delete this item?', 'Notice')

    expect(nativeConfirm).not.toHaveBeenCalled()
    confirmDialogCancel()
    await expect(confirmPromise).rejects.toThrow('cancel')
    nativeConfirm.mockRestore()
  })

  it('resolves confirmation from the application dialog', async () => {
    const wrapper = mount(ConfirmHost, { attachTo: document.body })
    const confirmPromise = ElMessageBox.confirm('Delete this item?', 'Notice')

    await nextTick()

    expect(document.body.textContent).toContain('Notice')
    expect(document.body.textContent).toContain('Delete this item?')

    const acceptButton = document.body.querySelector('[data-test="confirm-accept"]') as HTMLButtonElement
    acceptButton.click()
    await nextTick()

    await expect(confirmPromise).resolves.toBeUndefined()
    wrapper.unmount()
  })
})
