package com.messenger.app.data.local

data class DraftEntity(
    val conversationId: String,
    val text: String,
    val attachmentName: String? = null,
    val voiceDraftLabel: String? = null,
)

interface DraftDao {
    fun save(draft: DraftEntity)
    fun load(conversationId: String): DraftEntity?
    fun clear(conversationId: String)
}
