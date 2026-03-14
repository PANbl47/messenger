package com.messenger.app.data.remote

data class SendMessageRequest(
    val conversationId: String,
    val text: String,
    val attachmentName: String? = null,
    val voiceDraftLabel: String? = null,
)

interface MessengerApi {
    fun send(request: SendMessageRequest)
}
