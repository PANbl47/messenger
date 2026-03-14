import { useChatStoreSnapshot } from '../../lib/state/chatStore'

export function AuthShell() {
  const state = useChatStoreSnapshot()

  return (
    <section className="panel">
      <p className="eyebrow">Identity</p>
      <h3>{state.account.displayName}</h3>
      <dl className="identity-grid">
        <div>
          <dt>Username</dt>
          <dd>@{state.account.username}</dd>
        </div>
        <div>
          <dt>Phone</dt>
          <dd>{state.account.phone ?? 'Not linked'}</dd>
        </div>
        <div>
          <dt>Login</dt>
          <dd>{state.account.login ?? 'Phone-first'}</dd>
        </div>
      </dl>
    </section>
  )
}
