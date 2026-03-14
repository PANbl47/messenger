# Messenger Alpha Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the first working messenger alpha slice with account-centric identity, multi-device trust, E2EE-ready history sync, private 1:1 chat, offline-safe delivery, and primary client surfaces for Android plus the shared web client used by iPhone home screen web app and desktop web.

**Architecture:** Implement the alpha as a modular monorepo. Keep logical backend services as separate Rust crates behind one initial gateway binary, keep shared protocol and crypto rules in `core/rust-core`, build one shared web client for iPhone home screen web app and desktop web, then package that same web client in Tauri as the secondary desktop executable surface.

**Tech Stack:** Rust, Axum, Tokio, SQLx, PostgreSQL, object storage, WebSocket, React, TypeScript, Vite, Tauri, Kotlin, Jetpack Compose, Android Room, Vitest, Playwright, JUnit, GitHub Actions

---

## Implementation Strategy

This spec spans several large surfaces, so execution should follow the approved priority from the design:

- primary first: shared backend/core contracts, Android app, shared web client
- secondary later in alpha: Tauri desktop packaging

This plan still lives in one document because the alpha is one coherent release slice, but it is intentionally decomposed into independently testable milestones. Start with the highest-risk slice first:

1. account identity plus device trust and encrypted history access
2. text messaging plus offline queue and sync
3. media, voice, presence, and storage safety
4. client surfaces and end-to-end verification

Use `@test-driven-development`, `@backend-testing`, `@ui-design`, and `@verification-before-completion` during execution.

## File Structure Map

### Root workspace

- Create: `Cargo.toml`
- Create: `rust-toolchain.toml`
- Create: `.github/workflows/ci.yml`
- Create: `scripts/bootstrap-dev.ps1`
- Create: `scripts/check.ps1`

### Shared Rust core

- Create: `core/rust-core/Cargo.toml`
- Create: `core/rust-core/crates/domain-model/src/lib.rs`
- Create: `core/rust-core/crates/identity-contracts/src/lib.rs`
- Create: `core/rust-core/crates/device-trust/src/lib.rs`
- Create: `core/rust-core/crates/message-protocol/src/lib.rs`
- Create: `core/rust-core/crates/sync-engine/src/lib.rs`
- Create: `core/rust-core/crates/media-pipeline/src/lib.rs`

### Backend

- Create: `backend/gateway/Cargo.toml`
- Create: `backend/gateway/src/main.rs`
- Create: `backend/gateway/src/router.rs`
- Create: `backend/crates/account-service/src/lib.rs`
- Create: `backend/crates/directory-service/src/lib.rs`
- Create: `backend/crates/device-key-service/src/lib.rs`
- Create: `backend/crates/messaging-service/src/lib.rs`
- Create: `backend/crates/media-service/src/lib.rs`
- Create: `backend/crates/presence-service/src/lib.rs`
- Create: `backend/crates/persistence/src/lib.rs`
- Create: `backend/migrations/0001_initial.sql`

### Shared web client and desktop web surface

- Create: `apps/web/package.json`
- Create: `apps/web/vite.config.ts`
- Create: `apps/web/src/main.tsx`
- Create: `apps/web/src/app/App.tsx`
- Create: `apps/web/src/app/router.tsx`
- Create: `apps/web/src/features/auth/`
- Create: `apps/web/src/features/chat/`
- Create: `apps/web/src/features/settings/`
- Create: `apps/web/src/lib/api/`
- Create: `apps/web/src/lib/state/`
- Create: `apps/web/src/lib/storage/`

### Desktop executable

- Create: `apps/desktop/src-tauri/Cargo.toml`
- Create: `apps/desktop/src-tauri/src/main.rs`
- Create: `apps/desktop/src-tauri/tauri.conf.json`

### Android

- Create: `apps/android/settings.gradle.kts`
- Create: `apps/android/build.gradle.kts`
- Create: `apps/android/app/build.gradle.kts`
- Create: `apps/android/app/src/main/java/com/messenger/app/MainActivity.kt`
- Create: `apps/android/app/src/main/java/com/messenger/app/navigation/AppNavHost.kt`
- Create: `apps/android/app/src/main/java/com/messenger/app/features/auth/`
- Create: `apps/android/app/src/main/java/com/messenger/app/features/chat/`
- Create: `apps/android/app/src/main/java/com/messenger/app/features/settings/`
- Create: `apps/android/app/src/main/java/com/messenger/app/data/local/`
- Create: `apps/android/app/src/main/java/com/messenger/app/data/remote/`

