# Phases 2, 3, 4 - Analytics, Planning, AI Review Summary

**Status:** Ready to Implement  
**Date:** February 15, 2026  
**Architecture:** Full Rust Backend (NO Supabase)

## Overview

Phases 2-4 build on the core trade logging foundation (Phase 1) to add the three pillars that make TradeMaster AI unique: comprehensive analytics, proactive planning, and AI-powered insights.

---

## Phase 2 - Analytics Engine (4 Prompts)

**Total Estimated Time:** 18-24 hours

### Phase 2.1 - Analytics Computation Engine (Rust) ⏳
**Estimated Time:** 6-8 hours

**Deliverables:**
- `apps/api/src/services/analytics_engine.rs` - Core computation service
  - compute_summary() - All core metrics (win rate, profit factor, expectancy, Sharpe, Sortino, etc.)
  - compute_equity_curve() - Daily equity with drawdown tracking
  - compute_heatmap() - Calendar P&L heatmap
  - compute_r_distribution() - R-multiple histogram buckets
  - compute_time_analysis() - Hour-of-day performance
  - compute_dow_analysis() - Day-of-week performance
  - compute_strategy_breakdown() - Performance by setup
  - compute_instrument_matrix() - Performance by symbol
  - compute_session_split() - Performance by session type
  - compute_conviction_analysis() - Conviction level correlation
  - compute_duration_analysis() - Hold time optimization
  - compute_streak_analysis() - Win/loss streak patterns

- `apps/api/src/routes/analytics.rs` - Analytics endpoints
  - GET /api/v1/analytics/summary
  - GET /api/v1/analytics/equity-curve
  - GET /api/v1/analytics/heatmap
  - GET /api/v1/analytics/r-distribution
  - GET /api/v1/analytics/time-analysis
  - GET /api/v1/analytics/dow-analysis
  - GET /api/v1/analytics/strategy-breakdown
  - GET /api/v1/analytics/instrument-matrix
  - GET /api/v1/analytics/session-split
  - GET /api/v1/analytics/conviction-analysis
  - GET /api/v1/analytics/duration-analysis
  - GET /api/v1/analytics/streak-analysis
  - Response caching (1 hour TTL)

- `apps/api/src/models/analytics.rs` - All response types
- Unit tests (30+ test cases)

**Critical Requirements:**
- ALL financial math uses Decimal (NEVER f64)
- Handle division by zero
- Efficient SQL aggregation (not in-memory)
- Complete in under 2 seconds for 10,000+ trades
- Cache invalidation on trade changes

---

### Phase 2.2 - Analytics Dashboard Frontend ⏳
**Estimated Time:** 4-6 hours

**Deliverables:**
- `apps/web/src/routes/(app)/analytics/+page.svelte` - Main dashboard
  - Global filter bar (date range, asset class, setup, tags)
  - Tab navigation (Overview, Strategies, Time, Instruments, Advanced)
  - URL sync for filters

- `apps/web/src/lib/components/analytics/analytics-filters.svelte`
- `apps/web/src/lib/components/dashboard/stat-card.svelte` - KPI cards
- `apps/web/src/lib/components/analytics/metrics-grid.svelte`
  - 4x4 grid of core metrics
  - Total Trades, Win Rate, Profit Factor, Expectancy
  - Total P&L, Avg R, Sharpe, Max Drawdown
  - Avg Winner/Loser, Largest Winner/Loser
  - Avg Duration, Recovery Factor, Sortino, Consecutive W/L

- `apps/web/src/lib/components/analytics/equity-curve.svelte`
  - Apache ECharts line chart
  - Drawdown area overlay
  - Optional benchmark (SPY)
  - Zoom, pan, interactive tooltip
  - Toggle: $ P&L / % Return / R-Multiple

- `apps/web/src/lib/components/analytics/pnl-heatmap.svelte`
  - Calendar heatmap (ECharts)
  - Color scale: red (loss) → gray → green (win)
  - Click day to filter trades

- `apps/web/src/lib/services/analytics.ts` - Frontend service
- `apps/web/src/lib/stores/analytics.svelte.ts` - State management
- `apps/web/src/lib/utils/chart-theme.ts` - ECharts theme builder

