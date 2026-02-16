# TradeMaster AI — Complete Features Specification

## App Overview

TradeMaster AI is the ultimate all-in-one trading journal app that combines everything a serious trader needs: seamless trade logging, deep analytics, daily planning, AI-powered trade reviews (with chart screenshot analysis), risk tools, psychological tracking, and more. This app stands above current leaders like TradesViz, TraderSync, TradeZella, and Edgewonk by deeply integrating advanced AI for personalized, actionable insights — including direct AI review of your trades and charts.

The app is mobile-first (iOS/Android) with a full web version, featuring a clean dark-mode UI, intuitive navigation, and high performance. Core philosophy: minimize data entry friction, maximize actionable insights, and foster trading discipline through AI guidance.

The AI is not a bolt-on — it is the central nervous system of the entire app. Chart screenshot analysis combined with natural language review is a capability no other app delivers at this depth. The app covers the full trader lifecycle: Plan → Execute → Log → Review → Improve.

Target users: Day traders, swing traders, options traders, scalpers, crypto traders, futures traders — anyone serious about consistent profitability.

---

## 1. Onboarding & Initial Setup

### 1.1 Account Creation

Sign-up via email, Google/Apple OAuth, or broker OAuth for instant import testing. The registration process is clean, fast, and low-friction. No unnecessary fields — just what's needed to get started.

### 1.2 Quick Profile Quiz

A 3-5 minute guided quiz that calibrates the entire app experience from day one. The quiz covers trading style (day trading, swing trading, scalping, position trading), primary assets traded (stocks, options, forex, futures, crypto), experience level (beginner, intermediate, advanced), approximate account size range (used for setting default risk parameters), and personal trading goals (consistency over big wins, scaling to full-time, reducing losses, building discipline). The results directly configure default settings throughout the app — risk percentages, checklist templates, AI review tone, and analytics focus areas.

### 1.3 Broker/Exchange Connection

Integration with 100+ brokers and exchanges via API including Thinkorswim, Interactive Brokers, TradeStation, Binance, Coinbase, Bybit, MetaTrader 4/5, and TradingView webhooks. For brokers without API access, CSV upload with pre-built templates is available. Broker connection is entirely optional — the app is fully functional with manual trade entry alone. No feature is gated behind connecting a broker.

### 1.4 Initial AI Calibration

Users are encouraged (but not required) to upload 5-10 past trade screenshots during onboarding. The AI analyzes these and generates a "Baseline Health Report" — an initial estimate of win rate, commonly spotted mistakes, and personalized welcome tips. This gives the user an immediate taste of the AI's capabilities and makes the app feel like it already understands them from the first session.

### 1.5 Home Screen Tour

A quick interactive walkthrough or short video introducing the core loop: Plan → Log → Review → Improve. During the tour, users set their default risk percentage (e.g., 1%), preferred time zone, and active trading session times (NY open, London, Asia). The tour is skippable for experienced users.

### 1.6 AI Personality Selection

Users choose their preferred AI coaching personality: "Strict Coach" (blunt, no-nonsense feedback that calls out every mistake), "Encouraging Mentor" (supportive tone that highlights positives while gently addressing weaknesses), or "Balanced" (a mix of both). This setting affects the tone and framing of all AI-generated reviews, summaries, and feedback throughout the app. It can be changed at any time in settings.

---

## 2. Trade Logging & Entry

### 2.1 Manual Trade Entry

The primary and always-available method of logging trades. The entry form captures every field a serious trader needs: entry price, exit price, position size, direction (long/short), stop-loss level, take-profit level, commissions and fees, trade duration, and the instrument's asset class. Beyond the basics, the form includes customizable fields for setup type, market conditions, confluence factors, and any other metadata the trader wants to track. Auto-calculations run in real-time — R-multiple is computed from entry, exit, and stop-loss the moment those fields are filled. P&L, P&L percentage, and risk percentage are all calculated automatically. The form is designed for speed: tab through fields, smart defaults from user profile, and the ability to save partial entries and complete them later.

### 2.2 Quick-Log Widget

A floating button or notification shortcut available during live trading sessions that allows logging a trade in under 10 seconds. The quick-log captures only the essential fields (symbol, direction, entry price, exit price, size) with everything else auto-filled from defaults or left for later completion. On mobile, this is accessible from the home screen or notification shade without opening the full app. The goal is zero friction during active trading — log it fast, detail it later.

### 2.3 Screenshot & Media Upload

Unlimited uploads of chart screenshots, order confirmations, and short screen recordings attached to any trade. Drag-and-drop on desktop, camera capture on mobile. The AI automatically analyzes uploaded chart screenshots to identify patterns (double bottom with divergence, failed breakout on low volume, head and shoulders formation), extract key price levels (support, resistance, supply/demand zones), annotate visible trends, detect indicators (RSI divergence, moving average crossovers), and link all findings to the trade record. Screenshots are stored with timestamps and can be annotated with user drawings (trendlines, zones, notes) directly in the app. Each trade can have multiple screenshots pinned to a timeline showing the setup at different stages.

### 2.4 Intelligent Tagging System

A dual-layer tagging system combining AI-suggested tags with user-created custom tags. Tags are categorized: strategies (Breakout, Mean Reversion, Scalping, Swing Pullback, VWAP Bounce), mistakes (Revenge Trading, FOMO Entry, Overtrading, Moving Stop, Early Exit), emotions (Confident, Anxious, Frustrated, Calm, Overexcited), session types (Pre-Market, Regular Hours, Power Hour, After Hours), and market conditions (Trending, Ranging, Volatile, Low Volume, News-Driven). When logging a trade, the AI suggests relevant tags based on the trade's characteristics, uploaded charts, and the user's historical patterns — for example, "This looks like your 'FOMO Breakout' setup — correct?" Tags are color-coded, searchable, and filterable across every analytics view in the app.

### 2.5 Voice-to-Text & Quick Notes

Dictate post-trade thoughts via voice memo on any device. Voice is instantly converted to searchable text using speech recognition. The transcribed text is automatically analyzed for sentiment (frustrated, confident, uncertain, excited) and that sentiment is tagged to the trade. This allows traders to capture raw, unfiltered thoughts in the moment — something most traders won't do if it requires typing on a phone. Voice notes are stored alongside text notes and can be played back during review.

### 2.6 Trade Scaling / Partial Entry-Exit Logging

Most journals treat a trade as one entry and one exit. TradeMaster AI tracks each partial entry and exit (leg) separately within a single trade record. Every scale-in, scale-out, add, and trim is logged with its own price, size, timestamp, fees, and notes. The app then calculates the composite trade metrics (average entry, average exit, total P&L) while also providing leg-by-leg analysis. This is critical for traders who build into positions or scale out at multiple targets.