## Chunk 1: Foundation, Identity, And Trusted Device Access

### Task 1: Bootstrap The Monorepo And CI Guardrails

**Files:**
- Create: `Cargo.toml`
- Create: `rust-toolchain.toml`
- Create: `.github/workflows/ci.yml`
- Create: `scripts/bootstrap-dev.ps1`
- Create: `scripts/check.ps1`
- Modify: `README.md`
- Test: `backend/gateway/tests/health_check.rs`

- [ ] **Step 1: Write the failing backend smoke test**

```rust
#[tokio::test]
async fn health_endpoint_returns_ok() {
    let response = messenger_gateway::test_support::health().await;
    assert_eq!(response.status(), 200);
}
```

- [ ] **Step 2: Run the test to confirm the workspace is not ready yet**

Run: `cargo test -p gateway --test health_check`
Expected: FAIL because the workspace and gateway crate do not exist yet.

- [ ] **Step 3: Create the Rust workspace and the gateway crate skeleton**

Create a root workspace that includes `backend/gateway` and `core/rust-core` crates. Add a minimal `health` route plus a `test_support` helper so the smoke test can call an in-process app instead of requiring a live server.

- [ ] **Step 4: Add the developer scripts and CI entrypoint**

`scripts/check.ps1` should run Rust tests first, then client tests once those directories exist. Keep the script idempotent so later tasks can extend it without replacing earlier logic.

- [ ] **Step 5: Re-run the smoke test**

Run: `cargo test -p gateway --test health_check`
Expected: PASS with one passing test.

- [ ] **Step 6: Commit**

```bash
git add Cargo.toml rust-toolchain.toml .github/workflows/ci.yml scripts/bootstrap-dev.ps1 scripts/check.ps1 README.md backend/gateway
git commit -m "chore: bootstrap messenger workspace"
```

### Task 2: Lock The Shared Domain Contracts Before Building Services

**Files:**
- Create: `core/rust-core/Cargo.toml`
- Create: `core/rust-core/crates/domain-model/src/lib.rs`
- Create: `core/rust-core/crates/identity-contracts/src/lib.rs`
- Create: `core/rust-core/crates/device-trust/src/lib.rs`
- Create: `core/rust-core/crates/message-protocol/src/lib.rs`
- Test: `core/rust-core/crates/domain-model/tests/message_state_machine.rs`
- Test: `core/rust-core/crates/identity-contracts/tests/account_identity_rules.rs`
- Test: `core/rust-core/crates/device-trust/tests/device_trust_contract.rs`

- [ ] **Step 1: Write failing tests for the shared contracts**

```rust
#[test]
fn message_state_transitions_allow_retryable_failure_only_after_queueing() {}

#[test]
fn account_identity_rejects_duplicate_login_username_or_phone() {}

#[test]
fn new_signed_in_device_is_untrusted_until_enrolled() {}
```

- [ ] **Step 2: Run the shared-core test targets**

Run: `cargo test -p domain-model -p identity-contracts -p device-trust`
Expected: FAIL because the crates and types do not exist yet.

- [ ] **Step 3: Implement the minimal contract types**

Define:
- account identity value objects
- message lifecycle enums
- draft payload shapes
- device trust states
- wrapped-key reference types

Keep these crates free of transport and database code so both backend and clients can depend on them safely.

- [ ] **Step 4: Add serialization compatibility tests**

Write focused tests that round-trip the shared message and identity types through `serde` to prevent backend/client drift later.

- [ ] **Step 5: Re-run the shared-core suite**

Run: `cargo test -p domain-model -p identity-contracts -p device-trust -p message-protocol`
Expected: PASS with all contract tests green.

- [ ] **Step 6: Commit**

```bash
git add core/rust-core
git commit -m "feat: define messenger shared domain contracts"
```

### Task 3: Implement Account Identity And Searchable User Directory

