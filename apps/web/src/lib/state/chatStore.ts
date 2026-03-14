import { useSyncExternalStore } from 'react'
import {
  clearPersistedState,
  loadPersistedState,
  savePersistedState,
} from '../storage/indexedDb'

export type MessageStatus = 'queued' | 'delivered' | 'read' | 'failed'

type MessageRecord = {
  id: string
  author: string
  body: string
  status: MessageStatus
  attachmentName: string | null
  voiceDraftLabel: string | null
  queuedSeconds: number
}

type DraftRecord = {
  text: string
  attachmentName: string | null
  voiceDraftLabel: string | null
}

type Conversation = {
  id: string
  title: string
  subtitle: string
}

type StorageCategory = {
  id: string
  label: string
  sizeLabel: string
  protected: boolean
}

type ChatState = {
  account: {
    displayName: string
    username: string
    phone: string | null
    login: string | null
  }
  conversations: Conversation[]
  selectedConversationId: string
  messagesByConversation: Record<string, MessageRecord[]>
  draftsByConversation: Record<string, DraftRecord>
  networkAvailable: boolean
  privacy: {
    phoneLookupEnabled: boolean
    openInbound: boolean
  }
  storageCategories: StorageCategory[]
  cleanupFeedback: string
}

const listeners = new Set<() => void>()

const defaultDraft = (): DraftRecord => ({
  text: '',
  attachmentName: null,
  voiceDraftLabel: null,
})

const defaultState = (): ChatState => ({
  account: {
    displayName: 'Ser Makarov',
    username: 'ser',
    phone: '+7 999 000 11 22',
    login: 'ser-login',
  },
  conversations: [
    { id: 'chat-alex', title: 'Alex', subtitle: 'Weak network test buddy' },
  ],
  selectedConversationId: 'chat-alex',
  messagesByConversation: {
    'chat-alex': [
      {
        id: 'seed-1',
        author: 'Alex',
        body: 'Queue should feel calm even when the signal is not.',
        status: 'read',
        attachmentName: null,
        voiceDraftLabel: null,
        queuedSeconds: 0,
      },
    ],
  },
  draftsByConversation: {
    'chat-alex': defaultDraft(),
  },
  networkAvailable: true,
  privacy: {
    phoneLookupEnabled: true,
    openInbound: true,
  },
  storageCategories: [
    { id: 'cache-media', label: 'Downloaded cache', sizeLabel: '84 MB', protected: false },
    { id: 'cache-thumbs', label: 'Thumbnails', sizeLabel: '18 MB', protected: false },
    { id: 'drafts', label: 'Unsent drafts', sizeLabel: 'Protected', protected: true },
    { id: 'queue', label: 'Queued messages', sizeLabel: 'Protected', protected: true },
    { id: 'trust', label: 'Local trust material', sizeLabel: 'Protected', protected: true },
  ],
  cleanupFeedback: 'Nothing cleaned yet.',
})

let state = defaultState()

async function hydrateDrafts() {
  const persisted = await loadPersistedState()
  state = {
    ...state,
    draftsByConversation: {
      ...state.draftsByConversation,
      ...persisted.draftsByConversation,
    },
  }
}

await hydrateDrafts()

function emit() {
  for (const listener of listeners) {
    listener()
  }
}

function persistDrafts() {
  void savePersistedState({
    draftsByConversation: state.draftsByConversation,
  })
}

function setState(next: ChatState) {
  state = next
  persistDrafts()
  emit()
}

function getConversationMessages(conversationId: string) {
  return state.messagesByConversation[conversationId] ?? []
}

