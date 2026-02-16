# TradeMaster AI — Phase 1 through Phase 4 Prompts
# Apple Principal Engineer ICT Level 7+ Standards
# FULL RUST BACKEND

---

## ============================================================
## PHASE 1 — CORE TRADE LOGGING
## ============================================================
## Features covered: 2.1 through 2.15 from the Features Spec
## ============================================================

### PROMPT 1.1 — Trade TypeScript Types & Rust Models

```
[PASTE GLOBAL CONTEXT BLOCK + STACK OVERRIDE]

PHASE 1 — CORE TRADE LOGGING
PROMPT 1.1 — Trade TypeScript Types & Rust Models

OBJECTIVE: Define every TypeScript type and Rust model for trades, trade legs, trade media, and tags. These types are the contract between frontend and backend — they must match exactly.

DEPENDENCIES: Phase 0 complete (project scaffolding, database migrations, API scaffold)

DELIVERABLES:

1. packages/types/src/trade.ts — Complete TypeScript types:

   Trade (full database row representation):
   - Every field from the trades table in the database schema
   - All fields properly typed: string for text, number for decimals/integers, boolean, Date for timestamps
   - Nullable fields marked with | null
   - Enums as union types:
     - AssetClass = 'stock' | 'option' | 'forex' | 'futures' | 'crypto'
     - TradeDirection = 'long' | 'short'
     - TradeStatus = 'open' | 'closed' | 'partial'
     - MarketRegime = 'trending_up' | 'trending_down' | 'ranging' | 'volatile' | 'choppy' | 'low_volume'
     - SessionType = 'premarket' | 'first_30' | 'mid_morning' | 'midday' | 'afternoon' | 'power_hour' | 'after_hours'
     - TradeGrade = 'A' | 'B' | 'C' | 'D'
     - EntrySource = 'manual' | 'broker_sync' | 'csv_import'

   TradeCreateRequest — fields required/optional for creating a trade:
   - Required: symbol, asset_class, direction
   - Optional: everything else (entry_price, exit_price, etc.)
   - Does NOT include: id, user_id, created_at, updated_at (server-generated)

   TradeUpdateRequest — Partial<TradeCreateRequest> for updates

   TradeWithRelations — Trade extended with:
   - tags: Tag[]
   - media: TradeMedia[]
   - legs: TradeLeg[]
   - ai_review: AiReview | null

   TradeListResponse — paginated list:
   - trades: Trade[]
   - total: number
   - page: number
   - page_size: number
   - has_more: boolean

   TradeFilters:
   - date_from?: string (ISO date)
   - date_to?: string
   - symbol?: string
   - asset_class?: AssetClass
   - direction?: TradeDirection
   - status?: TradeStatus
   - tags?: string[] (tag IDs)
   - setup_name?: string
   - conviction_min?: number
   - conviction_max?: number
   - grade?: TradeGrade
   - market_regime?: MarketRegime
   - session_type?: SessionType
   - is_missed?: boolean
   - archived?: boolean
   - pnl_min?: number
   - pnl_max?: number

   TradeSortField = 'created_at' | 'entry_time' | 'exit_time' | 'pnl' | 'pnl_pct' | 'r_multiple' | 'symbol' | 'conviction_score'
   TradeSortOrder = 'asc' | 'desc'

   TradeLeg:
   - id, trade_id, leg_type ('entry' | 'exit' | 'add' | 'trim'), price, size, timestamp, fees, notes, sort_order

   TradeLegCreateRequest — without id, trade_id

   TradeMedia:
   - id, trade_id, media_type ('screenshot' | 'recording' | 'order_confirmation'), storage_url, thumbnail_url, file_name, file_size, mime_type, ai_analysis (object | null), annotations (object | null), captured_at, sort_order

   Tag:
   - id, name, category ('strategy' | 'mistake' | 'emotion' | 'session' | 'market_condition' | 'custom'), color, icon, sort_order

   TagCreateRequest — without id

   TradeStats (summary for a set of trades):
   - total_trades, winners, losers, breakeven
   - win_rate, avg_r_multiple, profit_factor, expectancy
   - total_pnl, avg_pnl, largest_winner, largest_loser
   - avg_winner, avg_loser, max_drawdown
   - sharpe_ratio, sortino_ratio, recovery_factor
   - avg_duration_minutes, avg_conviction

2. apps/api/src/models/trade.rs — Rust models:
   - Trade struct with all fields matching DB + sqlx::FromRow derive
   - TradeCreateRequest with serde::Deserialize + validator derives
   - TradeUpdateRequest with all Option<T> fields
   - TradeWithRelations (Trade + Vec<Tag> + Vec<TradeMedia> + Vec<TradeLeg>)
   - TradeListResponse with pagination
   - TradeFilters from query params
   - TradeLeg, TradeLegCreate
   - TradeMedia
   - Tag, TagCreate
   - All financial fields use rust_decimal::Decimal
   - All timestamps use chrono::DateTime<chrono::Utc>
   - All UUIDs use uuid::Uuid
   - Implement proper Display, Debug for all types
   - Validation on create requests: symbol not empty, asset_class valid enum, direction valid enum, prices > 0 if provided, conviction 1-5 if provided

3. apps/api/src/models/tag.rs — Rust tag models:
   - Tag, TagCreate, TagUpdate
   - TagCategory enum matching the frontend type

4. packages/types/src/index.ts — Barrel export all trade types

CRITICAL REQUIREMENTS:
- TypeScript types and Rust models must be a 1:1 match in field names and types
- Use snake_case for all field names (Rust convention, serde handles JSON conversion)
- Rust financial fields MUST be rust_decimal::Decimal — NEVER f64
- Every enum must be defined on both sides with matching values
- Validation rules must be defined in both TypeScript (Zod schemas) and Rust (validator crate)
- No field can be ambiguously typed — every field has one concrete type

OUTPUT: Complete file contents for every file.
```

---

### PROMPT 1.2 — Rust Trade CRUD API Endpoints

```
[PASTE GLOBAL CONTEXT BLOCK + STACK OVERRIDE]

PHASE 1 — CORE TRADE LOGGING
PROMPT 1.2 — Rust Trade CRUD API Endpoints

OBJECTIVE: Implement the complete trade CRUD endpoints in the Rust/Axum backend. This is the core data layer — list trades with filtering/sorting/pagination, get single trade with all relations, create trade, update trade, archive trade.

DEPENDENCIES: Prompt 1.1 (models defined), Phase 0 (database tables, auth middleware)

DELIVERABLES:

1. apps/api/src/routes/trades.rs — Trade route handlers:

   GET /api/v1/trades — List trades:
   - Accepts query params: all fields from TradeFilters + page (default 1) + page_size (default 50) + sort_by (default 'created_at') + sort_order (default 'desc')
   - Builds dynamic SQL WHERE clause from filters (only apply filters that are present)
   - ALWAYS scopes by user_id from auth middleware (WHERE user_id = $1 AND ...)
   - Default: archived = false (unless explicitly filtered)
   - Includes tag names as a comma-separated field or JSONB aggregate for each trade
   - Returns TradeListResponse with pagination metadata
   - SQL must use parameterized queries — NEVER string concatenation for filters
   - Count query runs in parallel with data query for total count

   GET /api/v1/trades/:id — Get single trade with relations:
   - Fetches trade by ID where user_id matches authenticated user
   - Returns 404 if not found or not owned by user
   - Includes: trade data + tags (via junction table) + media files + trade legs + latest AI review
   - Use LEFT JOINs or separate queries — benchmark and pick the faster approach
   - Return as TradeWithRelations

   POST /api/v1/trades — Create trade:
   - Accepts TradeCreateRequest body
   - Validates all fields (symbol required, valid enums, prices > 0, etc.)
   - Sets user_id from auth middleware
   - Auto-calculates derived fields if enough data present:
     - r_multiple = (exit_price - entry_price) / (entry_price - stop_loss) for longs (inverse for shorts)
     - pnl = (exit_price - entry_price) * entry_size - commissions for longs
     - pnl_pct = pnl / (entry_price * entry_size) * 100
     - duration_minutes = difference between exit_time and entry_time
     - status = 'closed' if both entry and exit prices present, 'open' otherwise
   - Auto-detects session_type from entry_time and user's timezone setting
   - Auto-detects sector from symbol (use a lookup map for common stocks, default to null)
   - If tags are provided in the request (as array of tag IDs), creates trade_tags junction records
   - Returns the created trade with 201 status

   PUT /api/v1/trades/:id — Update trade:
   - Accepts TradeUpdateRequest (all fields optional)
   - Validates ownership (user_id match)
   - Only updates fields that are provided (PATCH semantics via PUT)
   - Recalculates derived fields if relevant inputs changed
   - Updates updated_at timestamp
   - If tags array provided, replaces all trade_tags (delete existing, insert new)
   - Returns updated trade

   DELETE /api/v1/trades/:id — Archive trade (soft delete):
   - Sets archived = true
   - Does NOT delete the record
   - Returns 204 No Content

   POST /api/v1/trades/:id/restore — Restore archived trade:
   - Sets archived = false
   - Returns restored trade

2. apps/api/src/routes/trade_legs.rs — Trade leg endpoints:

   GET /api/v1/trades/:id/legs — List legs for a trade
   POST /api/v1/trades/:id/legs — Add a leg
   PUT /api/v1/trades/:id/legs/:leg_id — Update a leg
   DELETE /api/v1/trades/:id/legs/:leg_id — Remove a leg

   When legs are added/removed, recalculate the parent trade's:
   - Average entry price (weighted by size)
   - Average exit price (weighted by size)
   - Total size
   - is_scaled flag (true if more than 1 entry leg or more than 1 exit leg)

3. apps/api/src/services/trade_calculations.rs — Trade math:
   - calculate_r_multiple(entry, exit, stop, direction) -> Decimal
   - calculate_pnl(entry, exit, size, commissions, direction) -> Decimal
   - calculate_pnl_pct(pnl, entry, size) -> Decimal
   - calculate_duration(entry_time, exit_time) -> i32 (minutes)
   - detect_session_type(entry_time, timezone) -> SessionType
   - calculate_weighted_avg_price(legs: &[TradeLeg]) -> Decimal
   - All functions use rust_decimal::Decimal, never f64
   - Include unit tests for every function with edge cases:
     - Long and short trades
     - Zero stop loss (handle division by zero)
     - Negative P&L
     - Overnight trades spanning midnight
     - Very small prices (crypto: 0.00001234)
     - Very large sizes (forex: 100000 units)

4. apps/api/src/routes/tags.rs — Tag CRUD endpoints:

   GET /api/v1/tags — List all user tags (optional ?category= filter)
   POST /api/v1/tags — Create tag
   PUT /api/v1/tags/:id — Update tag
   DELETE /api/v1/tags/:id — Delete tag (cascades to trade_tags)
   POST /api/v1/trades/:id/tags — Add tag to trade (body: { tag_id })
   DELETE /api/v1/trades/:id/tags/:tag_id — Remove tag from trade

5. Update apps/api/src/routes/mod.rs — Register all new routes:
   - Mount trade routes at /api/v1/trades
   - Mount tag routes at /api/v1/tags
   - All behind auth middleware

CRITICAL REQUIREMENTS:
- EVERY query must include WHERE user_id = $auth_user_id — no exceptions, this is the security boundary
- Financial calculations MUST use rust_decimal — test with extreme values
- SQL injection is impossible because we use sqlx parameterized queries — verify no string formatting is used for SQL
- Pagination must be consistent: page 1 = first page, not page 0
- Sorting must be safe: validate sort_by against an allowlist of column names
- The dynamic filter builder must handle any combination of filters efficiently
- Trade creation should be wrapped in a database transaction (trade insert + tag inserts are atomic)
- Error responses must be user-friendly: "Trade not found" not "sqlx::Error::RowNotFound"
- Unit tests for trade_calculations.rs — at minimum 20 test cases covering all functions and edge cases

OUTPUT: Complete file contents for every file.
```

