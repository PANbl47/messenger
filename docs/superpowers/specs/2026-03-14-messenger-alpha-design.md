# Messenger Alpha Design

Date: 2026-03-14
Status: Draft approved for planning review

## 1. Purpose

This document defines the first real user-facing release slice for the messenger project.

The goal is not a toy prototype. The goal is a private, calm, premium-feeling alpha messenger that already supports real day-to-day 1:1 communication under weak or unstable network conditions.

This spec covers the user messenger only. Admin/backoffice is intentionally excluded and will be designed as a separate product stream.

## 2. Product Goal

The alpha release must let two real people communicate reliably across:

- Android as the main early app client
- iPhone through Safari home screen web app
- desktop through web and desktop executable

The experience must feel:

- calm
- clear
- reliable
- premium
- weak-network-aware
- private by default

The main differentiator in alpha is not feature count. It is the feeling that messages are safe, understandable, and under control even when connectivity is poor.

## 3. Alpha Scope

The alpha includes:

- account creation and sign-in
- phone-first identity with optional login/password path
- unique login
- unique username
- separate display name
- optional email attachment for password reset
- multi-device support from day one
- full history sync to newly authorized devices
- private 1:1 chats only
- text messages
- media and file attachments
- voice messages
- complete draft restoration
- automatic retry after reconnect
- explicit failure state after 3 minutes without success
- message edit
- delete for self
- delete for everyone
- replies
- forwarding
- search and discovery by username, display name, and phone when allowed by privacy settings
- presence and typing indicators
- storage management controls
- end-to-end encryption for text, media, and voice

The alpha excludes:

- groups
- calls
- reactions
- public channels
- public discovery layer
- admin panel

## 4. Platform Strategy

The alpha uses a hybrid platform strategy:

- Android is the primary early application client
- iPhone uses a Safari home screen web app as the early distribution path
- desktop is available as both web and executable

### Alpha platform exception

`ARCHITECTURE.md` remains the long-term source of truth for target clients:

- Android: Kotlin + Jetpack Compose
- iOS: Swift + SwiftUI
- Desktop: Tauri + Rust

For alpha, there is one intentional exception:

- iPhone ships first as a Safari home screen web app because of near-term distribution constraints

Desktop executable remains aligned with the long-term architecture:

- the alpha desktop executable is a Tauri shell around the same web client used for desktop web

### Launch surface priority

The planning sequence must treat surfaces differently:

- `Primary launch surfaces`: shared backend/core contracts, Android app, shared web client used by iPhone home screen web app and desktop web
- `Secondary alpha surface`: Tauri desktop executable packaging of the same web client after the web client is stable

All clients must share the same functional contract and domain semantics:

- same account model
- same device model
- same message states
- same delivery semantics
- same draft semantics
- same privacy rules

UI does not need to be visually identical across platforms. It must be natively comfortable on each platform while preserving one coherent product system.

## 5. Core User Scenarios

The alpha must handle these scenarios well:

1. A user signs up with phone or with login/password and completes a usable account.
2. A user finds another person by username, display name, or phone when privacy policy allows it.
3. A user starts a private 1:1 conversation.
4. A user sends a text while offline or on unstable connectivity and trusts that it will retry automatically.
5. A user sends a file or voice message without the app feeling chaotic.
6. A user opens the account on a second device and sees the same account history and state.
7. A user edits, deletes, replies to, and forwards messages without confusion.
8. A user understands typing, online state, and message state without visual noise.
9. A user manages local storage and clears unnecessary cached data.

## 6. Identity Model

The account model is account-centric, not device-centric.

### Identity fields

- `phone`: unique when attached, recommended sign-up path
- `login`: unique, optional sign-in identifier
- `password`: account secret when login/password mode is used
- `email`: optional, used for password reset and account operations
- `username`: unique public handle used in search and addressing
- `display_name`: non-unique presentation name

### Identity rules

- One phone maps to one account.
- One login maps to one account.
- One username maps to one account.
- Display names may repeat freely.
- Phone-first sign-up is the recommended path.
- Login/password remains a first-class alternate sign-in path.
- Phone, login, and email may all be linked to one account over time.
- `username` is mandatory before the account becomes fully active for search and chat.
- `display_name` is mandatory during onboarding.

