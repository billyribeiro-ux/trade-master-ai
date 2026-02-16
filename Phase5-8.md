# TradeMaster AI — Phase 5 through Phase 8 Prompts
# Apple Principal Engineer ICT Level 7+ Standards
# FULL RUST BACKEND

---

## ============================================================
## PHASE 5 — RISK MANAGEMENT SUITE
## ============================================================
## Features covered: 6.1 through 6.7 from the Features Spec
## ============================================================

### PROMPT 5.1 — Risk Management (Rust API + Frontend)

```
[PASTE GLOBAL CONTEXT BLOCK + STACK OVERRIDE]

PHASE 5 — RISK MANAGEMENT SUITE
PROMPT 5.1 — Complete Risk Management System (API + Frontend)

OBJECTIVE: Build the entire risk management module — position sizing calculators (5 methods), portfolio risk dashboard, scenario stress testing, position sizing consistency tracking, weekly "what if" simulator, and broker fee comparison. All computation in Rust, all UI in Svelte 5.

DEPENDENCIES: Phase 1 (trades), Phase 2 (analytics engine)

DELIVERABLES:

1. apps/api/src/services/risk_engine.rs — Risk computation service:

   POSITION SIZING — calculate_position_size():
   Input: PositionSizeRequest { method, account_balance, entry_price, stop_loss, risk_pct?, atr?, win_rate?, avg_payoff?, custom_formula? }
   Methods:
   - PercentageRisk: size = (account * risk_pct) / (entry - stop) — most common
   - AtrVolatility: size = (account * risk_pct) / (atr * atr_multiplier)
   - KellyCriterion: kelly_pct = win_rate - ((1 - win_rate) / avg_payoff), then size = (account * kelly_pct) / (entry - stop). Cap at 2x kelly (half-kelly as default).
   - FixedFractional: size = fixed_dollar_amount / (entry - stop)
   - Custom: evaluate user-provided formula (simple expression parser, NOT arbitrary code execution)
   Output: PositionSizeResult { shares_contracts, dollar_risk, risk_percentage, position_value, notional_exposure }
   ALL math in rust_decimal.

   PORTFOLIO HEAT — compute_portfolio_heat():
   Input: user_id
   Queries all open trades (status = 'open')
   Output: PortfolioHeat {
     total_open_positions, total_dollar_risk, total_risk_pct,
     positions: Vec<{symbol, direction, size, current_risk, risk_pct}>,
     correlation_warnings: Vec<String> — flag correlated positions,
     max_single_position_risk, max_sector_concentration
   }

   SCENARIO STRESS TEST — run_stress_test():
   Input: user_id, scenario { gap_pct?, vix_spike?, all_stops_hit? }
   Uses open positions to simulate:
   - "What if market gaps X%?" → compute P&L impact on all positions
   - "What if all stops hit?" → total loss
   - "What if VIX doubles?" → estimated impact based on position types
   Output: StressTestResult { scenario_description, total_impact_dollars, total_impact_pct, per_position_impact: Vec<...> }

   SIZING CONSISTENCY — compute_sizing_consistency():
   Input: user_id, date_range
   Queries all trades with actual_risk_pct
   Output: SizingConsistency {
     intended_risk_pct (from profile), actual_mean, actual_std_dev, actual_min, actual_max,
     consistency_score (0-100: lower std_dev = higher score),
     histogram: Vec<{bucket, count}> — distribution of actual risk percentages
   }

   WHAT IF SIMULATOR — run_what_if():
   Input: user_id, date_range, scenarios: Vec<WhatIfScenario>
   Scenarios:
   - "Honor all stops": recalculate P&L as if stop was always hit when triggered
   - "Exit at XR": recalculate if all trades exited at specified R-multiple
   - "Skip low conviction": remove trades below conviction threshold, recalculate stats
   - "Skip specific tag": remove trades with a given tag
   Output per scenario: WhatIfResult { scenario, original_pnl, modified_pnl, delta, modified_win_rate, modified_profit_factor }

   COMMISSION COMPARISON — compute_fee_comparison():
   Input: user_id, date_range
   Output: FeeComparison {
     current_total_commissions, gross_pnl, net_pnl, fees_as_pct_of_gross,
     avg_fee_per_trade, estimated_alt_broker_fees: Vec<{broker_name, estimated_fee}>
   }
   (Alternative broker fee estimates use hardcoded fee schedules for common brokers)

2. apps/api/src/routes/risk.rs — Risk endpoints:
   POST /api/v1/risk/position-size — Calculate position size
   GET /api/v1/risk/portfolio-heat — Current portfolio risk
   POST /api/v1/risk/stress-test — Run stress test scenario
   GET /api/v1/risk/sizing-consistency — Sizing consistency analysis
   POST /api/v1/risk/what-if — Run what-if simulation
   GET /api/v1/risk/fee-comparison — Commission comparison

3. apps/api/src/models/risk.rs — All request/response types

4. packages/types/src/risk.ts — TypeScript types matching Rust models

5. apps/web/src/routes/(app)/risk/+page.svelte — Risk management page:
   - Page title: "Risk Management"
   - Tabs: Calculator | Portfolio | What If | Fees

6. apps/web/src/lib/components/risk/position-calculator.svelte — Position sizing calculator:
   - Method selector (5 pills/tabs for each method)
   - Input fields change based on method:
     - Percentage Risk: account balance, entry price, stop loss, risk %
     - ATR Volatility: same + ATR value, ATR multiplier
     - Kelly: same + win rate, avg payoff ratio
     - Fixed Fractional: same + fixed dollar amount
   - "Calculate" button (or auto-calculate on input change)
   - Results display:
     - Position size (shares/contracts) — LARGE, primary output
     - Dollar risk
     - Risk percentage of account
     - Position value
   - Direction toggle (long/short) affects calculation
   - Save last-used settings
   - Quick-fill from user profile defaults (risk %, account balance)

7. apps/web/src/lib/components/risk/portfolio-heat.svelte — Portfolio heat dashboard:
   - Total risk gauge (circular, like the AI score gauge)
   - Open positions list: symbol, direction, size, risk $, risk %
   - Correlation warnings as alert banners
   - Sector concentration pie chart (ECharts)
   - "Run Stress Test" button opens scenario dialog

8. apps/web/src/lib/components/risk/stress-test.svelte — Stress test dialog:
   - Scenario inputs: gap percentage slider (-10% to +10%), toggle "All stops hit"
   - Results: total impact with per-position breakdown
   - Visual: bar chart showing each position's impact

9. apps/web/src/lib/components/risk/what-if-simulator.svelte — What-If:
   - Scenario selector (checkboxes): Honor all stops, Exit at 2R, Skip low conviction, Skip specific tag
   - Configurable parameters per scenario
   - Results table: original vs modified metrics side by side
   - Delta column showing improvement/decline with green/red coloring
   - "Your biggest leak:" highlight showing the scenario with largest positive delta

10. apps/web/src/lib/components/risk/sizing-consistency.svelte — Consistency view:
    - Score gauge (0-100)
    - Histogram of actual risk percentages (ECharts bar chart)
    - Intended vs actual comparison
    - Stats: mean, std dev, min, max
    - Insight text: "Your sizing varies from X% to Y%. Tighter consistency would improve edge."

11. apps/web/src/lib/components/risk/fee-comparison.svelte — Fee analysis:
    - Pie chart: gross P&L split (net + commissions + slippage)
    - Running commission total line chart
    - Alternative broker comparison table (if applicable)
    - Key stat: "Fees consumed X% of your gross profits"

12. apps/web/src/lib/services/risk.ts — Frontend risk service
13. apps/web/src/lib/stores/risk.svelte.ts — Risk state management

CRITICAL REQUIREMENTS:
- Position sizing math must be EXACT — this directly affects trader's money. Use decimal arithmetic on the frontend too (big.js or similar), or rely entirely on the Rust backend for calculations.
- The calculator must be usable during live trading — fast, clear, minimal clicks
- Kelly criterion must cap at a reasonable maximum (e.g., 25% of account) — full Kelly is too aggressive
- Stress test scenarios must clearly label assumptions
- What-if results must be clearly marked as hypothetical — "This analysis is based on modified trade data and does not represent actual results"
- Portfolio heat must auto-refresh when trades are created/closed (or show refresh button)
- Mobile: calculator inputs must use numeric keyboard, results must be large and readable at a glance

OUTPUT: Complete file contents for every file.
```