---

### PROMPT 1.3 — Trade Entry Form (Frontend)

```
[PASTE GLOBAL CONTEXT BLOCK + STACK OVERRIDE]

PHASE 1 — CORE TRADE LOGGING
PROMPT 1.3 — Trade Entry Form (Frontend)

OBJECTIVE: Build the complete trade entry form — the most-used screen in the app. This form must be fast, smart, and handle every field from the trades table while remaining approachable. It's the bridge between the trader's experience and the data that powers everything else.

DEPENDENCIES: Phase 0 (UI components, layout, API client), Prompt 1.1 (types), Prompt 1.2 (API endpoints)

DELIVERABLES:

1. apps/web/src/lib/components/trade/trade-form.svelte — The main trade entry form:

   LAYOUT:
   - Two-column layout on desktop (primary fields left, secondary right), single column on mobile
   - Organized into collapsible sections with clear headings:
     Section 1 — "Instrument" (always visible):
       - Symbol input (text, uppercase, auto-format, required)
       - Asset class selector (5 options as pill buttons: Stock, Option, Forex, Futures, Crypto)
       - Direction toggle (Long / Short — large, clear toggle with green/red coloring)
     
     Section 2 — "Entry & Exit" (always visible):
       - Entry price (number input, decimal precision based on asset class — 2 for stocks, 8 for crypto, 5 for forex)
       - Entry time (datetime picker, defaults to now)
       - Entry size / quantity (number input)
       - Exit price (number input, same precision rules)
       - Exit time (datetime picker)
       - Exit size (number, defaults to entry size)
     
     Section 3 — "Risk" (always visible):
       - Stop loss price (number input)
       - Take profit price (number input)
       - Planned risk % (number, auto-filled from user profile default)
       - R-Multiple (auto-calculated, read-only, displayed prominently)
       - P&L (auto-calculated, read-only, styled green/red)
       - P&L % (auto-calculated, read-only)
     
     Section 4 — "Details" (collapsible, expanded by default):
       - Setup name (select from playbook setups + free text option)
       - Conviction score (1-5 clickable stars or numbered buttons)
       - Trade grade (A/B/C/D buttons with grade colors from design system)
       - Market regime (select dropdown)
       - Commissions (number input, default from user settings)
       - Slippage (number input)
     
     Section 5 — "Notes & Thesis" (collapsible):
       - Pre-trade thesis (textarea, placeholder: "Why are you taking this trade?")
       - Notes (textarea, larger, for post-trade thoughts)
       - Voice note button (triggers recording, shows playback after recording)
     
     Section 6 — "Tags" (collapsible):
       - Tag picker: shows all user tags grouped by category
       - Click to toggle on/off
       - Search/filter tags
       - Quick-create new tag inline
       - AI-suggested tags appear as ghost pills that can be clicked to apply
     
     Section 7 — "Screenshots" (collapsible):
       - Drag-and-drop upload zone
       - Camera button for mobile
       - Thumbnail grid of uploaded screenshots
       - Click to expand/view
       - Reorder by dragging
       - Delete button on each
     
     Section 8 — "Options" (collapsible, only visible when asset_class = 'option'):
       - Strike price
       - Expiration date
       - Option type (Call / Put)
       - Implied volatility at entry
       - Greeks at entry (Delta, Gamma, Theta, Vega — all number inputs)
   
   CHECKBOX: "This is a missed trade" — when checked:
   - Entry/exit fields are optional
   - A note appears: "Log the setup you saw but didn't take"
   - Outcome field appears: "What happened?" (Winner / Loser / Unclear)

   BEHAVIOR:
   - Auto-calculations run on every relevant field change via $derived or $effect:
     - R-multiple updates when entry, exit, or stop changes
     - P&L updates when entry, exit, size, commissions change
     - P&L % updates when P&L and entry change
     - Duration updates when entry_time and exit_time change
   - Tab order follows logical trade logging flow
   - Enter key in the last field of a section moves focus to the next section
   - All number inputs accept keyboard entry and scroll-wheel adjustment
   - Form state is preserved in localStorage (auto-save draft) — if the user navigates away and comes back, the form restores. Clear draft on successful submit.
   - Validation runs on blur and on submit:
     - Symbol is required
     - Asset class is required
     - Direction is required
     - Prices must be > 0 if provided
     - Exit time must be after entry time if both provided
     - Conviction must be 1-5 if provided
     - Show inline errors below each field
   
   SUBMIT:
   - "Save Trade" primary button (always visible, sticky on mobile)
   - "Save & Add Another" secondary button (saves and resets form)
   - Loading state during submit
   - Success toast on save
   - Redirect to trade detail page on "Save Trade"
   - Reset form on "Save & Add Another"

2. apps/web/src/lib/components/trade/symbol-input.svelte — Smart symbol input:
   - Text input that auto-uppercases
   - Shows recent symbols the user has traded (from API or local cache)
   - No external API for symbol lookup (user types what they want)
   - Debounced input with clear button

3. apps/web/src/lib/components/trade/conviction-picker.svelte — Conviction score (1-5):
   - 5 numbered buttons in a row
   - Selected button is highlighted with conviction color from design system
   - Hover shows tooltip with user's historical win rate for that conviction level (if data exists)

4. apps/web/src/lib/components/trade/grade-picker.svelte — Trade grade selector:
   - 4 buttons: A, B, C, D
   - Each styled with grade color from design system (gold, blue, orange, red)
   - Single select

5. apps/web/src/lib/components/trade/tag-picker.svelte — Tag selection:
   - Shows all user tags grouped by category with category headers
   - Each tag is a pill/chip that toggles on click
   - Search input at top filters tags
   - "Create new tag" button at bottom opens inline form (name, category, color)
   - Selected tags shown at top with X to remove
   - Count of selected tags shown in section header

6. apps/web/src/lib/components/trade/media-upload.svelte — Screenshot/media upload:
   - Drag-and-drop zone with dashed border and upload icon
   - Click to open file picker
   - Camera button (uses navigator.mediaDevices.getUserMedia on mobile)
   - Accepts: image/png, image/jpeg, image/webp (max 10MB each)
   - Shows upload progress bar per file
   - After upload: thumbnail grid with:
     - Thumbnail preview
     - File name
     - Delete button (with confirmation)
     - Drag handle for reordering
   - Uploads go to Rust API /api/v1/media/upload endpoint
   - Returns storage URL that gets attached to the trade on save

7. apps/web/src/lib/components/trade/voice-note.svelte — Voice recording:
   - "Record" button with microphone icon
   - While recording: timer counting up, waveform visualization, "Stop" button
   - After recording: playback controls (play/pause, timeline scrubber, duration)
   - "Re-record" and "Delete" options
   - Uses MediaRecorder API
   - Saves as webm/opus
   - Uploads to storage on form submit
   - On mobile: large touch target for the record button

8. apps/web/src/routes/(app)/trades/new/+page.svelte — New trade page:
   - Renders the trade-form component
   - Sets page title: "Log Trade"
   - Handles form submission: calls API, shows toast, navigates

9. apps/web/src/routes/(app)/trades/[id]/edit/+page.svelte — Edit trade page:
   - Loads existing trade data from API
   - Passes trade data to trade-form as initial values
   - Sets page title: "Edit Trade"
   - Handles update submission

10. apps/web/src/lib/services/trades.ts — Frontend trade service:
    - createTrade(data: TradeCreateRequest): Promise<Trade>
    - updateTrade(id: string, data: TradeUpdateRequest): Promise<Trade>
    - uploadMedia(tradeId: string, file: File, mediaType: string): Promise<TradeMedia>
    - deleteMedia(mediaId: string): Promise<void>
    - All calls go through the typed API client to the Rust backend
    - Error handling with user-friendly messages

CRITICAL REQUIREMENTS:
- The form must feel FAST — no lag on input, no delay on auto-calculations
- Auto-calculations use $derived runes, NOT $effect (derived values, not side effects)
- Number inputs must handle locale-specific decimal separators
- The form must work flawlessly on mobile — stacked layout, large touch targets, proper keyboard types (numeric for prices, datetime for times)
- Draft auto-save to localStorage must not cause performance issues (debounce saves, not on every keystroke)
- Media uploads must work independently of form submit — files upload immediately on selection, URLs are collected and attached to the trade on save
- Voice recording must handle permission denial gracefully ("Microphone access is required for voice notes")
- The "missed trade" toggle must hide/show relevant fields smoothly with animation
- Options-specific fields must only appear when asset_class === 'option'
- Tab index must be logical: Symbol → Asset Class → Direction → Entry Price → Entry Time → Entry Size → Exit Price → Exit Time → Stop Loss → Take Profit → ...
- Form validation with Zod schema matching the Rust validation rules exactly

OUTPUT: Complete file contents for every file. Every component must be production-ready.
```

---

### PROMPT 1.4 — Trade List View

