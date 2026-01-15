# Dynamic Nginx Configuration Generator

This script (`configure.py`) is an interactive CLI tool designed to autonomously generate a production-ready `nginx.conf` by parsing OpenAPI (Swagger) specifications from multiple microservices.

## 🚀 Core Features

### 1. Hybrid Operation Mode
The script offers two distinct modes of operation:
*   **Manual Input:** Manually provide paths to local JSON spec files and define upstream details interactively.
*   **Docker Auto-Discovery:** Automatically scan running containers for specific labels, fetching configurations and specs dynamically without user intervention.

### 2. Multi-Mesh Support (Custom Prefixes)
Supports running multiple independent service meshes on the same host.
*   When using Auto-Discovery, you can define a custom **Label Prefix** (default: `proxy`).
*   Example: Using prefix `mesh1.proxy` will scan for `mesh1.proxy.enable`, while `mesh2.proxy` will scan for `mesh2.proxy.enable`.

### 3. Secure Sidecar Introspection
In Auto-Discovery mode, the script uses a **Sidecar Pattern** (`curlimages/curl`) to fetch OpenAPI specifications.
*   It spins up a temporary container attached to the target service's network namespace (`--network container:<id>`).
*   It fetches the spec via `localhost` (internal loopback).
*   **Benefit:** Microservices do **not** need to expose their API ports to the host machine. The script works safely in **Rootless Docker** environments without mounting the Docker socket.

### 4. Smart Path Parameter Handling (Regex)
OpenAPI paths containing parameters (e.g., `/users/{id}/profile`) are automatically converted into **Nginx Regex Location Blocks**:
*   **Input:** `/users/{id}/profile`
*   **Output:** `location ~ ^/users/([^/]+)/profile$`
This ensures strictly typed routing where requests must match the exact structure defined in the spec, offering superior security over broad prefix matching.

### 5. Intelligent Conflict Resolution
When multiple services define the same endpoint (e.g., a common `/health` route), the script performs a "Post-Input Analysis":
*   **Detection:** It identifies every overlapping route across all provided specs.
*   **Granular Selection:** It presents the user with a numbered list of all services providing that specific route.
*   **Explicit Choice:** The user explicitly selects which service "wins" the route.

### 6. Automatic Path Rewriting
If a **Path Prefix** is applied to a service, the script automatically generates Nginx `rewrite` rules. This ensures that a request to `nginx/auth/login` is sent to the backend as `/login`, matching the service's internal routing.

### 7. Hardened Proxy Headers
Every generated `location` block includes standard security and transparency headers:
*   `Host`: Preserves the original host header.
*   `X-Real-IP`: Passes the real client IP to the backend.
*   `X-Forwarded-For`: Maintains the chain of proxies.

---

## 🐳 Docker Configuration (Auto-Discovery)

To enable auto-discovery for a service, add the following labels to your `docker-compose.yml`.
You can customize the `proxy` prefix (e.g., `mesh1.proxy`) when running the script.

```yaml
services:
  auth_service:
    image: auth_service
    labels:
      - "proxy.enable=true"                  # REQUIRED: Enables discovery
      - "proxy.port=3000"                    # Internal port (default: 3000)
      - "proxy.spec_path=/api/docs/openapi.json" # Path to JSON spec (default: /api-docs/openapi.json)
      - "proxy.prefix=/auth"                 # Optional: URL prefix
      - "proxy.protocol=http"                # Optional: http or https (default: http)
```

**Note:** The service does *not* need `ports:` exposed to the host. The script will access it via the internal Docker network.

---

## 🛠️ How It Works

1.  **Discovery Phase:**
    *   **Manual:** User inputs file paths and host details manually.
    *   **Auto:** Script queries Docker API for labeled containers (filtering by custom prefix) and uses ephemeral sidecars to fetch specs.
2.  **Analysis Phase:** Flatten all paths, identify provider overlaps, and detect path parameters.
3.  **Resolution Phase:** Interactively prompt the user to resolve any detected route conflicts.
4.  **Generation Phase:** Write the optimized `nginx.conf`, utilizing standard locations for static paths and regex locations for dynamic paths.

## 📖 Usage

Run the script from the project root:

```bash
python3 scripts/state/configure.py
```

You will be presented with a menu:
```text
🚀 Nginx Dynamic Configuration Generator
----------------------------------------
1. Manual Input (Local Files)
2. Docker Auto-Discovery (Labels)
Select mode [1]:
```

If you select Mode 2, you will be asked for the prefix:
```text
Docker Label Prefix [proxy]: mesh1.proxy
```
This will scan for `mesh1.proxy.enable=true`, etc.