---

## ============================================================
## PHASE 6 — PSYCHOLOGY & DISCIPLINE TOOLS
## ============================================================
## Features covered: 7.1 through 7.13 from the Features Spec
## ============================================================

### PROMPT 6.1 — Mood, Tilt Detection, Alert Rules (Rust API)

```
[PASTE GLOBAL CONTEXT BLOCK + STACK OVERRIDE]

PHASE 6 — PSYCHOLOGY & DISCIPLINE TOOLS
PROMPT 6.1 — Psychology Engine (Rust API)

OBJECTIVE: Build the Rust backend for all psychology features — mood logging, tilt detection algorithm, custom alert rules engine, drawdown recovery mode, edge score computation, and streak tracking.

DEPENDENCIES: Phase 1 (trades), Phase 2 (analytics)

DELIVERABLES:

1. apps/api/src/services/tilt_detector.rs — Tilt detection engine:

   check_for_tilt(user_id) — Called after every trade creation:
   
   Detection rules (each independently evaluated):
   
   RAPID_TRADES: If 3+ trades entered within 15 minutes AND at least 1 of the previous 3 was a loser → trigger warning. If 5+ in 15 min → trigger alert. If 7+ in 15 min → trigger critical.
   
   SIZE_ESCALATION: If latest trade's position_size is >50% larger than the user's average size over last 20 trades AND the previous trade was a loser → trigger warning. If >100% larger → trigger alert.
   
   LOSS_CHASING: If 3+ consecutive losses AND the trader enters a new trade within 2 minutes of the last loss → trigger alert.
   
   DAILY_LIMIT: If cumulative daily P&L exceeds the user's max_daily_loss setting → trigger critical.
   
   WATCHLIST_DEVIATION: If trade symbol is NOT on today's watchlist AND today has an active plan → trigger warning.
   
   For each triggered detection:
   - Create tilt_events record with severity, trigger_type, trade_ids involved
   - Return TiltAlert { severity, trigger_type, message, suggested_action }
   
   Messages are human-readable: "Tilt detected: You've entered 4 trades in the last 15 minutes after a losing trade. Historically, trades taken in this state have a 23% win rate. Consider stepping away for 30 minutes."
   
   Historical context: query the user's past trades in similar tilt states to provide the actual win rate statistic.

2. apps/api/src/services/edge_score.rs — Personal Edge Score computation:

   compute_edge_score(user_id, date) — Daily composite score (0-100):
   
   Components (each 0-100, weighted):
   - Plan adherence (20%): from daily_plans.adherence_score, default 50 if no plan
   - Grade distribution (20%): % of A+B trades out of total. 100% A/B = 100 score, 0% = 0 score
   - Risk management (20%): sizing consistency score from risk engine
   - Journal quality (15%): % of trades with notes + screenshots + thesis. All three = 100, none = 0
   - Emotional stability (15%): inverse of tilt event count this week. 0 tilts = 100, 5+ tilts = 0
   - Normalized P&L (10%): P&L divided by VIX (vol-adjusted), normalized to 0-100 range based on user's historical distribution
   
   composite = sum of (component * weight)
   
   Save to edge_score_history with all component values and AI explanation.
   AI explanation generated via Claude: brief 2-sentence summary of what's driving the score up/down.

3. apps/api/src/services/streak_tracker.rs — Streak management:

   update_streaks(user_id) — Called daily or on relevant actions:
   
   Streak types:
   - journal: consecutive days with at least 1 trade that has notes + screenshots
   - planning: consecutive days with a completed daily plan
   - plan_adherence: consecutive days with adherence score > 80
   - review: consecutive days with at least 1 AI review requested
   
   Quality-weighted decay: missing a day reduces decay_value by 20% instead of resetting to 0.
   current_count: actual consecutive days
   decay_value: quality-weighted value (can be fractional)
   best_count: all-time maximum streak
   
   Update user_streaks table.

4. apps/api/src/services/drawdown_monitor.rs — Drawdown recovery mode:

   check_drawdown(user_id) — Called after each trade:
   - Compute current equity vs peak equity (from equity curve)
   - If drawdown exceeds user-configured threshold (default 10%):
     - Return DrawdownAlert with:
       - current_drawdown_pct
       - peak_equity, current_equity
       - suggested_actions: ["Reduce position size to 50%", "Only take A-grade setups", "Set a tighter daily loss limit"]
       - recovery_milestones: [break even today, break even this week, new equity high]

5. apps/api/src/routes/psychology.rs — Psychology endpoints:

   Mood:
   GET /api/v1/mood/:date — Get mood log for date
   POST /api/v1/mood — Create/update mood log
   GET /api/v1/mood?from=&to= — List mood logs in range

   Goals:
   GET /api/v1/goals — List goals (optional ?status= filter)
   POST /api/v1/goals — Create goal
   PUT /api/v1/goals/:id — Update goal (progress, status)
   DELETE /api/v1/goals/:id — Delete goal

   Tilt:
   GET /api/v1/tilt-events?from=&to= — List tilt events
   POST /api/v1/tilt-events/:id/acknowledge — Acknowledge tilt event

   Alert Rules:
   GET /api/v1/alert-rules — List user's alert rules
   POST /api/v1/alert-rules — Create rule
   PUT /api/v1/alert-rules/:id — Update rule
   DELETE /api/v1/alert-rules/:id — Delete rule
   PUT /api/v1/alert-rules/:id/toggle — Enable/disable rule

   Edge Score:
   GET /api/v1/edge-score/current — Today's score
   GET /api/v1/edge-score/history?from=&to= — Score history

   Streaks:
   GET /api/v1/streaks — All streak data for user

   Drawdown:
   GET /api/v1/drawdown/status — Current drawdown status

   Custom alert rule evaluation:
   POST /api/v1/alert-rules/evaluate — Evaluate all active rules against current state (called after each trade)

6. apps/api/src/models/psychology.rs — All models:
   MoodLog, MoodLogCreate, TradingGoal, GoalCreate, GoalUpdate
   TiltEvent, TiltAlert, AlertRule, AlertRuleCreate
   EdgeScore, EdgeScoreHistory, StreakData, DrawdownStatus

7. packages/types/src/psychology.ts — TypeScript types matching Rust

CRITICAL REQUIREMENTS:
- Tilt detection must run FAST (< 100ms) — it's called on every trade creation
- The tilt detection historical win rate stat must be accurate — it queries actual past trades in similar conditions
- Edge score computation can be slower (async, cached daily)
- Alert rules must support the conditions defined in the features spec: daily loss, trade count, consecutive losses, time windows, symbol restrictions
- Drawdown monitoring must handle the case where the trader has no historical trades (new account)
- All streak tracking must handle timezone correctly — a "day" is based on the user's timezone, not UTC

OUTPUT: Complete file contents for every file.
```