**Files:**
- Create: `backend/crates/account-service/src/lib.rs`
- Create: `backend/crates/account-service/src/model.rs`
- Create: `backend/crates/account-service/src/repository.rs`
- Create: `backend/crates/directory-service/src/lib.rs`
- Create: `backend/crates/directory-service/src/projection.rs`
- Create: `backend/crates/persistence/src/lib.rs`
- Create: `backend/migrations/0001_initial.sql`
- Modify: `backend/gateway/src/router.rs`
- Test: `backend/crates/account-service/tests/account_registration.rs`
- Test: `backend/crates/directory-service/tests/search_visibility.rs`
- Test: `backend/gateway/tests/account_and_search_routes.rs`

- [ ] **Step 1: Write the failing account and search tests**

```rust
#[tokio::test]
async fn phone_first_registration_requires_verified_phone_username_and_display_name() {}

#[tokio::test]
async fn login_first_registration_requires_unique_login_password_username_and_display_name() {}

#[tokio::test]
async fn phone_first_account_can_link_login_and_password_later() {}

#[tokio::test]
async fn login_first_account_can_link_phone_later() {}

#[tokio::test]
async fn username_search_returns_exact_unique_match() {}

#[tokio::test]
async fn display_name_search_returns_ranked_disambiguated_results() {}

#[tokio::test]
async fn phone_search_respects_target_privacy_setting() {}

#[tokio::test]
async fn link_email_persists_recovery_metadata_without_exposing_private_content() {}
```

- [ ] **Step 2: Run the targeted backend tests**

Run: `cargo test -p account-service -p directory-service`
Expected: FAIL because the registration and search flows are not implemented.

- [ ] **Step 3: Implement persistence and service modules**

Create tables and repositories for:
- accounts
- linked identity methods
- usernames
- display-name search projection
- phone discoverability settings

Expose minimal gateway routes for:
- sign-up
- sign-in bootstrap
- link-email
- link-phone-to-login-first-account
- link-login-password-to-phone-first-account
- username and display-name search
- phone search subject to privacy rules

- [ ] **Step 4: Add integration coverage for uniqueness constraints**

Verify that duplicate `login`, `username`, and attached `phone` values are rejected deterministically with stable error codes that clients can map cleanly.

- [ ] **Step 5: Add gateway route integration tests**

Verify the HTTP contract for:
- phone-first sign-up
- login-first sign-up
- link-email
- linking phone or login later
- username search
- ranked display-name search
- phone search with privacy control

- [ ] **Step 6: Re-run the backend identity suite**

Run: `cargo test -p account-service -p directory-service -p gateway --test account_and_search_routes`
Expected: PASS with registration, linking, and privacy search tests green.

- [ ] **Step 7: Commit**

```bash
git add backend/crates/account-service backend/crates/directory-service backend/crates/persistence backend/migrations/0001_initial.sql backend/gateway/src/router.rs
git commit -m "feat: add account identity and directory services"
```

### Task 4: Implement Device Trust, Wrapped Keys, And History Access Preconditions

**Files:**
- Create: `backend/crates/device-key-service/src/lib.rs`
- Create: `backend/crates/device-key-service/src/enrollment.rs`
- Create: `backend/crates/device-key-service/src/recovery.rs`
- Modify: `backend/gateway/src/router.rs`
- Test: `backend/crates/device-key-service/tests/device_enrollment.rs`
- Test: `backend/crates/device-key-service/tests/password_reset_recovery.rs`
- Test: `core/rust-core/crates/device-trust/tests/wrapped_history_access.rs`
- Test: `backend/gateway/tests/device_routes.rs`

- [ ] **Step 1: Write failing tests for trusted-device enrollment**

```rust
#[tokio::test]
async fn second_device_requires_existing_trusted_device_approval_when_available() {}

#[tokio::test]
async fn no_trusted_device_recovery_restores_account_access_but_not_guaranteed_history_access() {}

#[tokio::test]
async fn password_reset_invalidates_password_derived_recovery_wraps() {}

#[test]
fn history_access_requires_authorized_device_plus_wrapped_account_material() {}

#[tokio::test]
async fn list_devices_returns_metadata_trust_state_and_last_active_time() {}
```

- [ ] **Step 2: Run the device-trust suite**

