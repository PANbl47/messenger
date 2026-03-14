import { render, screen } from '@testing-library/react'
import userEvent from '@testing-library/user-event'
import { beforeEach, describe, expect, it } from 'vitest'
import { Composer } from './Composer'
import { chatActions, useChatStoreSnapshot } from '../../lib/state/chatStore'

function ComposerHarness() {
  const state = useChatStoreSnapshot()
  return (
    <div>
      <label className="toggle">
        <span>Network</span>
        <input
          aria-label="Network toggle"
          checked={state.networkAvailable}
          onChange={(event) =>
            chatActions.setNetworkAvailable(event.target.checked)
          }
          type="checkbox"
        />
      </label>
      <button type="button" onClick={() => chatActions.advanceFailureTimer(181)}>
        Simulate 3 min timeout
      </button>
      <Composer />
      <div aria-label="Queued count">
        {
          (state.messagesByConversation[state.selectedConversationId] ?? []).filter(
            (message) => message.status === 'queued' || message.status === 'failed',
          ).length
        }
      </div>
    </div>
  )
}

describe('Composer', () => {
  beforeEach(async () => {
    await chatActions.resetForTests()
  })

  it('shows queued or failed message states in the timeline', async () => {
    const user = userEvent.setup()
    render(<ComposerHarness />)

    await user.click(screen.getByLabelText('Network toggle'))
    await user.type(screen.getByLabelText('Message draft'), 'Offline hello')
    await user.click(screen.getByRole('button', { name: 'Send' }))
    await user.click(screen.getByRole('button', { name: 'Simulate 3 min timeout' }))

    expect(screen.getByLabelText('Queued count')).toHaveTextContent('1')
  })

  it('restores a full draft including attachment and voice placeholders', async () => {
    const user = userEvent.setup()
    const view = render(<Composer />)

    await user.type(screen.getByLabelText('Message draft'), 'Restore me')
    await user.click(screen.getByRole('button', { name: 'Attach file' }))
    await user.click(screen.getByRole('button', { name: 'Add voice draft' }))

    view.unmount()
    render(<Composer />)

    expect(screen.getByDisplayValue('Restore me')).toBeInTheDocument()
    expect(screen.getByText('Attachment: route-map.pdf')).toBeInTheDocument()
    expect(screen.getByText('Voice: voice-note-01')).toBeInTheDocument()
  })
})