---

### PROMPT 6.2 — Psychology Frontend

```
[PASTE GLOBAL CONTEXT BLOCK + STACK OVERRIDE]

PHASE 6 — PSYCHOLOGY & DISCIPLINE TOOLS
PROMPT 6.2 — Psychology & Discipline Frontend

OBJECTIVE: Build all psychology-related UI — mood journal, tilt alerts display, custom alert rules builder, goal tracking, edge score display, streak tracking, loss autopsy, drawdown recovery mode, trade DNA fingerprint, confidence calibration, and gamification badges.

DEPENDENCIES: Prompt 6.1 (psychology API)

DELIVERABLES:

1. apps/web/src/routes/(app)/psychology/+page.svelte — Psychology hub page:
   - Page title: "Psychology & Discipline"
   - Top section: Edge Score gauge (large) + streak counters
   - Tabs: Mood | Goals | Alerts | Edge Score | Trade DNA
   - Tilt alert banner (appears at top when tilt is detected — dismissible)
   - Drawdown recovery banner (appears when in drawdown recovery mode)

2. apps/web/src/lib/components/psychology/edge-score-display.svelte:
   - Large circular gauge (0-100) with the composite score
   - Color: red (0-30), orange (30-50), yellow (50-70), green-yellow (70-85), green (85-100)
   - Component breakdown below as smaller progress bars:
     - Plan Adherence: X/100
     - Trade Grades: X/100
     - Risk Management: X/100
     - Journal Quality: X/100
     - Emotional Stability: X/100
     - Normalized P&L: X/100
   - AI explanation text below
   - Trend sparkline: last 30 days of edge scores
   - Click to navigate to edge score history

3. apps/web/src/lib/components/psychology/mood-journal.svelte:
   - Date selector (today default)
   - Pre-market mood slider (1-10) with emoji indicators at key points
   - Post-market mood slider (1-10)
   - Stress level slider (1-10)
   - Confidence level slider (1-10)
   - Sleep quality slider (1-10)
   - Emotion tags (multi-select pills): Focused, Anxious, Overconfident, Calm, Frustrated, Excited, Bored, Fearful, Disciplined
   - Notes textarea
   - Auto-save on change
   - Historical view: mood trend chart (line chart of mood over time, overlaid with equity curve for correlation)

4. apps/web/src/lib/components/psychology/tilt-alert-banner.svelte:
   - Full-width banner at top of dashboard when tilt is detected
   - Severity color: warning (yellow border), alert (orange), critical (red)
   - Icon + message + suggested action
   - "Acknowledge" button (dismisses and records acknowledgment)
   - "View Details" expands to show: trigger type, trades that triggered it, historical win rate in this state
   - Optional cooldown timer: if user configured a cooldown rule, show countdown

5. apps/web/src/lib/components/psychology/revenge-trade-cooldown.svelte:
   - Full-screen overlay that appears when revenge trade is detected (opt-in via alert rules)
   - Countdown timer (30s-5min configurable)
   - Breathing exercise animation (expanding/contracting circle)
   - Display user's custom rule reminder text
   - "I've calmed down, let me trade" button (only enabled after timer expires)
   - "Step away" button (closes the form/quick-log)

6. apps/web/src/lib/components/psychology/goal-tracker.svelte:
   - List of active goals, each as a card:
     - Title + description
     - Progress bar (current_value / target_value)
     - Target date with days remaining
     - Status badge (Active/Achieved/Abandoned)
     - Edit button
   - "New Goal" button opens creation dialog:
     - Goal type selector: Win Rate, Profit Factor, Consistency, Risk Mgmt, Journal, Custom
     - Title, description, target value, target date
   - Achieved goals section (collapsed by default, celebratory styling)
   - AI-suggested goals based on current weaknesses (stub — populated if AI data available)

7. apps/web/src/lib/components/psychology/alert-rules-builder.svelte:
   - List of current rules, each showing: name, condition summary, action, enabled toggle, times triggered
   - "Create Rule" button opens builder:
     - Rule name (text)
     - Condition builder (visual, not code):
       - Trigger type dropdown: Daily Loss, Trade Count, Consecutive Losses, Time Since Last Trade, Symbol Not on Watchlist
       - Operator: exceeds, falls below, equals, within
       - Value: number input
       - Time window: minutes/hours (where applicable)
     - Action: Notify, Cooldown (with duration), Lockout (with duration), Require Journal Entry
     - Custom message (shown when triggered)
   - Enable/disable toggle per rule
   - Delete rule (with confirmation)

8. apps/web/src/lib/components/psychology/loss-autopsy.svelte:
   - Shows only losing trades
   - Each loss categorized: "Good Loss" (green badge — followed plan) or "Bad Loss" (red badge — broke rules)
   - Categorization: auto-suggested based on plan adherence + trade grade, user can override
   - Summary stats: good loss ratio (%), trend over time
   - Chart: good vs bad loss ratio over weeks/months (ECharts stacked bar)
   - Insight: "Your good loss ratio is X%, up from Y% last month"

9. apps/web/src/lib/components/psychology/streaks-display.svelte:
   - Streak cards for each type: Journaling, Planning, Plan Adherence, Review
   - Each card: current streak (fire icon + number), best streak, decay value bar
   - Quality indicator (not just binary — journal quality matters)
   - Calendar view showing streak days highlighted

10. apps/web/src/lib/components/psychology/trade-dna.svelte — Trade DNA fingerprint:
    - Radar chart (ECharts) with axes:
      - Hold Duration (short vs long)
      - Risk Taking (conservative vs aggressive)
      - Win Rate
      - Conviction Accuracy
      - Plan Adherence
      - Emotional Stability
      - Setup Selectivity (% of A/B grade trades)
      - Time Discipline (performance consistency by hour)
    - Each axis normalized 0-100
    - Overlay previous month's DNA for comparison
    - Text summary: "Your trading DNA shows a disciplined, conservative style with strong plan adherence but room for improvement in hold duration."

11. apps/web/src/lib/components/psychology/confidence-calibration.svelte:
    - Chart: X = conviction level (1-5), Y-axis dual: expected win rate (what they think) vs actual win rate
    - Calibration score: overall accuracy of self-assessment
    - Insight: "You're well-calibrated at high conviction but overconfident at mid-range"
    - Uses conviction_analysis data from analytics engine

12. apps/web/src/lib/components/psychology/gamification-badges.svelte:
    - Grid of badge icons (locked/unlocked state)
    - Badges defined in a config:
      - "First Review" — requested first AI review
      - "Week Warrior" — 7-day planning streak
      - "Century Club" — 100 trades logged
      - "Plan Master" — 30-day plan adherence above 80%
      - "Clean Trader" — Good loss ratio above 80% for a month
      - "Self-Aware" — 30-day mood journaling streak
      - "Risk Manager" — Sizing consistency score above 90 for a month
      - More as defined in features spec
    - Unlocked badges: full color with earned date
    - Locked badges: grayed out with progress indicator ("47/100 trades")
    - New badge unlocked: toast notification with animation

13. apps/web/src/lib/components/psychology/drawdown-recovery-banner.svelte:
    - Persistent banner when in drawdown recovery mode
    - Shows: current drawdown %, peak equity, current equity
    - Recovery milestones with progress: break even today → break even this week → new equity high
    - Suggested actions: "Reduce size to 50%", "A-grade setups only", "Tighter daily loss limit"
    - Can be dismissed but reappears next session

14. apps/web/src/lib/services/psychology.ts — Frontend service for all psychology endpoints
15. apps/web/src/lib/stores/psychology.svelte.ts — Psychology state management

CRITICAL REQUIREMENTS:
- Tilt alerts must be VISIBLE and UNMISSABLE — they appear at the top of every page, not buried in a tab
- The mood journal must be quick to fill out — sliders, not text inputs for numeric values
- Gamification must reward PROCESS, never P&L — no badges for "made $10K" 
- The revenge trade cooldown must be respectful — it's opt-in, the trader configured it themselves
- Edge score must update daily and show trends — a single number means nothing without context
- Trade DNA radar chart must handle missing data (new accounts with few trades)
- All psychology components must use warm, supportive language — this is the most emotionally sensitive part of the app
- Mobile: all sliders must be thumb-friendly, all badges must be tappable for details

OUTPUT: Complete file contents for every file.
```