### 2.7 CSV Import

For traders migrating from other platforms or logging historical trades, a CSV import system with downloadable templates for common formats. The importer handles parsing, field mapping, validation, duplicate detection, and conflict resolution. Trades can be previewed before final import. Supported formats include generic CSV, broker-specific exports (Thinkorswim, IBKR, TradeStation), and exports from competing journal apps.

### 2.8 Auto-Import from Brokers

For connected brokers, trades are automatically synced via API. The sync engine handles polling intervals, deduplication (preventing the same trade from appearing twice), field mapping from broker-specific formats to the app's schema, and conflict resolution when a manually-entered trade overlaps with an auto-imported one. Sync status is visible in a dashboard showing last sync time, any errors, and pending items. Auto-import is a convenience layer — it never overrides manually-entered data without user confirmation.

### 2.9 Missed Trade Logging

A dedicated feature for logging trades the user did not take. The trader uploads a screenshot of the setup they saw, writes their thesis for why they would have entered, and records the outcome they missed. The AI tracks missed opportunities over time and surfaces patterns: "You've passed on 14 valid breakout setups this month. If taken at your historical average, estimated gain: +8.3R." This quantifies fear-based avoidance — the silent killer most traders never measure. Missed trades appear in analytics as a separate category, allowing traders to compare "trades taken" performance vs. "trades seen" opportunity.

### 2.10 Pre-Trade Thesis Capture

Before or at the time of entry, traders write a one-line thesis explaining why they're taking the trade. This is a free-text field that captures the reasoning: "Breakout above 195 resistance with volume confirmation on 15-min chart." After the trade closes, the AI compares the thesis to the actual outcome and scores alignment: "Thesis: breakout with volume. Reality: entered before confirmation, volume was below average, price reversed at resistance. Thesis alignment: 2/5." This forces traders to confront the gap between what they planned and what they actually did.

### 2.11 Conviction Scoring

At entry, the trader rates their conviction on a 1-5 scale. This simple input becomes one of the most powerful analytics dimensions in the app. Performance is tracked by conviction level, revealing patterns most traders never see: "Your conviction-4 and conviction-5 trades have a 72% win rate. Conviction 1-2 trades sit at 39%. Consider only taking conviction-3+ setups." Over time, the app also tracks conviction calibration — are you accurate when you say "high conviction"?

### 2.12 Context Auto-Capture

At the moment of each trade, the app automatically captures market context data: VIX level and change, SPY/QQQ price, bid-ask spread on the instrument, volume at entry, and the trading session type (pre-market, first 30 minutes, midday, power hour, after-hours). For connected brokers or data feeds, additional internals are captured: TICK, ADD, VOLD, sector performance, and market breadth. This context is stored with the trade and becomes available in analytics — allowing the trader to discover environmental factors that correlate with their performance.

### 2.13 Market Regime Tagging

Each trading day and each trade is classified by market regime: trending, ranging, volatile, or choppy. This classification can be set manually or auto-detected from VIX levels, ATR values, and breadth data. All analytics can then be filtered by regime, revealing critical insights: "Your breakout strategy has a 71% win rate in trending markets but only 38% in range-bound markets." Without this segmentation, aggregate statistics hide regime-dependent performance.

### 2.14 Sector Auto-Tagging

Every trade is automatically tagged with its sector based on the symbol. This enables sector-level P&L analysis overlaid with sector ETF performance. A trader might discover: "I traded tech stocks 80% of the time last month, but tech was the weakest sector. My energy trades were 4/4 winners. I'm fishing in the wrong pond." This is a simple data enrichment step that unlocks a powerful analytics dimension.

### 2.15 Session Type Auto-Detection

Based on the trade's entry timestamp and the user's configured timezone and session times, each trade is automatically tagged with its session type: pre-market, first 30 minutes, mid-morning, midday/lunch, afternoon, power hour, or after-hours. This enables session-specific performance analysis without requiring the trader to manually categorize every trade.

---

## 3. Advanced Analytics Dashboard

### 3.1 Core Performance Metrics

The analytics engine computes and displays all essential trading metrics in a clean, scannable dashboard: win rate, average R-multiple, profit factor, expectancy, maximum drawdown, Sharpe ratio, Sortino ratio, recovery factor, total P&L, average winner size, average loser size, largest winner, largest loser, average trade duration, and total number of trades. All metrics can be filtered by any time period (daily, weekly, monthly, quarterly, yearly, custom range), any tag combination, any asset class, any setup type, or any market regime. Metrics update in real-time as new trades are logged.

### 3.2 Equity Curve

An interactive, zoomable equity curve showing cumulative P&L over time. The curve includes drawdown shading (highlighting periods of declining equity), benchmark overlays (SPY, QQQ, BTC, or custom indices for comparison), and clickable data points that link directly to the trade detail for that day. Users can toggle between dollar P&L, percentage P&L, and R-multiple views. The curve responds to all active filters — filter by a specific strategy and the equity curve shows performance for that strategy alone.

### 3.3 Monthly/Weekly P&L Heatmap Calendar

A calendar-style heatmap where each day is color-coded by P&L (green gradient for profit, red gradient for loss, gray for no trades). Hovering or tapping a day shows a summary: number of trades, total P&L, win rate, and best/worst trade. The calendar view makes patterns immediately visible — streaks of green, clusters of red, days of the week that consistently underperform. Weekly and monthly roll-up rows show aggregate performance.

### 3.4 Trade Distribution Histogram

A histogram showing the distribution of all trade outcomes by R-multiple. This reveals whether the trader's results follow a healthy distribution (clustered slightly positive with a long right tail) or an unhealthy one (fat left tail indicating large losses). Overlaid with statistical markers: mean, median, standard deviation, and skewness. This is one of the most important visualizations for understanding edge quality.

### 3.5 Strategy Performance Breakdown

Filterable views showing performance segmented by any tag or tag combination. Pie charts, bar charts, and tables showing win rate, expectancy, profit factor, and trade count per strategy. The trader can compare "Breakouts in trending markets" vs. "Breakouts in ranging markets" with a few clicks. This is where the tagging system pays off — the richer the tags, the deeper the strategy analysis.

### 3.6 Time-of-Day Performance Heatmap

A heatmap showing P&L by hour of day across all trading days. Each cell represents a specific hour, color-coded by cumulative performance. This reveals time-based patterns: "I make most of my money between 9:30-10:30 AM and give it back between 12:00-1:00 PM." One of the simplest and most actionable analytics views — if a trader consistently loses during a specific hour, they can simply stop trading during that hour.

### 3.7 Day-of-Week Performance Analysis

