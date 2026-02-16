# Phase 0 - Remaining Work (Rust Backend Architecture)

**Date:** February 15, 2026  
**Status:** Phases 0.1 and 0.2 Complete, Phases 0.3-0.8 Remaining

## Important: No Supabase

This project uses a **custom Rust/Axum backend** with PostgreSQL, NOT Supabase. All references to Supabase in the original prompts have been replaced with Rust API equivalents.

## Architecture Reminder

```
Frontend (SvelteKit 5) → Rust API (Axum) → PostgreSQL 16
                       ↓
                     S3 Storage (MinIO/Cloudflare R2)
```

**Authentication:** Custom JWT (access tokens 15min + refresh tokens 30 days)  
**Database:** PostgreSQL 16 via sqlx (no RLS, application-level security in Rust)  
**Storage:** S3-compatible (MinIO local, Cloudflare R2 production)

---

## Phase 0.3 - shadcn-svelte Base Components ⏳

**Status:** PENDING  
**Estimated Time:** 4-6 hours

### Components to Create (20 total)

All components must:
- Use Svelte 5 runes ($props, $state, $derived, $effect)
- Use snippets (not slots)
- Use cn() utility for class merging
- Accept class prop for external styling
- Have proper TypeScript types
- Be responsive (375px to 2560px)

#### Component List

1. **Button** - Variants: primary, secondary, outline, ghost, danger, profit, loss
2. **Input** - Text, number, email, password with validation states
3. **Select** - Dropdown with search/filter
4. **Textarea** - Auto-resize, character count
5. **Checkbox**
6. **Toggle/Switch**
7. **Badge** - All state variants
8. **Card** - With header/content/footer snippets
9. **Dialog/Modal** - With focus trap
10. **Tooltip**
11. **Tabs**
12. **Dropdown Menu**
13. **Toast/Notification** - With global toast() function
14. **Skeleton Loader**
15. **Empty State**
16. **Avatar**
17. **Separator**
18. **Progress Bar**
19. **Spinner**
20. **Icon** - Iconify wrapper (Phosphor icons)

---

## Phase 0.4 - App Shell & Navigation ⏳

**Status:** PENDING  
**Estimated Time:** 2-3 hours

### Deliverables

1. **Authenticated Layout** (`(app)/+layout.svelte`)
   - Checks auth via Rust API `/api/v1/auth/me`
   - Redirects to /login if not authenticated
   - Renders app shell

2. **Server-side Auth Check** (`(app)/+layout.server.ts`)
   - Calls Rust API to validate session
   - Returns user profile data
   - Redirects if no valid session

3. **App Shell Component** (`components/layout/app-shell.svelte`)
   - Three-zone layout: sidebar + main content
   - Collapsible sidebar (persists to localStorage)
   - Mobile: bottom nav instead of sidebar

4. **Sidebar Navigation** (`components/layout/sidebar.svelte`)
   - Navigation items with Phosphor icons
   - Active state highlighting
   - Tooltips when collapsed
   - User avatar at bottom

5. **Mobile Bottom Nav** (`components/layout/mobile-nav.svelte`)
   - 5 primary items + "More" drawer
   - Safe area padding for iOS

6. **Top Bar** (`components/layout/top-bar.svelte`)
   - Page title (dynamic)
   - Quick-log trade button (prominent CTA)
   - Notification bell
   - User dropdown

7. **Navigation Stores**
   - `stores/navigation.svelte.ts` - Sidebar/drawer state
   - `stores/page.svelte.ts` - Page metadata

8. **Placeholder Dashboard** - Proves shell works

---

## Phase 0.5 - Database Migrations ⏳

**Status:** PENDING  
**Estimated Time:** 2-3 hours

### Important Changes from Original Prompts

- **NO Supabase migrations** - Use sqlx-cli instead
- **NO Row Level Security (RLS)** - Application-level security in Rust
- **NO auth.users table** - Custom users table
- Migrations go in `apps/api/migrations/`

### Migration Files Needed

1. **001_users_and_auth.sql**
   - users table (id, email, password_hash, created_at)
   - refresh_tokens table (user_id, token_hash, expires_at, revoked)
   - user_profiles table (all onboarding fields)

2. **002_tags.sql**
   - tags table (user_id, name, category, color, icon)
   - Seed default tags (strategies, mistakes, emotions, sessions, market conditions)