---

## ============================================================
## PHASE 7 — SETUP PLAYBOOK & TRADE GRADING
## ============================================================
## Features covered: 8.1 through 8.6 from the Features Spec
## ============================================================

### PROMPT 7.1 — Playbook & Grading System (Rust API + Frontend)

```
[PASTE GLOBAL CONTEXT BLOCK + STACK OVERRIDE]

PHASE 7 — SETUP PLAYBOOK & TRADE GRADING
PROMPT 7.1 — Complete Playbook & Grading System

OBJECTIVE: Build the setup playbook builder, trade grading rubric system, setup matching on trade entry, exit quality scoring, and importable/shareable rulesets.

DEPENDENCIES: Phase 1 (trades), Phase 2 (analytics for setup performance tracking)

DELIVERABLES:

1. apps/api/src/routes/playbook.rs — Playbook endpoints:
   GET /api/v1/playbook — List all setups
   GET /api/v1/playbook/:id — Get setup with performance stats
   POST /api/v1/playbook — Create setup
   PUT /api/v1/playbook/:id — Update setup
   DELETE /api/v1/playbook/:id — Archive setup (soft delete via is_active)
   POST /api/v1/playbook/:id/match — Score a trade against this setup's criteria
   
   Rubrics:
   GET /api/v1/rubrics — List rubrics
   POST /api/v1/rubrics — Create rubric
   PUT /api/v1/rubrics/:id — Update rubric
   POST /api/v1/rubrics/default/:id — Set as default rubric
   POST /api/v1/rubrics/:id/grade — Grade a trade using this rubric (body: criteria scores)
   
   Shared Rulesets:
   GET /api/v1/rulesets — List public rulesets
   GET /api/v1/rulesets/mine — List user's shared rulesets
   POST /api/v1/rulesets — Create/share a ruleset
   POST /api/v1/rulesets/:id/import — Import a shared ruleset into user's account

2. apps/api/src/services/playbook_engine.rs:
   
   match_trade_to_setup(trade, setup):
   - Compare trade data against setup's criteria
   - Each criterion has a weight and a met/not-met evaluation
   - Return: { criteria_met, criteria_total, score_pct, matched_criteria, unmatched_criteria }
   
   compute_setup_stats(setup_id, user_id):
   - Query all trades with this setup_name
   - Return: trade_count, win_rate, avg_r, profit_factor, total_pnl
   - Update the playbook_setups record with cached stats
   
   grade_trade(trade, rubric, criterion_scores):
   - Apply weights to each criterion score
   - Compute composite grade (A/B/C/D based on thresholds: A>=85, B>=70, C>=55, D<55)
   - Return: grade, numeric_score, per_criterion_breakdown
   
   compute_exit_quality(trade):
   - Compare actual exit to MFE (max favorable excursion if available)
   - exit_quality = (actual_profit / mfe_profit) * 100
   - If MFE not available: compare exit to take_profit level
   - Return: exit_quality_pct, optimal_exit_price, delta

3. apps/api/src/models/playbook.rs — All models
4. packages/types/src/playbook.ts — TypeScript types

5. apps/web/src/routes/(app)/playbook/+page.svelte — Playbook page:
   - Page title: "Playbook"
   - Tabs: My Setups | Grading Rubric | Shared Rulesets

6. apps/web/src/lib/components/playbook/setup-list.svelte:
   - Grid of setup cards (responsive: 1-3 columns)
   - Each card: setup name, description preview, criteria count, performance stats (win rate, avg R, profit factor), trade count
   - Performance stats color-coded (green if profitable, red if not)
   - "Active" badge for active setups
   - Click to view/edit
   - "Create New Setup" card at end

7. apps/web/src/lib/components/playbook/setup-editor.svelte:
   - Name (text input)
   - Description (textarea)
   - Criteria builder:
     - Dynamic list of criteria
     - Each criterion: name (text), weight (slider 1-10), required toggle (boolean), description (text)
     - Add/remove criteria
     - Example criteria: "Volume above average", "Higher timeframe aligned", "Clean support/resistance", "Confirmation candle", "Risk/reward > 2:1"
   - Expected R-multiple range (two number inputs: min, max)
   - Minimum conviction (1-5 selector)
   - Preferred timeframe (select: 1min, 5min, 15min, 1H, 4H, Daily, Weekly)
   - Market regime filter (multi-select: trending up, trending down, ranging, volatile)
   - Example screenshots (upload zone)
   - Notes on common mistakes
   - "Save Setup" button

8. apps/web/src/lib/components/playbook/setup-performance.svelte:
   - Displayed on setup detail view
   - Stats: trade count, win rate, avg R, profit factor, total P&L, expectancy
   - Mini equity curve for trades using this setup
   - Win rate trend over time (rolling 20-trade window)
   - Decay detection alert if setup performance is declining

9. apps/web/src/lib/components/playbook/rubric-editor.svelte:
   - Rubric name
   - Dynamic criteria list:
     - Each criterion: name, weight (as percentage, all must sum to 100%), scale min-max (default 1-10), description
   - Grade thresholds (customizable): A >= 85, B >= 70, C >= 55, D < 55
   - Preview: "Score a sample trade" — enter criterion scores, see computed grade
   - "Set as Default" toggle
   - "Save Rubric" button

10. apps/web/src/lib/components/playbook/trade-grader.svelte:
    - Used in trade detail page and trade form
    - Shows the default rubric criteria as a list
    - Each criterion: name, description, slider (scale min to max)
    - As user scores each criterion, the composite grade updates live
    - Grade badge displayed prominently (A/B/C/D with color)
    - Numeric score shown below
    - "Save Grade" applies to the trade record

11. apps/web/src/lib/components/playbook/shared-rulesets.svelte:
    - Browse public rulesets: list with name, description, type, creator, import count
    - "Import" button on each: copies the ruleset into user's account
    - "Share My Ruleset" button: dialog to select what to share (checklist, rubric, or playbook setup) and set as public/private
    - Share link for private sharing

12. apps/web/src/lib/services/playbook.ts — Frontend service
13. apps/web/src/lib/stores/playbook.svelte.ts — Playbook state

14. Integration with trade form (update from Phase 1):
    - When user selects a setup_name from their playbook in the trade form:
      - Show setup criteria as a checklist
      - AI matches and scores adherence in real-time
      - "This matches 4/6 criteria — B-grade setup" displayed inline

CRITICAL REQUIREMENTS:
- Playbook setup performance stats must be accurate and update when trades are created/updated
- The rubric grading must compute in real-time as the user adjusts sliders — no submit-and-wait
- Weights in the rubric must sum to 100% — enforce this in the UI (auto-adjust or warn)
- Shared rulesets must never include sensitive data (no P&L, no account info)
- The setup editor must save screenshots to storage and reference them correctly
- Decay detection must alert when a setup's rolling win rate drops >15% from its historical average
- The playbook must feel like a personal trading manual — organized, visual, evolving

OUTPUT: Complete file contents for every file.
```

