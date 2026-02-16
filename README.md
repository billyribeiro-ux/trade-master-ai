# TradeMaster AI

The ultimate AI-powered trading journal app built with SvelteKit 5 and Rust.

## Architecture Overview

**Frontend:** SvelteKit 5 with Svelte 5 runes, TypeScript strict mode, Tailwind CSS 4  
**Backend:** Rust/Axum API with PostgreSQL 16  
**Storage:** S3-compatible (Cloudflare R2 in production, MinIO in development)  
**Auth:** Custom JWT-based authentication with refresh token rotation  
**AI:** Anthropic Claude API for trade reviews and insights

## Stack Migration Note

This project **does not use Supabase**. All references to Supabase in the original prompts have been replaced with:

- **Database:** Direct PostgreSQL access via sqlx in Rust
- **Auth:** Custom Rust auth service with JWT + refresh tokens
- **Storage:** S3-compatible storage (Cloudflare R2/MinIO)
- **API:** Single Rust/Axum backend handling all operations

The frontend communicates exclusively with the Rust API. No direct database access from the frontend.

## Project Structure

```
trademaster-ai/
├── apps/
│   ├── web/              # SvelteKit 5 frontend
│   └── api/              # Rust/Axum backend
├── packages/
│   ├── types/            # Shared TypeScript types
│   └── calculations/     # Trading math utilities
├── docker-compose.yml    # PostgreSQL + MinIO for local dev
└── pnpm-workspace.yaml   # Monorepo configuration
```

## Getting Started

### Prerequisites

- Node.js 20+
- pnpm 8+
- Rust 1.75+
- Docker & Docker Compose

### Local Development Setup

1. **Clone and install dependencies:**

```bash
pnpm install
```

2. **Start local services (PostgreSQL + MinIO):**

```bash
docker-compose up -d
```

3. **Set up environment variables:**

```bash
# Frontend (.env in root)
cp .env.example .env

# Backend (apps/api/.env)
cp apps/api/.env.example apps/api/.env
```

4. **Run database migrations:**

```bash
cd apps/api
sqlx migrate run
```

5. **Start the development servers:**

```bash
# Terminal 1: Frontend
pnpm dev

# Terminal 2: Backend
cd apps/api
cargo run
```

The frontend will be available at `http://localhost:5173`  
The backend API will be available at `http://localhost:3001`

## Environment Variables

### Frontend (`/.env`)

```env
PUBLIC_API_URL=http://localhost:3001
PUBLIC_APP_URL=http://localhost:5173
```

### Backend (`/apps/api/.env`)

See `apps/api/.env.example` for the complete list including:
- Database connection
- JWT secrets
- S3/MinIO configuration
- OAuth credentials
- SMTP settings

## API Architecture

All data operations flow through the Rust API:

```
Frontend (SvelteKit) → Rust API (Axum) → PostgreSQL
                     ↓
                   S3 Storage (MinIO/R2)
```

### Key API Endpoints

- **Auth:** `/api/v1/auth/*` - Registration, login, OAuth, password reset
- **Trades:** `/api/v1/trades/*` - CRUD operations for trades
- **Tags:** `/api/v1/tags/*` - Tag management
- **Media:** `/api/v1/media/*` - File uploads
- **Plans:** `/api/v1/plans/*` - Daily trading plans
- **AI:** `/api/v1/ai/*` - AI-powered trade reviews
- **Analytics:** `/api/v1/analytics/*` - Performance metrics

## Authentication Flow

1. User logs in → Backend returns access token (15min) + sets httpOnly refresh cookie (30 days)
2. Frontend stores access token in memory
3. All API requests include `Authorization: Bearer {token}`
4. On 401 response → Frontend auto-refreshes token using cookie
5. Refresh token rotation: each refresh issues new tokens and revokes old ones

## Development Commands

```bash
# Frontend
pnpm dev              # Start dev server
pnpm build            # Build for production
pnpm type-check       # TypeScript type checking
pnpm lint             # ESLint

# Backend
cargo run             # Start API server
cargo test            # Run tests
cargo build --release # Production build
sqlx migrate run      # Run migrations
```

## Phase 0 Status

**Phase 0.1 - Project Scaffolding:** ✅ Complete
- Monorepo structure created
- SvelteKit 5 configured with Tailwind CSS 4
- Rust API scaffolded with Cargo.toml
- Docker Compose for local services
- Auth service architecture implemented
- All Supabase dependencies removed

**Next Steps:**
- Phase 0.2: Design System Tokens
- Phase 0.3: shadcn-svelte Base Components
- Phase 0.4: App Shell & Navigation
- Phase 0.5: Database Migrations
- Phase 0.6: Rust Auth Implementation

## Tech Stack Details

### Frontend
- **Framework:** SvelteKit 5 with Svelte 5 runes ($state, $derived, $effect)
- **Language:** TypeScript (strict mode)
- **Styling:** Tailwind CSS 4 (dark-mode first)
- **UI:** shadcn-svelte + custom components
- **Charts:** Apache ECharts (analytics), Lightweight Charts (financial)
- **Icons:** Iconify (Phosphor & Carbon sets)

### Backend
- **Framework:** Axum (Rust web framework)
- **Database:** PostgreSQL 16 with sqlx
- **Auth:** argon2 + jsonwebtoken
- **Storage:** aws-sdk-s3 (S3-compatible)
- **Email:** lettre (SMTP)
- **OAuth:** oauth2 crate

## Contributing

This is a personal project following Apple/Microsoft Principal Engineer (ICT Level 7+) standards:
- Zero warnings, zero errors, zero type-safety shortcuts
- Production-ready code only - no TODOs, no placeholders
- 10-year longevity mindset
- Comprehensive error handling and edge case coverage

## License

Private project - All rights reserved