Run: `cargo test -p device-key-service -p device-trust`
Expected: FAIL because enrollment, revocation, and recovery semantics are not implemented.

- [ ] **Step 3: Implement the device and wrapped-key data model**

Create data structures and storage for:
- device records
- trust state
- enrollment approvals
- wrapped account key material references
- password-derived recovery material status
- last-active timestamps for each device

- [ ] **Step 4: Wire gateway routes for add-device, remove-device, list-devices, and recovery-enrollment**

Return only metadata and trust state. Do not expose decrypted content or raw key material in API responses.

- [ ] **Step 5: Add gateway route integration tests**

Verify:
- add-device approval flow
- remove-device flow
- list-devices includes last activity
- no-trusted-device recovery enrollment path

- [ ] **Step 6: Re-run the device-trust suite**

Run: `cargo test -p device-key-service -p device-trust -p gateway --test device_routes`
Expected: PASS with device approval, revocation, recovery, and reset tests green.

- [ ] **Step 7: Commit**

```bash
git add backend/crates/device-key-service backend/gateway/src/router.rs core/rust-core/crates/device-trust
git commit -m "feat: add trusted device enrollment and key wrapping"
```

## Chunk 2: Messaging, Rich Media, Clients, And Release Verification

### Task 5: Implement Offline-Safe Text Messaging And Sync

**Files:**
- Create: `backend/crates/messaging-service/src/lib.rs`
- Create: `backend/crates/messaging-service/src/queue.rs`
- Create: `backend/crates/messaging-service/src/sync.rs`
- Create: `core/rust-core/crates/sync-engine/src/lib.rs`
- Test: `backend/crates/messaging-service/tests/offline_retry.rs`
- Test: `backend/crates/messaging-service/tests/edit_delete_reply_forward.rs`
- Test: `core/rust-core/crates/sync-engine/tests/message_retry_timing.rs`

- [ ] **Step 1: Write failing text-message delivery tests**

```rust
#[tokio::test]
async fn queued_message_retries_automatically_after_reconnect() {}

#[tokio::test]
async fn message_becomes_retryable_failure_after_three_minutes() {}

#[tokio::test]
async fn edit_delete_reply_and_forward_events_sync_to_other_devices() {}
```

- [ ] **Step 2: Run the messaging test targets**

Run: `cargo test -p messaging-service -p sync-engine`
Expected: FAIL because queueing, retry timing, and sync events are not implemented.

- [ ] **Step 3: Implement the messaging queue and synchronization logic**

Support:
- local-save-first semantics
- automatic retry after reconnect
- logical-message duplicate prevention
- timeline-stable failed state
- edit, delete, reply, and forward events

- [ ] **Step 4: Add a WebSocket contract test**

Verify that a second device receives message, edit, and deletion events in the same order the service persists them.

- [ ] **Step 5: Re-run the messaging suite**

Run: `cargo test -p messaging-service -p sync-engine`
Expected: PASS with retry, timeout, and sync behavior green.

- [ ] **Step 6: Commit**

```bash
git add backend/crates/messaging-service core/rust-core/crates/sync-engine
git commit -m "feat: add offline-safe messaging and sync"
```

### Task 6: Implement Media, Voice, Presence, And Storage Safety

**Files:**
- Create: `backend/crates/media-service/src/lib.rs`
- Create: `backend/crates/media-service/src/upload_tickets.rs`
- Create: `backend/crates/presence-service/src/lib.rs`
- Create: `core/rust-core/crates/media-pipeline/src/lib.rs`
- Test: `backend/crates/media-service/tests/attachment_retry.rs`
- Test: `backend/crates/media-service/tests/voice_draft_recovery.rs`
- Test: `backend/crates/presence-service/tests/presence_priority.rs`
- Test: `core/rust-core/crates/media-pipeline/tests/storage_cleanup_guards.rs`

- [ ] **Step 1: Write failing media and presence tests**

```rust
#[tokio::test]
async fn interrupted_attachment_upload_retries_without_duplicate_message_creation() {}

#[tokio::test]
async fn cancelled_voice_send_returns_to_editable_draft_state() {}

#[tokio::test]
async fn presence_updates_do_not_block_message_delivery() {}
```

- [ ] **Step 2: Run the service tests**

