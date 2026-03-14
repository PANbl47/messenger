package com.messenger.app.features.chat

import com.messenger.app.data.local.DraftDao
import com.messenger.app.data.local.DraftEntity
import com.messenger.app.data.remote.MessengerApi
import com.messenger.app.data.remote.SendMessageRequest
import kotlin.test.Test
import kotlin.test.assertEquals
import kotlin.test.assertNull

class ConversationViewModelTest {
    @Test
    fun queued_message_stays_visible_until_retry_succeeds() {
        val draftDao = InMemoryDraftDao()
        val api = RecordingMessengerApi()
        val viewModel = ConversationViewModel("chat-1", draftDao, api)

        viewModel.updateDraft(text = "Offline hello")
        viewModel.sendDraft(networkAvailable = false)

        assertEquals(1, viewModel.timeline().size)
        assertEquals(DeliveryState.QUEUED, viewModel.timeline().first().state)

        viewModel.retryQueuedMessages(networkAvailable = true)

        assertEquals(DeliveryState.DELIVERED, viewModel.timeline().first().state)
        assertEquals(1, api.requests.size)
    }

    @Test
    fun full_draft_restores_after_process_restart() {
        val draftDao = InMemoryDraftDao()
        val api = RecordingMessengerApi()

        ConversationViewModel("chat-1", draftDao, api).apply {
            updateDraft(
                text = "Recover this",
                attachmentName = "route-map.pdf",
                voiceDraftLabel = "voice-note-01",
            )
        }

        val restored = ConversationViewModel("chat-1", draftDao, api)

        assertEquals("Recover this", restored.draft.text)
        assertEquals("route-map.pdf", restored.draft.attachmentName)
        assertEquals("voice-note-01", restored.draft.voiceDraftLabel)
    }
}

private class InMemoryDraftDao : DraftDao {
    private val drafts = mutableMapOf<String, DraftEntity>()

    override fun save(draft: DraftEntity) {
        drafts[draft.conversationId] = draft
    }

    override fun load(conversationId: String): DraftEntity? = drafts[conversationId]

    override fun clear(conversationId: String) {
        drafts.remove(conversationId)
    }
}

private class RecordingMessengerApi : MessengerApi {
    val requests = mutableListOf<SendMessageRequest>()

    override fun send(request: SendMessageRequest) {
        requests += request
    }
}