```
[PASTE GLOBAL CONTEXT BLOCK + STACK OVERRIDE]

PHASE 1 — CORE TRADE LOGGING
PROMPT 1.4 — Trade List View

OBJECTIVE: Build the trade list — the primary view where traders see all their logged trades. Features sortable columns, filtering, pagination, batch actions, and a clean tabular layout with key metrics visible at a glance.

DEPENDENCIES: Prompt 1.1 (types), Prompt 1.2 (API endpoints), Prompt 1.3 (trade form exists for "new trade" action)

DELIVERABLES:

1. apps/web/src/routes/(app)/trades/+page.svelte — Trade list page:
   - Page title: "Trades"
   - Top action bar:
     - "Log Trade" primary button (links to /trades/new)
     - "Import CSV" secondary button (links to import flow — built in later prompt)
     - Filter toggle button
     - View toggle: Table view / Card view (default: table on desktop, cards on mobile)
   - Filter panel (collapsible, animated slide-down):
     - Date range picker (from/to)
     - Symbol search input
     - Asset class multi-select pills
     - Direction pills (All / Long / Short)
     - Status pills (All / Open / Closed)
     - Tags multi-select
     - Setup name select
     - Grade pills (All / A / B / C / D)
     - Conviction range (1-5 slider or checkboxes)
     - Market regime select
     - P&L range (min/max number inputs)
     - "Show archived" toggle
     - "Clear all filters" button
     - Active filter count shown on the filter toggle button
   - Summary stats bar (above the table):
     - Total trades (matching filters), Win rate, Total P&L, Avg R, Profit Factor
     - These update when filters change
     - Styled as compact KPI pills

2. apps/web/src/lib/components/trade/trade-table.svelte — Table view:
   - Columns:
     - Date/Time (entry_time, formatted relative: "Today 9:45 AM", "Yesterday", "Jan 15")
     - Symbol (bold, with asset class icon)
     - Direction (Long/Short badge, green/red)
     - Entry (price)
     - Exit (price)
     - Size
     - P&L (dollar amount, green/red, bold)
     - P&L % (green/red)
     - R-Multiple (with color intensity based on value)
     - Grade (A/B/C/D badge with grade colors)
     - Tags (first 2-3 as small pills, "+N more" if overflow)
     - Status (Open/Closed badge)
   - Sortable columns: click header to sort, click again to reverse, arrow indicator
   - Row click navigates to trade detail page
   - Row hover: subtle background highlight
   - Alternating row backgrounds: none (flat design, use borders)
   - Missed trades shown with a distinct visual treatment (slightly faded, "MISSED" badge)
   - Archived trades (if shown): strikethrough or faded styling
   - Sticky header row on scroll
   - Responsive: on small screens, hide less important columns (Size, R-Multiple, Tags)

3. apps/web/src/lib/components/trade/trade-card.svelte — Card view (mobile-friendly):
   - Each trade as a card showing:
     - Symbol + Direction badge (top left)
     - P&L prominently displayed (top right, large, green/red)
     - Date + Time
     - Entry → Exit price with arrow
     - Tags as small pills
     - Grade badge
     - Conviction dots
   - Cards in a vertical list on mobile, 2-column grid on tablet, 3-column on desktop
   - Tap navigates to trade detail

4. apps/web/src/lib/components/trade/trade-filters.svelte — Filter panel component:
   - Encapsulates all filter inputs
   - Emits filter changes via callback prop
   - Remembers last-used filters in localStorage
   - "Clear all" resets to defaults
   - Filter count for the toggle button

5. apps/web/src/lib/components/trade/trade-summary-bar.svelte — Quick stats:
   - Compact horizontal bar above the trade list
   - Shows: trade count, win rate, total P&L, avg R, profit factor
   - Values computed from current filtered results
   - Each stat is a small card/pill with label above and value below
   - P&L colored green/red
   - Win rate shows as percentage with small bar indicator

6. apps/web/src/lib/stores/trade-list.svelte.ts — Trade list state management:
   - Svelte 5 runes
   - $state: trades, filters, sort, pagination, isLoading, error, viewMode
   - $derived: filteredCount, hasMore, summaryStats
   - Methods: loadTrades(), loadMore(), setFilter(key, value), clearFilters(), setSort(field, direction), toggleView()
   - On filter/sort change: resets to page 1, reloads from API
   - Infinite scroll support: loadMore() appends next page
   - URL sync: filters and sort reflected in URL query params (bookmarkable, shareable)

7. Pagination:
   - Infinite scroll on mobile (load more as user scrolls near bottom)
   - Page numbers + prev/next on desktop (at bottom of table)
   - Show "Showing 1-50 of 234 trades"
   - Page size selector: 25, 50, 100

8. Empty states:
   - No trades at all: illustration + "Log your first trade" CTA
   - No trades matching filters: "No trades match your filters" + "Clear filters" button
   - Loading: skeleton rows/cards matching the layout

CRITICAL REQUIREMENTS:
- The trade list must load FAST — initial render under 200ms for 50 trades
- Sorting must be server-side (not client-side) for consistency with pagination
- Filters must be debounced — don't fire API calls on every keystroke (300ms debounce on text inputs)
- URL query params must round-trip correctly: set filters → URL updates → reload page → same filters applied
- The table must handle trades with missing data gracefully (no price = "—", no grade = empty)
- P&L display: always show 2 decimal places for dollars, 2 for percentage, 2 for R-multiple
- Symbol display: always uppercase
- Mobile card view must be the default on screens under 768px
- Keyboard: arrow keys navigate rows, Enter opens trade detail
- The "Log Trade" button must be the most visually prominent action on the page

OUTPUT: Complete file contents for every file.
```

---

### PROMPT 1.5 — Trade Detail View

```
[PASTE GLOBAL CONTEXT BLOCK + STACK OVERRIDE]

PHASE 1 — CORE TRADE LOGGING
PROMPT 1.5 — Trade Detail View

OBJECTIVE: Build the trade detail page — the comprehensive view of a single trade showing all data, screenshots, notes, tags, scaling history, and AI review access.

DEPENDENCIES: Prompt 1.3 (trade form for edit), Prompt 1.4 (trade list links here)

DELIVERABLES:

1. apps/web/src/routes/(app)/trades/[id]/+page.svelte — Trade detail page:
   - Loads trade with all relations from API (GET /api/v1/trades/:id)
   - Sets page title to symbol + date: "AAPL — Jan 15, 2026"
   - Breadcrumb: Trades > AAPL

2. apps/web/src/routes/(app)/trades/[id]/+page.ts — Page load:
   - Fetches trade with relations from API
   - Returns trade data to page
   - Handles 404 (trade not found)

3. apps/web/src/lib/components/trade/trade-detail.svelte — Main detail layout:

   HEADER SECTION:
   - Symbol (large, bold) + Direction badge (Long/Short)
   - Asset class badge
   - Status badge (Open/Closed)
   - P&L displayed prominently (large font, green/red, dollar + percentage)
   - R-Multiple displayed next to P&L
   - Action buttons: Edit, Archive, "Ask AI to Review" (primary CTA)
   - If missed trade: "MISSED TRADE" banner with distinct styling

   TRADE METRICS GRID (2-3 columns of key/value pairs):
   - Entry Price → Exit Price (with arrow)
   - Size / Quantity
   - Stop Loss | Take Profit
   - Entry Time → Exit Time
   - Duration
   - Commissions + Slippage
   - Planned Risk % | Actual Risk %
   - Setup Name
   - Conviction (1-5 with colored indicator)
   - Grade (A/B/C/D badge)
   - Market Regime
   - Session Type
   - Sector

   TABS below the metrics:
   
   Tab 1 — "Screenshots" (default if screenshots exist):
   - Gallery of uploaded screenshots
   - Click to open full-screen lightbox with zoom/pan
   - If AI analysis exists for a screenshot, show it as an overlay or side panel
   - Upload more screenshots button
   - Reorder / delete

   Tab 2 — "Notes & Thesis":
   - Pre-trade thesis (if exists) with label
   - Post-trade notes
   - Voice note player (if exists)
   - Thesis alignment score (if AI-reviewed)
   - Inline edit: click text to edit, save on blur

   Tab 3 — "Scaling History" (only visible if is_scaled):
   - Timeline/table of all trade legs
   - Each leg: type (Entry/Add/Trim/Exit), price, size, time, fees
   - Visual timeline showing scaling progression
   - Total position size at each point

   Tab 4 — "Tags":
   - All tags displayed as pills grouped by category
   - Add/remove tags inline
   - Tag picker component reused from trade form

   Tab 5 — "AI Review" (only visible if review exists or as empty state prompting review):
   - If review exists: show full AI review content
     - Overall score (large, prominent)
     - Strengths list
     - Weaknesses list
     - Key lesson
     - Actionable fixes
     - Alternative scenario
     - Execution, risk management, plan adherence sub-scores
     - "Continue conversation" button to open review chat
   - If no review: empty state with "Ask AI to Review This Trade" button (links to AI review flow — Phase 4)

   Tab 6 — "Context" (market context at time of trade):
   - VIX at entry
   - Volume at entry
   - Bid/ask spread
   - Market snapshot data (if captured): TICK, ADD, VOLD, SPY, QQQ
   - Sector performance at time

4. apps/web/src/lib/components/trade/screenshot-gallery.svelte — Screenshot viewer:
   - Thumbnail grid (responsive: 2 cols mobile, 3-4 cols desktop)
   - Click thumbnail opens lightbox
   - Lightbox: full-screen overlay, zoom/pan, arrow keys for next/prev, X to close, pinch-zoom on mobile
   - Shows AI analysis annotations if available
   - Upload button within gallery

5. apps/web/src/lib/components/trade/trade-legs-timeline.svelte — Scaling visualization:
   - Vertical timeline showing each leg chronologically
   - Each node: leg type icon, price, size, time
   - Color-coded: entries/adds green, exits/trims red
   - Running position size shown between nodes
   - Clean, visual representation of how the position was built and exited

6. apps/web/src/lib/components/trade/lightbox.svelte — Full-screen image viewer:
   - Overlay with dark backdrop
   - Image centered with zoom/pan
   - Navigation arrows for multiple images
   - Close on X, Escape, or backdrop click
   - Pinch-zoom on mobile
   - Smooth enter/exit animation

7. Navigation between trades:
   - Previous / Next trade buttons in the header (based on current list sort order)
   - Keyboard: left/right arrow keys to navigate between trades

CRITICAL REQUIREMENTS:
- The page must load with a skeleton state that matches the final layout — no layout shift
- Empty fields show "—" not blank space
- The P&L display must be the most visually prominent element on the page
- Screenshots must lazy-load thumbnails (don't load full-res until lightbox opens)
- The "Ask AI to Review" button must be visually prominent — this is the gateway to the app's killer feature
- Tab state should persist in URL hash (#screenshots, #notes, #tags) so refresh stays on the same tab
- The detail page must work for trades with minimal data (just symbol + direction) and trades with everything filled
- Print-friendly: the page should render reasonably when printed (for traders who print trade journals)

OUTPUT: Complete file contents for every file.
```

---

### PROMPT 1.6 — CSV Import & Quick-Log

