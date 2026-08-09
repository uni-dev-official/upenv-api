# UpEnv API

A Rust + Axum REST API for the UpEnv desktop application.

## Architecture

The API is intentionally thin:

- **Supabase Auth** authenticates users.
- **Supabase PostgREST** stores devices and backup metadata.
- **UpEnv Desktop** performs machine-level restore operations.
- The API never needs a user's machine credentials.

The API validates the caller's Supabase access token through:

`/auth/v1/user`

and then forwards the same user token to Supabase PostgREST. This means Supabase Row Level Security remains the boundary between users' data.

## Requirements

- Rust 1.77+
- A Supabase project
- A Supabase publishable/anon key

## Setup

```bash
cp .env.example .env
```

Fill in:

```env
SUPABASE_URL=https://YOUR_PROJECT.supabase.co
SUPABASE_ANON_KEY=YOUR_PUBLISHABLE_OR_ANON_KEY
HOST=0.0.0.0
PORT=8080
CORS_ORIGINS=http://localhost:1420
```

**Never put the Supabase `service_role` key in this API configuration.**

Run `supabase/schema.sql` in the Supabase SQL editor.

Then:

```bash
cargo run
```

The API will be available at:

```text
http://localhost:8080
```

## Endpoints

### Public

```http
GET /health
```

### Authenticated

All of these require:

```http
Authorization: Bearer <SUPABASE_ACCESS_TOKEN>
```

```http
GET    /api/me

GET    /api/devices
POST   /api/devices
DELETE /api/devices/:id

GET    /api/backups
POST   /api/backups
DELETE /api/backups/:id
```

## Create a device

```http
POST /api/devices
Authorization: Bearer <token>
Content-Type: application/json
```

```json
{
  "name": "MacBook Pro",
  "platform": "macos",
  "architecture": "arm64",
  "app_version": "0.1.0"
}
```

## Create a backup

```http
POST /api/backups
Authorization: Bearer <token>
Content-Type: application/json
```

```json
{
  "device_id": "DEVICE_UUID",
  "name": "My MacBook backup",
  "manifest": {
    "homebrew": {
      "packages": ["git", "node", "python"],
      "casks": ["visual-studio-code"]
    },
    "vscode": {
      "extensions": ["rust-lang.rust-analyzer"]
    },
    "git": {
      "user_name": "Example User"
    },
    "python": {
      "packages": ["requests", "numpy"]
    },
    "node": {
      "packages": ["typescript", "vite"]
    }
  }
}
```

## Next API milestones

1. Backup file/object storage using Supabase Storage.
2. Restore job endpoints and progress tracking.
3. API keys for CLI usage.
4. Device heartbeat / last-seen state.
5. Backup versioning and deduplication.
6. Rate limiting.
7. OpenAPI documentation.
8. Production deployment at `api.upenv.dev`.

## Security notes

The API does not execute shell commands and does not restore software itself.

A restore request should eventually create a job. The desktop client polls or subscribes to that job and performs the local operations.

This keeps the remote API from becoming a remote-code-execution service for the user's machine.
