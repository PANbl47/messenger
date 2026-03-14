type PersistedDraft = {
  text: string
  attachmentName: string | null
  voiceDraftLabel: string | null
}

type PersistedState = {
  draftsByConversation: Record<string, PersistedDraft>
}

const STORAGE_KEY = 'messenger-alpha-web-state'
const memoryFallback = new Map<string, string>()

function readRaw(): string | null {
  if (typeof window !== 'undefined' && window.localStorage) {
    return window.localStorage.getItem(STORAGE_KEY)
  }

  return memoryFallback.get(STORAGE_KEY) ?? null
}

function writeRaw(value: string) {
  if (typeof window !== 'undefined' && window.localStorage) {
    window.localStorage.setItem(STORAGE_KEY, value)
    return
  }

  memoryFallback.set(STORAGE_KEY, value)
}

export async function loadPersistedState(): Promise<PersistedState> {
  const raw = readRaw()
  if (!raw) {
    return { draftsByConversation: {} }
  }

  try {
    return JSON.parse(raw) as PersistedState
  } catch {
    return { draftsByConversation: {} }
  }
}

export async function savePersistedState(state: PersistedState): Promise<void> {
  writeRaw(JSON.stringify(state))
}

export async function clearPersistedState(): Promise<void> {
  if (typeof window !== 'undefined' && window.localStorage) {
    window.localStorage.removeItem(STORAGE_KEY)
    return
  }

  memoryFallback.delete(STORAGE_KEY)
}