Bar or heatmap showing aggregate performance by day of week. Reveals weekly rhythms: "Mondays and Fridays are my worst days. Tuesday through Thursday I'm net positive." Combined with time-of-day data, this pinpoints the exact windows where a trader has edge.

### 3.8 Instrument Performance Matrix

A color-coded grid showing every ticker the trader has traded, with cells colored by net profitability. Instantly visible: "I'm consistently profitable on SPY, NVDA, and TSLA but I bleed money every time I touch AMZN and crypto." Sortable by P&L, win rate, number of trades, or profit factor. Simple data, massive insight, rarely surfaced in competing journals.

### 3.9 Session Performance Split

Performance broken down by market session: pre-market, first 30 minutes, mid-morning, midday/lunch, afternoon, power hour, and after-hours. Each session shows its own equity curve, win rate, and average R. Most traders have one session where they consistently profit and another where they consistently give it back. This view makes that undeniable.

### 3.10 Trade Duration Optimization

A scatter plot or curve showing P&L as a function of hold time. Reveals the trader's optimal holding period: "Trades held under 5 minutes average -0.3R. Trades held 15-45 minutes average +1.1R. Trades held over 2 hours average +0.4R. Your sweet spot is 15-45 minutes." This lets the data determine the trader's optimal timeframe instead of guessing or following generic advice.

### 3.11 Trade Clustering Visualizer

A timeline view plotting all trades within each day, with each trade represented as a point or bar colored by outcome (green for winner, red for loser). This visualizes trading tempo and clusters. Patterns emerge immediately: "Every time I take 5+ trades before 10:30 AM, the day ends red." Overlay daily P&L to show how clustering correlates with overall performance.

### 3.12 Scale-In / Scale-Out Analytics

Dedicated analysis of whether partial entries and exits actually improve results. Compares average R-multiple and win rate for scaled trades vs. single-entry/exit trades. Breaks down the impact of each scaling behavior: "When you add to winners, your average R improves by 0.4. When you average down on losers, your average R drops by 1.2." Quantifies what traders argue about endlessly with their own data.

### 3.13 Overnight Risk Analysis

Isolates performance on trades held overnight. Tracks gap risk, gap direction vs. position direction, and the cumulative impact of overnight holds. "You've held overnight 34 times. 21 gapped in your favor, 13 against. But the 13 against cost more than the 21 in favor gained. Your overnight edge is negative." This is data most retail traders never look at.

### 3.14 Earnings & Catalyst P&L Isolation

Auto-tags trades that occurred during or around earnings, FOMC, CPI, NFP, or other major catalysts. Separates catalyst-driven P&L from "normal" trading P&L. Many traders believe they're good at earnings plays because they remember the big wins and forget the blowups. This view shows the isolated reality: "Your non-catalyst trading has a 2.1 profit factor. Your earnings plays have a 0.7. Stop playing earnings."

### 3.15 Conviction Score vs. Outcome Correlation

Charts the relationship between pre-trade conviction ratings and actual outcomes. Shows whether high-conviction trades genuinely outperform low-conviction trades and by how much. Also tracks conviction calibration: when the trader says "high conviction," how often are they right? Over time, this trains better self-assessment.

### 3.16 Commission & Slippage Cumulative Impact

Tracks actual fill price vs. expected price across every trade, showing cumulative slippage drag. Also tracks commissions as a percentage of gross P&L. Surfaces hidden costs: "Slippage cost you $2,847 this quarter. That's 1.2R of edge erased." And: "You paid $4,200 in commissions. Your gross P&L was $12,000, meaning 35% went to fees." Some traders are profitable before costs and don't realize costs are killing them.

### 3.17 Asymmetric Risk Alert

A prominent indicator showing whether the trader's average winner is larger than their average loser. This is one of the most fundamental metrics in trading, yet most journals bury it in a statistics table. TradeMaster AI surfaces it as a real-time alert: "Warning: Your average loser ($340) is 1.7x your average winner ($200). You're cutting winners short and letting losers run." Displayed prominently, not hidden in a data table.

### 3.18 Streak & Momentum Awareness

Statistical analysis of performance following win streaks and loss streaks. Not gamification — actionable data: "You're on a 7-win streak. Historically, your 8th trade after a streak has a 35% win rate because you get overconfident and size up." And conversely: "After 3 consecutive losses, your next trade wins 62% of the time — but only when you reduce size. When you maintain or increase size after 3 losses, your win rate drops to 28%."

### 3.19 Volatility-Adjusted P&L

Raw P&L normalized by market volatility (VIX or instrument ATR). A $500 day when the market barely moves is exceptional performance. A $500 day when VIX is at 40 is underperformance. This normalization reveals the trader's true skill curve independent of market conditions. The adjusted equity curve sits alongside the raw curve for comparison.

### 3.20 MFE/MAE Analysis (Max Favorable/Adverse Excursion)

For every trade, the app calculates how far price moved in the trader's favor (MFE) before they exited and how far it moved against them (MAE) before recovery or stop. This is institutional-grade analysis that retail journals completely skip. Visualization shows: "Your average MFE is 2.3R but you exit at 1.1R. You're leaving half your profits on the table." And: "Your average MAE is 0.8R but your stops are set at 1.5R. You could tighten stops without getting stopped out more often."

### 3.21 Multi-Timeframe Alignment Tracking

When logging trades, traders note which timeframes aligned at entry (daily, 4H, 1H, 15min, 5min). Analytics then track win rate by alignment count. Most traders discover their win rate jumps 20%+ when 3 or more timeframes agree. This proves confluence matters — not with theory, but with their own data.

### 3.22 Sector Rotation Awareness

Sector-level P&L overlaid with sector ETF performance over the same period. Shows whether the trader is swimming with or against the sector tide. Identifies missed opportunities: "You ignored healthcare completely last month, but XLV was up 8%. Your best-performing sector was energy, which also led the market." This encourages traders to align with sector momentum rather than fighting it.

### 3.23 Fatigue / Session Decay Tracking

Performance tracked by hour-into-active-session (not clock time, but time since the trader started trading that day). Most traders degrade after 2-3 hours of active trading but don't realize it. The equity curve per-hour-of-trading makes this visible: "Your P&L turns negative after hour 3 consistently. Consider setting a hard stop at 3 hours of active trading."

### 3.24 Correlation Heatmap Across Concurrent Trades

Analysis of what happens when the trader holds multiple positions simultaneously. Measures win rate and average outcome when trading correlated instruments at the same time vs. uncorrelated instruments vs. single positions. Reveals focus-splitting: "When you trade AAPL and MSFT simultaneously, your win rate drops 22% because you split focus."

### 3.25 Monte Carlo Simulation