**Critical Requirements:**
- ECharts lazy-loaded (dynamic import)
- Charts resize on window/sidebar changes
- All colors from design system
- Handle gaps in equity curve
- Filters update within 500ms
- Mobile: charts stack vertically

---

### Phase 2.3 - Strategy, Time, Instrument Charts ⏳
**Estimated Time:** 4-5 hours

**Deliverables:**
- `apps/web/src/lib/components/analytics/strategy-breakdown.svelte`
  - Horizontal bar chart by setup
  - Table with sortable columns
  - Click to filter trades

- `apps/web/src/lib/components/analytics/time-heatmap.svelte`
  - Hour x Day heatmap
  - P&L by hour bar chart
  - P&L by day bar chart

- `apps/web/src/lib/components/analytics/instrument-matrix.svelte`
  - Grid/treemap of symbols
  - Color by profitability
  - Size by trade count
  - Toggle: grid or table view

- `apps/web/src/lib/components/analytics/session-split.svelte`
  - Stacked/grouped bars by session type
  - Highlight best/worst sessions

- `apps/web/src/lib/components/analytics/conviction-chart.svelte`
  - Bar chart: conviction 1-5
  - Dual axis: win rate (line) + trade count (bars)
  - Calibration score

- `apps/web/src/lib/components/analytics/duration-chart.svelte`
  - Hold duration buckets
  - Highlight optimal range

- `apps/web/src/lib/components/analytics/r-distribution.svelte`
  - Histogram with R-multiple buckets
  - Statistical markers (mean, median, std dev)
  - Normal curve overlay

- `apps/web/src/lib/components/analytics/streak-chart.svelte`
  - Current streak display
  - Performance after N consecutive wins/losses

- `apps/web/src/lib/components/analytics/asymmetric-risk-alert.svelte`
  - Warning when avg loser > avg winner

**Critical Requirements:**
- All charts use centralized theme
- Responsive and resize correctly
- Loading skeleton and empty states
- Handle small data sets gracefully
- Click interactions filter trade list

---

### Phase 2.4 - Advanced Analytics ⏳
**Estimated Time:** 4-5 hours

**Deliverables:**
- `apps/api/src/services/advanced_analytics.rs`
  - run_monte_carlo() - 10,000 simulations
  - compute_vol_adjusted() - VIX-normalized P&L
  - compute_overnight_analysis() - Gap analysis
  - compute_catalyst_analysis() - Event-driven P&L
  - compute_commission_impact() - Fee drag
  - compute_fatigue_curve() - Performance by hour into session
  - compute_trade_correlation() - Single vs multi-position

- API endpoints for all advanced analytics

- Frontend charts:
  - `monte-carlo-chart.svelte` - Fan chart with percentile bands
  - `vol-adjusted-curve.svelte` - Dual-line chart
  - `overnight-analysis.svelte` - Gap distribution
  - `commission-tracker.svelte` - Fee breakdown
  - `fatigue-curve.svelte` - Performance cliff detection
  - `benchmark-comparison.svelte` - vs SPY/QQQ

**Critical Requirements:**
- Monte Carlo: seeded RNG for reproducibility
- Compute percentile bands server-side
- Decimal for all financial math
- Explanatory text for complex concepts

---

## Phase 3 - Daily Planning Module (1 Prompt)

**Total Estimated Time:** 5-7 hours

### Phase 3.1 - Complete Planning System ⏳

**Deliverables:**
- `apps/api/src/routes/plans.rs` - Plan CRUD
  - GET /api/v1/plans
  - GET /api/v1/plans/today
  - GET /api/v1/plans/:date
  - POST /api/v1/plans
  - PUT /api/v1/plans/:id
  - POST /api/v1/plans/:id/complete
  - POST /api/v1/plans/:id/carry-forward
  - Watchlist CRUD endpoints
  - POST /api/v1/plans/:id/adherence

- `apps/api/src/models/plan.rs` - Rust models
- `packages/types/src/plan.ts` - TypeScript types

- `apps/web/src/routes/(app)/plan/+page.svelte` - Plan page
  - Date navigator
  - Auto-create empty plan

- `apps/web/src/lib/components/planning/plan-editor.svelte`
  - Section 1: Market Bias (Bullish/Bearish/Neutral)
  - Section 2: Session Goals (max trades, max loss)
  - Section 3: Pre-Trade Checklist (customizable items)
  - Section 4: Watchlist (expandable cards per symbol)
  - Section 5: Notes
  - Auto-save (1s debounce)
  - "Mark Complete" button
  - "Carry Forward" button