```
[PASTE GLOBAL CONTEXT BLOCK + STACK OVERRIDE]

PHASE 1 — CORE TRADE LOGGING
PROMPT 1.6 — CSV Import & Quick-Log Widget

OBJECTIVE: Build the CSV trade import flow for bulk historical trade loading, and the quick-log widget for fast in-session trade capture.

DEPENDENCIES: Prompt 1.2 (trade API endpoints), Prompt 1.3 (trade form components)

DELIVERABLES:

1. apps/api/src/routes/import.rs — Rust CSV import endpoint:

   POST /api/v1/trades/import — Bulk import:
   - Accepts multipart form with CSV file
   - Parses CSV with configurable column mapping
   - Request body includes a mapping object: { symbol_column: "Symbol", entry_price_column: "Entry", ... }
   - Validates each row, collects errors per row
   - Deduplicates against existing trades (match on symbol + entry_time + entry_price)
   - Inserts valid rows in a single transaction
   - Returns: { imported: number, skipped: number, errors: [{row: number, message: string}] }
   - Limit: 5000 rows per import

   GET /api/v1/trades/import/templates — Download template CSVs:
   - Returns CSV templates for common brokers: Generic, Thinkorswim, Interactive Brokers, TradeStation
   - Each template has the correct column headers with example data rows

2. apps/web/src/routes/(app)/trades/import/+page.svelte — CSV import page:
   
   Multi-step flow:
   
   STEP 1 — Upload:
   - Drag-and-drop zone for CSV file
   - Or click to browse
   - Accepts .csv and .tsv files
   - File size limit: 10MB
   - After upload: show file name, size, row count
   
   STEP 2 — Column Mapping:
   - Preview first 5 rows of the CSV as a table
   - Each column header has a dropdown: map to trade field or "Skip"
   - Auto-detect common column names (Symbol, Entry, Exit, Size, Date, etc.)
   - Required mappings highlighted: Symbol must be mapped
   - Show sample values for each column to help user map correctly
   
   STEP 3 — Preview & Validate:
   - Show all rows that will be imported as a table
   - Rows with validation errors highlighted red with error message
   - Duplicate rows highlighted yellow with "Already exists" note
   - Counts: "Ready to import: 142, Errors: 3, Duplicates: 5"
   - User can deselect individual rows
   
   STEP 4 — Import:
   - "Import X Trades" button
   - Progress bar during import
   - Results: "Successfully imported 142 trades. 3 rows had errors. 5 duplicates skipped."
   - "View imported trades" button links to trade list filtered by import date
   
   Download template buttons at the top of the page for each broker format.

3. apps/web/src/lib/components/trade/quick-log.svelte — Quick-log dialog:
   - Triggered by the prominent "Log Trade" button in the top bar
   - Opens as a modal/dialog overlay — NOT a full page navigation
   - Minimal fields for fast entry:
     - Symbol (text, auto-uppercase, auto-focus)
     - Direction (Long/Short toggle)
     - Entry price (number)
     - Exit price (number, optional — leave blank for open trades)
     - Size (number)
     - Stop loss (number, optional)
     - Conviction (1-5 quick buttons)
   - "Save" and "Save & Log Another" buttons
   - Total time to log a trade: under 10 seconds for a basic entry
   - After save: toast notification, dialog stays open if "Log Another"
   - Keyboard: Tab through fields rapidly, Enter to save
   - All other fields (tags, notes, screenshots, etc.) can be added later by editing the trade

4. apps/web/src/lib/components/trade/quick-log-trigger.svelte — The trigger button:
   - Used in the top bar
   - Prominent accent-colored button with plus icon and "Log Trade" text
   - On mobile: floating action button (FAB) in bottom-right corner, round, accent color, always visible
   - Click opens the quick-log dialog

CRITICAL REQUIREMENTS:
- CSV parsing must handle: quoted fields, commas in values, different line endings (CRLF, LF), UTF-8 and Latin-1 encoding
- Column auto-detection must be case-insensitive and handle common variations: "Entry Price", "entry_price", "EntryPrice", "Entry"
- The import preview must not freeze the browser for large files — use web workers or chunked processing for files over 1000 rows
- Quick-log must open in under 100ms — it's the trader's most frequent interaction during live sessions
- Quick-log auto-focuses the symbol field on open
- Quick-log remembers the last-used asset class and direction for faster subsequent entries

OUTPUT: Complete file contents for every file.
```

---

## ============================================================
## PHASE 2 — ANALYTICS ENGINE
## ============================================================
## Features covered: 3.1 through 3.30 from the Features Spec
## ============================================================

### PROMPT 2.1 — Analytics Computation Engine (Rust)

```
[PASTE GLOBAL CONTEXT BLOCK + STACK OVERRIDE]

PHASE 2 — ANALYTICS ENGINE
PROMPT 2.1 — Analytics Computation Engine (Rust Backend)

OBJECTIVE: Build the core analytics computation engine in Rust. This is the math brain of the app — it takes raw trade data and computes every performance metric, time-series, distribution, and statistical measure the dashboard needs.

DEPENDENCIES: Phase 1 complete (trades exist in database)

DELIVERABLES:

1. apps/api/src/services/analytics_engine.rs — Core computation service:

   CORE METRICS (function: compute_summary):
   Input: user_id, date range, optional filters (same as trade filters)
   Output: TradeStats struct with:
   - total_trades, winners (pnl > 0), losers (pnl < 0), breakeven (pnl == 0)
   - win_rate = winners / total_trades * 100
   - avg_r_multiple = mean of all r_multiples
   - profit_factor = sum of winning pnl / abs(sum of losing pnl)
   - expectancy = (win_rate * avg_winner) - (loss_rate * avg_loser)
   - total_pnl = sum of all pnl
   - avg_pnl = mean pnl per trade
   - largest_winner = max pnl
   - largest_loser = min pnl (most negative)
   - avg_winner = mean pnl of winning trades
   - avg_loser = mean pnl of losing trades (as positive number for display)
   - max_drawdown = calculate from equity curve
   - max_drawdown_pct = max_drawdown / peak_equity * 100
   - sharpe_ratio = mean_daily_return / std_daily_return * sqrt(252)
   - sortino_ratio = mean_daily_return / downside_deviation * sqrt(252)
   - recovery_factor = total_pnl / max_drawdown
   - avg_duration_minutes = mean of all trade durations
   - avg_conviction = mean of all conviction scores (where not null)
   - avg_winner_r = mean r_multiple of winners
   - avg_loser_r = mean r_multiple of losers (absolute value)
   - consecutive_wins_max, consecutive_losses_max
   - All calculations use rust_decimal — NEVER f64 for financial math

   EQUITY CURVE (function: compute_equity_curve):
   Input: user_id, date range, filters, starting_balance (optional)
   Output: Vec<EquityPoint> where each point has:
   - date (Date)
   - cumulative_pnl (Decimal)
   - equity (starting_balance + cumulative_pnl)
   - drawdown (current equity - peak equity)
   - drawdown_pct
   - trade_count_that_day (i32)
   - daily_pnl (Decimal)
   Logic:
   - Group trades by exit_date
   - Compute running cumulative P&L
   - Track peak equity for drawdown calculation
   - Include days with no trades (carry forward previous equity, zero drawdown change)

   CALENDAR HEATMAP (function: compute_heatmap):
   Input: user_id, year, month (optional)
   Output: Vec<HeatmapDay> with:
   - date, pnl, trade_count, win_count, loss_count, best_trade_pnl, worst_trade_pnl
   Grouped by calendar day.

   R-MULTIPLE DISTRIBUTION (function: compute_r_distribution):
   Input: user_id, filters
   Output: Vec<DistributionBucket> with:
   - range_start, range_end, count, percentage
   Buckets: <-3R, -3 to -2, -2 to -1, -1 to 0, 0 to 1, 1 to 2, 2 to 3, >3R

   TIME-OF-DAY ANALYSIS (function: compute_time_analysis):
   Input: user_id, filters
   Output: Vec<TimeSlot> with:
   - hour (0-23), trade_count, win_rate, avg_pnl, total_pnl
   Grouped by hour of entry_time in user's timezone.

   DAY-OF-WEEK ANALYSIS (function: compute_dow_analysis):
   Input: user_id, filters
   Output: Vec<DayOfWeek> with:
   - day (Mon-Sun), trade_count, win_rate, avg_pnl, total_pnl

   STRATEGY BREAKDOWN (function: compute_strategy_breakdown):
   Input: user_id, filters
   Output: Vec<StrategyStats> grouped by setup_name with:
   - setup_name, trade_count, win_rate, avg_r, profit_factor, expectancy, total_pnl

   INSTRUMENT MATRIX (function: compute_instrument_matrix):
   Input: user_id, filters
   Output: Vec<InstrumentStats> grouped by symbol with:
   - symbol, trade_count, win_rate, total_pnl, avg_r, profit_factor

   SESSION SPLIT (function: compute_session_split):
   Input: user_id, filters
   Output: Vec<SessionStats> grouped by session_type with same fields as strategy breakdown

   CONVICTION CORRELATION (function: compute_conviction_analysis):
   Input: user_id
   Output: Vec<ConvictionLevel> (1-5) each with:
   - level, trade_count, win_rate, avg_r, avg_pnl

   TRADE DURATION ANALYSIS (function: compute_duration_analysis):
   Input: user_id, filters
   Output: Vec<DurationBucket> with:
   - duration_range (e.g., "0-5 min", "5-15 min", "15-30 min", etc.), trade_count, win_rate, avg_r, avg_pnl

   STREAK ANALYSIS (function: compute_streak_analysis):
   Input: user_id, filters
   Output: StreakData with:
   - current_streak (positive = wins, negative = losses)
   - max_win_streak, max_loss_streak
   - performance_after_win_streak: Vec<{streak_length, next_trade_win_rate, next_trade_avg_r}>
   - performance_after_loss_streak: same structure

   MFE/MAE (function: compute_mfe_mae):
   Note: This requires intraday price data which we may not have. For now, compute from entry/exit/stop/high/low if available, or skip if not. Define the types and stub the logic.

2. apps/api/src/routes/analytics.rs — Analytics API endpoints:

   All endpoints accept query params for date range and filters (same as trade filters).
   All scoped by authenticated user_id.

   GET /api/v1/analytics/summary — Returns TradeStats
   GET /api/v1/analytics/equity-curve — Returns Vec<EquityPoint>
   GET /api/v1/analytics/heatmap?year=2026&month=1 — Returns Vec<HeatmapDay>
   GET /api/v1/analytics/r-distribution — Returns Vec<DistributionBucket>
   GET /api/v1/analytics/time-analysis — Returns Vec<TimeSlot>
   GET /api/v1/analytics/dow-analysis — Returns Vec<DayOfWeek>
   GET /api/v1/analytics/strategy-breakdown — Returns Vec<StrategyStats>
   GET /api/v1/analytics/instrument-matrix — Returns Vec<InstrumentStats>
   GET /api/v1/analytics/session-split — Returns Vec<SessionStats>
   GET /api/v1/analytics/conviction-analysis — Returns Vec<ConvictionLevel>
   GET /api/v1/analytics/duration-analysis — Returns Vec<DurationBucket>
   GET /api/v1/analytics/streak-analysis — Returns StreakData
   GET /api/v1/analytics/benchmark?index=spy — Returns benchmark comparison data

   Response caching: each endpoint checks analytics_cache table first. If cache exists and is not expired (1 hour TTL), return cached. Otherwise compute, cache, and return.

3. apps/api/src/models/analytics.rs — All response types:
   - Every struct listed above with proper Serialize derives
   - All financial fields as Decimal
   - All dates as NaiveDate or DateTime<Utc>

4. Unit tests in apps/api/src/services/analytics_engine_tests.rs:
   - Test compute_summary with known trade sets (verify win rate, profit factor, expectancy, etc.)
   - Test equity curve with drawdown calculations
   - Test edge cases: zero trades, one trade, all winners, all losers, trades with no exit price
   - At minimum 30 test cases

CRITICAL REQUIREMENTS:
- ALL financial math uses rust_decimal — NO f64 ANYWHERE in this file
- Division by zero must be handled (zero trades, zero losing trades for profit factor, etc.)
- Null/missing data must be handled gracefully (trades without r_multiple, without conviction, etc.)
- Queries must be efficient — use SQL aggregation where possible instead of loading all trades into memory
- For large accounts (10,000+ trades), computation must complete in under 2 seconds
- Cache invalidation: when a trade is created/updated/deleted, invalidate relevant cache entries
- The analytics engine is the most performance-critical code in the entire app — optimize aggressively

OUTPUT: Complete file contents for every file.
```