Runs 10,000 randomized simulations of the trader's actual trade sequence to model potential future outcomes. Shows probability distributions of account growth, maximum drawdown risk, and blowup probability. Visualized as a fan chart with confidence intervals: "Based on your trade distribution, there's a 5% chance of a 30%+ drawdown over the next 100 trades." This is institutional-grade risk analysis made accessible.

### 3.26 Benchmark Comparison

Compare the trader's returns against major indices (S&P 500, Nasdaq, Bitcoin) or custom benchmarks over any time period. Shows whether the trader is actually adding value vs. simply being long a bull market. Includes risk-adjusted comparison using Sharpe ratio differentials.

### 3.27 First Trade of Day Analysis

Isolates performance specifically on the first trade of every session. This trade carries the most psychological weight — it sets the tone for the day. "Your first trade of the day wins 44% of the time, below your overall 57%. You're rushing entries in the first 15 minutes. Delaying your first trade to 10:00 AM would improve expected R by 0.6."

### 3.28 Risk-Adjusted Period Ranking

Ranks the trader's months (or weeks, quarters) not by raw P&L but by Sharpe ratio, profit factor, or Personal Edge Score. Reframes what "good" means: the best month might not be the one with the most profit — it might be the one with the most disciplined, consistent execution. Builds the mindset that process matters more than outcomes.

### 3.29 Seasonal Performance Patterns

After 6-12 months of data accumulation, the app surfaces monthly and seasonal patterns in the trader's own performance. "You've been profitable in January and March every year but negative in September. September historically has higher volatility and you tend to overtrade. Consider reducing size in September." These patterns only emerge with time and are invisible without systematic tracking.

### 3.30 Analytics Export

All analytics views and reports are exportable to PDF and CSV. PDF exports are formatted as professional reports suitable for sharing with coaches, accountability partners, or prop firms. CSV exports provide raw data for traders who want to run their own analysis in Excel, Python, or R.

---

## 4. Daily Planning & "Plan of Attack" Module

### 4.1 Pre-Market Planner Template

The default landing page before market open. A structured daily template with sections for market bias (bullish, bearish, neutral, with reasoning), the day's watchlist, potential setups with key levels, economic calendar highlights, a pre-trade checklist, and session goals. The template is customizable — traders can add, remove, or reorder sections to match their pre-market routine. The plan auto-saves and can be referenced throughout the trading day.

### 4.2 Watchlist Builder

Within the daily plan, traders build their watchlist for the session. Each watchlist item includes the symbol, key price levels (support, resistance, supply zones, demand zones) with notes on each level, upcoming catalysts (earnings, FDA approval, sector rotation), and the trader's setup description. The AI enhances each watchlist item with auto-pulled recent price action context, volume profile analysis, and setup probability estimates: "AAPL showing bull flag on daily, potential breakout above 195 with volume. Historical win rate on this pattern: 64%." Watchlist items can be auto-imported from TradingView or broker watchlists.

### 4.3 Pre-Trade Checklist

A fully customizable checklist that the trader defines and must review before entering any trade. Examples: "Is this an A+ setup?", "Did I wait for confirmation?", "Is my risk under 1%?", "Am I following my plan?", "No open risk overnight?", "Have I checked the economic calendar?". Checklist items can be marked as required (must be checked before logging a trade) or optional (tracked but not enforced). The checklist is versioned so the trader can evolve their rules over time and track how checklist changes correlate with performance changes.

### 4.4 Economic Calendar Integration

Full integration with economic calendar data (Forex Factory, Investing.com, or equivalent API). Events are displayed with impact ratings (high/medium/low), expected values, and time until the event. The AI provides a summary of potential market impact: "High impact: CPI at 8:30 AM — expect elevated volatility in USD pairs and equity indices for 30 minutes post-release." Events are auto-tagged to trades that occur within their impact window.

### 4.5 AI-Generated "Plan of Attack"

The flagship planning feature. The trader inputs their overnight analysis — either uploading charts, describing their market bias, or both — and the AI generates a concise "Plan of Attack" briefing. The output is a one-page summary: top 5 prioritized watchlist ideas with key levels and risk/reward, a market bias summary with supporting and invalidating evidence, specific risk alerts for the day (high-impact events, unusual pre-market activity), and a motivational reminder tailored to the trader's recent weaknesses (e.g., "You've been cutting winners too early — let your targets breathe today"). The plan is exportable to PDF or as a phone lock screen widget for quick reference.

### 4.6 Scenario Planning

Within each watchlist item, the trader can describe or draw scenarios: "If AAPL breaks above 195, target 200 with stop at 192." The AI calculates risk-reward ratios for each scenario and suggests position sizing based on the trader's account and risk rules. Multiple scenarios per symbol are supported — bullish, bearish, and neutral plays can all be pre-planned.

### 4.7 Session Goals & Rules Reminder

A dedicated section for the day's trading rules: maximum number of trades, maximum daily loss amount, minimum setup quality (A+ only), session end time, and any behavioral goals ("No trading in the first 15 minutes", "Journal every trade immediately"). These goals carry through the day and are referenced during the post-market review to calculate plan adherence.

### 4.8 Plan Continuity

At the end of each day, the app automatically identifies unfinished watches, unmet goals, open positions, and lessons learned. These are carried forward to the next day's plan template, pre-populated for the trader to review, modify, or dismiss. No insight is lost between sessions.

### 4.9 Plan-to-Trade Linkage

Every trade logged during the day is linked back to the daily plan. The app tracks which watchlist items became actual trades, which checklist items were honored, and which session goals were met. This linkage is the foundation of plan adherence scoring.

### 4.10 Plan Adherence Scoring

At the end of each trading day, the AI evaluates how well the trader followed their plan. The score considers: Did trades come from the watchlist? Were checklist items honored? Were position sizes within planned risk? Were session goals met? Was the daily loss limit respected? The adherence score (0-100) is tracked over time and becomes a key component of the trader's Personal Edge Score.

### 4.11 Pre-Market Routine Timer

An optional guided workflow that walks the trader through their pre-market routine step by step with time allocations: "Review overnight futures (5 min) → Scan watchlist (10 min) → Mark key levels (10 min) → Complete Plan of Attack (5 min) → Review checklist (2 min)." Helps establish consistent habits, especially for traders who tend to skip planning when they're eager to start trading.

---

## 5. AI-Powered Trade Review System

### 5.1 AI Trade Reviewer — The Core Feature

The star feature of the entire app. For any trade, the user clicks "Ask AI to Review This Trade" and uploads trade details plus chart screenshots. The AI processes the full context — chart images (via multimodal vision), P&L data, journal notes, voice note transcriptions, tags, thesis, conviction score, and checklist adherence — and delivers a thorough, honest, structured review.