Run: `cargo test -p media-service -p presence-service -p media-pipeline`
Expected: FAIL because upload tickets, voice recovery, and storage safety guards are not implemented.

- [ ] **Step 3: Implement media and voice pipeline rules**

Support:
- renewable upload tickets
- interrupted upload recovery
- storage-full guardrails
- background-safe voice draft persistence
- one logical message lifecycle for text plus attachment

- [ ] **Step 4: Implement presence as a lower-priority signal path**

Batch or throttle typing updates so transport pressure on presence never delays durable message delivery.

- [ ] **Step 5: Add storage cleanup protection tests**

Verify that cache cleanup never deletes queued messages, unsent drafts, pending edits, or local trust material.

- [ ] **Step 6: Re-run the suite**

Run: `cargo test -p media-service -p presence-service -p media-pipeline`
Expected: PASS with media retry, voice recovery, presence priority, and storage safety tests green.

- [ ] **Step 7: Commit**

```bash
git add backend/crates/media-service backend/crates/presence-service core/rust-core/crates/media-pipeline
git commit -m "feat: add media voice presence and storage safety"
```

### Task 7: Build The Shared Web Client For Auth, Chat, And Settings

**Files:**
- Create: `apps/web/package.json`
- Create: `apps/web/vite.config.ts`
- Create: `apps/web/src/main.tsx`
- Create: `apps/web/src/app/App.tsx`
- Create: `apps/web/src/app/router.tsx`
- Create: `apps/web/src/features/auth/AuthShell.tsx`
- Create: `apps/web/src/features/chat/ChatListScreen.tsx`
- Create: `apps/web/src/features/chat/ConversationScreen.tsx`
- Create: `apps/web/src/features/chat/Composer.tsx`
- Create: `apps/web/src/features/settings/PrivacySettingsScreen.tsx`
- Create: `apps/web/src/features/settings/StorageSettingsScreen.tsx`
- Create: `apps/web/src/lib/state/chatStore.ts`
- Create: `apps/web/src/lib/storage/indexedDb.ts`
- Test: `apps/web/src/features/chat/Composer.test.tsx`
- Test: `apps/web/src/features/settings/StorageSettingsScreen.test.tsx`

- [ ] **Step 1: Write failing web UI tests for the highest-value flows**

```tsx
it("shows queued or failed message states in the timeline", () => {});
it("restores a full draft including attachment and voice placeholders", () => {});
it("blocks destructive cleanup for unsent work", () => {});
```

- [ ] **Step 2: Run the web tests**

Run: `pnpm --dir apps/web test -- --run`
Expected: FAIL because the web app and tests do not exist yet.

- [ ] **Step 3: Scaffold the shared web client**

Build:
- auth flow
- chat list
- conversation screen
- composer with draft restoration hooks
- settings screens for privacy and storage

Use calm visual states and explicit delivery icons instead of text-heavy status labels.

- [ ] **Step 4: Connect the client to local-first state and IndexedDB persistence**

Persist drafts, queued messages, and storage metadata locally before any network acknowledgement.

- [ ] **Step 5: Add a browser integration test for offline send then reconnect**

Use Playwright to verify:
- offline message stays visible
- reconnect triggers automatic retry
- three-minute timeout becomes an actionable failure state

- [ ] **Step 6: Re-run unit and browser tests**

Run: `pnpm --dir apps/web test -- --run`
Run: `pnpm --dir apps/web playwright test`
Expected: PASS for unit tests and the offline-retry browser flow.

- [ ] **Step 7: Commit**

```bash
git add apps/web
git commit -m "feat: add shared web messenger client"
```

### Task 8: Build The Android Client And Secondary Desktop Packaging

