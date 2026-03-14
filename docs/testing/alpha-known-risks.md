# Alpha Known Risks

## iPhone Home Screen Web App

- The current iPhone surface is the shared web client installed from Safari, not a native SwiftUI app.
- Background execution and system integration are weaker than a native client.
- Push, deep-link, and offline UX need a dedicated later pass before wider rollout.

## Recovery Model

- Recovery enrollment can restore account access without promising historical content access.
- Password reset invalidates password-derived recovery wraps until new recovery material exists.
- This is intentional for the current privacy posture, but user education must be explicit.

## Android Foundation

- The current Android module is a JVM-friendly early skeleton that makes ViewModel and queue behavior testable without a full Android SDK.
- A full Android SDK / Compose integration pass is still needed before store release distribution.

## Desktop Shell

- The Tauri shell currently validates as a thin wrapper around the shared web client.
- Packaging and platform polish remain follow-up work beyond compile validation.

## Media And Presence

- Media and voice flows are still in-memory and service-local in this foundation stage.
- Presence prioritization is implemented as a throttled queue, but not yet connected to a live transport.
- Persistent storage for voice drafts and upload recovery across process restarts still needs a dedicated follow-up.