The review output includes: an overall score (1-10), a list of strengths (3-5 bullet points of what the trader did well), a list of weaknesses (3-5 bullet points of mistakes or missed opportunities), a key lesson distilled into one actionable sentence, specific improvement suggestions with concrete steps ("Move your stop to breakeven earlier when price reaches 1R in trending moves"), an alternative scenario analysis ("If you had waited for the 5-minute candle close for confirmation, your entry would have been 0.3% better and your R-multiple would have improved from 1.2 to 1.8"), execution quality assessment (entry timing, exit timing, scaling decisions), risk management evaluation (was risk appropriate, was stop placed well, did they honor the stop), and a psychological read based on notes and trade behavior (e.g., "The rapid entry after your previous loss suggests possible revenge trading behavior").

The review adapts to the user's chosen AI personality — strict coach gives harsher but more direct feedback, encouraging mentor leads with positives and frames weaknesses as growth areas.

### 5.2 Chart Screenshot Analysis

The multimodal AI engine analyzes uploaded chart screenshots in depth. It identifies candlestick patterns (engulfing, doji, hammer, shooting star, inside bars), chart patterns (head and shoulders, double tops/bottoms, flags, wedges, triangles, channels), support and resistance levels visible on the chart, trend direction and strength, volume characteristics (increasing, decreasing, divergent from price), visible indicators (moving averages, RSI, MACD, VWAP, Bollinger Bands) and their current signals, and order flow or level 2 context if visible. The analysis is referenced in the trade review: "The chart shows RSI divergence on the 15-minute timeframe at entry, which you didn't mention in your thesis. This was actually a strong additional confluence factor."

### 5.3 Review Chat Follow-Up

After receiving an AI review, the trader can continue the conversation. Ask follow-up questions: "How could I have sized this better?", "Is this strategy still valid given current market conditions?", "What entry signal should I have waited for?", "Show me what a better exit would have looked like." The chat maintains full context of the trade and the review, allowing deep exploration of any aspect. Conversation history is stored with the trade for future reference.

### 5.4 Guided Self-Review Prompts

Before or alongside the AI review, the app provides structured self-review prompts: "What went well in this trade?", "What would you change if you could redo it?", "Was this trade on your plan?", "Did you follow your checklist?", "Rate your execution (1-10).", "Was this an A, B, C, or D setup?". The self-review is compared with the AI's assessment — discrepancies between the trader's self-evaluation and the AI's evaluation are flagged as learning opportunities.

### 5.5 AI Trade Thesis vs. Outcome Comparison

The AI takes the trader's pre-trade thesis and compares it against what actually happened. Scores thesis alignment on a 1-5 scale and provides specific feedback on where the thesis was accurate and where it diverged from reality. Tracks thesis alignment over time to show improvement in the trader's ability to read markets.

### 5.6 AI Devil's Advocate Mode

An optional pre-trade feature. Before entering a trade, the trader snaps their chart and writes their thesis. The AI argues the other side: "You're bullish on this breakout, but volume is declining, RSI is overbought on the hourly, and there's a supply zone 1% above entry. What's your invalidation level?" This isn't designed to prevent trades — it forces a 10-second pause for deliberate thinking and ensures the trader has considered the opposing case.

### 5.7 Pre-Trade AI Gate

A streamlined version of the devil's advocate for quick checks. Snap a chart, input entry/stop/target, and get an instant quality score: setup quality rating (1-10), risk/reward assessment, alignment with the daily plan, and historical performance on this type of setup. Not a gatekeeper — the trader can always proceed — but a structured pause that externalizes the mental checklist elite traders do automatically.

### 5.8 Weekly AI Summary Report

Every week, the AI reviews all trades from the past 7 days and generates a narrative summary. The report includes: week-over-week performance comparison, best and worst trades with brief analysis, behavioral patterns observed (e.g., "Your win rate improved 12% this week because you avoided FOMO entries"), plan adherence trend, emotional stability assessment, top 3 strengths of the week, top 3 areas for improvement, and a 3-step action plan for the coming week. The report is formatted as a clean, readable document — not a data dump.

### 5.9 Monthly AI Summary Report

A deeper analysis covering the full month. Includes everything in the weekly report plus: strategy-level performance breakdown, setup evolution (which setups gained or lost edge), risk management trend, goal progress assessment, comparison with previous months, and long-term trajectory analysis. Identifies patterns that only emerge over longer timeframes.

### 5.10 Quarterly Deep Review

The most comprehensive AI-generated report. A full-quarter retrospective formatted as a professional PDF. Includes: quarterly equity curve with drawdown analysis, best and worst month breakdown, strategy evolution summary, behavioral trend analysis over 12 weeks, goal achievement scorecard, Personal Edge Score trajectory, risk management audit, detailed improvement plan for next quarter, and forward-looking recommendations based on market regime expectations. This is the caliber of report a prop firm or institutional desk would produce for their traders.

### 5.11 Long-Term Pattern Detection

The AI continuously monitors the trader's behavior over weeks and months, flagging recurring patterns that the trader may not notice themselves. Examples: "You lose most in the first hour — consider delaying entries", "80% of your losses occur on low-conviction trades", "Your performance degrades after 3 consecutive winning days — you appear to get overconfident", "You overtrade on Fridays — possibly rushing to hit weekly targets." These alerts are surfaced proactively, not buried in reports.

### 5.12 Setup Decay Detection

The AI monitors each named setup in the trader's playbook over rolling 30/60/90-day windows. When a previously profitable setup begins degrading, the AI alerts: "Your VWAP bounce strategy had a 68% win rate in Q1 but has dropped to 41% over the last 45 trades. Market regime may have shifted — consider pausing this setup or adapting the criteria." This prevents traders from clinging to setups that no longer work.

### 5.13 AI Suggested Study Material

Based on the trader's identified weaknesses, the AI recommends specific concepts to study: "Your biggest leak is exit management. Study these 3 concepts: trailing stops using ATR multiples, scaling out at structure levels, and time-based exits for day trades." Recommendations are pulled from the trader's actual data gaps, not generic advice. As the trader improves in an area, recommendations shift to the next weakest link.

### 5.14 Prompt Versioning System

All AI prompts used for reviews, summaries, and analysis are versioned and stored in the database. This allows A/B testing of prompt variations to optimize review quality, tracking of which prompt versions produce the most helpful feedback (based on user ratings), and continuous improvement of the AI engine without disrupting user experience. Cost per review (token usage) is also tracked per prompt version.

---

## 6. Risk Management & Position Sizing Suite

### 6.1 Position Sizing Calculator

