package com.messenger.app

import com.messenger.app.data.local.DraftDao
import com.messenger.app.data.local.DraftEntity
import com.messenger.app.data.remote.MessengerApi
import com.messenger.app.data.remote.SendMessageRequest
import com.messenger.app.features.chat.ConversationViewModel
import com.messenger.app.features.chat.DeliveryState
import kotlin.test.Test
import kotlin.test.assertEquals

class OfflineQueueFlowTest {
    @Test
    fun offline_send_then_retry_keeps_message_stable_in_timeline() {
        val draftDao = IntegrationDraftDao()
        val api = IntegrationMessengerApi()
        val viewModel = ConversationViewModel("chat-android", draftDao, api)

        viewModel.updateDraft(text = "Alpha queue")
        viewModel.sendDraft(networkAvailable = false)
        viewModel.retryQueuedMessages(networkAvailable = true)

        assertEquals(DeliveryState.DELIVERED, viewModel.timeline().single().state)
        assertEquals(1, api.requests.size)
    }
}

private class IntegrationDraftDao : DraftDao {
    private val drafts = mutableMapOf<String, DraftEntity>()

    override fun save(draft: DraftEntity) {
        drafts[draft.conversationId] = draft
    }

    override fun load(conversationId: String): DraftEntity? = drafts[conversationId]

    override fun clear(conversationId: String) {
        drafts.remove(conversationId)
    }
}

private class IntegrationMessengerApi : MessengerApi {
    val requests = mutableListOf<SendMessageRequest>()

    override fun send(request: SendMessageRequest) {
        requests += request
    }
}
