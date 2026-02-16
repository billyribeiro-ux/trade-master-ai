# TradeMaster AI — Supabase to Full Rust Backend Migration Prompt

## Paste this prompt BEFORE starting Phase 0. It overrides all Supabase references in every subsequent prompt.

---

```
CRITICAL STACK CHANGE — APPLY TO ALL PROMPTS IN THIS PROJECT:

There is NO Supabase in this project. Every reference to Supabase in any prompt must be replaced with the Rust equivalent below. This applies retroactively to the Global Context Block and every Phase prompt.

WHAT WAS REMOVED:
- @supabase/supabase-js — REMOVED
- @supabase/ssr — REMOVED
- Supabase Auth — REMOVED
- Supabase Storage — REMOVED
- Supabase Realtime — REMOVED
- Supabase Row Level Security (RLS) — REMOVED
- Supabase migrations — REMOVED
- Supabase client (browser/server/admin) — REMOVED
- All supabase/ directory and config — REMOVED
- PUBLIC_SUPABASE_URL, PUBLIC_SUPABASE_ANON_KEY, SUPABASE_SERVICE_ROLE_KEY env vars — REMOVED

WHAT REPLACES IT:

1. DATABASE:
   - PostgreSQL 16 accessed directly from Rust via sqlx with compile-time checked queries
   - Connection pooling via sqlx::PgPool
   - Migrations via sqlx-cli (files live in apps/api/migrations/)
   - All database access goes through the Rust API — the frontend NEVER talks to the database directly
   - Every query is scoped by user_id in the WHERE clause — this is the application-level equivalent of RLS

2. AUTHENTICATION:
   - Custom Rust auth service in the Axum backend
   - Password hashing: argon2 (via the argon2 crate)
   - Tokens: JWT access tokens (15 min expiry) + refresh tokens (30 day expiry) using jsonwebtoken crate
   - Access tokens are stored in memory (JS variable) on the frontend, refresh tokens in httpOnly secure cookies
   - OAuth2: Google and Apple sign-in via the oauth2 crate — backend handles the full OAuth flow, frontend just redirects
   - Session management: refresh token rotation — every refresh issues a new refresh token and invalidates the old one
   - Database tables: users (email, password_hash, created_at), refresh_tokens (user_id, token_hash, expires_at, revoked)
   - Auth middleware in Axum extracts and validates JWT from Authorization: Bearer header on every protected route
   - Registration endpoint: POST /api/v1/auth/register (email, password, display_name)
   - Login endpoint: POST /api/v1/auth/login (email, password) → returns access_token + sets refresh cookie
   - Refresh endpoint: POST /api/v1/auth/refresh → reads refresh cookie, returns new access_token + rotates refresh cookie
   - Logout endpoint: POST /api/v1/auth/logout → revokes refresh token, clears cookie
   - OAuth endpoints: GET /api/v1/auth/google, GET /api/v1/auth/google/callback, same pattern for Apple
   - Password reset: POST /api/v1/auth/forgot-password (sends email), POST /api/v1/auth/reset-password (with token)

3. FILE STORAGE:
   - S3-compatible object storage via the aws-sdk-s3 crate (Rust)
   - Production: Cloudflare R2 (S3-compatible, no egress fees)
   - Local development: MinIO running in Docker (S3-compatible)
   - Upload flow: frontend sends file to Rust API (multipart form) → Rust validates (type, size) → Rust uploads to S3 → returns public URL
   - Upload endpoints on the Rust API:
     - POST /api/v1/media/upload (multipart, fields: trade_id, media_type, file)
     - DELETE /api/v1/media/:id
   - File validation in Rust: max 10MB images, max 50MB recordings, allowed MIME types only
   - Storage path structure: {user_id}/{trade_id}/{timestamp}_{filename}
   - Presigned URLs for reading private files (if needed)
   - Environment variables: S3_ENDPOINT, S3_BUCKET, S3_ACCESS_KEY, S3_SECRET_KEY, S3_REGION, S3_PUBLIC_URL

4. REAL-TIME:
   - Axum WebSockets for any real-time features (tilt alerts, live session tracking)
   - Tower layer for WebSocket auth (validate JWT on connection upgrade)
   - Not needed for Phase 0 — implemented when tilt detection and live features are built

5. API ARCHITECTURE:
   - The Rust/Axum API is the ONLY backend. There is no second backend, no BaaS, no serverless functions.
   - ALL data operations go through the Rust API: auth, CRUD, analytics, AI, file uploads, everything.
   - The SvelteKit frontend calls the Rust API for everything via typed fetch calls.
   - SvelteKit server-side routes (+page.server.ts, +layout.server.ts) can also call the Rust API for SSR data loading.
   - The frontend has NO direct database access — all DB queries live in Rust.
   - API base URL comes from environment: PUBLIC_API_URL (e.g., http://localhost:3001 in dev)

6. FRONTEND AUTH INTEGRATION:
   - apps/web/src/lib/services/auth.ts replaces all Supabase auth code:
     - login(email, password): POST to /api/v1/auth/login, stores access_token in memory
     - register(email, password, displayName): POST to /api/v1/auth/register
     - logout(): POST to /api/v1/auth/logout, clears access_token
     - refreshToken(): POST to /api/v1/auth/refresh (cookie-based), updates in-memory access_token
     - googleLogin(): redirects to /api/v1/auth/google
     - appleLogin(): redirects to /api/v1/auth/apple
     - forgotPassword(email): POST to /api/v1/auth/forgot-password
     - resetPassword(token, newPassword): POST to /api/v1/auth/reset-password
   - apps/web/src/lib/services/api/client.ts — the fetch wrapper:
     - Automatically attaches Authorization: Bearer {accessToken} header
     - On 401 response: automatically calls refreshToken(), retries the original request once
     - If refresh fails: redirects to /login
     - Credentials: 'include' on all fetch calls (for httpOnly refresh cookie)
   - apps/web/src/lib/stores/auth.svelte.ts — auth state (Svelte 5 runes):
     - $state for: user, accessToken, isLoading, isAuthenticated
     - Initialize by calling a /api/v1/auth/me endpoint on app load (validates token, returns user profile)
     - GET /api/v1/auth/me on the Rust side: validates JWT, returns user + profile data
   - SvelteKit hooks.server.ts: on every SSR request, check for refresh cookie, call /api/v1/auth/me to get session, attach to event.locals

7. UPDATED ENVIRONMENT VARIABLES:
   Frontend (.env):
   - PUBLIC_API_URL (e.g., http://localhost:3001)
   - PUBLIC_APP_URL (e.g., http://localhost:5173)

   Backend (.env):
   - DATABASE_URL (PostgreSQL connection string)
   - PORT (API port, default 3001)
   - JWT_SECRET (for signing access tokens)
   - JWT_REFRESH_SECRET (for signing refresh tokens)
   - CORS_ORIGINS (comma-separated allowed origins)
   - ANTHROPIC_API_KEY
   - S3_ENDPOINT
   - S3_BUCKET
   - S3_ACCESS_KEY
   - S3_SECRET_KEY
   - S3_REGION
   - S3_PUBLIC_URL
   - GOOGLE_CLIENT_ID
   - GOOGLE_CLIENT_SECRET
   - GOOGLE_REDIRECT_URL
   - APPLE_CLIENT_ID
   - APPLE_CLIENT_SECRET
   - APPLE_REDIRECT_URL
   - SMTP_HOST, SMTP_PORT, SMTP_USER, SMTP_PASSWORD, SMTP_FROM (for password reset emails)

8. UPDATED FILE STRUCTURE:
   - DELETE: supabase/ directory entirely
   - DELETE: apps/web/src/lib/services/supabase/ directory entirely
   - MOVE: all migrations to apps/api/migrations/
   - ADD: apps/api/src/routes/auth.rs (registration, login, refresh, logout, OAuth, password reset)
   - ADD: apps/api/src/routes/media.rs (file upload, delete)
   - ADD: apps/api/src/services/auth.rs (password hashing, JWT creation/validation, OAuth flows)
   - ADD: apps/api/src/services/storage.rs (S3 upload/delete/presign)
   - ADD: apps/api/src/services/email.rs (SMTP email sending for password resets)
   - ADD: apps/api/src/models/auth.rs (User, RefreshToken, LoginRequest, RegisterRequest, TokenResponse)
   - ADD: docker-compose.yml at project root (PostgreSQL + MinIO for local development)

9. UPDATED Cargo.toml DEPENDENCIES:
   - axum (with features: ws, multipart)
   - tokio (full)
   - sqlx (features: runtime-tokio, postgres, uuid, chrono, json, decimal, migrate)
   - serde, serde_json
   - tower, tower-http (cors, trace, timeout, compression, limit)
   - tracing, tracing-subscriber
   - reqwest (for Claude API, OAuth token exchange)
   - jsonwebtoken
   - argon2
   - uuid
   - chrono
   - rust_decimal
   - dotenvy
   - anyhow, thiserror
   - aws-sdk-s3 (or aws-sdk-rust S3 client)
   - oauth2
   - lettre (SMTP email)
   - validator (request validation)
   - base64
   - rand (for token generation)

10. KEY RUST API ROUTES (complete list for Phase 0):
    Auth:
    - POST   /api/v1/auth/register
    - POST   /api/v1/auth/login
    - POST   /api/v1/auth/refresh
    - POST   /api/v1/auth/logout
    - GET    /api/v1/auth/me
    - GET    /api/v1/auth/google
    - GET    /api/v1/auth/google/callback
    - GET    /api/v1/auth/apple
    - GET    /api/v1/auth/apple/callback
    - POST   /api/v1/auth/forgot-password
    - POST   /api/v1/auth/reset-password

    User Profile:
    - GET    /api/v1/profile
    - PUT    /api/v1/profile
    - POST   /api/v1/profile/onboarding

    Trades (CRUD):
    - GET    /api/v1/trades (list with filters, sort, pagination)
    - GET    /api/v1/trades/:id (single trade with relations)
    - POST   /api/v1/trades
    - PUT    /api/v1/trades/:id
    - DELETE /api/v1/trades/:id (soft delete / archive)

    Tags:
    - GET    /api/v1/tags
    - POST   /api/v1/tags
    - PUT    /api/v1/tags/:id
    - DELETE /api/v1/tags/:id
    - POST   /api/v1/trades/:id/tags (add tag to trade)
    - DELETE /api/v1/trades/:id/tags/:tag_id (remove tag from trade)

    Media:
    - POST   /api/v1/media/upload (multipart)
    - DELETE /api/v1/media/:id
    - GET    /api/v1/trades/:id/media

    Plans:
    - GET    /api/v1/plans (list)
    - GET    /api/v1/plans/:date (by date)
    - POST   /api/v1/plans
    - PUT    /api/v1/plans/:id
    - GET    /api/v1/plans/:id/watchlist
    - POST   /api/v1/plans/:id/watchlist
    - PUT    /api/v1/plans/:id/watchlist/:item_id
    - DELETE /api/v1/plans/:id/watchlist/:item_id

    AI:
    - POST   /api/v1/ai/review-trade
    - POST   /api/v1/ai/review-chat
    - POST   /api/v1/ai/plan-of-attack
    - POST   /api/v1/ai/weekly-summary
    - POST   /api/v1/ai/monthly-summary

    Analytics:
    - GET    /api/v1/analytics/summary
    - GET    /api/v1/analytics/equity-curve
    - GET    /api/v1/analytics/heatmap
    - GET    /api/v1/analytics/monte-carlo
    - GET    /api/v1/analytics/strategy-breakdown
    - GET    /api/v1/analytics/time-analysis
    - GET    /api/v1/analytics/instrument-matrix

    Risk:
    - POST   /api/v1/risk/position-size
    - GET    /api/v1/risk/portfolio-heat
    - POST   /api/v1/risk/what-if

    Psychology:
    - GET    /api/v1/mood/:date
    - POST   /api/v1/mood
    - GET    /api/v1/goals
    - POST   /api/v1/goals
    - PUT    /api/v1/goals/:id
    - GET    /api/v1/tilt-events
    - GET    /api/v1/alert-rules
    - POST   /api/v1/alert-rules
    - PUT    /api/v1/alert-rules/:id

    Health:
    - GET    /api/health

APPLY THIS CHANGE TO EVERY PROMPT GOING FORWARD. When any prompt references Supabase, replace it with the Rust equivalent described above. The frontend API client talks to the Rust API. The Rust API talks to PostgreSQL and S3. That's it. No BaaS, no third-party database services, no managed auth. Pure Rust.
```