---

### PROMPT 2.2 — Analytics Dashboard (Frontend)

```
[PASTE GLOBAL CONTEXT BLOCK + STACK OVERRIDE]

PHASE 2 — ANALYTICS ENGINE
PROMPT 2.2 — Analytics Dashboard Frontend (Core Metrics + Equity Curve + Heatmap)

OBJECTIVE: Build the main analytics dashboard page with the core metrics display, interactive equity curve chart, and P&L calendar heatmap. This is the first set of analytics visualizations — additional charts come in subsequent prompts.

DEPENDENCIES: Prompt 2.1 (analytics API), Phase 0 (design system, ECharts theme)

DELIVERABLES:

1. apps/web/src/routes/(app)/analytics/+page.svelte — Analytics dashboard page:
   - Page title: "Analytics"
   - Global filter bar at top:
     - Date range presets: Today, This Week, This Month, This Quarter, This Year, All Time, Custom
     - Custom date range picker (from/to)
     - Asset class filter pills
     - Setup name filter
     - Tag filter
     - Active filters persist in URL query params
   - Below filters: tab navigation for different analytics views:
     - Overview (default — core metrics + equity curve + heatmap)
     - Strategies
     - Time Analysis
     - Instruments
     - Advanced (Monte Carlo, MFE/MAE, correlations — later prompts)
   - Loading state: skeleton cards + skeleton chart matching layout
   - Empty state: "Log some trades to see your analytics" with CTA

2. apps/web/src/lib/components/analytics/analytics-filters.svelte — Filter bar:
   - Date range presets as pills (clickable, one active at a time)
   - Custom date range with calendar popover
   - Additional filter dropdowns
   - "Reset" button
   - Compact layout: all filters in one horizontal row on desktop, scrollable on mobile

3. apps/web/src/lib/components/dashboard/stat-card.svelte — KPI stat card:
   - Props: label, value, format (number/currency/percentage/r-multiple), trend (up/down/flat with delta value), size (sm/md/lg)
   - Displays: label above, value large and bold below, optional trend indicator with color
   - Uses design system typography: value in font-mono (JetBrains Mono) for numbers
   - Currency format: $1,234.56 with green/red coloring for P&L
   - Percentage format: 62.5% with optional color
   - R-multiple format: 1.45R

4. apps/web/src/lib/components/analytics/metrics-grid.svelte — Core metrics display:
   - Grid of stat-card components showing:
     Row 1: Total Trades | Win Rate | Profit Factor | Expectancy
     Row 2: Total P&L | Avg R-Multiple | Sharpe Ratio | Max Drawdown
     Row 3: Avg Winner | Avg Loser | Largest Winner | Largest Loser
     Row 4: Avg Duration | Recovery Factor | Sortino Ratio | Consecutive Wins/Losses
   - Responsive: 4 columns desktop, 2 columns tablet, 1 column mobile
   - Each card has a tooltip explaining the metric on hover
   - P&L values are green/red
   - Win rate shows a small progress bar inside the card
   - Max drawdown shows in red

5. apps/web/src/lib/components/analytics/equity-curve.svelte — Equity curve chart:
   - Built with Apache ECharts
   - Uses the ECharts dark theme configured in Phase 0 design system
   - Main line: cumulative P&L (or equity if starting balance set)
   - Drawdown area: shaded red area below the equity line showing drawdown depth
   - Benchmark overlay: optional SPY line (toggle on/off)
   - Interactive:
     - Zoom: mouse wheel / pinch to zoom into date ranges
     - Pan: drag to pan along timeline
     - Hover tooltip: shows date, equity value, daily P&L, drawdown, trade count
     - Click a point: links to that day's trades
   - Toggle buttons above chart: $ P&L / % Return / R-Multiple cumulative
   - Time axis: auto-scales labels (days for short ranges, months for long ranges)
   - Responsive: full width, height 300px mobile / 400px desktop
   - Loading: skeleton rectangle matching chart dimensions

6. apps/web/src/lib/components/analytics/pnl-heatmap.svelte — Calendar heatmap:
   - Built with Apache ECharts (calendar heatmap chart type)
   - Monthly calendar grid: each day is a colored cell
   - Color scale: deep red (big loss) → light red → gray (no trades/breakeven) → light green → deep green (big win)
   - Hover: tooltip showing date, P&L, trade count, win rate for that day
   - Click: navigates to trade list filtered for that day
   - Month/year navigation arrows
   - Current day highlighted with border
   - Days with no trades: neutral gray
   - Shows month labels and day-of-week labels (M, T, W, T, F, S, S)
   - Responsive: scales down on mobile, remains readable

7. apps/web/src/lib/services/analytics.ts — Frontend analytics service:
   - getSummary(filters): fetch core metrics
   - getEquityCurve(filters): fetch equity curve data
   - getHeatmap(year, month, filters): fetch heatmap data
   - getRDistribution(filters): fetch R distribution
   - getTimeAnalysis(filters): fetch time-of-day data
   - getDowAnalysis(filters): fetch day-of-week data
   - getStrategyBreakdown(filters): fetch strategy stats
   - getInstrumentMatrix(filters): fetch instrument data
   - getSessionSplit(filters): fetch session data
   - getConvictionAnalysis(filters): fetch conviction data
   - getDurationAnalysis(filters): fetch duration data
   - getStreakAnalysis(filters): fetch streak data
   - All functions typed with request params and response types
   - Error handling on every call

8. apps/web/src/lib/stores/analytics.svelte.ts — Analytics state:
   - $state: filters, summary, equityCurve, heatmap, isLoading, error, activeTab
   - Methods: loadOverview(), setDateRange(), setFilter(), clearFilters()
   - On filter change: reload all visible data
   - Cache results client-side for the session (don't re-fetch if filters haven't changed)

9. apps/web/src/lib/utils/chart-theme.ts — ECharts theme builder:
   - Function that returns a complete ECharts theme object matching the design system
   - Colors: profit green, loss red, accent blue, all background/text colors
   - Font: Inter for labels, JetBrains Mono for values
   - Grid lines: subtle, using border colors from design system
   - Tooltip: dark background, proper padding, rounded corners
   - Legend: positioned top-right, text in secondary color
   - All chart components in the app use this theme

CRITICAL REQUIREMENTS:
- ECharts must be lazy-loaded (dynamic import) to avoid blocking initial page load
- Charts must resize on window resize and sidebar toggle — use ResizeObserver
- All chart colors must come from the design system — no hardcoded hex values in chart configs
- The equity curve must handle gaps (weekends, days with no trades) correctly
- The heatmap must handle months with different day counts correctly
- Number formatting must be consistent: use Intl.NumberFormat for all displayed values
- The dashboard must feel responsive — filters update charts within 500ms (show loading indicator if API takes longer)
- Mobile: charts stack vertically, full-width, touch-optimized tooltips

OUTPUT: Complete file contents for every file.
```

---

### PROMPT 2.3 — Analytics: Strategy, Time, Instrument, Session Charts

```
[PASTE GLOBAL CONTEXT BLOCK + STACK OVERRIDE]

PHASE 2 — ANALYTICS ENGINE
PROMPT 2.3 — Analytics Charts: Strategy Breakdown, Time Analysis, Instruments, Sessions

OBJECTIVE: Build the remaining core analytics chart views — strategy performance breakdown, time-of-day/day-of-week analysis, instrument performance matrix, session split, conviction analysis, duration analysis, streak awareness, and R-distribution histogram.

DEPENDENCIES: Prompt 2.1 (API endpoints), Prompt 2.2 (analytics page structure, chart theme, services)

DELIVERABLES:

1. apps/web/src/lib/components/analytics/strategy-breakdown.svelte:
   - Bar chart (horizontal): each setup_name as a bar, length = profit factor or total P&L (toggle)
   - Color: green for positive profit factor, red for negative
   - Table below chart: setup_name, trade count, win rate, avg R, profit factor, total P&L
   - Sort table by any column
   - Click a strategy row to navigate to trade list filtered by that setup

2. apps/web/src/lib/components/analytics/time-heatmap.svelte:
   - Heatmap: X-axis = hour of day (6AM-8PM), Y-axis = day of week (Mon-Fri)
   - Cell color = P&L for that hour+day combination
   - Hover: tooltip with trade count, win rate, avg P&L for that slot
   - Below heatmap: separate bar charts for:
     - P&L by hour of day
     - P&L by day of week
   - Toggle: show P&L, win rate, or trade count

3. apps/web/src/lib/components/analytics/instrument-matrix.svelte:
   - Grid/table of all traded symbols
   - Each cell: symbol name, colored background (green gradient for profitable, red for unprofitable)
   - Size of cell proportional to trade count
   - Hover: detailed stats (trade count, win rate, P&L, avg R, profit factor)
   - Sort by: P&L, win rate, trade count, profit factor
   - View toggle: grid (treemap style) or table
   - Click symbol to filter trade list by that symbol

4. apps/web/src/lib/components/analytics/session-split.svelte:
   - Stacked bar chart or grouped bar: one group per session type
   - Each group shows: win rate, avg R, total P&L
   - Table below with full stats per session
   - Highlight the best-performing and worst-performing sessions

5. apps/web/src/lib/components/analytics/conviction-chart.svelte:
   - Bar chart: conviction levels 1-5 on X-axis
   - Dual Y-axis: win rate (line) and trade count (bars)
   - Color bars by conviction color from design system
   - Below: calibration score — "Your conviction accuracy: X%"
   - Shows expected vs actual win rate per conviction level

6. apps/web/src/lib/components/analytics/duration-chart.svelte:
   - Scatter plot or bar chart: X-axis = hold duration buckets, Y-axis = avg R or P&L
   - Buckets: <1 min, 1-5 min, 5-15 min, 15-30 min, 30-60 min, 1-2 hr, 2-4 hr, 4+ hr, overnight
   - Highlight the optimal duration range (highest avg R)
   - Annotation: "Your sweet spot: 15-30 minutes"

7. apps/web/src/lib/components/analytics/r-distribution.svelte:
   - Histogram: R-multiple buckets on X-axis, trade count on Y-axis
   - Green bars for positive R, red for negative
   - Statistical markers overlaid: mean (vertical line), median, +/- 1 std dev
   - Normal distribution curve overlay (optional toggle)
   - Annotation with skewness value and what it means

8. apps/web/src/lib/components/analytics/streak-chart.svelte:
   - Current streak display: "You're on a 5-win streak" with appropriate styling
   - Chart: performance after N consecutive wins/losses
   - X-axis: streak length (1-10+), Y-axis: next trade win rate
   - Two lines: after win streaks, after loss streaks
   - Insight text: "After 3+ consecutive wins, your win rate drops to X%"

9. apps/web/src/lib/components/analytics/asymmetric-risk-alert.svelte:
   - Prominent card/banner displayed when avg loser > avg winner
   - Shows: avg winner, avg loser, ratio
   - Visual: two bars side by side (winner green, loser red)
   - Text: warning message explaining the imbalance
   - Hidden when risk is balanced (avg winner >= avg loser)

10. Update analytics page tabs to include all new views:
    - Overview: metrics grid + equity curve + heatmap
    - Strategies: strategy breakdown chart + table
    - Time: time heatmap + day-of-week + session split
    - Instruments: instrument matrix
    - Performance: conviction + duration + R-distribution + streaks
    - Each tab lazy-loads its data on first visit

CRITICAL REQUIREMENTS:
- Every chart uses the centralized ECharts theme — no inline color overrides
- Every chart is responsive and resizes correctly
- Every chart has loading skeleton and empty state
- Every chart tooltip shows precise formatted numbers
- Charts must handle small data sets gracefully (1-5 trades should still render something meaningful)
- Click interactions on charts should navigate to filtered trade lists where applicable
- The "Strategies" tab should show "Name your setups for richer analysis" if no setup_name data exists
- All charts lazy-load ECharts via dynamic import

OUTPUT: Complete file contents for every file.
```

