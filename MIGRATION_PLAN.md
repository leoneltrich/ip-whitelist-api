# Migration Plan: Splitting Access Proxy into Microservices

This document outlines the architectural shift from a monolithic Access Proxy to a decoupled system following the Unix Philosophy.

## 1. Goal Architecture
*   **`auth_service`**: Handles user identity, registration, and JWT issuance (RS256 Private Key).
*   **`firewall_service`**: Handles infrastructure access and NFTables management (RS256 Public Key).
*   **`shared`**: Common models (Claims), error types, and cryptographic verification logic.

---

## Phase 1: Preparation
- [ ] Create a full backup of `application.db` and the current `src/` directory.
- [ ] Generate RSA-2048 Key Pair:
    - `private.pem` (Auth Service only)
    - `public.pem` (All services)
- [ ] Verify `Cargo.toml` for workspace compatibility.

## Phase 2: Workspace Scaffolding
- [ ] Create root `Cargo.toml` defining members: `crates/*`.
- [ ] Create folder structure:
    ```text
    /crates/shared
    /crates/auth_service
    /crates/firewall_service
    ```

## Phase 3: Extraction of `shared` Library
- [ ] Initialize `crates/shared` as a library.
- [ ] Move `src/errors.rs` to `shared`.
- [ ] Move `src/models/api/auth.rs` (Claims) to `shared`.
- [ ] Implement RSA key loading and token verification logic in `shared`.
- [ ] Update `shared` dependencies: `jsonwebtoken`, `serde`, `thiserror`.

## Phase 4: Implementation of `auth_service`
- [ ] Initialize `crates/auth_service` as a binary.
- [ ] Migrate User-related logic:
    - Routes: `/login`, `/users`, `/profile`.
    - Services: `auth.rs`, `user.rs`.
    - Repositories: `user.rs`.
- [ ] Implement Token Issuance using the **Private Key**.
- [ ] Database: Uses `auth.db` (contains only `users` table).

## Phase 5: Refactoring `firewall_service`
- [ ] Initialize `crates/firewall_service` as a binary (or move remaining `src`).
- [ ] Remove all user management routes and password hashing logic.
- [ ] Update Authentication Middleware:
    - Instead of checking a shared secret, use `shared::verify_with_public_key()`.
- [ ] Database: Uses `firewall.db` (contains `servers`, `whitelist`).
- [ ] Retain NFTables integration and Access request logic.

## Phase 6: Validation & Cleanup
- [ ] Ensure all services compile independently.
- [ ] Update `.gitignore` to handle new database files and `.pem` keys.
- [ ] (Optional) Create a `docker-compose.yml` to orchestrate both services.
- [ ] Delete the legacy `src/` directory.
