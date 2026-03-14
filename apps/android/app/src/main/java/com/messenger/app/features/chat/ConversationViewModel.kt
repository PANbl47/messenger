package com.messenger.app.features.chat

import com.messenger.app.data.local.DraftDao
import com.messenger.app.data.local.DraftEntity
import com.messenger.app.data.remote.MessengerApi
import com.messenger.app.data.remote.SendMessageRequest

data class ComposerDraft(
    val text: String = "",
    val attachmentName: String? = null,
    val voiceDraftLabel: String? = null,
)

enum class DeliveryState {
    QUEUED,
    DELIVERED,
}

data class TimelineMessage(
    val id: String,
    val body: String,
    val attachmentName: String? = null,
    val voiceDraftLabel: String? = null,
    val state: DeliveryState,
)

class ConversationViewModel(
    private val conversationId: String,
    private val draftDao: DraftDao,
    private val messengerApi: MessengerApi,
) {
    private val timeline = mutableListOf<TimelineMessage>()
    var draft: ComposerDraft = draftDao.load(conversationId)?.let {
        ComposerDraft(it.text, it.attachmentName, it.voiceDraftLabel)
    } ?: ComposerDraft()
        private set

    fun timeline(): List<TimelineMessage> = timeline.toList()

    fun updateDraft(
        text: String = draft.text,
        attachmentName: String? = draft.attachmentName,
        voiceDraftLabel: String? = draft.voiceDraftLabel,
    ) {
        draft = ComposerDraft(text, attachmentName, voiceDraftLabel)
        draftDao.save(
            DraftEntity(
                conversationId = conversationId,
                text = draft.text,
                attachmentName = draft.attachmentName,
                voiceDraftLabel = draft.voiceDraftLabel,
            ),
        )
    }

    fun sendDraft(networkAvailable: Boolean) {
        if (draft.text.isBlank() && draft.attachmentName == null && draft.voiceDraftLabel == null) {
            return
        }

        val message = TimelineMessage(
            id = "msg-${timeline.size + 1}",
            body = draft.text.ifBlank { "Attachment-only message" },
            attachmentName = draft.attachmentName,
            voiceDraftLabel = draft.voiceDraftLabel,
            state = if (networkAvailable) DeliveryState.DELIVERED else DeliveryState.QUEUED,
        )

        timeline += message

        if (networkAvailable) {
            messengerApi.send(
                SendMessageRequest(
                    conversationId = conversationId,
                    text = message.body,
                    attachmentName = message.attachmentName,
                    voiceDraftLabel = message.voiceDraftLabel,
                ),
            )
        }

        draft = ComposerDraft()
        draftDao.clear(conversationId)
    }

    fun retryQueuedMessages(networkAvailable: Boolean) {
        if (!networkAvailable) return

        val delivered = timeline.map { message ->
            if (message.state == DeliveryState.QUEUED) {
                messengerApi.send(
                    SendMessageRequest(
                        conversationId = conversationId,
                        text = message.body,
                        attachmentName = message.attachmentName,
                        voiceDraftLabel = message.voiceDraftLabel,
                    ),
                )
                message.copy(state = DeliveryState.DELIVERED)
            } else {
                message
            }
        }

        timeline.clear()
        timeline.addAll(delivered)
    }
}