---

### PROMPT 2.4 — Monte Carlo, Benchmark, Volatility-Adjusted, Advanced Analytics

```
[PASTE GLOBAL CONTEXT BLOCK + STACK OVERRIDE]

PHASE 2 — ANALYTICS ENGINE
PROMPT 2.4 — Advanced Analytics: Monte Carlo, Benchmark, Volatility-Adjusted P&L, Overnight, Catalyst, Commission Analysis

OBJECTIVE: Build the advanced analytics features — Monte Carlo simulation, benchmark comparison, volatility-adjusted P&L, overnight risk analysis, catalyst P&L isolation, commission/slippage tracking, fatigue tracking, and correlation analysis. These are the features that push the app into institutional-grade territory.

DEPENDENCIES: Prompt 2.1 (analytics engine), Prompt 2.2/2.3 (analytics page)

DELIVERABLES:

1. apps/api/src/services/advanced_analytics.rs — Advanced computation:

   MONTE CARLO (function: run_monte_carlo):
   - Input: user's trade R-multiples (Vec<Decimal>), num_simulations (10000), num_trades_forward (100-500)
   - Logic: randomly resample (with replacement) from actual R-multiples, build simulated equity curves
   - Output: MonteCarloResult with:
     - percentile curves: p5, p25, p50 (median), p75, p95 — each as Vec<Decimal> (cumulative R at each trade number)
     - max_drawdown_distribution: Vec<Decimal> (max drawdown for each simulation)
     - blowup_probability: Decimal (% of simulations that hit a defined loss threshold, e.g., -50%)
     - expected_cagr_range: (p25_cagr, p50_cagr, p75_cagr)
   - Must be performant: 10,000 simulations of 500 trades each should complete in under 3 seconds
   - Use rand crate for random sampling

   VOLATILITY-ADJUSTED P&L (function: compute_vol_adjusted):
   - Input: user_id, date range
   - For each trade: divide P&L by VIX at entry (or ATR if VIX not available)
   - Output: Vol-adjusted equity curve alongside raw equity curve
   - Handles missing VIX data gracefully (skip normalization for those trades)

   OVERNIGHT ANALYSIS (function: compute_overnight_analysis):
   - Filter trades where held_overnight = true
   - Output: OvernightStats:
     - total_overnight_holds, gap_favorable, gap_against
     - total_overnight_pnl, avg_gap_pct
     - overnight_pnl vs intraday_pnl comparison

   CATALYST ANALYSIS (function: compute_catalyst_analysis):
   - Separate trades into catalyst-adjacent and normal
   - Use economic_events table to determine if a trade occurred within 30 min of a high-impact event
   - Output: catalyst P&L vs normal P&L comparison stats

   COMMISSION ANALYSIS (function: compute_commission_impact):
   - Total commissions, total slippage, commissions as % of gross P&L
   - Gross P&L (before fees) vs Net P&L (after fees)
   - Commission per trade average

   FATIGUE ANALYSIS (function: compute_fatigue_curve):
   - Group trades by "hour into session" (relative to user's first trade each day)
   - Output: Vec<FatigueSlot> with hour_into_session, trade_count, win_rate, avg_pnl

   CORRELATION ANALYSIS (function: compute_trade_correlation):
   - Find trades that overlap in time (concurrent positions)
   - Compare performance when single vs multiple positions open
   - Output: single_position_stats vs multi_position_stats

2. apps/api/src/routes/analytics.rs — Add endpoints:
   POST /api/v1/analytics/monte-carlo (POST because it has config params in body)
   GET /api/v1/analytics/volatility-adjusted
   GET /api/v1/analytics/overnight
   GET /api/v1/analytics/catalyst
   GET /api/v1/analytics/commissions
   GET /api/v1/analytics/fatigue
   GET /api/v1/analytics/correlation
   GET /api/v1/analytics/benchmark?index=spy

3. Frontend charts:

   apps/web/src/lib/components/analytics/monte-carlo-chart.svelte:
   - Fan chart: X-axis = trade number, Y-axis = cumulative R
   - 5 percentile bands (p5-p95) as shaded areas with decreasing opacity
   - Median line prominent
   - Controls: adjust simulation count, forward trades
   - Text below: "5% chance of X drawdown", "Median outcome after 100 trades: +Y R"

   apps/web/src/lib/components/analytics/vol-adjusted-curve.svelte:
   - Dual-line chart: raw equity curve + volatility-adjusted curve
   - Toggle between them or overlay
   - Annotation showing where the curves diverge most

   apps/web/src/lib/components/analytics/overnight-analysis.svelte:
   - Split view: overnight stats vs intraday stats
   - Bar chart comparing P&L, win rate
   - Gap distribution histogram

   apps/web/src/lib/components/analytics/commission-tracker.svelte:
   - Pie chart: gross P&L split into net P&L + commissions + slippage
   - Running total line chart: cumulative commission drag
   - Stat cards: total fees, fees as % of gross, avg fee per trade

   apps/web/src/lib/components/analytics/fatigue-curve.svelte:
   - Line chart: X = hours into session, Y = avg P&L or win rate
   - Annotation marking the "performance cliff" point
   - Recommendation text based on data

   apps/web/src/lib/components/analytics/benchmark-comparison.svelte:
   - Dual-line chart: trader's equity curve vs benchmark (SPY default)
   - Stat cards: trader Sharpe vs benchmark Sharpe, alpha, beta
   - Benchmark selector: SPY, QQQ, BTC

4. Update analytics page "Advanced" tab to include all new charts:
   - Monte Carlo, Vol-Adjusted, Overnight, Catalyst, Commissions, Fatigue, Benchmark, Correlation
   - Each section collapsible with explanation header

CRITICAL REQUIREMENTS:
- Monte Carlo must use proper random sampling — use a seeded RNG for reproducibility (same seed = same results for debugging)
- Monte Carlo visualization must not transmit 10,000 full curves to the frontend — compute percentile bands server-side, send only the 5 band curves
- Financial math: rust_decimal only
- Benchmark comparison needs historical SPY/QQQ data — for now, stub with a note that this requires a market data feed. Define the types and chart, use mock data for development
- The "Advanced" tab should show explanatory text for each visualization — these are complex concepts that need brief explanations for intermediate traders

OUTPUT: Complete file contents for every file.
```

---

## ============================================================
## PHASE 3 — DAILY PLANNING MODULE
## ============================================================
## Features covered: 4.1 through 4.11 from the Features Spec
## ============================================================

### PROMPT 3.1 — Planning Rust API + Frontend