**Files:**
- Create: `apps/android/settings.gradle.kts`
- Create: `apps/android/build.gradle.kts`
- Create: `apps/android/app/build.gradle.kts`
- Create: `apps/android/app/src/main/java/com/messenger/app/MainActivity.kt`
- Create: `apps/android/app/src/main/java/com/messenger/app/navigation/AppNavHost.kt`
- Create: `apps/android/app/src/main/java/com/messenger/app/features/chat/ConversationViewModel.kt`
- Create: `apps/android/app/src/main/java/com/messenger/app/data/local/AppDatabase.kt`
- Create: `apps/android/app/src/main/java/com/messenger/app/data/local/DraftDao.kt`
- Create: `apps/android/app/src/main/java/com/messenger/app/data/remote/MessengerApi.kt`
- Create: `apps/desktop/src-tauri/Cargo.toml`
- Create: `apps/desktop/src-tauri/src/main.rs`
- Create: `apps/desktop/src-tauri/tauri.conf.json`
- Test: `apps/android/app/src/test/java/com/messenger/app/features/chat/ConversationViewModelTest.kt`
- Test: `apps/android/app/src/androidTest/java/com/messenger/app/OfflineQueueFlowTest.kt`

- [ ] **Step 1: Write failing Android tests for local-first chat behavior**

```kotlin
@Test
fun queued_message_stays_visible_until_retry_succeeds() {}

@Test
fun full_draft_restores_after_process_restart() {}
```

- [ ] **Step 2: Run the Android tests**

Run: `./gradlew :apps:android:app:testDebugUnitTest`
Expected: FAIL because the Android project and view models do not exist yet.

- [ ] **Step 3: Implement the Android app foundation**

Build:
- navigation shell
- auth flow
- chat list and conversation screen
- Compose composer with attachment and voice draft placeholders
- Room-backed local persistence for drafts and queued messages

- [ ] **Step 4: Add Android instrumentation coverage for offline send and retry**

Use a fake transport or mock web socket layer so instrumentation tests can assert queueing, reconnect, and failure-state transitions without relying on flaky live networking.

- [ ] **Step 5: Package the shared web client in Tauri after the web app is stable**

Point Tauri at the already working `apps/web` build output. Do not fork UI logic for desktop at this stage.

- [ ] **Step 6: Re-run Android and desktop packaging checks**

Run: `./gradlew :apps:android:app:testDebugUnitTest`
Run: `cargo check -p desktop-shell`
Expected: PASS for Android unit tests and Tauri compile checks.

- [ ] **Step 7: Commit**

```bash
git add apps/android apps/desktop
git commit -m "feat: add android client and desktop shell"
```

### Task 9: Run End-To-End Verification And Produce Alpha Release Artifacts

**Files:**
- Modify: `.github/workflows/ci.yml`
- Create: `apps/web/playwright.config.ts`
- Create: `docs/testing/alpha-manual-test-checklist.md`
- Create: `docs/testing/alpha-known-risks.md`

- [ ] **Step 1: Write the release-verification checklist first**

Document manual and automated checks for:
- phone-first sign-up
- login-first sign-up
- add second device
- offline send and auto-retry
- failure after three minutes
- edit/delete/reply/forward
- attachment send and voice recovery
- privacy-controlled phone search
- storage cleanup safety

- [ ] **Step 2: Add CI jobs for all implemented surfaces**

Run Rust tests, web unit tests, Playwright flows, and Android unit tests from `.github/workflows/ci.yml`.

- [ ] **Step 3: Run the full automated verification suite**

Run: `powershell -ExecutionPolicy Bypass -File scripts/check.ps1`
Expected: PASS with Rust, web, and Android checks green.

- [ ] **Step 4: Execute the manual alpha checklist with two real accounts and two devices**

Record concrete pass/fail evidence, not just "works on my machine".

- [ ] **Step 5: Write down the remaining known risks**

Capture:
- iPhone home screen web app limitations
- recovery limitations after total trusted-device loss
- any media/presence edge cases deferred past alpha

- [ ] **Step 6: Commit**

```bash
git add .github/workflows/ci.yml docs/testing/alpha-manual-test-checklist.md docs/testing/alpha-known-risks.md apps/web/playwright.config.ts
git commit -m "chore: add alpha verification and release checks"
```

## Execution Notes

- Start with Chunk 1 and do not begin client implementation until Task 4 is green.
- Treat Android plus the shared web client as the primary alpha path.
- Treat Tauri packaging as secondary hardening after the shared web client is already running.
- Do not add admin/backoffice work to this plan.
- Keep commits small and reversible.
- If any planned file starts to grow beyond one clear responsibility, split it before adding more behavior.

## Plan Complete

Plan complete and saved to `docs/superpowers/plans/2026-03-14-messenger-alpha-implementation.md`. Ready to execute?