- Watchlist card components:
  - Symbol input
  - Key levels (support/resistance)
  - Catalysts
  - Setup description
  - Risk/Reward ratio
  - Position size suggestion
  - Drag-to-reorder
  - Post-market: outcome tracking

- `apps/web/src/lib/components/planning/economic-calendar.svelte`
  - Today's events
  - Impact indicators (high/medium/low)

- `apps/web/src/lib/components/planning/plan-adherence.svelte`
  - Score 0-100 with color
  - Breakdown by category
  - AI explanation

- `apps/web/src/lib/services/plans.ts` - Frontend service
- `apps/web/src/lib/stores/plan.svelte.ts` - State management

**Critical Requirements:**
- Plan page as default landing before market open
- Auto-save seamless (no save button)
- Load in under 300ms
- Drag-drop reordering with animations
- Satisfying checkbox interactions
- Intelligent carry-forward (only incomplete items)
- Economic calendar highlights high-impact events
- Mobile-optimized (one-hand use)

---

## Phase 4 - AI Trade Review System (2 Prompts)

**Total Estimated Time:** 8-12 hours

### Phase 4.1 - Claude API Integration (Rust) ⏳
**Estimated Time:** 5-7 hours

**Deliverables:**
- `apps/api/src/services/ai_engine.rs` - Claude integration
  - struct AiEngine with reqwest client
  - send_to_claude() - Core API method
    - POST to Anthropic Messages API
    - Handle text + image (base64) content
    - Rate limit handling (429 retry)
    - Token tracking
    - 60s timeout
    - 3 retries with exponential backoff

  - review_trade() - Trade review
    - Fetch trade + media + tags + profile + plan
    - Download screenshots from S3, convert to base64
    - Build structured prompt
    - Adapt to user's AI personality (strict/encouraging/balanced)
    - Parse JSON response
    - Save to ai_reviews table
    - Track tokens and cost

  - chat_follow_up() - Review chat
    - Load conversation history
    - Send to Claude
    - Save to ai_review_messages

  - generate_plan_of_attack() - Daily plan AI
    - Generate briefing from plan context
    - Save to daily_plans.ai_plan_of_attack

  - generate_weekly_summary() - Weekly review
  - generate_monthly_summary() - Monthly review

- `apps/api/src/services/ai_prompts.rs` - All prompts
  - TRADE_REVIEW_SYSTEM_PROMPT_STRICT
  - TRADE_REVIEW_SYSTEM_PROMPT_ENCOURAGING
  - TRADE_REVIEW_SYSTEM_PROMPT_BALANCED
  - PLAN_OF_ATTACK_SYSTEM_PROMPT
  - WEEKLY_SUMMARY_SYSTEM_PROMPT
  - MONTHLY_SUMMARY_SYSTEM_PROMPT
  - Each with VERSION constant
  - All produce structured JSON output

- `apps/api/src/routes/ai.rs` - AI endpoints
  - POST /api/v1/ai/review-trade
  - POST /api/v1/ai/review-chat
  - POST /api/v1/ai/plan-of-attack
  - POST /api/v1/ai/weekly-summary
  - POST /api/v1/ai/monthly-summary
  - GET /api/v1/ai/reviews/:trade_id
  - GET /api/v1/ai/reviews/:review_id/messages
  - GET /api/v1/ai/usage
  - Rate limits: 10 reviews/hour, 30 messages/hour

- `apps/api/src/models/ai.rs` - AI models
  - AiReviewResult (JSON schema)
  - AiReviewRecord (database)
  - AiChatMessage
  - AiPlanOfAttack
  - AiWeeklySummary
  - AiUsageStats

- `packages/types/src/ai.ts` - TypeScript types

**Review JSON Structure:**
```json
{
  "overall_score": 1-10,
  "strengths": ["..."],
  "weaknesses": ["..."],
  "key_lesson": "...",
  "actionable_fixes": ["..."],
  "alternative_scenario": "...",
  "execution_quality_score": 1-10,
  "risk_management_score": 1-10,
  "plan_adherence_score": 1-10,
  "emotional_state_detected": "...",
  "thesis_alignment_score": 1-5,
  "chart_analysis": "..."
}
```