---

## ============================================================
## PHASE 8 — ADVANCED REVIEW & REPLAY
## ============================================================
## Features covered: 9.1 through 9.5 from the Features Spec
## ============================================================

### PROMPT 8.1 — Trade Replay, Screenshot Timeline, Weekend Review, Data Audit

```
[PASTE GLOBAL CONTEXT BLOCK + STACK OVERRIDE]

PHASE 8 — ADVANCED REVIEW & REPLAY
PROMPT 8.1 — Trade Replay, Decision Point Training, Screenshot Timeline, Weekend Review, Data Audit

OBJECTIVE: Build the advanced review and replay features — trade replay on historical charts, decision-point training mode, chronological screenshot timeline, structured weekend review ritual, and data integrity audit.

DEPENDENCIES: Phase 1 (trades with screenshots), Phase 4 (AI reviews), Lightweight Charts library from Phase 0

DELIVERABLES:

1. apps/web/src/routes/(app)/review/replay/[id]/+page.svelte — Trade replay page:
   - Loads trade data + chart data (candle data around the trade timeframe)
   - Note: Historical candle data is needed for replay. For now, this feature requires the user to have uploaded a screenshot. Future integration with market data feeds (TradingView, polygon.io) will enable full candle replay. Build the UI framework and use screenshot-based replay as the initial implementation.

2. apps/web/src/lib/components/review/trade-replay.svelte — Replay viewer:
   - If screenshot available: display screenshot with trade entry/exit overlaid as annotated markers
   - Speed controls: 0.5x, 1x, 2x, 5x
   - Play/pause button
   - Scrub bar to jump to any point in the trade
   - Overlay: entry marker (green arrow), exit marker (red arrow), stop loss line, take profit line
   - Trade info panel alongside: P&L, R, duration, notes
   - "What would you do?" decision points (see below)
   - Future: when market data API is connected, render actual candles via Lightweight Charts with animated playback

3. apps/web/src/lib/components/review/decision-point.svelte — Fork-in-the-road training:
   - At key moments in the replay (entry, potential add, potential scale, exit):
     - Pause replay automatically
     - Show dialog: "What would you do here?"
     - Options: "Enter/Add", "Wait", "Exit/Trim", "Move Stop", "Do Nothing"
     - User selects their practice decision
     - Reveal what actually happened + AI's assessment of the optimal decision
     - Score the practice decision
   - After completing all decision points: summary score
   - Tracked over time: decision accuracy improving or declining

4. apps/web/src/routes/(app)/review/timeline/+page.svelte — Screenshot timeline:
   - Chronological gallery of ALL trade screenshots across all trades
   - Layout: vertical timeline with dates as section headers
   - Each screenshot: thumbnail, trade symbol, P&L, date
   - Click to open in lightbox with trade context
   - Filter by: date range, symbol, win/loss, grade
   - Infinite scroll for large collections
   - "Your trading visual history" — scroll through months of charts
   - Search by symbol or date

5. apps/web/src/lib/components/review/weekend-review.svelte — Weekend review ritual:
   - Guided 15-minute structured review
   - Fetches all data from the past week automatically
   - Step-by-step flow:
     Step 1: "Week at a Glance" — summary stats (trades, win rate, P&L, edge score trend)
     Step 2: "Best Trade" — auto-selected by R-multiple, displayed with review
     Step 3: "Worst Trade" — auto-selected, displayed with review
     Step 4: "Biggest Lesson" — AI-generated based on the week's data
     Step 5: "Plan Adherence" — adherence trend chart for the week
     Step 6: "Emotional Arc" — mood trend for the week overlaid with P&L
     Step 7: "Focus for Next Week" — AI recommendation
     Step 8: "Reflection" — free-text journal entry for the trader's thoughts
   - Progress indicator showing current step
   - "Export as PDF" button at the end
   - Saved as a weekly review record
   - Streak tracked: consecutive weeks with completed weekend review

6. apps/api/src/routes/review.rs — Review-specific endpoints:
   GET /api/v1/review/weekend-data?week_start= — Fetch all data needed for weekend review
   POST /api/v1/review/weekend-complete — Save weekend review completion + reflection notes
   GET /api/v1/review/timeline?from=&to=&symbol= — Paginated screenshot timeline data
   GET /api/v1/review/decision-history — Decision point training history and scores

7. apps/web/src/lib/components/review/data-integrity-audit.svelte — Monthly data audit:
   - Runs analysis on trade data quality
   - Shows: trades without screenshots (count), trades without stop loss (count), trades without notes (count), trades without tags (count), trades without grade (count)
   - Overall data quality score (0-100)
   - Per-category progress bars
   - "Fix Now" buttons linking to the first incomplete trade for each category
   - Tip: "Improving data quality to 90%+ makes your AI reviews significantly more accurate"
   - Displayed as a card on the dashboard (dismissible for the month)

8. apps/web/src/lib/components/review/exit-quality-display.svelte — Exit quality analysis:
   - Score display: "You capture X% of the available move on average"
   - Comparison: your exit vs MFE (or take profit if MFE unavailable)
   - Histogram: distribution of exit quality percentages
   - Trend over time: are you getting better at exits?
   - Insight text based on data
   - Displayed in trade detail page and analytics

9. Update dashboard to include:
   - Weekend review reminder (Saturday/Sunday: "Time for your weekly review")
   - Data integrity alert if quality score drops below 70%
   - Recent AI reviews quick-access list

10. apps/web/src/lib/services/review.ts — Frontend service for all review endpoints
11. apps/web/src/lib/stores/review.svelte.ts — Review state management

CRITICAL REQUIREMENTS:
- Trade replay must work with screenshots as the initial implementation — don't block on market data API
- Decision point training must save results for long-term tracking
- The weekend review must feel like a ritual, not a chore — guided, visual, and completable in 15 minutes
- Screenshot timeline must handle hundreds of images efficiently (virtual scrolling / lazy loading)
- Data integrity audit should be encouraging, not punishing — "You're at 71%, let's get to 80%!" not "Your data is incomplete"
- PDF export of weekend review must produce a clean, professional one-pager
- The exit quality metric must clearly explain what it measures and why it matters

OUTPUT: Complete file contents for every file.
```

