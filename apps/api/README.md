# TradeMaster AI - Rust API

Production-ready Rust/Axum backend API for TradeMaster AI.

## Features

- ✅ Custom JWT authentication (no Supabase)
- ✅ PostgreSQL 16 via sqlx
- ✅ Argon2 password hashing
- ✅ Token refresh rotation
- ✅ Comprehensive error handling
- ✅ CORS configuration
- ✅ Structured logging
- ✅ Database migrations
- ✅ Graceful shutdown

## Prerequisites

- Rust 1.75+ (`rustup update`)
- PostgreSQL 16 (via Docker Compose)
- sqlx-cli: `cargo install sqlx-cli --no-default-features --features postgres`

## Setup

### 1. Start Database

```bash
# From project root
docker-compose up -d postgres
```

### 2. Configure Environment

```bash
# Copy example env file
cp .env.example .env

# Edit .env with your values
# Minimum required:
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/trademaster
JWT_SECRET=your-secret-key-at-least-32-characters-long
```

### 3. Run Migrations

```bash
sqlx database create
sqlx migrate run
```

### 4. Run the API

```bash
# Development
cargo run

# Production build
cargo build --release
./target/release/trademaster-api
```

## API Endpoints

### Health Check
- `GET /api/health` - Health check with database status

### Authentication
- `POST /api/v1/auth/register` - Register new user
- `POST /api/v1/auth/login` - Login
- `POST /api/v1/auth/refresh` - Refresh access token
- `POST /api/v1/auth/logout` - Logout (revoke refresh tokens)
- `GET /api/v1/auth/me` - Get current user info (requires auth)

## Environment Variables

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `DATABASE_URL` | Yes | - | PostgreSQL connection string |
| `JWT_SECRET` | Yes | - | JWT signing secret (min 32 chars) |
| `PORT` | No | 3000 | Server port |
| `CORS_ORIGINS` | No | http://localhost:5173 | Comma-separated allowed origins |
| `JWT_ACCESS_EXPIRY_SECONDS` | No | 900 | Access token expiry (15 min) |
| `JWT_REFRESH_EXPIRY_SECONDS` | No | 2592000 | Refresh token expiry (30 days) |
| `MAX_POOL_CONNECTIONS` | No | 10 | Database connection pool size |
| `ANTHROPIC_API_KEY` | No | - | Claude API key for AI features |
| `S3_ENDPOINT` | No | http://localhost:9000 | S3-compatible endpoint |
| `S3_REGION` | No | us-east-1 | S3 region |
| `S3_BUCKET` | No | trademaster-media | S3 bucket name |
| `S3_ACCESS_KEY` | No | minioadmin | S3 access key |
| `S3_SECRET_KEY` | No | minioadmin | S3 secret key |

## Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_password_hashing
```

## Project Structure

```
src/
├── main.rs              # Application entry point
├── config.rs            # Configuration management
├── error.rs             # Error types and handling
├── middleware/
│   ├── mod.rs
│   └── auth.rs          # JWT authentication middleware
├── models/
│   ├── mod.rs
│   ├── user.rs          # User models
│   └── auth.rs          # Auth request/response types
├── routes/
│   ├── mod.rs
│   ├── health.rs        # Health check endpoint
│   └── auth.rs          # Authentication endpoints
└── services/
    ├── mod.rs
    └── auth.rs          # Authentication service (JWT, password hashing)
```

## Development

### Adding a New Route

1. Create route handler in `src/routes/`
2. Add route to `src/routes/mod.rs`
3. Register route in `src/main.rs`

### Adding a New Service

1. Create service in `src/services/`
2. Add service to `src/services/mod.rs`
3. Initialize in `main.rs` and add to app state

### Database Migrations

```bash
# Create new migration
sqlx migrate add <name>

# Run migrations
sqlx migrate run

# Revert last migration
sqlx migrate revert
```

## Security

- Passwords hashed with Argon2 (memory-hard, resistant to GPU attacks)
- JWT tokens with configurable expiry
- Refresh token rotation (old tokens revoked on refresh)
- CORS protection
- SQL injection protection via sqlx parameterized queries
- No sensitive data in error responses

## Performance

- Connection pooling (configurable size)
- Async/await throughout (Tokio runtime)
- Efficient database queries with indexes
- Graceful shutdown handling

## Deployment

See `Dockerfile` for containerized deployment.

```bash
# Build Docker image
docker build -t trademaster-api .

# Run container
docker run -p 3000:3000 --env-file .env trademaster-api
```

## License

Proprietary - All rights reserved
