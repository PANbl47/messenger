use media_pipeline::{CleanupCandidate, CleanupClass, StorageCleanupGuard};

#[test]
fn cleanup_never_deletes_protected_local_material() {
    let guard = StorageCleanupGuard::default();

    let kept = guard.filter_deletable(vec![
        CleanupCandidate::new("queued-1", CleanupClass::QueuedMessage),
        CleanupCandidate::new("draft-1", CleanupClass::UnsentDraft),
        CleanupCandidate::new("edit-1", CleanupClass::PendingEdit),
        CleanupCandidate::new("trust-1", CleanupClass::LocalTrustMaterial),
        CleanupCandidate::new("cache-1", CleanupClass::EphemeralCache),
    ]);

    assert_eq!(kept, vec![CleanupCandidate::new("cache-1", CleanupClass::EphemeralCache)]);
}
