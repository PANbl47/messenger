import { chatActions, useChatStoreSnapshot } from '../../lib/state/chatStore'

export function ChatListScreen() {
  const state = useChatStoreSnapshot()

  return (
    <section className="panel">
      <p className="eyebrow">Chats</p>
      <ul className="chat-list">
        {state.conversations.map((conversation) => (
          <li key={conversation.id}>
            <button
              className={
                conversation.id === state.selectedConversationId
                  ? 'chat-row active'
                  : 'chat-row'
              }
              onClick={() => chatActions.selectConversation(conversation.id)}
              type="button"
            >
              <span className="chat-avatar" aria-hidden="true">
                {conversation.title.slice(0, 1)}
              </span>
              <span>
                <strong>{conversation.title}</strong>
                <small>{conversation.subtitle}</small>
              </span>
            </button>
          </li>
        ))}
      </ul>
    </section>
  )
}