### Alpha account creation matrix

1. `Phone-first account`
   - required at creation: verified phone, username, display name
   - optional at creation: login, password, email
   - may add login/password and email later
2. `Login-first account`
   - required at creation: unique login, password, username, display name
   - optional at creation: phone, email
   - may add phone and email later

An account without a password may still function, but password-backed recovery-dependent features must not be assumed to work after total device loss until a recovery credential is established.

### Discovery rules

- Search by username is always supported.
- Search by display name is supported, but returns ranked results rather than pretending names are unique.
- Search by phone is supported only if the target user's privacy settings permit it.
- Phone search is presented as a recommended option during phone-first onboarding, but remains user-controllable.
- If a phone-registered mobile user grants access, the app may offer contact import to discover existing users.
- Desktop should treat contact access as optional capability, not a required flow.

### Alpha inbound and discoverability rules

- Default inbound rule: any user may start a 1:1 chat with another user.
- Default username discoverability: enabled.
- Default display-name discoverability: enabled through ranked search results.
- Default phone discoverability: explicitly chosen by the user during onboarding and changeable later in settings.
- Alpha does not include request/approve gates for new conversations.

## 7. Privacy and Security Model

Private 1:1 chats are end-to-end encrypted by default.

The server may route, queue, and store encrypted payloads, but it must not require access to plaintext message content.

### Security posture

- No self-invented cryptography
- Honest privacy claims
- Metadata minimization where practical
- Admins cannot read user chat content through support tooling

### Recovery posture

This product explicitly prioritizes private content ownership over operator recovery of content.

- Account access recovery is allowed.
- Content recovery by admins is not allowed.
- Support operations may modify account metadata such as login or email after user verification.
- Support operations may not expose private messages or media.

### Important design inference

The product goal is that a newly authorized device receives the user's history automatically. To preserve E2EE while keeping operators unable to read content, the system must use an encrypted account-level key distribution or backup scheme where the server stores only encrypted material and wrapped keys, not readable conversation data. Password reset may restore account access without guaranteeing readable historical content if the user no longer possesses the required cryptographic recovery material.

This inference is required to reconcile:

- E2EE for all private content
- full-history sync on new devices
- no admin access to chat content

## 8. Multi-Device Model

Multi-device is required from the first release.

The account is a set of trusted devices, not a single magical cloud session.

### Required capabilities

- add device
- remove device
- list devices
- show device metadata
- show last active time
- show trust status
- sync account history and state to newly authorized devices

### Trust and enrollment contract

- A newly signed-in device is not trusted automatically.
- If an existing trusted device is available, device enrollment should be approved by that trusted device.
- If no trusted device is available, enrollment must use the defined account recovery path and can restore account access without pretending content access is always guaranteed.
- Device addition must create a new device record, trust state, session set, and wrapped key material set.

### Key ownership and wrapping contract

- Each account has an account-level encrypted material set used to unlock synced content on trusted devices.
- The server may store wrapped key material and encrypted payloads only.
- Each trusted device gets its own wrapped access to the account-level encrypted material.
- Recovery-capable credentials may hold an additional recovery wrap when the user has configured them.

### Rotation, revocation, and reset rules

- Removing a device revokes future sync and future key delivery to that device.
- Removing a device does not guarantee deletion of content already decrypted and stored locally on that device.
- Password reset must invalidate password-derived recovery access until new recovery material is established.
- If a user loses all trusted devices and no valid recovery-capable material remains, account access may return before historical content access returns.

### UX principles

- Device trust must be visible.
- Device addition must not be hidden.
- New device onboarding must feel secure but not intimidating.
- The user should understand that devices are members of the account trust set.

## 9. Conversation and Messaging Scope

The alpha is strictly:

- private
- one-to-one
- message-centric

Each conversation supports:

- text
- file and media attachments
- voice messages
- full drafts
- replies
- forwarding
- editing
- deletion for self
- deletion for everyone
- presence and typing

Out of scope for alpha:

- groups
- calls
- reactions

## 10. Message Lifecycle

The UI should not expose a noisy technical state machine, but the domain model must support one.

### Internal message states

