import { useState } from 'react'
import { chatActions } from '../../lib/state/chatStore'

type Mode = 'phone' | 'login'

export function AuthGateway() {
  const [mode, setMode] = useState<Mode>('phone')
  const [displayName, setDisplayName] = useState('')
  const [username, setUsername] = useState('')
  const [phone, setPhone] = useState('')
  const [login, setLogin] = useState('')
  const [password, setPassword] = useState('')

  function submit() {
    if (mode === 'phone') {
      chatActions.completePhoneSignup({
        displayName,
        username,
        phone,
      })
      return
    }

    chatActions.completeLoginSignup({
      displayName,
      username,
      login,
    })
    void password
  }

  const disabled =
    !displayName.trim() ||
    !username.trim() ||
    (mode === 'phone' ? !phone.trim() : !login.trim() || !password.trim())

  return (
    <main className="auth-screen">
      <section className="auth-card">
        <p className="eyebrow">Serenity</p>
        <h1>Private messaging without fake shortcuts.</h1>
        <p className="status-copy">
          Start from a real sign-up screen. No preloaded account. No seeded
          inbox pretending to be a product.
        </p>

        <div className="auth-mode-switch">
          <button
            className={mode === 'phone' ? 'auth-mode active' : 'auth-mode'}
            onClick={() => setMode('phone')}
            type="button"
          >
            Continue with phone
          </button>
          <button
            className={mode === 'login' ? 'auth-mode active' : 'auth-mode'}
            onClick={() => setMode('login')}
            type="button"
          >
            Continue with login
          </button>
        </div>

        <label className="field">
          <span>Display name</span>
          <input
            onChange={(event) => setDisplayName(event.target.value)}
            placeholder="How people should see you"
            value={displayName}
          />
        </label>

        <label className="field">
          <span>Username</span>
          <input
            onChange={(event) => setUsername(event.target.value)}
            placeholder="@username"
            value={username}
          />
        </label>

        {mode === 'phone' ? (
          <label className="field">
            <span>Phone</span>
            <input
              onChange={(event) => setPhone(event.target.value)}
              placeholder="+7 999 000 00 00"
              value={phone}
            />
          </label>
        ) : (
          <>
            <label className="field">
              <span>Login</span>
              <input
                onChange={(event) => setLogin(event.target.value)}
                placeholder="Unique login"
                value={login}
              />
            </label>
            <label className="field">
              <span>Password</span>
              <input
                onChange={(event) => setPassword(event.target.value)}
                placeholder="Password"
                type="password"
                value={password}
              />
            </label>
          </>
        )}

        <button disabled={disabled} onClick={submit} type="button">
          Create account
        </button>
      </section>
    </main>
  )
}
