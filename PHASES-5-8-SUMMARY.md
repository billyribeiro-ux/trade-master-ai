# Phases 5-8 - Risk, Psychology, Playbook, Review Summary

**Status:** Ready to Implement  
**Date:** February 15, 2026  
**Architecture:** Full Rust Backend (NO Supabase)

## Overview

Phases 5-8 complete the core feature set with advanced risk management, psychology tracking, playbook building, and sophisticated review tools. These phases transform TradeMaster AI from a trade logger into a complete trading performance system.

---

## Phase 5 - Risk Management Suite (1 Prompt)

**Total Estimated Time:** 6-8 hours

### Phase 5.1 - Complete Risk Management System ⏳

**Deliverables:**

**Backend (Rust):**
- `apps/api/src/services/risk_engine.rs` - Risk computation
  - calculate_position_size() - 5 methods:
    - Percentage Risk (most common)
    - ATR Volatility
    - Kelly Criterion (capped at 2x kelly)
    - Fixed Fractional
    - Custom (expression parser)
  - compute_portfolio_heat() - Open position risk
  - run_stress_test() - Scenario analysis
  - compute_sizing_consistency() - Actual vs intended risk
  - run_what_if() - Hypothetical scenarios
  - compute_fee_comparison() - Broker fee analysis

- `apps/api/src/routes/risk.rs` - Risk endpoints
  - POST /api/v1/risk/position-size
  - GET /api/v1/risk/portfolio-heat
  - POST /api/v1/risk/stress-test
  - GET /api/v1/risk/sizing-consistency
  - POST /api/v1/risk/what-if
  - GET /api/v1/risk/fee-comparison

**Frontend:**
- `apps/web/src/routes/(app)/risk/+page.svelte` - Risk hub
  - Tabs: Calculator | Portfolio | What If | Fees

- `apps/web/src/lib/components/risk/position-calculator.svelte`
  - 5 method selector
  - Dynamic inputs per method
  - Large position size display
  - Direction toggle (long/short)
  - Quick-fill from profile

- `apps/web/src/lib/components/risk/portfolio-heat.svelte`
  - Total risk gauge
  - Open positions list
  - Correlation warnings
  - Sector concentration chart
  - Stress test trigger

- `apps/web/src/lib/components/risk/stress-test.svelte`
  - Scenario inputs (gap %, all stops hit)
  - Per-position impact breakdown
  - Visual bar chart

- `apps/web/src/lib/components/risk/what-if-simulator.svelte`
  - Scenario checkboxes
  - Original vs modified metrics
  - Delta highlighting
  - "Biggest leak" detection

- `apps/web/src/lib/components/risk/sizing-consistency.svelte`
  - Consistency score gauge (0-100)
  - Risk % histogram
  - Intended vs actual comparison

- `apps/web/src/lib/components/risk/fee-comparison.svelte`
  - Fee breakdown pie chart
  - Running commission total
  - Alternative broker comparison

**Critical Requirements:**
- Position sizing math EXACT (Decimal arithmetic)
- Calculator fast for live trading
- Kelly criterion capped (max 25% of account)
- Stress test assumptions clearly labeled
- What-if results marked as hypothetical
- Portfolio heat auto-refresh
- Mobile: numeric keyboard, large readable results

---

## Phase 6 - Psychology & Discipline Tools (2 Prompts)

**Total Estimated Time:** 8-12 hours

### Phase 6.1 - Psychology Engine (Rust API) ⏳
**Estimated Time:** 4-6 hours

**Deliverables:**
- `apps/api/src/services/tilt_detector.rs` - Tilt detection
  - check_for_tilt() - Called after every trade
  - Detection rules:
    - RAPID_TRADES (3+ in 15min after loss)
    - SIZE_ESCALATION (>50% larger after loss)
    - LOSS_CHASING (3+ losses, new trade in 2min)
    - DAILY_LIMIT (exceeds max daily loss)
    - WATCHLIST_DEVIATION (not on plan)
  - Severity: warning, alert, critical
  - Historical context (actual win rate in tilt state)

- `apps/api/src/services/edge_score.rs` - Personal Edge Score
  - compute_edge_score() - Daily composite (0-100)
  - Components (weighted):
    - Plan adherence (20%)
    - Grade distribution (20%)
    - Risk management (20%)
    - Journal quality (15%)
    - Emotional stability (15%)
    - Normalized P&L (10%)
  - AI explanation via Claude