- `draft`
- `local_saved`
- `queued`
- `sending`
- `sent`
- `read`
- `failed_retryable`

### UI interpretation

The user primarily sees three calm, recognizable visual states:

- message has a problem and needs attention
- message has been sent
- message has been read

Before an error is shown, the system retries automatically after reconnect or temporary transport recovery.

### Failure rule

- If a message has not completed successfully after 3 minutes, it moves to an explicit error state in the chat.
- The failed message remains in place in the timeline.
- The user can retry from the message itself.

## 11. Draft Model

Drafts are full conversation drafts, not text-only scratch buffers.

A draft may contain:

- text
- selected attachments
- prepared voice messages
- reply context if relevant

Drafts must survive:

- navigation away from the chat
- application restart
- temporary connectivity loss

Local persistence is mandatory.

## 12. Media and Voice Behavior

### Attachments

Text plus attachment is one logical message in the user experience.

This means:

- one message bubble
- one retry concept
- one send action
- one visible status outcome

Under the hood, the pipeline may still use multiple technical phases such as encrypting, uploading, and committing message metadata.

When connectivity is poor and attachments are large, the user must be able to choose:

- send as-is
- compress or optimize first

### Voice messages

Voice messages require a rich composer experience in alpha:

- record gesture designed for comfort
- preview before send
- cancel or discard
- re-record

Voice handling must not make the composer feel overloaded.

### Media and voice failure contract

- user cancellation before final send commit must leave the draft editable and must not create a ghost message
- interrupted upload must resume or retry without duplicating the logical message
- expired upload tickets must be renewed transparently before the message is marked failed
- backgrounding during upload or recording must preserve recoverable local state
- storage-full conditions must produce a clear local error before pretending the message is queued successfully
- text-plus-attachment duplicate prevention must be enforced at the logical message level, not per transport step

## 13. Presence and Typing

Presence and typing are part of alpha, but are lower priority than message delivery.

Rules:

- presence must never interfere with message transport reliability
- typing must be lightweight and suppressible under poor conditions
- text delivery remains higher priority than presence signaling

The UI should be informative but calm, not chatty.

## 14. Privacy Settings

The alpha must include a minimal but real privacy settings layer.

At minimum, users must be able to control:

- whether they can be found by phone
- whether phone discoverability is enabled at all
- whether they accept the default open inbound rule or a stricter inbound rule when stricter controls are added later

The baseline rule for alpha is:

- anyone may message a user by default
- the privacy settings model must be designed so stricter inbound limitations can be introduced without changing the core identity or messaging architecture

## 15. Storage Management

The product must expose local storage management from the first release.

Users should be able to:

- see what categories consume space
- clear cached or local media data
- understand that clearing local storage should not silently destroy server-synced account state

Storage management is especially important because the alpha already includes media, voice, and multi-device history.

### Storage safety matrix

- `Safe to delete`: thumbnails, downloaded media cache, temporary upload files, temporary voice-processing files
- `Protected from one-tap cleanup`: queued outgoing messages, unsent drafts, pending edits, local device trust material, local auth/session material
- `Conditionally re-downloadable`: synced encrypted history and synced attachments, but only when the account and device still have the required authorization and cryptographic access

The storage UI must distinguish cache from local-only critical state so the user cannot accidentally destroy unsent work.

## 16. System Architecture

The product should be designed as a local-first, account-centric, device-aware system.

### Client layers

Each client should contain:

1. presentation layer
2. UI state layer
3. local database
4. draft storage
5. domain layer
6. sync engine
7. transport layer
8. crypto and device trust layer
9. media and voice pipeline

### Client architectural rules

- Local database is the source of truth for UI rendering.
- UI should render local state first, then converge with sync state.
- Drafts, queued messages, and pending edits must be representable locally without server confirmation.
- Weak-network behavior must be designed into the sync engine, not bolted onto the UI.

### Backend service boundaries

The alpha backend should be split into clear units with explicit ownership:

- `Auth/Account Service`
  - owns: account record, linked identity methods, password/email recovery metadata, account support metadata
  - exposes: sign-in, identity linking, password reset, account metadata mutation
  - does not own: search index, device trust records, message payloads
