import { chatActions, useChatStoreSnapshot } from '../../lib/state/chatStore'

export function StorageSettingsScreen() {
  const state = useChatStoreSnapshot()

  return (
    <section className="panel settings-panel">
      <p className="eyebrow">Storage</p>
      <h3>Local storage management</h3>

      <ul className="storage-list">
        {state.storageCategories.map((category) => (
          <li key={category.id}>
            <strong>{category.label}</strong>
            <span>{category.sizeLabel}</span>
            <small>{category.protected ? 'Protected' : 'Cache'}</small>
          </li>
        ))}
      </ul>

      <button type="button" onClick={() => chatActions.cleanupStorage()}>
        Clear safe cache
      </button>

      <p aria-live="polite" className="storage-feedback">
        {state.cleanupFeedback}
      </p>
    </section>
  )
}
