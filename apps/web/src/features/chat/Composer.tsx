import { chatActions, useChatStoreSnapshot } from '../../lib/state/chatStore'

export function Composer() {
  const state = useChatStoreSnapshot()
  const draft = state.draftsByConversation[state.selectedConversationId]
  const hasConversation = Boolean(state.selectedConversationId)

  if (!hasConversation) {
    return (
      <section className="panel composer-panel empty-block">
        <p className="eyebrow">Composer</p>
        <h3>Waiting for your first chat</h3>
        <p className="status-copy">
          The composer unlocks once a real conversation exists.
        </p>
      </section>
    )
  }

  return (
    <section className="panel composer-panel">
      <div className="panel-header">
        <div>
          <p className="eyebrow">Composer</p>
          <h3>Reliable local draft</h3>
        </div>
        <button
          type="button"
          onClick={() => chatActions.markReadOnActiveConversation()}
        >
          Mark read
        </button>
      </div>

      <label className="field">
        <span>Message</span>
        <textarea
          aria-label="Message draft"
          onChange={(event) =>
            chatActions.updateDraft(state.selectedConversationId, {
              text: event.target.value,
            })
          }
          placeholder="Message will stay safe locally first"
          value={draft.text}
        />
      </label>

      <div className="composer-tools">
        <button
          type="button"
          onClick={() =>
            chatActions.updateDraft(state.selectedConversationId, {
              attachmentName: draft.attachmentName ? null : 'route-map.pdf',
            })
          }
        >
          {draft.attachmentName ? 'Remove attachment' : 'Attach file'}
        </button>
        <button
          type="button"
          onClick={() =>
            chatActions.updateDraft(state.selectedConversationId, {
              voiceDraftLabel: draft.voiceDraftLabel ? null : 'voice-note-01',
            })
          }
        >
          {draft.voiceDraftLabel ? 'Discard voice draft' : 'Add voice draft'}
        </button>
      </div>

      <div className="draft-preview" aria-live="polite">
        {draft.attachmentName ? (
          <span className="draft-chip">Attachment: {draft.attachmentName}</span>
        ) : null}
        {draft.voiceDraftLabel ? (
          <span className="draft-chip">Voice: {draft.voiceDraftLabel}</span>
        ) : null}
      </div>

      <div className="composer-actions">
        <button
          type="button"
          onClick={() => chatActions.sendDraft(state.selectedConversationId)}
        >
          Send
        </button>
      </div>
    </section>
  )
}