**Critical Requirements:**
- Claude API key NEVER exposed to frontend
- Screenshots resized (max 1568px, JPEG 85%)
- Robust JSON parsing (handle markdown wrapping)
- Server-side rate limiting
- Track token costs per user
- Prompt versioning for A/B testing
- Save raw responses for debugging
- Excellent prompt quality (invest heavily)

---

### Phase 4.2 - AI Review Frontend ⏳
**Estimated Time:** 3-5 hours

**Deliverables:**
- `apps/web/src/routes/(app)/review/+page.svelte` - Review hub
  - Recent reviews list
  - "Request Weekly Summary" button
  - "Request Monthly Summary" button
  - Unreviewed trades list

- `apps/web/src/lib/components/ai/review-request.svelte`
  - Step 1: Confirm trade details
  - Step 2: Add context (optional)
  - Step 3: Submit
  - Premium loading state with stages
    - "Reading chart..."
    - "Analyzing execution..."
    - "Evaluating risk management..."
    - "Generating insights..."

- `apps/web/src/lib/components/ai/review-display.svelte`
  - Overall score: large circular gauge (0-10)
  - Sub-scores: 3 smaller gauges (execution, risk, plan adherence)
  - Strengths (green, checkmarks)
  - Weaknesses (red/orange, warnings)
  - Key Lesson (highlighted card)
  - Actionable Fixes (numbered list)
  - Alternative Scenario ("What if?")
  - Chart Analysis (screenshot + AI text)
  - Emotional State (color-coded badge)
  - Thesis Alignment (1-5 score)
  - "Continue Conversation" button

- `apps/web/src/lib/components/ai/review-chat.svelte`
  - Chat interface
  - User messages right-aligned
  - AI messages left-aligned
  - Suggested follow-up questions as chips
  - Typing animation while AI responds
  - Messages persist (database)

- `apps/web/src/lib/components/dashboard/score-gauge.svelte`
  - SVG circular progress
  - Color interpolation (red → yellow → green)
  - Smooth animation

- `apps/web/src/lib/components/ai/ai-thinking.svelte`
  - Pulsing brain icon
  - Rotating status messages
  - Progress bar
  - "Usually takes 10-20 seconds"

- `apps/web/src/lib/components/ai/weekly-summary-display.svelte`
  - Narrative format
  - Sections: Overview, Best/Worst Trade, Patterns, Action Plan
  - Export to PDF button

- `apps/web/src/lib/services/ai.ts` - Frontend service
- `apps/web/src/lib/stores/ai-review.svelte.ts` - State

**Critical Requirements:**
- Premium loading experience
- Smooth score gauge animation
- Optimistic UI for chat
- Scannable review (key takeaway in 2 seconds)
- Mobile-optimized
- Smart suggested questions
- Rate limit feedback (friendly message)

---

## Implementation Order

### Recommended Sequence

**Phase 2 - Analytics Engine:**
1. Phase 2.1 - Rust computation engine (foundation)
2. Phase 2.2 - Dashboard frontend (core metrics + equity curve)
3. Phase 2.3 - Additional charts (strategies, time, instruments)
4. Phase 2.4 - Advanced analytics (Monte Carlo, etc.)

**Phase 3 - Daily Planning:**
1. Phase 3.1 - Complete planning system (API + frontend)

**Phase 4 - AI Review:**
1. Phase 4.1 - Claude API integration (backend)
2. Phase 4.2 - Review frontend (request, display, chat)

---

## Key Features Implemented

### Phase 2 - Analytics
- ✅ 40+ performance metrics
- ✅ Equity curve with drawdown
- ✅ Calendar P&L heatmap
- ✅ R-multiple distribution
- ✅ Time-of-day analysis
- ✅ Day-of-week analysis
- ✅ Strategy breakdown
- ✅ Instrument matrix
- ✅ Session split
- ✅ Conviction correlation
- ✅ Duration optimization
- ✅ Streak analysis
- ✅ Monte Carlo simulation
- ✅ Benchmark comparison
- ✅ Volatility-adjusted P&L
- ✅ Overnight analysis
- ✅ Commission tracking
- ✅ Fatigue curve