- `apps/api/src/services/streak_tracker.rs` - Streak management
  - update_streaks() - Daily update
  - Types: journal, planning, plan_adherence, review
  - Quality-weighted decay (20% reduction vs reset)

- `apps/api/src/services/drawdown_monitor.rs` - Drawdown recovery
  - check_drawdown() - After each trade
  - Threshold detection (default 10%)
  - Suggested actions
  - Recovery milestones

- `apps/api/src/routes/psychology.rs` - Psychology endpoints
  - Mood CRUD
  - Goals CRUD
  - Tilt events
  - Alert rules CRUD
  - Edge score (current + history)
  - Streaks
  - Drawdown status

**Critical Requirements:**
- Tilt detection FAST (< 100ms)
- Historical win rate stat accurate
- Edge score cached daily
- Alert rules support all conditions
- Timezone-aware streak tracking

---

### Phase 6.2 - Psychology Frontend ⏳
**Estimated Time:** 4-6 hours

**Deliverables:**
- `apps/web/src/routes/(app)/psychology/+page.svelte` - Psychology hub
  - Edge Score gauge (large)
  - Streak counters
  - Tabs: Mood | Goals | Alerts | Edge Score | Trade DNA
  - Tilt alert banner (dismissible)
  - Drawdown recovery banner

- `apps/web/src/lib/components/psychology/edge-score-display.svelte`
  - Large circular gauge (0-100)
  - Color gradient (red → green)
  - Component breakdown (6 progress bars)
  - AI explanation
  - 30-day trend sparkline

- `apps/web/src/lib/components/psychology/mood-journal.svelte`
  - Date selector
  - 5 sliders (pre-market, post-market, stress, confidence, sleep)
  - Emotion tags (multi-select pills)
  - Notes textarea
  - Auto-save
  - Historical mood trend chart

- `apps/web/src/lib/components/psychology/tilt-alert-banner.svelte`
  - Full-width banner (top of dashboard)
  - Severity color-coded
  - Message + suggested action
  - "Acknowledge" button
  - "View Details" expansion

- `apps/web/src/lib/components/psychology/revenge-trade-cooldown.svelte`
  - Full-screen overlay (opt-in)
  - Countdown timer (30s-5min)
  - Breathing exercise animation
  - Custom reminder text
  - "I've calmed down" button (after timer)

- `apps/web/src/lib/components/psychology/goal-tracker.svelte`
  - Active goals list (cards)
  - Progress bars
  - Target dates with countdown
  - Status badges
  - "New Goal" dialog
  - Achieved goals section

- `apps/web/src/lib/components/psychology/alert-rules-builder.svelte`
  - Current rules list
  - Visual rule builder (no code)
  - Condition types + operators
  - Actions (notify, cooldown, lockout, journal required)
  - Enable/disable toggles

- `apps/web/src/lib/components/psychology/loss-autopsy.svelte`
  - Losing trades categorized
  - "Good Loss" vs "Bad Loss" badges
  - Summary stats + trend
  - Stacked bar chart

- `apps/web/src/lib/components/psychology/streaks-display.svelte`
  - 4 streak cards (journal, planning, adherence, review)
  - Current + best + decay value
  - Quality indicator
  - Calendar view

- `apps/web/src/lib/components/psychology/trade-dna.svelte`
  - Radar chart (8 axes)
  - Hold duration, risk taking, win rate, conviction accuracy, plan adherence, emotional stability, setup selectivity, time discipline
  - Overlay previous month
  - Text summary

- `apps/web/src/lib/components/psychology/confidence-calibration.svelte`
  - Expected vs actual win rate by conviction
  - Calibration score
  - Insight text

- `apps/web/src/lib/components/psychology/gamification-badges.svelte`
  - Badge grid (locked/unlocked)
  - Progress indicators
  - Earned dates
  - Toast on unlock

- `apps/web/src/lib/components/psychology/drawdown-recovery-banner.svelte`
  - Persistent banner in drawdown mode
  - Current drawdown %
  - Recovery milestones
  - Suggested actions

**Critical Requirements:**
- Tilt alerts VISIBLE and UNMISSABLE
- Mood journal quick to fill (sliders, not text)
- Gamification rewards PROCESS (never P&L)
- Revenge cooldown respectful (opt-in)
- Edge score shows trends
- Warm, supportive language throughout
- Mobile: thumb-friendly sliders, tappable badges

---

## Phase 7 - Setup Playbook & Trade Grading (1 Prompt)

