# Database Migrations

This directory contains all PostgreSQL migrations for the TradeMaster AI application.

## Prerequisites

1. **PostgreSQL 16** running (via Docker Compose)
2. **sqlx-cli** installed: `cargo install sqlx-cli --no-default-features --features postgres`

## Running Migrations

### First Time Setup

```bash
# From the apps/api directory
cd apps/api

# Create the database
sqlx database create

# Run all migrations
sqlx migrate run
```

### Adding New Migrations

```bash
# Create a new migration
sqlx migrate add <migration_name>

# Edit the generated file in migrations/
# Then run it
sqlx migrate run
```

### Reverting Migrations

```bash
# Revert the last migration
sqlx migrate revert
```

## Migration Files

| File | Description | Tables Created |
|------|-------------|----------------|
| `001_users_and_auth.sql` | User authentication | users, refresh_tokens, user_profiles |
| `002_tags.sql` | Tag system | tags + seed function |
| `003_trades.sql` | Core trades | trades |
| `004_trade_tags.sql` | Trade-tag relationship | trade_tags |
| `005_trade_legs.sql` | Position scaling | trade_legs |
| `006_trade_media.sql` | Screenshots/media | trade_media |
| `007_daily_plans.sql` | Daily planning | daily_plans, watchlist_items |
| `008_ai_reviews.sql` | AI trade reviews | ai_reviews, ai_review_messages |
| `009_psychology.sql` | Psychology tracking | mood_logs, trading_goals, tilt_events, alert_rules |
| `010_playbook.sql` | Setup playbook | playbook_setups, grading_rubrics, shared_rulesets |
| `011_risk_and_scoring.sql` | Risk & edge score | market_snapshots, edge_score_history |
| `012_social_and_system.sql` | Supporting tables | user_streaks, accountability_links, broker_connections, analytics_cache, economic_events, weekly_reviews |

## Total Tables: 24

## Environment Variables

Make sure your `.env` file has:

```env
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/trademaster
```

## Verification

After running migrations, verify with:

```bash
# Connect to database
psql postgresql://postgres:postgres@localhost:5432/trademaster

# List all tables
\dt

# Should see 24 tables
```

## Notes

- All migrations use `gen_random_uuid()` for primary keys
- All tables have `created_at` timestamps
- Most tables have `updated_at` with automatic triggers
- Foreign keys use `ON DELETE CASCADE` where appropriate
- Indexes created for common query patterns
- No RLS policies (application-level security in Rust)
