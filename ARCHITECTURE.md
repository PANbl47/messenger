# ARCHITECTURE.md
## Project: Messenger

---

## 1. Project vision

We are building a next-generation privacy-first messenger designed for real-world usage in weak, unstable, throttled, or degraded internet conditions.

This project is not a simplistic Telegram clone.

The goal is to build a messenger that can surpass Telegram in:
- usability
- visual polish
- product coherence
- daily comfort
- weak-network reliability
- clarity of experience

The product must feel:
- premium
- calm
- fast
- modern
- highly usable
- visually clean
- reliable under stress

This messenger must work over the normal internet, across long distances, without relying on Bluetooth or mesh networking as the main transport model.

---

## 2. Core product goal

The main strategic goal is to create a messenger that combines:

1. outstanding user experience
2. strong privacy-first architecture
3. excellent behavior in weak and unstable networks

The product should not feel like a technical demo or security toy.  
It should feel like a world-class consumer product.

---

## 3. What we are not building

We are not building:
- a Bluetooth messenger
- a mesh networking app
- a generic AI-generated chat UI
- a visually noisy productivity app
- a Telegram copy with different colors
- a project based on self-invented cryptography
- a product optimized only for perfect Wi-Fi conditions

---

## 4. Product philosophy

This product should feel like:
- Signal in seriousness
- Telegram in usefulness
- a premium modern consumer app in quality and polish

The messenger must respect the user’s attention and reduce friction.

It should feel:
- quiet
- stable
- trustworthy
- responsive
- effortless
- clear

---

## 5. Main product principles

### 5.1 UX comes first
Users choose messaging apps based on comfort, speed, clarity, and habit.  
Security matters, but UX determines whether they stay.

### 5.2 Weak internet is a primary scenario
Weak, unstable, throttled, or high-latency internet is not an edge case.  
The product must be designed for it from day one.

### 5.3 Privacy by default
Private chats and private groups should be end-to-end encrypted by default.

### 5.4 Calm premium design
The product must be visually clean, elegant, and coherent.  
No generic “AI-generated” UI.

### 5.5 Consistency over improvisation
All screens, flows, and interactions must feel like parts of one system.

---

## 6. Main user scenarios

The product must be excellent in these scenarios:

1. Opening the chat list and immediately understanding what matters.
2. Sending a text message in weak or unstable network conditions.
3. Seeing that a message is safely queued and not lost.
4. Sending media without blocking the main chat flow.
5. Switching between conversations quickly and comfortably.
6. Understanding message state clearly.
7. Understanding account/device/security state clearly.
8. Navigating settings without getting lost.

---

## 7. How we want to surpass Telegram

### 7.1 Better UX
- cleaner composition
- better hierarchy
- less visual noise
- more coherent interaction logic
- more premium feel

### 7.2 Better weak-network behavior
- reliable queued delivery
- graceful retry logic
- text-first prioritization
- adaptive transport behavior
- low-bandwidth mode

### 7.3 Better privacy model
- privacy is not an optional special mode
- device trust is visible and understandable
- server should not read private chat content

### 7.4 Better product coherence
The product must feel intentionally designed, not historically accumulated.

---

## 8. Architectural direction

The system should be composed of:
- polished clients
- secure shared core
- robust store-and-forward backend
- adaptive transport layer
- scalable message delivery infrastructure
- architecture that minimizes server access to sensitive user data

---

## 9. Security philosophy

### 9.1 No self-invented cryptography
Do not invent custom ciphers or homemade crypto protocols.

### 9.2 End-to-end encryption by default
Private chats must use E2EE by default.

### 9.3 Strong device model
Each device is a trusted participant in the account model.

### 9.4 Honest security posture
We do not claim “unbreakable encryption”.
We aim for strong practical real-world security.

---

## 10. Technical principles

### 10.1 Offline-first where appropriate
User actions should be safely represented locally first, then synchronized.

### 10.2 Store-and-forward delivery
The backend should behave as a delivery and queueing system, not as a reader of user content.

### 10.3 Idempotent message handling
Retries must not create duplicates.

### 10.4 Incremental sync
Prefer delta sync and event-based synchronization instead of full resyncs.

### 10.5 Low-bandwidth-first
Text messages have higher priority than media.

### 10.6 Safe multi-device model
Multi-device support must not weaken the trust model silently.

---

## 11. Weak-network-first behavior

The app must:
- remain usable with unstable connectivity
- allow text delivery under high latency
- recover after short disconnects
- minimize unnecessary requests
- keep UI calm and informative
- adapt behavior to network quality

### Low-bandwidth behavior rules
- text is highest priority
- media is separate from the text path
- image/video sending may be compressed or deferred
- auto-download of media must be limited in poor connections
- read receipts and typing indicators may be batched or reduced
- retries must be automatic where appropriate