3. **003_trades.sql**
   - trades table (comprehensive trade data)
   - All indexes for common queries

4. **004_trade_tags.sql**
   - Junction table for trade-tag relationships

5. **005_trade_legs.sql**
   - Scaling/position management

6. **006_trade_media.sql**
   - Screenshots, recordings, order confirmations

7. **007_daily_plans.sql**
   - daily_plans table
   - watchlist_items table

8. **008_ai_reviews.sql**
   - ai_reviews table
   - ai_review_messages table (chat history)

9. **009_psychology.sql**
   - mood_logs table
   - trading_goals table
   - tilt_events table
   - alert_rules table

10. **010_playbook.sql**
    - playbook_setups table
    - grading_rubrics table

11. **011_risk_and_scoring.sql**
    - market_snapshots table
    - edge_score_history table

12. **012_social_and_system.sql**
    - user_streaks table
    - shared_rulesets table
    - accountability_links table
    - broker_connections table
    - analytics_cache table
    - economic_events table

### TypeScript Types

Update `packages/types/src/database.ts` with types for ALL tables.

---

## Phase 0.6 - Service Layer (Already Partially Complete) ⏳

**Status:** PARTIALLY COMPLETE  
**Estimated Time:** 3-4 hours

### Already Complete ✅
- API client with token refresh
- Auth service
- Error handling
- Auth store

### Remaining Services

1. **Trade Service** (`services/trades.ts`)
   - All CRUD via Rust API (NOT direct DB access)
   - List, get, create, update, archive trades
   - Filter, sort, pagination

2. **Tag Service** (`services/tags.ts`)
   - Tag CRUD
   - Add/remove tags from trades

3. **Media Service** (`services/media.ts`)
   - Upload to S3 via Rust API
   - Delete, reorder, annotate

4. **Plan Service** (`services/plans.ts`)
   - Daily plan CRUD
   - Watchlist management
   - Carry forward functionality

5. **Psychology Service** (`services/psychology.ts`)
   - Mood logs
   - Goals
   - Tilt events
   - Alert rules

6. **AI Service** (`services/ai.ts`)
   - Request trade reviews (calls Rust API → Claude)
   - Chat follow-ups
   - Daily plan generation
   - Weekly/monthly summaries

7. **Trade Store** (`stores/trades.svelte.ts`)
   - Svelte 5 runes-based state
   - Optimistic updates
   - Error recovery

---

## Phase 0.7 - Auth Pages ⏳

**Status:** PENDING  
**Estimated Time:** 2-3 hours

### Important: No Supabase OAuth

- Use Rust API OAuth endpoints instead
- `/api/v1/auth/google` and `/api/v1/auth/google/callback`
- `/api/v1/auth/apple` and `/api/v1/auth/apple/callback`

### Pages to Create

1. **Auth Layout** (`(auth)/+layout.svelte`)
   - Centered, minimal design

2. **Login Page** (`(auth)/login/+page.svelte`)
   - Email + password
   - OAuth buttons (call Rust API)
   - Forgot password link
   - Remember email (localStorage)

3. **Register Page** (`(auth)/register/+page.svelte`)
   - Email + password + confirm
   - Password strength indicator
   - OAuth options
   - Terms acceptance

4. **Forgot Password** (`(auth)/forgot-password/+page.svelte`)
   - Email input
   - Calls Rust API `/api/v1/auth/forgot-password`

5. **Reset Password** (`(auth)/reset-password/+page.svelte`)
   - New password form
   - Handles token from email

6. **Onboarding Quiz** (`(auth)/onboarding/+page.svelte`)
   - 5-step wizard with progress indicator
   - Step 1: Trading style
   - Step 2: Primary assets
   - Step 3: Experience level
   - Step 4: Account & risk
   - Step 5: Goals & AI personality
   - Saves to user_profiles via Rust API

7. **Password Strength Component** (`components/auth/password-strength.svelte`)

8. **OAuth Buttons Component** (`components/auth/oauth-buttons.svelte`)

---

## Phase 0.8 - Rust API Implementation ⏳

**Status:** SCAFFOLDED (Cargo.toml exists)  
**Estimated Time:** 6-8 hours

### Core Implementation

1. **Main Application** (`src/main.rs`)
   - Load config
   - Initialize logging
   - Create DB pool
   - Build router
   - Apply middleware
   - Graceful shutdown