```
[PASTE GLOBAL CONTEXT BLOCK + STACK OVERRIDE]

PHASE 3 — DAILY PLANNING MODULE
PROMPT 3.1 — Complete Daily Planning System (API + Frontend)

OBJECTIVE: Build the entire daily planning module — Rust API endpoints for plans and watchlist items, and the complete frontend for creating, viewing, and managing daily trading plans including the pre-trade checklist, watchlist builder, economic calendar, and plan continuity features.

DEPENDENCIES: Phase 0 (database tables for daily_plans, watchlist_items), Phase 1 (trades exist for plan-to-trade linkage)

DELIVERABLES:

1. apps/api/src/routes/plans.rs — Plan CRUD:

   GET /api/v1/plans — List plans (paginated, sorted by date desc)
   GET /api/v1/plans/today — Get today's plan (create empty one if none exists)
   GET /api/v1/plans/:date — Get plan for specific date (YYYY-MM-DD)
   POST /api/v1/plans — Create plan
   PUT /api/v1/plans/:id — Update plan
   POST /api/v1/plans/:id/complete — Mark plan completed, trigger adherence scoring
   POST /api/v1/plans/:id/carry-forward — Create tomorrow's plan from unfinished items

   Plan create/update body:
   - market_bias, bias_reasoning, session_goals, max_trades, max_daily_loss
   - checklist_items: [{text, required, checked}]
   - notes

   GET /api/v1/plans/:id/watchlist — List watchlist items for a plan
   POST /api/v1/plans/:id/watchlist — Add watchlist item
   PUT /api/v1/plans/:id/watchlist/:item_id — Update watchlist item
   DELETE /api/v1/plans/:id/watchlist/:item_id — Remove watchlist item
   PUT /api/v1/plans/:id/watchlist/reorder — Reorder watchlist items (body: ordered IDs)

   POST /api/v1/plans/:id/adherence — Calculate and save adherence score:
   - Compare plan's watchlist to actual trades taken that day
   - Check how many checklist items were honored
   - Check if max_trades was respected
   - Check if max_daily_loss was respected
   - Score 0-100, save to plan record plus AI explanation

2. apps/api/src/models/plan.rs — Rust models:
   - DailyPlan, DailyPlanCreate, DailyPlanUpdate
   - WatchlistItem, WatchlistItemCreate, WatchlistItemUpdate
   - ChecklistItem { text: String, required: bool, checked: bool }
   - PlanAdherenceResult { score: Decimal, details: serde_json::Value }
   - KeyLevel { price: Decimal, level_type: String, notes: String }

3. packages/types/src/plan.ts — TypeScript types matching Rust models exactly

4. apps/web/src/routes/(app)/plan/+page.svelte — Daily plan page:
   - Default shows today's plan
   - Date navigator at top: < Yesterday | Today (bold) | Tomorrow >
   - Calendar icon to jump to any date
   - If no plan exists for today: "Create Today's Plan" button or auto-create empty plan

5. apps/web/src/lib/components/planning/plan-editor.svelte — Main plan editor:
   
   LAYOUT — Vertical sections, each collapsible:
   
   Section 1 — "Market Bias":
   - Three large toggle buttons: Bullish (green), Bearish (red), Neutral (gray)
   - Bias reasoning textarea below: "Why?"
   - AI suggestion button: "Get AI perspective" (calls AI Plan of Attack — Phase 4, stub for now)
   
   Section 2 — "Session Goals":
   - Dynamic list of goals (text inputs)
   - Add goal button
   - Remove goal (X button)
   - Pre-populated defaults from user profile if first plan
   - Max trades (number input, default from profile)
   - Max daily loss (currency input, default from profile)
   
   Section 3 — "Pre-Trade Checklist":
   - Dynamic list of checklist items
   - Each item: checkbox + text + required toggle (star icon)
   - Add item button
   - Default items loaded from user profile settings:
     - "Is this an A+ setup?"
     - "Am I following my plan?"
     - "Is risk within my limits?"
     - "Did I check the economic calendar?"
   - User can customize, add, remove items
   - Customized checklist saves as the user's default for future plans
   
   Section 4 — "Watchlist":
   - Vertical list of watchlist items, each as an expandable card
   - Add watchlist item button at bottom
   
   Each watchlist card (watchlist-item.svelte):
   - Symbol input (large, prominent, auto-uppercase)
   - Key levels: dynamic list of {price, type (support/resistance/supply/demand), notes}
     - Add level button
     - Each level: price input + type dropdown + notes input
   - Catalysts (textarea): "Upcoming events, news..."
   - Setup description (textarea): "What setup am I watching for?"
   - Risk/Reward ratio (number input)
   - Position size suggested (number input)
   - Collapse/expand toggle
   - Delete button
   - Drag handle for reordering
   - After market: "Was this traded?" toggle and outcome selector (winner/loser/missed/no setup)
   
   Section 5 — "Notes":
   - Free-form textarea for additional thoughts

   ACTIONS:
   - Auto-save: plan saves on every field change (debounced 1s)
   - "AI Plan of Attack" button (stub — implemented in Phase 4): sends plan context to AI, receives formatted brief
   - "Mark Complete" button at bottom: triggers adherence calculation, marks plan complete
   - "Carry Forward" button: copies unfinished watchlist items and unmet goals to tomorrow's plan

6. apps/web/src/lib/components/planning/economic-calendar.svelte — Calendar widget:
   - Displays today's economic events in a compact list
   - Each event: time, title, impact (high=red dot, medium=yellow, low=gray), currency
   - Data source: for now, stub with placeholder data. Define the types and UI. Full integration (fetching real data) comes in a later phase.
   - Sorted by time
   - High-impact events highlighted with colored left border

7. apps/web/src/lib/components/planning/plan-adherence.svelte — End-of-day adherence display:
   - Large score number (0-100) with color (green >80, yellow 50-80, red <50)
   - Breakdown: checklist adherence %, watchlist adherence (how many planned items were actually traded), max trades respected (yes/no), max loss respected (yes/no)
   - AI explanation text
   - Displayed on completed plans

8. apps/web/src/lib/components/planning/plan-history-list.svelte — Recent plans list:
   - Shown when navigating to past dates
   - Each plan card: date, market bias badge, adherence score, trade count
   - Click to view/edit that day's plan

9. apps/web/src/lib/services/plans.ts — Frontend plan service:
   - getTodayPlan(), getPlanByDate(date), createPlan(data), updatePlan(id, data)
   - addWatchlistItem(planId, data), updateWatchlistItem(planId, itemId, data), removeWatchlistItem(planId, itemId)
   - completePlan(id), carryForward(id)
   - calculateAdherence(id)

10. apps/web/src/lib/stores/plan.svelte.ts — Plan state:
    - $state: currentPlan, watchlistItems, isLoading, isDirty, error
    - $derived: completedChecklist count, totalChecklist count
    - Auto-save debounce logic
    - Optimistic updates

CRITICAL REQUIREMENTS:
- The plan page must be the DEFAULT LANDING PAGE before market open (configurable in settings)
- Auto-save must be seamless — no save button needed for edits, just debounced background saves
- The plan must load in under 300ms
- Watchlist reordering must use drag-and-drop with smooth animations
- The checklist must be satisfying to use — checkbox animations, subtle sound option
- Plan continuity must be intelligent: only carry forward items marked as "not traded / no setup" — don't carry forward completed items
- The economic calendar must visually highlight high-impact events that could affect trading
- Mobile: everything stacks vertically, large touch targets, easy to use with one hand while reviewing charts
- The plan editor should feel like a focused workspace, not a busy form

OUTPUT: Complete file contents for every file.
```

---

## ============================================================
## PHASE 4 — AI TRADE REVIEW SYSTEM
## ============================================================
## Features covered: 5.1 through 5.14 from the Features Spec
## ============================================================

### PROMPT 4.1 — Claude API Integration (Rust)

```
[PASTE GLOBAL CONTEXT BLOCK + STACK OVERRIDE]

PHASE 4 — AI TRADE REVIEW SYSTEM
PROMPT 4.1 — Claude API Integration Service (Rust)

OBJECTIVE: Build the Rust service that communicates with the Anthropic Claude API for all AI features — trade reviews with chart screenshot analysis, trade chat follow-ups, daily plan generation, and periodic summaries. This is the AI brain of the app.

DEPENDENCIES: Phase 0 (API scaffold, auth), Phase 1 (trades), Phase 3 (plans)

DELIVERABLES:

1. apps/api/src/services/ai_engine.rs — Claude API integration service:

   struct AiEngine:
   - Fields: reqwest::Client, api_key: String, pg_pool: PgPool
   - Constructor: new(api_key, pg_pool)

   CORE METHOD — send_to_claude():
   - Input: system_prompt, user_messages (Vec<Message>), model (default claude-sonnet-4-20250514), max_tokens, temperature
   - Messages can include text and image content (base64 encoded screenshots)
   - Handles the full Anthropic Messages API request:
     - POST to https://api.anthropic.com/v1/messages
     - Headers: x-api-key, anthropic-version: 2023-06-01, content-type: application/json
     - Body: { model, max_tokens, system, messages: [{role, content}] }
     - Content can be text blocks or image blocks: { type: "image", source: { type: "base64", media_type, data } }
   - Response parsing: extract text content from response
   - Error handling: rate limits (429 with retry), API errors, timeout (60s for reviews)
   - Token tracking: extract input_tokens and output_tokens from response, return with result
   - Retry logic: 3 retries with exponential backoff on 429/5xx

   TRADE REVIEW METHOD — review_trade():
   - Input: trade_id (Uuid), user_id (Uuid)
   - Fetches: trade data, trade media (screenshots), trade tags, user profile (for AI personality), daily plan for that date (for adherence check)
   - Downloads screenshot images from S3, converts to base64
   - Builds the prompt:
     SYSTEM PROMPT (adapts based on user's ai_personality setting):
     ```
     You are an expert trading coach reviewing a trader's trade. You analyze chart screenshots, trade execution, risk management, and psychological factors.

     [IF strict_coach]: Be direct and unsparing. Call out every mistake clearly. No sugarcoating.
     [IF encouraging_mentor]: Lead with positives, then address areas for improvement constructively.
     [IF balanced]: Provide honest, balanced feedback with both praise and critique.

     Respond in this exact JSON structure:
     {
       "overall_score": <number 1-10>,
       "strengths": ["<strength 1>", "<strength 2>", ...],
       "weaknesses": ["<weakness 1>", "<weakness 2>", ...],
       "key_lesson": "<one sentence key takeaway>",
       "actionable_fixes": ["<specific fix 1>", "<specific fix 2>", ...],
       "alternative_scenario": "<what could have been done differently and estimated impact>",
       "execution_quality_score": <number 1-10>,
       "risk_management_score": <number 1-10>,
       "plan_adherence_score": <number 1-10>,
       "emotional_state_detected": "<calm|confident|anxious|frustrated|overconfident|revenge_trading|fomo|disciplined>",
       "thesis_alignment_score": <number 1-5 or null if no thesis provided>,
       "chart_analysis": "<description of what the AI sees in the chart screenshots>"
     }
     ```

     USER MESSAGE:
     - Trade data formatted as structured text (symbol, direction, entry/exit prices, P&L, R-multiple, duration, tags, setup name, thesis, notes, conviction, grade, market regime, session type)
     - Daily plan context (if plan exists for that day): market bias, watchlist, goals
     - Screenshots attached as image content blocks
     - "Please review this trade thoroughly."

   - Parses Claude's JSON response into AiReviewResult struct
   - Saves to ai_reviews table (trade_id, user_id, all score fields, raw_response, tokens_used, cost_usd, prompt_version, model_used)
   - Returns the review

   REVIEW CHAT METHOD — chat_follow_up():
   - Input: review_id, user_message (String)
   - Loads: original review, all previous chat messages, trade data
   - Builds conversation history for Claude (system prompt + trade context + review + all messages)
   - Sends to Claude
   - Saves assistant response to ai_review_messages table
   - Returns the response text
   - Token tracking on each message

   DAILY PLAN METHOD — generate_plan_of_attack():
   - Input: user_id, plan context (bias, watchlist symbols, notes), optional chart screenshots
   - System prompt: "You are a trading coach helping a trader prepare for the day. Generate a concise Plan of Attack briefing."
   - Output structure (JSON):
     {
       "top_watches": [{"symbol", "setup", "key_levels", "risk_reward"}],
       "market_bias_summary": "<text>",
       "risk_alerts": ["<alert 1>", ...],
       "focus_reminder": "<personalized reminder based on recent weaknesses>"
     }
   - Saves AI plan text to daily_plans.ai_plan_of_attack
   - Returns the plan

   WEEKLY SUMMARY METHOD — generate_weekly_summary():
   - Input: user_id, date_range (start, end)
   - Fetches: all trades in range, all plans in range, mood logs, tilt events
   - Builds comprehensive context about the trader's week
   - System prompt: weekly review coaching prompt
   - Output: narrative summary with metrics, patterns, and action plan
   - Saves to ai_reviews table with review_type = 'weekly'
   - Returns the summary

   MONTHLY SUMMARY METHOD — generate_monthly_summary():
   - Same pattern as weekly but deeper, with month-level analysis
   - Saves with review_type = 'monthly'

   PROMPT VERSIONING:
   - Each prompt has a version string (e.g., "trade_review_v1.0")
   - Version is stored with each review in the database
   - Prompts are defined as constants in a separate module:

2. apps/api/src/services/ai_prompts.rs — All AI prompts:
   - TRADE_REVIEW_SYSTEM_PROMPT_STRICT
   - TRADE_REVIEW_SYSTEM_PROMPT_ENCOURAGING
   - TRADE_REVIEW_SYSTEM_PROMPT_BALANCED
   - PLAN_OF_ATTACK_SYSTEM_PROMPT
   - WEEKLY_SUMMARY_SYSTEM_PROMPT
   - MONTHLY_SUMMARY_SYSTEM_PROMPT
   - DEVILS_ADVOCATE_SYSTEM_PROMPT
   - Each with a VERSION constant
   - All prompts are detailed, well-crafted, and produce consistent structured output
   - Each prompt instructs Claude to respond in valid JSON

3. apps/api/src/routes/ai.rs — AI API endpoints:

   POST /api/v1/ai/review-trade:
   - Body: { trade_id: Uuid }
   - Triggers review_trade()
   - Returns the review result
   - Rate limit: max 10 reviews per hour per user

   POST /api/v1/ai/review-chat:
   - Body: { review_id: Uuid, message: String }
   - Triggers chat_follow_up()
   - Returns { response: String, tokens_used: i32 }
   - Rate limit: max 30 messages per hour per user

   POST /api/v1/ai/plan-of-attack:
   - Body: { plan_id: Uuid, screenshots: Vec<String> (base64 optional) }
   - Triggers generate_plan_of_attack()
   - Returns the plan brief

   POST /api/v1/ai/weekly-summary:
   - Body: { start_date: NaiveDate, end_date: NaiveDate }
   - Triggers generate_weekly_summary()
   - Returns the summary

   POST /api/v1/ai/monthly-summary:
   - Body: { year: i32, month: i32 }
   - Triggers generate_monthly_summary()

   GET /api/v1/ai/reviews/:trade_id — Get existing review for a trade
   GET /api/v1/ai/reviews/:review_id/messages — Get chat history for a review
   GET /api/v1/ai/usage — Get user's AI usage stats (reviews this month, tokens used, estimated cost)

4. apps/api/src/models/ai.rs — AI models:
   - AiReviewResult (all fields matching the JSON schema above)
   - AiReviewRecord (database row)
   - AiChatMessage { id, review_id, role, content, tokens_used, created_at }
   - AiPlanOfAttack (structured plan output)
   - AiWeeklySummary
   - AiUsageStats { reviews_this_month, tokens_used, estimated_cost }
   - ReviewTradeRequest, ReviewChatRequest, PlanOfAttackRequest, etc.

5. packages/types/src/ai.ts — TypeScript types matching all Rust AI models

CRITICAL REQUIREMENTS:
- The Claude API key must NEVER be exposed to the frontend — all AI calls go through the Rust backend
- Screenshot images must be resized/compressed before sending to Claude — max 1568px on longest side, JPEG quality 85 (reduces token cost significantly)
- JSON parsing from Claude must be robust — handle cases where Claude wraps JSON in markdown code blocks or adds preamble
- Rate limiting must be enforced server-side — the frontend can show remaining quota but cannot bypass limits
- Token costs must be tracked per user — this is critical for cost management at scale
- The prompt versioning system allows A/B testing different prompts without code changes
- Error handling must cover: Claude API down, rate limited, context too long (too many screenshots), malformed response
- All AI responses must be saved raw (raw_response field) for debugging and prompt improvement
- The system prompts must be EXCELLENT — they are the difference between a useful review and a generic one. Invest heavily in prompt quality.

OUTPUT: Complete file contents for every file.
```