Five calculation methods available: percentage risk (risk a fixed % of account per trade), ATR-based volatility sizing (adjust size based on instrument volatility), Kelly criterion (mathematically optimal sizing based on win rate and payoff ratio), fixed fractional (fixed dollar amount per trade), and custom formulas (user-defined sizing logic). The calculator takes current account balance, entry price, stop-loss price, and the chosen method, and outputs the exact position size, number of shares/contracts, dollar risk, and percentage risk. Available as a standalone tool and integrated into the trade entry form.

### 6.2 Risk Dashboard

A real-time portfolio risk overview showing total account exposure (heat), number of open positions, total dollar risk across all positions, correlation between positions, and worst-case scenario modeling. The dashboard updates as trades are opened and closed, giving the trader a constant awareness of their overall risk posture — not just per-trade risk but aggregate portfolio risk.

### 6.3 Position Correlation Warnings

When a trader opens multiple positions in correlated instruments (e.g., AAPL and QQQ, or EURUSD and GBPUSD), the app warns about correlated risk: "You have long positions in AAPL, MSFT, and GOOGL. These are 85%+ correlated. Your effective risk is 3x what individual position sizing suggests." This prevents accidental concentration that single-position risk metrics miss.

### 6.4 Scenario Stress Testing

A simulator that models portfolio impact under adverse conditions: "What if the market gaps down 3% overnight?", "What if my 3 correlated positions all hit their stops?", "What if volatility doubles?" The simulator uses the trader's actual open positions and risk parameters to show concrete dollar and percentage impact. This makes abstract risk tangible.

### 6.5 Position Sizing Consistency Score

Tracks whether the trader actually sizes according to their stated rules. Compares intended risk percentage to actual risk percentage across all trades and computes consistency: "You say you risk 1% per trade, but your actual sizing varies from 0.3% to 2.8%. Standard deviation of your risk is 0.9%. Inconsistent sizing is costing you edge." Most traders have no idea how inconsistent they are — this makes it measurable.

### 6.6 Weekly "What If" Simulator

Takes the trader's actual trades for the week and runs alternate scenario models: "What if you honored every stop-loss? +2.4R better. What if you took profits at 2R instead of holding for 3R? +1.1R better. What if you skipped all low-conviction trades? +1.8R better." Concrete counterfactuals grounded in real data, not abstract theory.

### 6.7 Broker Fee Comparison

For traders with multiple connected brokers, tracks commission costs across each one. Shows side-by-side comparison: "You paid $1,200 in commissions on IBKR this month for options. The same volume on Tastytrade would have cost $740. Consider switching for options-heavy months." Actionable savings that most traders never calculate because it requires comparing fee schedules manually.

---

## 7. Psychological & Discipline Tools

### 7.1 Mood & Emotion Journal

A daily log capturing the trader's emotional state before and after the market session. Pre-market mood (1-10), post-market mood (1-10), stress level, confidence level, sleep quality, and free-text notes. Emotion tags allow quick labeling: focused, anxious, overconfident, calm, frustrated, excited, bored, distracted. All mood data is linked to performance data, enabling correlation analysis: "Your win rate is 68% on days you rate stress below 4, and 41% on days stress is above 7." This surfaces how mindset directly impacts results.

### 7.2 Tilt Detection Engine

An algorithmic system that monitors trading behavior in real-time for signs of emotional trading (tilt). Detection triggers include: rapid trade entries (3+ trades within a short window after a loss), position size escalation after losses, trading outside planned hours, deviating from watchlist, and ignoring checklist items. When tilt is detected, the severity is classified (warning, alert, critical) and the trader is notified with context: "Tilt detected: You've entered 4 trades in the last 20 minutes after a losing streak. Historically, trades taken in this state have a 23% win rate. Consider stepping away for 30 minutes." Tilt events are logged and tracked over time.

### 7.3 Revenge Trade Detector (Proactive)

A proactive intervention, not just post-hoc analysis. If the app detects the trader just closed a losing trade and opens the trade entry screen within 60 seconds, a 30-second cooldown screen appears with a breathing exercise or the trader's own custom rule reminder. This is opt-in and configurable — the trader chooses their cooldown duration and what appears on the screen. The goal is to interrupt the revenge impulse before it becomes a revenge trade.

### 7.4 Custom Alert Rules Engine

Traders build their own behavioral alerts using simple conditional logic: "If my daily loss exceeds $500, lock me out for 30 minutes." "If I take more than 4 trades before noon, show a warning." "If my last 3 trades were losers, require a 10-minute cooldown and journal entry before next trade." "If I trade a symbol not on today's watchlist, show a confirmation dialog." The rules are programmable discipline — the trader defines their own guardrails, and the app enforces them. Rules can be enabled/disabled and track how often they trigger.

### 7.5 Emotional Contagion Detector

For traders connected to social feeds, chat rooms, or alert services, the app tracks whether trade timing correlates with external signals. "This trade was entered within 2 minutes of a bullish alert in your chat room. Your win rate on socially-triggered trades is 38% vs. 61% on self-sourced setups." This quantifies whether community noise helps or hurts the individual trader's performance. No judgment — just data showing the correlation.

### 7.6 Goal Tracking System

SMART goal creation and tracking: set measurable trading goals with specific targets, deadlines, and progress visualization. Examples: "Achieve 60% win rate by Q3", "Reduce average loss size by 20% this month", "Follow pre-trade checklist on 90% of trades", "Journal every trade within 30 minutes". Progress bars, trend charts, and AI coaching on goal achievement. When a goal is hit, it's celebrated and a new goal is suggested. When a goal is missed, the AI analyzes why and recommends adjustments.

### 7.7 Journaling Streaks with Quality-Weighted Decay

Tracks consistent journaling and planning habits, but with nuance — a one-word entry doesn't count the same as a thoughtful review. The AI scores journal entry quality (thoroughness, self-reflection, actionable insights) and applies that to the streak calculation. The decay model is forgiving: missing a day doesn't reset the streak to zero but gradually decays the streak value. This is less punishing and more realistic than binary streak tracking, encouraging consistency without triggering all-or-nothing thinking.

### 7.8 Loss Autopsy Mode