**Total Estimated Time:** 5-7 hours

### Phase 7.1 - Complete Playbook & Grading System ⏳

**Deliverables:**

**Backend (Rust):**
- `apps/api/src/routes/playbook.rs` - Playbook endpoints
  - Setup CRUD
  - POST /api/v1/playbook/:id/match - Score trade against setup
  - Rubric CRUD
  - POST /api/v1/rubrics/:id/grade - Grade trade
  - Shared rulesets (list, create, import)

- `apps/api/src/services/playbook_engine.rs`
  - match_trade_to_setup() - Criteria matching
  - compute_setup_stats() - Performance tracking
  - grade_trade() - Apply rubric
  - compute_exit_quality() - Exit vs MFE

**Frontend:**
- `apps/web/src/routes/(app)/playbook/+page.svelte`
  - Tabs: My Setups | Grading Rubric | Shared Rulesets

- `apps/web/src/lib/components/playbook/setup-list.svelte`
  - Grid of setup cards
  - Performance stats per setup
  - Active badges
  - "Create New Setup" card

- `apps/web/src/lib/components/playbook/setup-editor.svelte`
  - Name, description
  - Criteria builder (dynamic list)
  - Each criterion: name, weight, required, description
  - Expected R-multiple range
  - Min conviction
  - Timeframe preference
  - Market regime filter
  - Example screenshots
  - Common mistakes notes

- `apps/web/src/lib/components/playbook/setup-performance.svelte`
  - Stats display
  - Mini equity curve
  - Win rate trend
  - Decay detection alert

- `apps/web/src/lib/components/playbook/rubric-editor.svelte`
  - Rubric name
  - Dynamic criteria (weights sum to 100%)
  - Grade thresholds (customizable)
  - Preview with sample scoring

- `apps/web/src/lib/components/playbook/trade-grader.svelte`
  - Criteria sliders
  - Live grade computation
  - Grade badge (A/B/C/D)
  - Numeric score

- `apps/web/src/lib/components/playbook/shared-rulesets.svelte`
  - Browse public rulesets
  - Import button
  - Share dialog
  - Private share links

**Trade Form Integration:**
- When setup selected: show criteria checklist
- AI matches and scores adherence
- "Matches 4/6 criteria — B-grade setup" inline

**Critical Requirements:**
- Setup stats accurate and auto-update
- Rubric grading real-time (no submit-wait)
- Weights sum to 100% (enforce)
- Shared rulesets no sensitive data
- Screenshots saved correctly
- Decay detection (>15% drop)
- Feels like personal trading manual

---

## Phase 8 - Advanced Review & Replay (1 Prompt)

**Total Estimated Time:** 5-7 hours

### Phase 8.1 - Trade Replay, Timeline, Weekend Review, Audit ⏳

**Deliverables:**

**Frontend:**
- `apps/web/src/routes/(app)/review/replay/[id]/+page.svelte`
  - Trade replay page
  - Loads trade + chart data

- `apps/web/src/lib/components/review/trade-replay.svelte`
  - Screenshot-based replay (initial)
  - Speed controls (0.5x-5x)
  - Play/pause, scrub bar
  - Entry/exit/stop/TP overlays
  - Trade info panel
  - Decision points
  - Future: Lightweight Charts with candle data

- `apps/web/src/lib/components/review/decision-point.svelte`
  - Fork-in-the-road training
  - Pause at key moments
  - "What would you do?" dialog
  - Options: Enter, Wait, Exit, Move Stop, Do Nothing
  - Reveal actual + optimal decision
  - Score practice decisions
  - Track accuracy over time

- `apps/web/src/routes/(app)/review/timeline/+page.svelte`
  - Chronological screenshot gallery
  - Vertical timeline with date headers
  - Thumbnail + trade context
  - Click for lightbox
  - Filters (date, symbol, win/loss, grade)
  - Infinite scroll
  - Search

- `apps/web/src/lib/components/review/weekend-review.svelte`
  - Guided 15-minute ritual
  - 8-step flow:
    1. Week at a Glance (summary stats)
    2. Best Trade (auto-selected by R)
    3. Worst Trade (auto-selected)
    4. Biggest Lesson (AI-generated)
    5. Plan Adherence (trend chart)
    6. Emotional Arc (mood + P&L overlay)
    7. Focus for Next Week (AI recommendation)
    8. Reflection (free-text journal)
  - Progress indicator
  - "Export as PDF" button
  - Saved as weekly review record
  - Streak tracking