- `Directory/Search Service`
  - owns: searchable user projection, username index, display-name search index, phone discoverability projection
  - exposes: user lookup and search subject to privacy rules
  - depends on: account projections from Auth/Account Service
- `Device/Key Service`
  - owns: device records, trust state, wrapped key material, device enrollment and revocation state
  - exposes: add/remove/list devices, trust operations, wrapped material retrieval for authorized flows
  - does not own: message delivery state or search records
- `Messaging Service`
  - owns: encrypted message envelopes, queueing, acknowledgements, edit/delete/reply/forward events, delivery state
  - exposes: message send and synchronization APIs
  - depends on: authorized device context and encrypted references from Device/Key Service, media references from Media Service
- `Media Service`
  - owns: upload tickets, encrypted blob references, media lifecycle metadata
  - exposes: encrypted upload/download preparation and attachment resolution
- `Presence Service`
  - owns: online presence and typing state
  - exposes: lightweight presence signaling
- `Notification Gateway`
  - owns: push routing and wake-up delivery only
  - exposes: notification dispatch

### Pipeline separation

The architecture must keep these flows separate enough that they do not interfere with each other:

- text delivery
- attachment upload and retrieval
- voice preparation and upload
- typing and presence updates
- full-history sync

Even when the product treats a text-plus-attachment as one visible message, the implementation must avoid letting media activity degrade core text reliability.

## 17. Error Handling and Weak-Network Behavior

Weak-network behavior is a first-class architectural concern.

The product must:

- remain usable while offline
- preserve user actions locally before network success
- retry automatically after reconnect
- avoid duplicate message creation during retries
- keep failed items understandable and recoverable

The user experience in bad connectivity should communicate:

- the message is safe
- the app is still in control
- retry is happening automatically
- user attention is needed only when automatic recovery has failed long enough

## 18. Search and List Presentation Rules

Because display names are not unique, search results must disambiguate users with additional identifiers.

Search results should combine:

- display name
- username
- avatar if present
- optional contact cues when allowed

The interface must not pretend repeated names are unique.

## 19. Delivery and Interaction Rules

### Sending

- Messages save locally first.
- Automatic retry is the default behavior.
- Failed messages stay in the chat timeline.

### Editing

- Edited messages must synchronize across devices.
- The UI must show that a message was edited without visual clutter.

### Deletion

- Delete for self and delete for everyone are both required.

### Reply and forward

- Replies must retain clear context.
- Forwarding must preserve message intent without making the timeline confusing.

## 20. Testing and Verification Requirements

This design is only valid if it can be verified through realistic scenarios.

Planning and implementation must include tests for:

- sign-up and sign-in across identity methods
- linking phone, login, and email to one account
- multi-device authorization and history sync
- sending while offline
- automatic retry after reconnect
- explicit failure after 3 minutes
- duplicate prevention during retries
- full draft restoration
- attachment send flows
- voice preview and send flows
- edit/delete/reply/forward synchronization
- privacy-controlled search by phone
- storage cleanup behavior
- presence and typing degradation under weak connectivity

Verification must include both:

- domain and protocol tests
- end-to-end user scenario tests on target clients

## 21. Alpha Boundaries

This alpha should be treated as one coherent release slice, not as the whole product roadmap.

The release is complete when a small real group of users can:

- create accounts
- find each other
- chat privately across devices
- trust message safety under bad network conditions
- use text, files, and voice comfortably

The release is not blocked on:

- groups
- calls
- reactions
- channels
- admin panel

## 22. Major Risks

The highest-risk areas for implementation planning are:

1. reconciling E2EE, full-history sync, and operator inability to read content
2. making multi-device work from day one without hidden trust shortcuts
3. preserving calm UX while supporting full drafts, retries, voice, and attachments
4. keeping text reliability high while media and presence layers exist
5. delivering an acceptable iPhone web app experience as an early distribution path

## 23. Planning Guidance

The implementation plan should decompose this alpha into bounded workstreams, but should preserve the release contract described here.

The best planning shape is likely:

- identity and account foundation
- device and key foundation
- local-first chat and sync foundation
- text delivery foundation
- media and voice layer
- search and privacy layer
- multi-device polish and verification

Admin/backoffice must not be merged into this plan. It belongs to a separate design and planning cycle.