export const chatActions = {
  selectConversation(conversationId: string) {
    setState({ ...state, selectedConversationId: conversationId })
  },
  updateDraft(conversationId: string, patch: Partial<DraftRecord>) {
    setState({
      ...state,
      draftsByConversation: {
        ...state.draftsByConversation,
        [conversationId]: {
          ...(state.draftsByConversation[conversationId] ?? defaultDraft()),
          ...patch,
        },
      },
    })
  },
  sendDraft(conversationId: string) {
    const draft = state.draftsByConversation[conversationId] ?? defaultDraft()
    if (!draft.text && !draft.attachmentName && !draft.voiceDraftLabel) {
      return
    }

    const nextMessage: MessageRecord = {
      id: `msg-${Date.now()}`,
      author: 'You',
      body: draft.text || 'Attachment-only message',
      status: state.networkAvailable ? 'delivered' : 'queued',
      attachmentName: draft.attachmentName,
      voiceDraftLabel: draft.voiceDraftLabel,
      queuedSeconds: 0,
    }

    setState({
      ...state,
      messagesByConversation: {
        ...state.messagesByConversation,
        [conversationId]: [...getConversationMessages(conversationId), nextMessage],
      },
      draftsByConversation: {
        ...state.draftsByConversation,
        [conversationId]: defaultDraft(),
      },
      cleanupFeedback: state.networkAvailable
        ? 'Message synced.'
        : 'Message saved locally and queued.',
    })
  },
  setNetworkAvailable(available: boolean) {
    const updatedMessages = Object.fromEntries(
      Object.entries(state.messagesByConversation).map(([conversationId, messages]) => [
        conversationId,
        messages.map((message) =>
          available && message.status === 'queued'
            ? { ...message, status: 'delivered', queuedSeconds: 0 }
            : message,
        ),
      ]),
    ) as Record<string, MessageRecord[]>

    setState({
      ...state,
      networkAvailable: available,
      messagesByConversation: updatedMessages,
      cleanupFeedback: available
        ? 'Queued messages retried automatically.'
        : 'Network limited. New sends will queue safely.',
    })
  },
  advanceFailureTimer(seconds: number) {
    const updatedMessages = Object.fromEntries(
      Object.entries(state.messagesByConversation).map(([conversationId, messages]) => [
        conversationId,
        messages.map((message) => {
          if (message.status !== 'queued') {
            return message
          }

          const queuedSeconds = message.queuedSeconds + seconds
          if (queuedSeconds > 180) {
            return { ...message, status: 'failed', queuedSeconds }
          }

          return { ...message, queuedSeconds }
        }),
      ]),
    ) as Record<string, MessageRecord[]>

    setState({
      ...state,
      messagesByConversation: updatedMessages,
      cleanupFeedback: 'Long-stalled queued messages now require attention.',
    })
  },
  retryMessage(messageId: string) {
    const updatedMessages = Object.fromEntries(
      Object.entries(state.messagesByConversation).map(([conversationId, messages]) => [
        conversationId,
        messages.map((message) => {
          if (message.id !== messageId) {
            return message
          }

          return {
            ...message,
            status: state.networkAvailable ? 'delivered' : 'queued',
            queuedSeconds: 0,
          }
        }),
      ]),
    ) as Record<string, MessageRecord[]>

    setState({
      ...state,
      messagesByConversation: updatedMessages,
      cleanupFeedback: state.networkAvailable
        ? 'Retry delivered immediately.'
        : 'Retry queued until the connection returns.',
    })
  },
  markReadOnActiveConversation() {
    const conversationId = state.selectedConversationId
    setState({
      ...state,
      messagesByConversation: {
        ...state.messagesByConversation,
        [conversationId]: getConversationMessages(conversationId).map((message) =>
          message.author === 'You' && message.status === 'delivered'
            ? { ...message, status: 'read' }
            : message,
        ),
      },
    })
  },
  setTyping(_isTyping: boolean) {
    setState({
      ...state,
      cleanupFeedback: 'Presence updates stayed behind message delivery in priority.',
    })
  },
  setPhoneLookupEnabled(enabled: boolean) {
    setState({
      ...state,
      privacy: { ...state.privacy, phoneLookupEnabled: enabled },
    })
  },
  cleanupStorage() {
    const removed = state.storageCategories.filter((category) => !category.protected)
    const protectedCount = state.storageCategories.length - removed.length
    setState({
      ...state,
      storageCategories: state.storageCategories.filter((category) => category.protected),
      cleanupFeedback: `Cleared ${removed.length} cache buckets. Protected ${protectedCount} active items.`,
    })
  },
  async resetForTests() {
    await clearPersistedState()
    state = defaultState()
    emit()
  },
}

export function useChatStoreSnapshot() {
  return useSyncExternalStore(
    (listener) => {
      listeners.add(listener)
      return () => listeners.delete(listener)
    },
    () => state,
  )
}