---

## ============================================================
## REMAINING PHASES — SUMMARY & PROMPTS
## ============================================================

### PHASE 9 — BROKER INTEGRATION (2 prompts)

```
PROMPT 9.1 — Broker Integration Architecture:
- Abstract broker adapter trait in Rust (BrokerAdapter with methods: authenticate, sync_trades, get_positions, get_account_info)
- Concrete implementations for Thinkorswim, Interactive Brokers, TradingView webhook receiver
- Sync engine: polling, deduplication, conflict resolution, error handling
- Broker connection management API (encrypted credential storage)
- Market internals auto-capture service (TICK, ADD, VOLD, VIX at trade time)
- Frontend: broker connection settings page, sync status dashboard, auto-capture toggle

PROMPT 9.2 — Additional Brokers + Sync Dashboard:
- TradeStation, MetaTrader, Coinbase, Binance, Bybit adapters
- Broker fee comparison data (hardcoded fee schedules)
- Sync history and error log UI
- Manual sync trigger
- Connection health monitoring
```

### PHASE 10 — ACCOUNTABILITY & SOCIAL (1 prompt)

```
PROMPT 10.1 — Accountability System:
- Coach/mentor dashboard: read-only view with permission controls
- Invitation system (email invite, accept/decline, expiration)
- Permission toggles: plan, grades, tilt, P&L, mood, reviews
- Anonymous trade sharing: strip PII, show only R-multiples and chart
- Community leaderboard (opt-in): ranked by edge score, adherence, streaks — NOT P&L
- Frontend: sharing settings page, coach dashboard view, community feed
```

