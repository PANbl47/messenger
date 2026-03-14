import { chatActions, useChatStoreSnapshot } from '../../lib/state/chatStore'

export function PrivacySettingsScreen() {
  const state = useChatStoreSnapshot()

  return (
    <section className="panel settings-panel">
      <p className="eyebrow">Privacy</p>
      <h3>Discovery controls</h3>

      <label className="toggle">
        <span>Allow phone lookup</span>
        <input
          checked={state.privacy.phoneLookupEnabled}
          onChange={(event) =>
            chatActions.setPhoneLookupEnabled(event.target.checked)
          }
          type="checkbox"
        />
      </label>

      <label className="toggle">
        <span>Anyone can start a chat</span>
        <input checked={state.privacy.openInbound} readOnly type="checkbox" />
      </label>
    </section>
  )
}
