import { startTransition } from 'react'
import { NavLink, Route, Routes, useNavigate } from 'react-router-dom'
import { AuthShell } from '../features/auth/AuthShell'
import { ChatListScreen } from '../features/chat/ChatListScreen'
import { Composer } from '../features/chat/Composer'
import { ConversationScreen } from '../features/chat/ConversationScreen'
import { PrivacySettingsScreen } from '../features/settings/PrivacySettingsScreen'
import { StorageSettingsScreen } from '../features/settings/StorageSettingsScreen'
import { chatActions, useChatStoreSnapshot } from '../lib/state/chatStore'

function MessengerLayout() {
  const state = useChatStoreSnapshot()
  const navigate = useNavigate()
  const activeConversation = state.conversations.find(
    (conversation) => conversation.id === state.selectedConversationId,
  )

  function navigateTo(path: string) {
    startTransition(() => navigate(path))
  }

  return (
    <div className="app-shell">
      <aside className="app-sidebar">
        <div className="brand-block">
          <span className="brand-mark">SE</span>
          <div>
            <p className="eyebrow">Messenger Alpha</p>
            <h1>Serenity</h1>
          </div>
        </div>

        <AuthShell />
        <ChatListScreen />

        <nav className="secondary-nav" aria-label="Secondary navigation">
          <button type="button" onClick={() => navigateTo('/')}>
            Messenger
          </button>
          <NavLink to="/settings/privacy">Privacy</NavLink>
          <NavLink to="/settings/storage">Storage</NavLink>
        </nav>
      </aside>

      <main className="app-main">
        <header className="status-bar">
          <div>
            <p className="eyebrow">Current conversation</p>
            <h2>{activeConversation?.title ?? 'No chat selected'}</h2>
          </div>
          <div className="network-cluster">
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
            <button
              type="button"
              onClick={() => chatActions.advanceFailureTimer(181)}
            >
              Simulate 3 min timeout
            </button>
          </div>
        </header>

        <Routes>
          <Route
            path="/"
            element={
              <section className="messenger-grid">
                <ConversationScreen />
                <Composer />
              </section>
            }
          />
          <Route path="/settings/privacy" element={<PrivacySettingsScreen />} />
          <Route path="/settings/storage" element={<StorageSettingsScreen />} />
        </Routes>
      </main>
    </div>
  )
}

export function AppRouter() {
  return <MessengerLayout />
}
