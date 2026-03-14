# Messenger

Privacy-first messenger built for weak and unstable networks.

## Architecture

The project direction lives in:

- `ARCHITECTURE.md`
- `AGENTS.md`

Current bootstrap status:

- Rust workspace rooted at `Cargo.toml`
- initial backend crate at `backend/gateway`
- placeholder shared core crate at `core/rust-core`
- CI entrypoint at `.github/workflows/ci.yml`

## Local Development

Bootstrap the Rust toolchain and fetch dependencies:

```powershell
./scripts/bootstrap-dev.ps1
```

Run the repository checks:

```powershell
./scripts/check.ps1
```

## Principles

- weak-network-first
- privacy-first
- premium UX
- clean architecture
- no generic AI-looking UI
- no self-invented cryptography