### UX in bad network conditions
The user should feel:
- the message is safe
- the app is in control
- retry is happening intelligently
- nothing is lost

The user should not feel:
- panic
- chaos
- uncertainty
- random failure

---

## 12. Product modes

### 12.1 Private chats
- E2EE by default
- text
- attachments
- voice messages
- reactions
- replies
- forwards
- drafts
- queued delivery

### 12.2 Private groups
- E2EE
- initially small and medium-sized groups
- clear participant and trust model

### 12.3 Public channels
Public channels can be a separate scalable product mode and do not have to follow the exact same privacy model as private chats.

### 12.4 Calls
- audio first
- video later
- adaptive to poor network conditions

---

## 13. Recommended stack

### Clients
- Android: Kotlin + Jetpack Compose
- iOS: Swift + SwiftUI
- Desktop: Tauri + Rust

### Shared core
- Rust

The shared core should own:
- protocol abstractions
- message pipeline logic
- device model
- transport abstractions
- reusable security-related logic
- serialization and shared business rules

### Backend
- Rust preferred
- Go acceptable if strongly justified

### Infra/data
- PostgreSQL
- Redis or NATS
- object storage for encrypted files
- push providers
- STUN/TURN for calls
- Docker
- Terraform
- GitHub Actions

---

## 14. Backend services

Minimum backend services:

### 14.1 API Gateway
Responsible for:
- client entry
- auth routing
- rate limiting
- request routing

### 14.2 Messaging Service
Responsible for:
- receiving encrypted messages
- delivery queueing
- pending message retrieval
- delivery acknowledgements
- message lifecycle events

### 14.3 Device / Key Service
Responsible for:
- device records
- public key material
- device trust model
- adding/removing devices

### 14.4 Media Service
Responsible for:
- encrypted upload/download handling
- upload URLs/tickets
- media object linkage

### 14.5 Push Service
Responsible for:
- APNs / FCM integration
- client wake-up signaling
- privacy-preserving push behavior

### 14.6 Call Signalling Service
Responsible for:
- call session setup
- signaling
- STUN/TURN coordination

---

## 15. Client architecture

Each client should be built around these layers:

1. presentation layer
2. UI state layer
3. domain layer
4. local database
5. sync engine
6. transport layer
7. secure abstractions
8. media pipeline

### 15.1 Presentation layer
- screens
- components
- navigation
- sheets
- modals
- animations

### 15.2 UI state layer
- loading states
- empty states
- offline states
- optimistic states
- message lifecycle states

### 15.3 Domain layer
- chat model
- message model
- user model
- device model
- settings model

### 15.4 Local DB
- source of truth for UI
- drafts
- message queue
- cached messages
- media references
- sync metadata

### 15.5 Sync engine
- outgoing queued messages
- incoming events
- reconnect handling
- deduplication
- server/local merge logic

### 15.6 Transport layer
- websocket or equivalent persistent mode
- fallback modes
- backoff
- reconnect logic
- network quality awareness

---

## 16. Message lifecycle

Every message should have a clear lifecycle.

Example states:
- draft
- local_pending
- encrypted
- queued
- sending
- server_accepted
- delivered
- read
- failed_retryable
- failed_terminal

Rules:
- state must be understandable
- state must not create visual stress
- retry must be smart
- duplicate prevention is mandatory

---

## 17. Multi-device model

### 17.1 Principle
An account is a set of trusted devices, not one magical cloud session.

### 17.2 Required capabilities
- device list
- device metadata
- add new device flow
- remove device flow
- device trust visibility
- last active information
- security-friendly pairing flow

### 17.3 Forbidden shortcuts
- hidden device addition
- insecure convenience sync
- trust model that is invisible to the user

---

## 18. Media and attachments

### Principles
- media must not block text UX
- large uploads must not freeze the app
- progress must be clear
- cancellation should be possible
- weak-network-aware behavior is required

### UX expectations
- smart compression where appropriate
- visible progress
- good retry behavior
- clear failure handling
- separation from text send flow

---

## 19. Calls

Calls are important, but not the first priority of MVP.

### Principles
- audio before video
- graceful degradation
- adaptive quality
- clean interface
- weak-network-aware behavior

---

## 20. Design instructions for implementation

This section is mandatory for Codex.

Codex must not invent the design arbitrarily.  
Codex must implement the product according to these design constraints.

### 20.1 Design philosophy
UI must be:
- premium
- calm
- clean
- modern
- readable
- light
- coherent
- elegant
- non-generic

### 20.2 Visual character
The interface should feel like:
- calm technical premium
- stable
- precise
- effortless
- visually refined

### 20.3 Avoid
Do not use:
- neon colors
- cyberpunk security aesthetics
- noisy gradients
- random accent colors
- visually heavy cards
- cluttered layouts
- excessive shadows
- overdesigned gimmicks
- generic AI-looking UI
- enterprise-dashboard aesthetics