A dedicated analytical view for losing trades only. Every loss is categorized into one of two buckets: "good loss" (followed the plan, stop was hit, the setup just didn't work — this is normal and acceptable) and "bad loss" (broke rules, revenge traded, oversized, moved stop, no thesis). The ratio between good losses and bad losses is more important than win rate — a trader with 90% good losses and 10% bad losses is on a path to profitability even with a sub-50% win rate. The AI tracks this ratio and surfaces it prominently: "Your good loss ratio this month is 73%, up from 61% last month. You're improving your discipline."

### 7.9 Drawdown Recovery Playbook

When a trader hits a user-defined drawdown threshold (e.g., 10% from peak equity), the app automatically shifts into recovery mode. Recovery mode includes: reduced position sizing suggestions (halve normal size), stricter checklist enforcement (all items required), AI encouragement calibrated to the trader's psychological profile, a structured "climb back" plan with progressive milestones (first milestone: break even on the day, then break even on the week, then new equity high), and a focus on high-conviction setups only. Most traders spiral during drawdowns because nobody tells them to downshift. This feature is that coach in the corner telling them to take a breath and fight smart.

### 7.10 Confidence Calibration Tracker

Compares the trader's pre-trade conviction scores to actual outcomes over time. Measures calibration accuracy: "When you say conviction 5, you win 80% — well calibrated. When you say conviction 3, you win 55% but you expect 60% — slightly overconfident on mid-range setups. When you say conviction 1, you win 42% but you should be at 20% — you actually do better than you think on low-conviction plays." This is metacognition training backed by data — teaching traders to trust their gut when it's right and question it when it's wrong.

### 7.11 Trade DNA Fingerprint

Over hundreds of trades, the AI builds a unique visual profile of the trader's tendencies: average hold time, preferred setups, risk tolerance patterns, time-of-day bias, asset concentration, sizing behavior, emotional trading frequency, win rate trajectory, and execution quality. Represented as a visual fingerprint or radar chart that evolves over time. Useful for self-awareness and for coaches to quickly understand a new student's trading DNA.

### 7.12 Personal Edge Score

A single composite number (0-100) that updates daily, combining plan adherence, trade grade distribution, risk management consistency, journal quality, emotional stability, and P&L normalized by volatility. Like a credit score for trading. The trader watches it trend over months. The AI explains what's dragging it down ("Risk management consistency dropped 8 points — you've been oversizing on low-conviction trades") and what's lifting it ("Plan adherence is at an all-time high — you're following your checklist 94% of the time").

### 7.13 Gamification (Meaningful, Not Gimmicky)

Streaks for consistent planning and journaling, badges for genuine milestones ("100 Plan-Following Trades", "30-Day Journal Streak", "First Quarterly Review Completed", "Good Loss Ratio Above 80%"). Gamification is tied to process metrics, not P&L — the app rewards discipline and consistency, not lucky wins. No leaderboards comparing P&L, only leaderboards comparing process metrics for traders who opt in to community features.

---

## 8. Setup Playbook & Trade Grading

### 8.1 Playbook Builder

Traders codify their A+ setups as reusable templates. Each playbook entry includes: setup name, description, entry criteria (with weights — some criteria are essential, others are nice-to-have), expected R-multiple range, preferred timeframe, market regime filter (only valid in trending markets, for example), minimum conviction threshold, example screenshots with annotations, and notes on common mistakes. The playbook is a living document — it evolves as the trader refines their edge.

### 8.2 Setup Matching on Trade Entry

When logging a trade, the AI compares it against the trader's playbook and scores adherence: "This matches your 'VWAP Bounce' setup. Criteria met: 4 out of 6 — B-grade setup. Missing: volume confirmation and higher timeframe alignment." This forces the trader to see whether they're actually trading their defined setups or improvising. Over time, the playbook's performance data proves which setups have genuine edge.

### 8.3 Trade Grading Rubric (Customizable)

Traders define what an A, B, C, and D trade means to them using weighted criteria: plan adherence (30%), entry quality (20%), risk management (20%), exit execution (15%), emotional state (15%). Each criterion is scored, weights are applied, and the composite grade is calculated. The rubric is fully customizable — traders choose their own criteria and weights. Grade distribution is tracked over time: "This month you took 12 A-trades, 8 B-trades, 15 C-trades, and 5 D-trades. Last month was 6 A-trades and 18 C-trades. Your setup selectivity is improving."

### 8.4 Exit Quality Score

Separate from the trade grade, this metric evaluates where the trader actually exited vs. where the optimal exit was — based on their own rules, not hindsight perfection. "Your average exit captures 62% of the available move (measured from entry to MFE). Traders in your experience bracket average 55%. Your exits are actually above average, but there's room to reach 70% by holding through the first pullback after reaching 1R."

### 8.5 Importable Rulesets from Mentors

Coaches and mentors can export their pre-trade checklist, grading rubric, setup playbook, and behavioral rules as a shareable template package. Students import it with one click and immediately begin grading themselves against their mentor's framework. This is designed for the coaching relationship — Billy's 18K community members could import his methodology and track their adherence to his system.

### 8.6 Shared Ruleset Marketplace

Rulesets can be shared publicly or kept private. A marketplace (or simple directory) lets traders browse and import rulesets from successful traders or coaches. Import counts are tracked. Rulesets are versioned so updates from the creator can be pushed to importers.

---

## 9. Advanced Review & Replay

### 9.1 Trade Replay Mode

Animated playback of completed trades on historical charts with the trader's annotations, levels, and entry/exit points overlaid. The chart plays forward candle-by-candle (speed adjustable) showing exactly how the trade unfolded. The trader watches their setup develop, sees their entry, watches the trade progress, and sees their exit — all in context. Useful for post-trade learning and for reviewing old trades months later.

### 9.2 Decision Point Replay (Fork-in-the-Road Training)

An enhanced replay mode that pauses at each decision point — entry, potential add, scale, exit — and asks "What would you do here?" before revealing what the trader actually did. This turns review into active practice, not passive watching. The trader practices decision-making in real scenarios from their own history. The AI can score the practice decision against the actual outcome and the optimal decision.

### 9.3 Trade Screenshot Timeline

A visual, chronological timeline of all the trader's chart screenshots. Scroll through trading history like a photo album. See chart reading quality evolve over time. See annotation accuracy improve. See market phases come and go. This is both a learning tool (visual pattern recognition gets better with review) and a motivation tool (seeing tangible progress).

### 9.4 Weekend Review Ritual

A structured Saturday session guided by the AI. The AI compiles the full week: best trade, worst trade, biggest lesson, plan adherence trend, emotional arc, tilt events, goal progress, and a "focus for next week" recommendation. The ritual takes 15 minutes and produces a clean one-pager. Exportable as a PDF that a coach or accountability partner can review. Consistency with the weekend review is tracked as part of the journaling streak.

### 9.5 Data Integrity Audit

A monthly AI check on data quality: "You have 12 trades without screenshots, 8 without stop-loss logged, 23 without journal notes, and 6 without tags. Your data quality score is 71%. Improving to 90%+ will make AI reviews significantly more accurate and your analytics more reliable." This incentivizes complete data entry without nagging during trading hours — the audit runs in the background and surfaces as a monthly report.

---

## 10. Accountability & Social Features

### 10.1 Coach/Mentor Dashboard

Traders can share a read-only dashboard with a mentor, coach, or accountability partner. The coach sees the trader's plan adherence scores, trade grades, tilt alerts, AI summary reports, and Personal Edge Score — without seeing P&L if the trader prefers. This is built for the coaching relationship: the coach gets signal-rich data about the trader's discipline and process without needing to review every trade manually.

### 10.2 Permission Controls

Granular control over what the accountability partner can see. The trader toggles visibility for: daily plans, trade grades, tilt events, mood logs, P&L, analytics, AI reviews. Each permission is independently controllable. Invitations are managed through the app with expiration dates and revocable access.

### 10.3 Anonymous Trade Sharing

Opt-in sharing of individual trades (anonymized — no account size, no P&L in dollars, only R-multiples and percentages) for community feedback. Other traders can view the setup, the chart, and the outcome, and provide comments. Useful for getting outside perspectives on setups and execution.

### 10.4 Community Leaderboards

Opt-in leaderboards ranked by process metrics — Personal Edge Score, plan adherence, journaling consistency, good loss ratio — not by P&L. This prevents toxic comparison while encouraging discipline competition. The leaderboard rewards the behaviors that lead to long-term profitability, not short-term results.

---

## 11. Additional Power Features

### 11.1 Lite Backtesting

Define simple strategy rules in plain English or structured inputs. The AI tests those rules against the trader's own historical trades (not market data — their actual execution history). "If I only took trades where 3+ timeframes aligned, what would my results look like?" "If I exited at 2R instead of letting trades run to 3R?" This isn't a full backtesting engine — it's historical "what if" analysis on the trader's own data.

### 11.2 News & Catalyst Integration

Trades are auto-tagged with relevant news events active at the time of the trade. The AI summarizes potential impact: "This trade was entered 15 minutes after Fed Chair comments — elevated volatility detected." News context is stored with the trade and available in analytics for filtering.

### 11.3 Options-Specific Tools

For options traders: Greeks calculator (Delta, Gamma, Theta, Vega), payoff diagrams (visual P&L at expiration and current), probability cones (expected move based on implied volatility), and options flow integration if data feed is available. Options trades log additional fields: strike, expiration, option type, implied volatility at entry, and Greeks at entry.

### 11.4 Custom Alert Rules Engine (Expanded)

Beyond tilt detection, the rules engine supports arbitrary behavioral triggers with configurable actions. Triggers can be based on: trade count, time windows, P&L thresholds, consecutive outcomes, position size, symbols traded, session timing, or any combination. Actions include: notifications, cooldown timers, mandatory journal entries, lockout periods, or display of custom messages. This is programmable discipline — the trader writes their own rules and the app enforces them.

### 11.5 Offline Mode

Full trade logging and planning functionality works offline. Trades are queued locally and synced automatically when connectivity returns. The sync engine handles conflicts and deduplication. Critical for traders in areas with unreliable connectivity or those who trade from locations without internet access.

### 11.6 Security & Privacy

End-to-end encryption for sensitive data (account balances, P&L, broker credentials). Biometric login (Face ID, fingerprint) on mobile. All data is user-owned and exportable at any time. Full GDPR compliance with data deletion on request. No data is shared with third parties. Broker API credentials are encrypted at rest and in transit.

### 11.7 Customization Depth

Fully customizable dashboards (drag, drop, resize analytics widgets), checklists, tags, AI review styles, grading rubrics, alert rules, and planning templates. Every trader's workflow is different — the app adapts to the trader, not the other way around.

### 11.8 Market Internals Auto-Capture

At the moment of each trade, the app automatically snapshots market internals (when available through connected data feeds): TICK, ADD, VOLD, VIX level and direction, sector rotation data, and breadth indicators. These snapshots are stored with the trade and power analytics that show: "You perform best when TICK is above zero and VIX is declining." This is institutional-level context tracking that retail journals completely ignore.

### 11.9 Seasonal Performance Pattern Detection

After 6-12 months of data, the AI surfaces calendar-based patterns in the trader's performance. Monthly tendencies, seasonal strengths and weaknesses, and performance around recurring events (triple witching, end of quarter, January effect). These patterns only emerge with time and systematic tracking — the app does the analysis automatically.

### 11.10 Trade Screenshot AI Auto-Annotation

When a chart screenshot is uploaded, the AI doesn't just analyze it — it generates a visual annotation overlay highlighting detected patterns, levels, and signals. The trader sees their chart with AI markup: trendlines traced, support/resistance levels marked, pattern boundaries drawn, and indicator signals highlighted. This overlay can be saved with the trade or dismissed.

---

## 12. Mobile Experience

### 12.1 Progressive Web App (Phase 1)

The initial mobile experience is a PWA with service worker for offline support, home screen installation, and fast load times. The PWA covers 90% of mobile use cases: quick trade logging, plan review, analytics viewing, and AI review reading.

### 12.2 Native Shell via Capacitor (Phase 2)

A Capacitor wrapper for iOS and Android adds native capabilities: push notifications for alerts and tilt warnings, biometric authentication, direct camera access for screenshot capture, and home screen widgets showing the daily plan summary, equity curve, or Personal Edge Score.

### 12.3 Home Screen Widgets

Native widgets showing at-a-glance information: today's P&L, equity curve mini-chart, plan adherence score, Personal Edge Score, or the day's watchlist. Configurable to show what matters most to the individual trader.

### 12.4 Quick-Log from Notification

When a tilt alert or trade reminder notification fires, the notification action button allows immediate trade entry without opening the full app. One tap to start logging, pre-filled with the current time and recently-traded symbols.

---

## 13. Infrastructure & Performance

### 13.1 Analytics Caching

Precomputed analytics for common queries (monthly stats, strategy performance, core metrics) stored in a cache layer that refreshes when new trades are logged. Ensures the analytics dashboard loads instantly even with thousands of trades.

### 13.2 Token Usage Tracking

Every AI interaction (trade review, planning, summary) tracks token usage and cost. A dashboard shows the trader's AI usage: number of reviews, tokens consumed, cost per review. This enables the app to manage AI costs at scale and provide transparency to users.

### 13.3 Database Optimization

Indexes on all common query patterns (user_id + date ranges, user_id + tags, user_id + setup_name). Materialized views for heavy analytics computations. Connection pooling for concurrent users. Query performance monitoring.

### 13.4 Error Monitoring

Comprehensive error tracking and alerting. Every API error, failed sync, and crashed computation is logged with context. Users see friendly error messages with actionable guidance, never raw stack traces.

### 13.5 Data Export & Backup

Full data export at any time: all trades, plans, reviews, screenshots, and analytics in CSV, JSON, or PDF format. Automated backups with point-in-time recovery. The trader's data is their property — they can take it anywhere.
