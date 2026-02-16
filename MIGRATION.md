# Supabase to Rust Backend Migration

This document details the complete migration from Supabase to a custom Rust backend.

## What Was Removed

### Dependencies
- `@supabase/supabase-js` - Removed
- `@supabase/ssr` - Removed
- All Supabase-related packages

### Services
- Supabase Auth
- Supabase Storage
- Supabase Realtime
- Supabase Row Level Security (RLS)
- Supabase client (browser/server/admin)

### Files & Directories
- `supabase/` directory - Deleted
- `apps/web/src/lib/services/supabase/` - Deleted
- Supabase-related environment variables

## What Was Added

### Backend Infrastructure

#### 1. Rust API (`apps/api/`)
Complete Axum-based REST API handling:
- Authentication (JWT + refresh tokens)
- Database operations (sqlx with PostgreSQL)
- File storage (S3-compatible)
- Email sending (SMTP)
- OAuth flows (Google, Apple)

#### 2. Docker Compose (`docker-compose.yml`)
Local development services:
- PostgreSQL 16
- MinIO (S3-compatible storage)
- Automatic bucket creation

#### 3. Database Migrations (`apps/api/migrations/`)
SQL migrations managed by sqlx-cli (moved from `supabase/migrations/`)

### Frontend Changes

#### 1. Auth Service (`apps/web/src/lib/services/auth.ts`)
Replaces Supabase auth with custom implementation:
- `login()` - POST to `/api/v1/auth/login`
- `register()` - POST to `/api/v1/auth/register`
- `logout()` - POST to `/api/v1/auth/logout`
- `refreshToken()` - POST to `/api/v1/auth/refresh`
- `googleLogin()` - Redirect to OAuth endpoint
- `appleLogin()` - Redirect to OAuth endpoint

#### 2. API Client (`apps/web/src/lib/services/api/client.ts`)
Typed fetch wrapper with:
- Automatic JWT token attachment
- Automatic token refresh on 401
- Request timeout handling
- Error parsing and retry logic
- File upload support

#### 3. Auth Store (`apps/web/src/lib/stores/auth.svelte.ts`)
Svelte 5 runes-based state management:
- `$state` for user, accessToken, isLoading, isAuthenticated
- Reactive getters for user properties
- Login/logout methods

#### 4. Error Handling (`apps/web/src/lib/services/api/errors.ts`)
Typed error system:
- `ApiError` class with status codes
- User-friendly error messages
- Error code enum

### Type Definitions

#### Updated `app.d.ts`
Removed Supabase types, added custom auth types:
```typescript
interface Locals {
  user: { id: string; email: string; displayName: string | null } | null;
}
```

## Authentication Flow Comparison

### Before (Supabase)
```
Frontend → Supabase Auth → Supabase Database
         ↓
       Supabase Storage
```

### After (Rust)
```
Frontend → Rust API → PostgreSQL
         ↓
       S3 Storage (MinIO/R2)
```

## Token Management

### Access Tokens
- **Lifetime:** 15 minutes
- **Storage:** In-memory (JavaScript variable)
- **Format:** JWT signed with `JWT_SECRET`
- **Claims:** user_id, email, exp

### Refresh Tokens
- **Lifetime:** 30 days
- **Storage:** httpOnly secure cookie
- **Format:** JWT signed with `JWT_REFRESH_SECRET`
- **Rotation:** New token issued on each refresh, old token revoked

## Database Access

### Before (Supabase)
- Frontend had direct database access via Supabase client
- Row Level Security (RLS) policies enforced access control
- Real-time subscriptions available

### After (Rust)
- Frontend has **zero** direct database access
- All queries go through Rust API
- Application-level security: every query scoped by `user_id` in WHERE clause
- WebSockets for real-time features (when needed)

## File Storage

### Before (Supabase Storage)
```typescript
supabase.storage.from('bucket').upload(path, file)
```

### After (S3-compatible)
```typescript
// Frontend sends file to API
const formData = new FormData();
formData.append('file', file);
formData.append('trade_id', tradeId);

await apiClient.uploadFile('/media/upload', formData);
```

Backend handles:
- File validation (type, size)
- S3 upload
- Database record creation
- URL generation

## Environment Variables

### Removed
```env
PUBLIC_SUPABASE_URL
PUBLIC_SUPABASE_ANON_KEY
SUPABASE_SERVICE_ROLE_KEY
```

### Added (Frontend)
```env
PUBLIC_API_URL=http://localhost:3001
PUBLIC_APP_URL=http://localhost:5173
```

### Added (Backend)
```env
DATABASE_URL=postgresql://...
JWT_SECRET=...
JWT_REFRESH_SECRET=...
S3_ENDPOINT=...
S3_BUCKET=...
S3_ACCESS_KEY=...
S3_SECRET_KEY=...
GOOGLE_CLIENT_ID=...
GOOGLE_CLIENT_SECRET=...
SMTP_HOST=...
```

## API Endpoints

All backend operations now go through REST endpoints:

### Auth
- `POST /api/v1/auth/register`
- `POST /api/v1/auth/login`
- `POST /api/v1/auth/refresh`
- `POST /api/v1/auth/logout`
- `GET /api/v1/auth/me`
- `GET /api/v1/auth/google`
- `GET /api/v1/auth/google/callback`
- `POST /api/v1/auth/forgot-password`
- `POST /api/v1/auth/reset-password`

### Data Operations
- `GET/POST/PUT/DELETE /api/v1/trades`
- `GET/POST/PUT/DELETE /api/v1/tags`
- `POST /api/v1/media/upload`
- `GET/POST/PUT /api/v1/plans`
- `POST /api/v1/ai/review-trade`
- `GET /api/v1/analytics/*`

## Migration Checklist

- [x] Remove Supabase dependencies from package.json
- [x] Create Rust API with Cargo.toml
- [x] Set up Docker Compose for local services
- [x] Create custom auth service (frontend)
- [x] Create API client with token refresh
- [x] Create auth store with Svelte 5 runes
- [x] Update app.d.ts types
- [x] Update environment variable templates
- [x] Create README and migration docs
- [ ] Implement Rust auth routes
- [ ] Implement Rust database models
- [ ] Create database migrations
- [ ] Implement S3 storage service
- [ ] Implement email service
- [ ] Add OAuth flows
- [ ] Add comprehensive error handling
- [ ] Add request validation
- [ ] Add rate limiting
- [ ] Add logging and monitoring

## Benefits of Migration

1. **Full Control:** Complete ownership of auth, storage, and data layer
2. **Performance:** Direct database access, no network hop to Supabase
3. **Cost:** No per-user pricing, only infrastructure costs
4. **Security:** Custom security model tailored to app needs
5. **Flexibility:** Can implement any feature without BaaS limitations
6. **Type Safety:** End-to-end type safety with Rust + TypeScript
7. **Scalability:** Can optimize and scale each component independently

## Development Workflow

1. Start local services: `docker-compose up -d`
2. Run migrations: `cd apps/api && sqlx migrate run`
3. Start backend: `cd apps/api && cargo run`
4. Start frontend: `pnpm dev`
5. Access app at `http://localhost:5173`

## Production Deployment

### Frontend (Vercel)
- Build: `pnpm build`
- Environment: `PUBLIC_API_URL` pointing to production API

### Backend (Fly.io)
- Build: `cargo build --release`
- Database: Managed PostgreSQL
- Storage: Cloudflare R2
- Environment: All secrets in Fly.io secrets

## Next Steps

Proceed with Phase 0 prompts, replacing all Supabase references with the Rust equivalents documented here.