### PHASE 11 — MOBILE NATIVE & POLISH (2 prompts)

```
PROMPT 11.1 — PWA Enhancement + Capacitor Shell:
- Service worker for offline trade logging and plan viewing
- Offline queue: log trades offline, sync when reconnected
- Capacitor project setup (iOS + Android)
- Push notifications: tilt alerts, plan reminders, streak warnings
- Biometric auth integration (Face ID, fingerprint)
- Camera integration for direct screenshot capture

PROMPT 11.2 — Home Screen Widgets + Polish:
- iOS/Android home screen widgets: daily P&L, edge score, plan summary
- Quick-log from notification action
- App Store metadata and screenshots
- Performance optimization pass: lazy loading, code splitting, image optimization
- Accessibility audit and fixes
- Final responsive design polish across all breakpoints
```

### PHASE 12 — PERFORMANCE, SECURITY & SCALE (1 prompt)

```
PROMPT 12.1 — Production Hardening:
- Database optimization: query analysis, index tuning, materialized views for heavy analytics
- Connection pooling configuration for production load
- Rate limiting on all API endpoints (tower-governor or custom)
- End-to-end encryption for broker credentials (at-rest encryption with vault/KMS)
- GDPR compliance: full data export endpoint, account deletion endpoint
- Error monitoring integration (Sentry or equivalent)
- Structured logging for production (JSON format, log levels, request tracing)
- Load testing with k6 or similar
- Backup strategy: automated PostgreSQL backups, point-in-time recovery
- CI/CD hardening: staging environment, integration tests, deployment rollback strategy
- Docker Compose for complete local development environment (PostgreSQL, MinIO, Rust API, SvelteKit)
```

---

## TOTAL PROMPT COUNT

| Phase | Prompts | Focus |
|-------|---------|-------|
| 0 | 10 | Foundation & Infrastructure |
| 1 | 6 | Core Trade Logging |
| 2 | 4 | Analytics Engine |
| 3 | 1 | Daily Planning |
| 4 | 2 | AI Trade Review |
| 5 | 1 | Risk Management |
| 6 | 2 | Psychology & Discipline |
| 7 | 1 | Playbook & Grading |
| 8 | 1 | Advanced Review & Replay |
| 9 | 2 | Broker Integration |
| 10 | 1 | Accountability & Social |
| 11 | 2 | Mobile Native & Polish |
| 12 | 1 | Performance & Security |
| **TOTAL** | **34** | **Complete Application** |

Every feature from the Features Specification is covered. Execute in order. Skip nothing.