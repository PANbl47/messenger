# AGENTS.md

## Purpose
This repository is for building a next-generation privacy-first messenger optimized for weak and unstable internet connections.

The product goal is not to clone Telegram mechanically.  
The goal is to surpass Telegram in:
- UX quality
- visual polish
- architecture consistency
- comfort of daily use
- weak-network reliability

This project must feel:
- premium
- calm
- fast
- clean
- modern
- highly usable
- visually coherent

## Source of truth
Always follow:
1. `ARCHITECTURE.md`
2. this file
3. repository structure and existing code conventions

If there is a conflict, follow `ARCHITECTURE.md`.

## Core principles
- privacy-first
- weak-network-first
- offline-first where appropriate
- store-and-forward delivery model
- text messages are higher priority than media
- no self-invented cryptography
- no generic AI-looking UI
- no random architecture decisions
- no inconsistent styling between screens

## Product rules
When making decisions, optimize for:
- clarity
- speed
- reliability
- calm UX
- premium visual quality
- thumb-friendly mobile interactions
- low-bandwidth usability

Do not optimize for:
- flashy gimmicks
- overcomplicated visuals
- feature bloat
- noisy UI
- “demo app” aesthetics

## Design rules
UI must be:
- calm
- premium
- minimal but not empty
- highly readable
- spacious
- modern
- visually consistent

Avoid:
- neon colors
- cyberpunk style
- loud gradients
- overly heavy cards
- cluttered layouts
- random shadows
- cheap startup landing-page aesthetics
- default generic AI-generated UI patterns

## UX rules
Every screen must clearly communicate:
- where the user is
- what matters most
- what action is primary
- what the network/message state is

Weak network behavior must always be considered.
Design and implementation must include:
- queued message states
- retryable flows
- offline/loading/error states
- low-bandwidth-friendly behavior

## Coding rules
Before implementing a feature:
1. identify the user scenario
2. identify UI states
3. identify network states
4. identify backend/data impact
5. implement the smallest correct version

Prefer:
- simple architecture
- readable code
- explicit naming
- modular structure
- future-safe abstractions only when justified

Avoid:
- premature complexity
- speculative abstractions
- giant files
- hidden state mutations
- magic constants

## Messenger-specific implementation rules
For messaging features, always think through:
- draft state
- queued state
- sending state
- delivered state
- read state
- retry behavior
- duplicate prevention
- local persistence
- reconnect behavior

For media features, always think through:
- upload state
- compression behavior
- cancellation
- weak-network fallback
- progress indication
- separation from text message flow

## Repo workflow
When asked to build something:
- first align with `ARCHITECTURE.md`
- keep changes scoped
- do not rewrite unrelated files
- do not introduce unnecessary dependencies
- preserve consistency across apps/backend/core

## Output quality bar
Anything added to this repository should feel like part of a premium messaging product, not a generic generated prototype.