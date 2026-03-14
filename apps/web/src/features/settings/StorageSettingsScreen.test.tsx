import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { beforeEach, describe, expect, it } from 'vitest'
import { chatActions } from '../../lib/state/chatStore'
import { StorageSettingsScreen } from './StorageSettingsScreen'

describe('StorageSettingsScreen', () => {
  beforeEach(async () => {
    await chatActions.resetForTests()
  })

  it('blocks destructive cleanup for unsent work', async () => {
    const user = userEvent.setup()
    render(<StorageSettingsScreen />)

    await user.click(screen.getByRole('button', { name: 'Clear safe cache' }))

    expect(screen.getByText(/Protected 3 active items/i)).toBeInTheDocument()
    expect(screen.getByText('Unsent drafts')).toBeInTheDocument()
    expect(screen.getByText('Queued messages')).toBeInTheDocument()
  })
})