- `apps/api/src/routes/review.rs` - Review endpoints
  - GET /api/v1/review/weekend-data
  - POST /api/v1/review/weekend-complete
  - GET /api/v1/review/timeline
  - GET /api/v1/review/decision-history

- `apps/web/src/lib/components/review/data-integrity-audit.svelte`
  - Monthly data quality analysis
  - Missing data counts (screenshots, stops, notes, tags, grades)
  - Overall quality score (0-100)
  - Per-category progress bars
  - "Fix Now" buttons
  - Tip about AI review accuracy

- `apps/web/src/lib/components/review/exit-quality-display.svelte`
  - Exit quality score
  - Your exit vs MFE/TP
  - Distribution histogram
  - Trend over time
  - Insight text

**Dashboard Updates:**
- Weekend review reminder (Sat/Sun)
- Data integrity alert (< 70%)
- Recent AI reviews quick-access

**Critical Requirements:**
- Replay works with screenshots initially
- Decision point results saved
- Weekend review feels like ritual (15min)
- Screenshot timeline efficient (virtual scroll)
- Data audit encouraging (not punishing)
- PDF export clean one-pager
- Exit quality clearly explained

---

## Remaining Phases (9-12) - Summary

### Phase 9 - Broker Integration (2 Prompts)
**Estimated Time:** 8-12 hours

**Prompt 9.1** - Broker Integration Architecture
- Abstract BrokerAdapter trait (Rust)
- Implementations: Thinkorswim, Interactive Brokers, TradingView webhook
- Sync engine (polling, deduplication, conflict resolution)
- Encrypted credential storage
- Market internals auto-capture (TICK, ADD, VOLD, VIX)
- Frontend: connection settings, sync status

**Prompt 9.2** - Additional Brokers + Dashboard
- TradeStation, MetaTrader, Coinbase, Binance, Bybit
- Broker fee comparison data
- Sync history and error log UI
- Manual sync trigger
- Connection health monitoring

---

### Phase 10 - Accountability & Social (1 Prompt)
**Estimated Time:** 4-6 hours

**Prompt 10.1** - Accountability System
- Coach/mentor dashboard (read-only with permissions)
- Invitation system (email, accept/decline)
- Permission toggles (plan, grades, tilt, P&L, mood, reviews)
- Anonymous trade sharing (strip PII)
- Community leaderboard (edge score, adherence, streaks - NOT P&L)
- Frontend: sharing settings, coach view, community feed

---

### Phase 11 - Mobile Native & Polish (2 Prompts)
**Estimated Time:** 10-14 hours

**Prompt 11.1** - PWA Enhancement + Capacitor
- Service worker (offline trade logging, plan viewing)
- Offline queue (sync when reconnected)
- Capacitor setup (iOS + Android)
- Push notifications (tilt, plan reminders, streaks)
- Biometric auth (Face ID, fingerprint)
- Camera integration

**Prompt 11.2** - Home Screen Widgets + Polish
- iOS/Android widgets (daily P&L, edge score, plan)
- Quick-log from notification
- App Store metadata
- Performance optimization (lazy loading, code splitting, images)
- Accessibility audit
- Responsive design polish

---

### Phase 12 - Performance, Security & Scale (1 Prompt)
**Estimated Time:** 8-12 hours

**Prompt 12.1** - Production Hardening
- Database optimization (query analysis, indexes, materialized views)
- Connection pooling
- Rate limiting (all endpoints)
- End-to-end encryption (broker credentials)
- GDPR compliance (data export, account deletion)
- Error monitoring (Sentry)
- Structured logging (JSON, request tracing)
- Load testing (k6)
- Backup strategy (automated, point-in-time recovery)
- CI/CD hardening (staging, integration tests, rollback)
- Complete Docker Compose environment

---

## Complete Project Summary

### Total Prompt Count: 34 Prompts

| Phase | Prompts | Time Estimate | Focus |
|-------|---------|---------------|-------|
| 0 | 10 | 40-60 hours | Foundation & Infrastructure |
| 1 | 6 | 23-31 hours | Core Trade Logging |
| 2 | 4 | 18-24 hours | Analytics Engine |
| 3 | 1 | 5-7 hours | Daily Planning |
| 4 | 2 | 8-12 hours | AI Trade Review |
| 5 | 1 | 6-8 hours | Risk Management |
| 6 | 2 | 8-12 hours | Psychology & Discipline |
| 7 | 1 | 5-7 hours | Playbook & Grading |
| 8 | 1 | 5-7 hours | Advanced Review & Replay |
| 9 | 2 | 8-12 hours | Broker Integration |
| 10 | 1 | 4-6 hours | Accountability & Social |
| 11 | 2 | 10-14 hours | Mobile Native & Polish |
| 12 | 1 | 8-12 hours | Performance & Security |
| **TOTAL** | **34** | **148-212 hours** | **Complete Application** |