### 20.4 Composition rules
- clear hierarchy
- spacious layout
- strong rhythm
- minimal competing signals
- one dominant action per screen
- excellent readability

### 20.5 Typography
Typography must:
- be highly readable
- feel premium
- build hierarchy clearly
- avoid decorative excess

### 20.6 Color philosophy
Colors must:
- be restrained
- support calmness and trust
- avoid loudness
- support clarity, not decoration

### 20.7 Shape/radius philosophy
- soft modern geometry
- friendly but not childish
- consistent radius usage
- no random geometric mixing

### 20.8 Iconography
- simple
- coherent
- clean
- system-friendly in spirit

### 20.9 Motion
Animations must:
- be subtle
- be fast
- explain transitions
- never feel theatrical
- never reduce clarity

---

## 21. UX rules for Codex

### 21.1 Primary action clarity
Every screen must make the primary action obvious.

### 21.2 Thumb-friendly layout
Important mobile actions must be reachable comfortably.

### 21.3 Explicit state design
Every screen must account for:
- loading
- empty
- offline
- error
- retry
- low-bandwidth mode

### 21.4 Weak network must not feel like app failure
The interface must communicate control and continuity.

### 21.5 Information density
Information should be rich but not overwhelming.

### 21.6 Settings
Settings must be grouped, readable, and not intimidating.

### 21.7 No screen-by-screen improvisation
The product must feel system-designed.

---

## 22. Key screens

Codex should begin with these screens:

### 22.1 Chat list
Must be:
- readable
- fast
- clean
- hierarchy-driven
- visually light
- status-aware

### 22.2 Conversation screen
This is one of the most important screens.

It must be:
- extremely clear
- comfortable
- visually light
- highly readable
- state-aware
- built around a strong composer

### 22.3 Composer
The composer is critical.

It must:
- support text
- support voice
- support attachments
- support drafts
- support queued-send UX
- remain simple

### 22.4 Chat/contact profile
Must present important details without turning into a cluttered action dump.

### 22.5 Device screen
Must clearly explain:
- trusted devices
- security meaning
- last activity
- remove-device action

### 22.6 Settings
Must be:
- grouped
- readable
- calm
- understandable

### 22.7 Low-bandwidth mode screen
Must provide controls for:
- media auto-download
- compression behavior
- data saving
- weak-network adaptation

### 22.8 Onboarding
Must sell the feeling of:
- speed
- reliability
- privacy
- comfort
- confidence in bad networks

---

## 23. Codex implementation requirements

Codex must:
- follow this architecture
- preserve consistency
- prefer clarity over flourish
- build the repo incrementally
- keep changes scoped
- design for weak network from the start
- use premium calm UI principles
- avoid generic generated-looking output

Codex must not:
- introduce random style shifts
- optimize for flashy visuals over usability
- add unnecessary dependencies
- rewrite architecture without need
- create fake-polished but impractical UI

---

## 24. Suggested repository structure

```text
messenger/
  README.md
  ARCHITECTURE.md
  AGENTS.md
  apps/
    android/
    ios/
    desktop/
  core/
    rust-core/
  backend/
    gateway/
    messaging-service/
    device-service/
    media-service/
    push-service/
    call-signalling/
  infra/
    docker/
    terraform/
  scripts/
  .github/
    workflows/
```


## 25. Development phases
Phase 0 — Foundation

repository setup

architecture docs

repo structure

base conventions

initial automation

skeleton apps/services

Phase 1 — MVP messaging

auth/bootstrap

device model

1:1 chats

text messaging

queued delivery

local DB

reconnect/retry

push

initial polished UI

Phase 2 — Media and polish

attachments

image sending

voice messages

reactions

reply/forward

better composer

weak-network polish

Phase 3 — Groups and multi-device

private groups

participant management

stronger trust/device flows

multi-device improvements

Phase 4 — Calls and advanced resilience

audio calls

video calls

transport fallback

resilience improvements

Phase 5 — Public layer

public channels

public discovery mechanics

scaling and public-facing features

26. Success criteria

The project is successful if the user feels:

the app is more pleasant than a typical messenger

the app remains calm and reliable in poor network conditions

the UI feels premium and coherent

messages feel safe and controlled

privacy feels natural, not scary

the product is worth using daily

27. Final decision rule

When there is uncertainty, prefer the solution that best matches:

fast + clear + reliable + calm + premium + privacy-first + weak-network-first


---

## 5. Что сделать дальше в VS Code

Создай файлы:
- `README.md`
- `AGENTS.md`
- `ARCHITECTURE.md`
- `.gitignore`

Потом открой терминал в VS Code и выполни:

```bash
git add .
git commit -m "Add initial project foundation"
git push