---

### PROMPT 4.2 — AI Review Frontend

```
[PASTE GLOBAL CONTEXT BLOCK + STACK OVERRIDE]

PHASE 4 — AI TRADE REVIEW SYSTEM
PROMPT 4.2 — AI Trade Review Frontend (Request, Display, Chat)

OBJECTIVE: Build the complete frontend for requesting AI trade reviews, displaying review results, and conducting follow-up chat conversations with the AI about specific trades.

DEPENDENCIES: Prompt 4.1 (AI API endpoints), Phase 1 (trade detail page)

DELIVERABLES:

1. apps/web/src/routes/(app)/review/+page.svelte — Review hub page:
   - Page title: "AI Review"
   - Shows recent AI reviews in a list:
     - Each review card: trade symbol, date, overall score (color-coded), review type (trade/weekly/monthly), snippet of key lesson
     - Click to view full review
   - "Request Weekly Summary" button
   - "Request Monthly Summary" button
   - Recent trades without reviews listed below: "These trades haven't been reviewed yet" with "Review" button on each
   - Loading/empty states

2. apps/web/src/lib/components/ai/review-request.svelte — Review request flow:
   - Used from trade detail page ("Ask AI to Review" button)
   - Step 1: Confirm trade details (shows trade summary, screenshots that will be analyzed)
   - Step 2: Optional — add additional screenshots or notes for context
   - Step 3: "Submit for Review" button
   - Loading state: animated AI thinking indicator with estimated wait time ("Analyzing your trade... ~15 seconds")
   - The loading state should feel premium: pulsing brain icon, progress stages ("Reading chart...", "Analyzing execution...", "Evaluating risk management...", "Generating insights...")
   - On completion: smooth transition to review display

3. apps/web/src/lib/components/ai/review-display.svelte — Full review display:
   
   LAYOUT:
   - Overall score: large circular gauge (0-10), color from red through yellow to green
   - Score label below: "Excellent" (8-10), "Good" (6-7.9), "Average" (4-5.9), "Needs Work" (2-3.9), "Poor" (0-1.9)
   
   Sub-scores row (3 smaller gauges):
   - Execution Quality (0-10)
   - Risk Management (0-10)
   - Plan Adherence (0-10)
   
   Strengths section:
   - Green left border
   - Checkmark icon per item
   - Each strength as a clear sentence
   
   Weaknesses section:
   - Red/orange left border
   - Warning icon per item
   - Each weakness as a clear sentence
   
   Key Lesson:
   - Highlighted card with lightbulb icon
   - Large, readable text
   - This is the single most important takeaway
   
   Actionable Fixes:
   - Numbered list with action icon
   - Each fix is specific and concrete
   
   Alternative Scenario:
   - "What if?" card with scenario text
   - Shows estimated R-multiple improvement
   
   Chart Analysis:
   - AI's description of what it saw in the screenshots
   - Side-by-side: screenshot thumbnail + AI's analysis text
   
   Emotional State:
   - Badge showing detected emotional state
   - Color-coded (calm=green, focused=blue, anxious=yellow, frustrated/revenge=red)
   
   Thesis Alignment:
   - Score (1-5) if thesis was provided
   - AI's comparison of thesis vs reality
   
   "Continue Conversation" button at bottom → opens chat

4. apps/web/src/lib/components/ai/review-chat.svelte — Follow-up chat:
   - Chat interface below the review display (expandable section)
   - Message list: alternating user/assistant messages
   - User messages right-aligned (accent background), AI messages left-aligned (surface background)
   - Text input at bottom with send button
   - Suggested follow-up questions as clickable chips:
     - "How could I have sized this better?"
     - "Is this setup still valid?"
     - "What entry signal should I have waited for?"
     - "What would you have done differently?"
   - Loading indicator while AI responds (typing animation)
   - Messages persist (saved to database, loaded on revisit)
   - Keyboard: Enter to send (Shift+Enter for newline)

5. apps/web/src/lib/components/dashboard/score-gauge.svelte — Circular score gauge:
   - SVG-based circular progress indicator
   - Props: value (0-10), size (sm/md/lg), label, showValue
   - Color interpolation: 0=red → 5=yellow → 10=green
   - Smooth animation on value change
   - Center: large number, bottom: label text
   - Used for overall score and sub-scores throughout the app

6. apps/web/src/lib/components/ai/ai-thinking.svelte — AI loading state:
   - Animated component shown while waiting for AI response
   - Pulsing brain/sparkle icon
   - Rotating status messages: "Reading your chart...", "Analyzing execution...", etc.
   - Progress bar (estimated, not actual — just gives visual feedback)
   - "This usually takes 10-20 seconds" text
   - Must feel polished and premium — this is when the user is most engaged

7. apps/web/src/lib/components/ai/weekly-summary-display.svelte — Weekly summary:
   - Narrative format (not structured scores like trade review)
   - Sections: Week Overview, Best Trade, Worst Trade, Behavioral Patterns, Plan Adherence Trend, Action Plan for Next Week
   - Each section with appropriate icon and formatting
   - "3-Step Improvement Plan" highlighted at the bottom
   - Export to PDF button

8. apps/web/src/lib/services/ai.ts — Frontend AI service:
   - requestTradeReview(tradeId): trigger review, return result
   - getTradeReview(tradeId): get existing review
   - sendChatMessage(reviewId, message): send follow-up
   - getChatMessages(reviewId): load chat history
   - requestWeeklySummary(startDate, endDate): trigger weekly
   - requestMonthlySummary(year, month): trigger monthly
   - getAiUsage(): get usage stats
   - All with proper typing and error handling

9. apps/web/src/lib/stores/ai-review.svelte.ts — AI review state:
   - $state: currentReview, chatMessages, isReviewing, isChatting, error
   - Methods: submitReview(tradeId), sendMessage(text), loadReview(tradeId), loadChat(reviewId)

10. Update trade detail page (apps/web/src/routes/(app)/trades/[id]/+page.svelte):
    - Wire up "Ask AI to Review" button to open review-request flow
    - If review exists, show review-display in the AI Review tab
    - If chat messages exist, show them in the expandable chat section

CRITICAL REQUIREMENTS:
- The review request flow must feel like a premium experience — the loading state is critical for user perception
- The score gauge must be smooth and visually satisfying (animated fill on mount)
- Chat must feel instant — optimistic UI (show user message immediately, then wait for AI response)
- The review display must be scannable — a trader should get the key takeaway (score + key lesson) in 2 seconds, then dig deeper if they want
- Mobile: review display must be fully scrollable and readable, chat input must not be blocked by keyboard
- The "Suggested follow-up questions" must be smart — they should reflect common things traders want to know after a review
- PDF export of weekly/monthly summaries must produce a clean, professional document (implement in a later prompt, but design the UI for it now)
- Rate limit feedback: if user hits rate limit, show a friendly message with reset time, not an error

OUTPUT: Complete file contents for every file.
```

---

## ============================================================
## PHASE SUMMARY — PHASES 1 THROUGH 4
## ============================================================

Phase 1 (6 prompts): Complete trade logging system
- 1.1: TypeScript + Rust types/models
- 1.2: Rust CRUD endpoints with calculations
- 1.3: Trade entry form (full-featured)
- 1.4: Trade list (table + cards + filters + pagination)
- 1.5: Trade detail page (all tabs, screenshots, scaling, AI review link)
- 1.6: CSV import + quick-log widget

Phase 2 (4 prompts): Full analytics engine
- 2.1: Rust analytics computation (all metrics, all aggregations)
- 2.2: Dashboard frontend (metrics grid, equity curve, heatmap)
- 2.3: Strategy, time, instrument, session, conviction, duration, streak charts
- 2.4: Monte Carlo, benchmark, vol-adjusted, overnight, catalyst, commissions, fatigue, correlation

Phase 3 (1 prompt): Daily planning module
- 3.1: Complete planning system (API + frontend — plan editor, watchlist, checklist, adherence)

Phase 4 (2 prompts): AI trade review system
- 4.1: Claude API integration (Rust — review, chat, plan, weekly/monthly summaries, prompts)
- 4.2: Review frontend (request flow, display, chat, gauges, weekly summary display)

---

## NEXT: PHASES 5 THROUGH 8

Phase 5 — Risk Management Suite
Phase 6 — Psychology & Discipline Tools
Phase 7 — Setup Playbook & Trade Grading
Phase 8 — Advanced Review & Replay

These will be delivered in the next document.