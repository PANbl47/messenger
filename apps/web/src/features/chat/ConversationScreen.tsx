import { useDeferredValue } from 'react'
import {
  chatActions,
  type MessageStatus,
  useChatStoreSnapshot,
} from '../../lib/state/chatStore'

const statusGlyph: Record<MessageStatus, string> = {
  queued: '○',
  delivered: '✓',
  read: '✓✓',
  failed: '!',
}

export function ConversationScreen() {
  const state = useChatStoreSnapshot()
  const activeMessages = useDeferredValue(
    state.messagesByConversation[state.selectedConversationId] ?? [],
  )

  return (
    <section className="panel conversation-panel">
      <div className="panel-header">
        <div>
          <p className="eyebrow">Timeline</p>
          <h3>Conversation</h3>
        </div>
        <button type="button" onClick={() => chatActions.setTyping(true)}>
          Emit typing
        </button>
      </div>

      <ol className="timeline">
        {activeMessages.map((message) => (
          <li className="bubble" key={message.id}>
            <div className="bubble-head">
              <span>{message.author}</span>
              <button
                aria-label={`Retry ${message.id}`}
                disabled={message.status !== 'failed'}
                onClick={() => chatActions.retryMessage(message.id)}
                type="button"
              >
                Retry
              </button>
            </div>
            <p>{message.body}</p>
            {message.attachmentName ? (
              <p className="bubble-meta">Attachment: {message.attachmentName}</p>
            ) : null}
            {message.voiceDraftLabel ? (
              <p className="bubble-meta">Voice: {message.voiceDraftLabel}</p>
            ) : null}
            <div className="bubble-foot">
              <span aria-label={`Status ${message.status}`}>
                {statusGlyph[message.status]}
              </span>
              <small>{message.status}</small>
            </div>
          </li>
        ))}
      </ol>
    </section>
  )
}