2. **Configuration** (`src/config.rs`)
   - Load all env vars
   - Validate on startup

3. **Error Handling** (`src/error.rs`)
   - AppError enum
   - IntoResponse implementation
   - User-friendly error messages

4. **Auth Middleware** (`src/middleware/auth.rs`)
   - Validate JWT from Authorization header
   - Extract user_id
   - AuthUser extractor

5. **Routes**
   - `routes/health.rs` - Health check
   - `routes/auth.rs` - All auth endpoints (register, login, refresh, logout, OAuth, password reset)
   - `routes/profile.rs` - User profile CRUD
   - `routes/trades.rs` - Trade CRUD
   - `routes/tags.rs` - Tag CRUD
   - `routes/media.rs` - File upload/delete
   - `routes/plans.rs` - Daily plan CRUD
   - `routes/ai.rs` - AI review endpoints
   - `routes/analytics.rs` - Analytics calculations
   - `routes/risk.rs` - Risk calculations
   - `routes/psychology.rs` - Mood, goals, tilt

6. **Services**
   - `services/auth.rs` - Password hashing (argon2), JWT generation/validation
   - `services/storage.rs` - S3 upload/delete/presign
   - `services/email.rs` - SMTP email sending
   - `services/ai_engine.rs` - Claude API integration
   - `services/analytics_engine.rs` - Analytics computations

7. **Models**
   - All database models matching schema
   - Request/response types
   - Use Decimal for all financial values

8. **Deployment**
   - Dockerfile (multi-stage build)
   - fly.toml (Fly.io config)

---

## Key Differences from Original Prompts

### What Changed

1. **No Supabase Client** - All data access via Rust API
2. **No RLS Policies** - Application-level security (WHERE user_id = $1)
3. **No Supabase Auth** - Custom JWT implementation
4. **No Supabase Storage** - S3-compatible storage via Rust
5. **No Direct DB Access from Frontend** - Everything through API
6. **Custom Users Table** - Not auth.users
7. **Refresh Token Rotation** - Implemented in Rust
8. **OAuth via Rust** - Not Supabase OAuth

### What Stayed the Same

1. Database schema (same tables, same fields)
2. Frontend architecture (SvelteKit 5, Svelte 5 runes)
3. Design system (Tailwind CSS 4, dark-mode first)
4. Component library (shadcn-svelte base)
5. Chart libraries (ECharts, Lightweight Charts)
6. AI integration (Claude API)
7. Feature set (all features remain)

---

## Implementation Priority

### High Priority (Core Functionality)
1. Phase 0.8 - Rust API auth implementation
2. Phase 0.5 - Database migrations
3. Phase 0.7 - Auth pages
4. Phase 0.4 - App shell

### Medium Priority (Enhanced UX)
1. Phase 0.3 - UI components
2. Phase 0.6 - Service layer completion

### Can Be Iterative
- Additional analytics endpoints
- AI review refinements
- Broker sync features

---

## Next Steps

To complete Phase 0, implement in this order:

1. **Database Migrations** (Phase 0.5) - Foundation for everything
2. **Rust Auth API** (Phase 0.8 - Auth routes only) - Enable login/register
3. **Auth Pages** (Phase 0.7) - User can sign up and log in
4. **App Shell** (Phase 0.4) - Navigation structure
5. **UI Components** (Phase 0.3) - As needed for features
6. **Service Layer** (Phase 0.6) - Complete remaining services
7. **Rust API Routes** (Phase 0.8 - Remaining routes) - Feature endpoints

---

## Testing Checklist

After Phase 0 completion, verify:

- [ ] User can register with email/password
- [ ] User can log in
- [ ] User can complete onboarding
- [ ] User is redirected to dashboard
- [ ] Sidebar navigation works
- [ ] Mobile bottom nav works
- [ ] Token refresh works automatically
- [ ] Logout works
- [ ] Password reset flow works
- [ ] OAuth (Google/Apple) works
- [ ] All TypeScript compiles with zero errors
- [ ] All Rust code compiles
- [ ] Database migrations run successfully
- [ ] Docker Compose services start
- [ ] Health check endpoint responds

---

**Phase 0 Status:** 2/8 phases complete (25%)  
**Estimated Remaining Time:** 19-27 hours  
**Ready to Continue:** YES