### Phase 3 - Planning
- ✅ Daily plan editor
- ✅ Market bias selection
- ✅ Session goals
- ✅ Pre-trade checklist
- ✅ Watchlist builder
- ✅ Key levels tracking
- ✅ Economic calendar
- ✅ Plan adherence scoring
- ✅ Carry-forward functionality

### Phase 4 - AI Review
- ✅ Trade review with screenshot analysis
- ✅ Multi-dimensional scoring
- ✅ Strengths/weaknesses identification
- ✅ Actionable fixes
- ✅ Alternative scenarios
- ✅ Follow-up chat
- ✅ AI personality adaptation
- ✅ Daily plan generation
- ✅ Weekly summaries
- ✅ Monthly summaries
- ✅ Token tracking
- ✅ Cost management

---

## Technical Specifications

### Backend (Rust)
- Decimal for all financial calculations
- Efficient SQL aggregation
- Response caching (1 hour TTL)
- Cache invalidation on data changes
- Claude API integration
- Screenshot processing (resize, compress)
- Rate limiting (server-side)
- Token cost tracking
- Prompt versioning

### Frontend (SvelteKit)
- Apache ECharts for all charts
- Lazy-loaded (dynamic import)
- Centralized theme
- Responsive design
- Interactive tooltips
- Zoom/pan on charts
- URL state sync
- Optimistic UI updates
- Premium loading states

### Performance Targets
- Analytics computation: < 2s for 10,000+ trades
- Dashboard load: < 500ms
- Plan page load: < 300ms
- AI review: 10-20 seconds
- Chart resize: instant
- Filter updates: < 500ms

---

## Testing Checklist

### Phase 2 - Analytics
- [ ] Core metrics calculate correctly
- [ ] Equity curve shows drawdowns
- [ ] Heatmap colors trades correctly
- [ ] All charts render on mobile
- [ ] Filters work across all views
- [ ] Charts resize on window change
- [ ] Empty states show correctly
- [ ] Monte Carlo runs in < 3s
- [ ] Cache invalidates on trade changes

### Phase 3 - Planning
- [ ] Can create daily plan
- [ ] Auto-save works
- [ ] Watchlist reordering works
- [ ] Checklist items toggle
- [ ] Carry-forward copies correct items
- [ ] Adherence score calculates
- [ ] Economic calendar displays
- [ ] Mobile layout works

### Phase 4 - AI Review
- [ ] Can request trade review
- [ ] Screenshots upload and analyze
- [ ] Review displays all sections
- [ ] Score gauges animate
- [ ] Chat works bidirectionally
- [ ] Suggested questions work
- [ ] Rate limits enforced
- [ ] Token costs tracked
- [ ] Weekly summary generates
- [ ] Monthly summary generates

---

## Estimated Total Time

**Phases 2-4 Total:** 31-43 hours

- Phase 2: 18-24 hours
  - 2.1: 6-8 hours
  - 2.2: 4-6 hours
  - 2.3: 4-5 hours
  - 2.4: 4-5 hours

- Phase 3: 5-7 hours
  - 3.1: 5-7 hours

- Phase 4: 8-12 hours
  - 4.1: 5-7 hours
  - 4.2: 3-5 hours

**Combined with Phase 1:** 54-74 hours total for Phases 1-4

---

## Dependencies

### Phase 2 Prerequisites
- Phase 1 complete (trades in database)
- Phase 0.2 complete (design system, ECharts theme)

### Phase 3 Prerequisites
- Phase 0.5 complete (database migrations for plans)
- Phase 1 complete (trades for plan-to-trade linkage)

### Phase 4 Prerequisites
- Phase 0.8 complete (Rust API scaffold)
- Phase 1 complete (trades to review)
- Phase 3 helpful (plan context for reviews)
- Anthropic API key configured

---

## Next Steps

After Phases 1-4 completion:
- **Phase 5:** Risk Management Suite
- **Phase 6:** Psychology & Discipline Tools
- **Phase 7:** Setup Playbook & Trade Grading
- **Phase 8:** Advanced Review & Replay

---

**Phases 2-4 Status:** Ready to Implement  
**Prerequisites:** Phase 0 complete, Phase 1 for full functionality  
**Architecture:** Full Rust Backend, NO Supabase  
**AI Provider:** Anthropic Claude (Sonnet 4)
