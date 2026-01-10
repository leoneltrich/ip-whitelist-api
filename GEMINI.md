# Project Status: Access Proxy Microservices Refactor
**Date:** Saturday, January 10, 2026
**Current Context:** Migration from Monolith to Unix-style Microservices.

---
 
## 🚀 Accomplishments
- **Workspace Architecture:** Converted the monolith into a **Cargo Workspace**.
  - `crates/shared`: Common logic for RSA JWT (RS256), `AppError`, and `HealthResponse`.
  - `crates/auth_service`: Identity provider (Port 3000). Owns the `users` table. Signs tokens with Private Key.
  - `crates/firewall_service`: Infrastructure agent (Port 3001). Owns `servers` and `whitelist` tables. Verifies tokens with Public Key.
- **Security Upgrade:** Moved from a shared symmetric secret to **Asymmetric RS256** (RSA 2048-bit).
    - Auth Service uses `private.pem`.
    - Firewall Service uses `public.pem`.
 - **Infrastructure:**
     - **Docker:** Multi-stage Alpine builds for minimal images (~20MB).
     - **Nginx:** Acting as the API Gateway/Ingress.
         - Routes `/login`, `/admin/users` -> Auth Service.
         - Routes `/users/access`, `/admin/servers`, `/health` -> Firewall Service.
     - **Health Checks:** Standardized across all services via the `shared` crate.

---

 ## 📂 Current Structure
 - `/crates/shared`: Shared library (models, errors, jwt logic).
 - `/crates/auth_service`: Port 3000.
 - `/crates/firewall_service`: Port 3001.
 - `/data`: Persistent folder for `auth.db` and `firewall.db`.
 - `nginx.conf`: Routing rules.
 - `docker-compose.yml`: Local development orchestration.

 ---

 ## 🚧 Status & Troubleshooting
 - **Last Fix:** Updated `firewall_service/src/initialization.rs` to use the `sqlite://` prefix to resolve a `code 14: unable to open database file` error caused by absolute pa
parsing in Docker.
 - **Service Status:** `auth_service` is confirmed working. `firewall_service` and `nginx` are pending verification after the latest build.

 ---

 ## 🎯 Next Steps
 1. **Verification:** Confirm all containers are stable using `docker-compose up --build -d`.
 2. **Admin Setup:** Retrieve the bootstrap admin password from the `auth_service` logs.
 3. **Notes Extension:** Implement a new `notes_service` for sharing Minecraft coordinates.
     - It will be added as a third member to the workspace.
     - It will trust the Auth Service's Public Key for authentication.
     - Nginx will be updated to route `/notes` traffic to it.