### Implementation Order

**Critical Path (MVP):**
1. Phase 0 (Foundation) - Required for everything
2. Phase 1 (Trade Logging) - Core functionality
3. Phase 2 (Analytics) - Value demonstration
4. Phase 4 (AI Review) - Killer feature
5. Phase 3 (Planning) - Proactive workflow

**Enhancement Path:**
6. Phase 5 (Risk Management) - Professional tools
7. Phase 6 (Psychology) - Discipline system
8. Phase 7 (Playbook) - Setup optimization
9. Phase 8 (Review) - Advanced learning

**Scale Path:**
10. Phase 9 (Broker Integration) - Automation
11. Phase 10 (Social) - Community
12. Phase 11 (Mobile) - Accessibility
13. Phase 12 (Production) - Scale & security

---

## Key Features Summary

### Phase 5 - Risk Management
- ✅ 5 position sizing methods
- ✅ Portfolio heat monitoring
- ✅ Stress testing
- ✅ Sizing consistency tracking
- ✅ What-if simulator
- ✅ Fee comparison

### Phase 6 - Psychology
- ✅ Tilt detection (5 rules)
- ✅ Personal Edge Score (6 components)
- ✅ Mood journaling
- ✅ Goal tracking
- ✅ Alert rules builder
- ✅ Revenge trade cooldown
- ✅ Streak tracking (4 types)
- ✅ Loss autopsy
- ✅ Trade DNA fingerprint
- ✅ Confidence calibration
- ✅ Gamification badges
- ✅ Drawdown recovery mode

### Phase 7 - Playbook
- ✅ Setup builder with criteria
- ✅ Performance tracking per setup
- ✅ Grading rubric system
- ✅ Real-time trade grading
- ✅ Exit quality scoring
- ✅ Shared rulesets
- ✅ Setup decay detection

### Phase 8 - Review
- ✅ Trade replay (screenshot-based)
- ✅ Decision point training
- ✅ Screenshot timeline
- ✅ Weekend review ritual (8 steps)
- ✅ Data integrity audit
- ✅ Exit quality analysis
- ✅ PDF export

---

## Technical Specifications

### Backend (Rust)
- Decimal for all financial calculations
- Fast tilt detection (< 100ms)
- Efficient SQL aggregation
- Encrypted credential storage
- Rate limiting
- Structured logging
- Comprehensive error handling

### Frontend (SvelteKit)
- Svelte 5 runes throughout
- Apache ECharts for visualizations
- Responsive design (375px-2560px)
- Offline support (PWA)
- Push notifications
- Biometric auth
- Home screen widgets

### Performance Targets
- Tilt detection: < 100ms
- Position calculator: instant
- Portfolio heat: < 500ms
- Weekend review: completable in 15min
- Screenshot timeline: efficient with 100s of images
- Data audit: encouraging, not punishing

---

## Architecture Consistency

**All Phases Use:**
- ✅ Full Rust backend (NO Supabase)
- ✅ PostgreSQL 16 via sqlx
- ✅ S3-compatible storage (MinIO/R2)
- ✅ JWT authentication
- ✅ Decimal for financial values
- ✅ Svelte 5 runes
- ✅ TypeScript strict mode
- ✅ Tailwind CSS v4
- ✅ Design system tokens
- ✅ Responsive design
- ✅ Accessibility standards

---

## Next Steps

The complete TradeMaster AI application is now fully documented across:
- Phase 0 prompts (Foundation)
- Phase 1-4 prompts (Core features)
- Phase 5-8 prompts (Advanced features)
- Phase 9-12 summary (Integration & scale)

**Total: 34 prompts covering every feature from the specification**

Ready to implement any phase on request. All documentation is complete, architecture is consistent, and the path to a production-ready application is clear.

---

**Phases 5-8 Status:** Ready to Implement  
**Prerequisites:** Phases 0-4 for full functionality  
**Architecture:** Full Rust Backend, NO Supabase  
**Estimated Total Time:** 24-34 hours for Phases 5-8
