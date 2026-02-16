# Phase 1 - Core Trade Logging Summary

**Status:** Ready to Implement  
**Date:** February 15, 2026  
**Architecture:** Full Rust Backend (NO Supabase)

## Overview

Phase 1 implements the core trade logging functionality - the foundation of the entire application. This phase covers features 2.1 through 2.15 from the Features Spec.

## Phase 1 Prompts (6 Total)

### Phase 1.1 - Trade TypeScript Types & Rust Models ⏳
**Estimated Time:** 2-3 hours

**Deliverables:**
- `packages/types/src/trade.ts` - Complete TypeScript types
  - Trade, TradeCreateRequest, TradeUpdateRequest
  - TradeWithRelations, TradeListResponse
  - TradeFilters, TradeSortField, TradeSortOrder
  - TradeLeg, TradeMedia, Tag types
  - TradeStats for analytics
  - All enums (AssetClass, TradeDirection, TradeStatus, etc.)

- `apps/api/src/models/trade.rs` - Rust models
  - Trade struct with sqlx::FromRow
  - TradeCreateRequest with validation
  - TradeUpdateRequest with Option<T> fields
  - TradeWithRelations
  - All using Decimal for financial values
  - All using DateTime<Utc> for timestamps
  - Validation rules matching frontend

- `apps/api/src/models/tag.rs` - Tag models
  - Tag, TagCreate, TagUpdate
  - TagCategory enum

**Critical Requirements:**
- 1:1 match between TypeScript and Rust types
- snake_case field names (serde handles JSON conversion)
- Decimal for all financial values (NEVER f64)
- All enums defined on both sides

---

### Phase 1.2 - Rust Trade CRUD API Endpoints ⏳
**Estimated Time:** 4-6 hours

**Deliverables:**
- `apps/api/src/routes/trades.rs` - Trade endpoints
  - GET /api/v1/trades - List with filtering, sorting, pagination
  - GET /api/v1/trades/:id - Single trade with relations
  - POST /api/v1/trades - Create trade
  - PUT /api/v1/trades/:id - Update trade
  - DELETE /api/v1/trades/:id - Archive (soft delete)
  - POST /api/v1/trades/:id/restore - Restore archived

- `apps/api/src/routes/trade_legs.rs` - Scaling endpoints
  - GET /api/v1/trades/:id/legs
  - POST /api/v1/trades/:id/legs
  - PUT /api/v1/trades/:id/legs/:leg_id
  - DELETE /api/v1/trades/:id/legs/:leg_id

- `apps/api/src/services/trade_calculations.rs` - Trade math
  - calculate_r_multiple()
  - calculate_pnl()
  - calculate_pnl_pct()
  - calculate_duration()
  - detect_session_type()
  - calculate_weighted_avg_price()
  - Unit tests for all functions

- `apps/api/src/routes/tags.rs` - Tag endpoints
  - GET /api/v1/tags
  - POST /api/v1/tags
  - PUT /api/v1/tags/:id
  - DELETE /api/v1/tags/:id
  - POST /api/v1/trades/:id/tags
  - DELETE /api/v1/trades/:id/tags/:tag_id

**Critical Requirements:**
- EVERY query scoped by user_id (security boundary)
- Decimal for all financial calculations
- Parameterized queries (no SQL injection)
- Dynamic filter builder for any combination
- Transaction for trade + tags (atomic)
- User-friendly error messages
- 20+ unit tests for calculations

---

### Phase 1.3 - Trade Entry Form (Frontend) ⏳
**Estimated Time:** 6-8 hours

**Deliverables:**
- `apps/web/src/lib/components/trade/trade-form.svelte` - Main form
  - 8 collapsible sections
  - Auto-calculations with $derived
  - Form validation with Zod
  - Draft auto-save to localStorage
  - Responsive layout

- `apps/web/src/lib/components/trade/symbol-input.svelte`
- `apps/web/src/lib/components/trade/conviction-picker.svelte`
- `apps/web/src/lib/components/trade/grade-picker.svelte`
- `apps/web/src/lib/components/trade/tag-picker.svelte`
- `apps/web/src/lib/components/trade/media-upload.svelte`
- `apps/web/src/lib/components/trade/voice-note.svelte`

- `apps/web/src/routes/(app)/trades/new/+page.svelte`
- `apps/web/src/routes/(app)/trades/[id]/edit/+page.svelte`

- `apps/web/src/lib/services/trades.ts` - Frontend service
  - createTrade()
  - updateTrade()
  - uploadMedia()
  - deleteMedia()

**Form Sections:**
1. Instrument (symbol, asset class, direction)
2. Entry & Exit (prices, times, sizes)
3. Risk (stop loss, take profit, R-multiple, P&L)
4. Details (setup, conviction, grade, regime)
5. Notes & Thesis (text, voice notes)
6. Tags (multi-select with categories)
7. Screenshots (drag-drop upload)
8. Options (only for options trades)

**Critical Requirements:**
- Form must feel FAST (no lag)
- Auto-calculations use $derived (not $effect)
- Mobile-optimized (stacked layout, large touch targets)
- Draft auto-save (debounced)
- Media uploads immediate (not on form submit)
- Tab order logical
- Zod validation matching Rust

---

### Phase 1.4 - Trade List View ⏳
**Estimated Time:** 4-5 hours

**Deliverables:**
- `apps/web/src/routes/(app)/trades/+page.svelte` - List page
  - Filter panel (collapsible)
  - Summary stats bar
  - View toggle (table/cards)

- `apps/web/src/lib/components/trade/trade-table.svelte`
  - Sortable columns
  - Row click to detail
  - Sticky header
  - Responsive (hide columns on mobile)

- `apps/web/src/lib/components/trade/trade-card.svelte`
  - Mobile-friendly card layout
  - 1-3 column grid

- `apps/web/src/lib/components/trade/trade-filters.svelte`
  - All filter inputs
  - Clear all button
  - Filter count

- `apps/web/src/lib/components/trade/trade-summary-bar.svelte`
  - Quick stats (count, win rate, P&L, avg R, profit factor)

- `apps/web/src/lib/stores/trade-list.svelte.ts`
  - State management with Svelte 5 runes
  - URL sync for filters/sort
  - Infinite scroll support

**Critical Requirements:**
- Load fast (under 200ms for 50 trades)
- Server-side sorting
- Debounced filters (300ms)
- URL query params (bookmarkable)
- Skeleton loading states
- Empty states

---

### Phase 1.5 - Trade Detail View ⏳
**Estimated Time:** 4-5 hours

**Deliverables:**
- `apps/web/src/routes/(app)/trades/[id]/+page.svelte`
- `apps/web/src/routes/(app)/trades/[id]/+page.ts`

- `apps/web/src/lib/components/trade/trade-detail.svelte`
  - Header with P&L prominent
  - Metrics grid
  - 6 tabs (Screenshots, Notes, Scaling, Tags, AI Review, Context)

- `apps/web/src/lib/components/trade/screenshot-gallery.svelte`
  - Thumbnail grid
  - Lightbox viewer

- `apps/web/src/lib/components/trade/trade-legs-timeline.svelte`
  - Visual scaling history

- `apps/web/src/lib/components/trade/lightbox.svelte`
  - Full-screen image viewer
  - Zoom/pan
  - Navigation

**Critical Requirements:**
- Skeleton loading (no layout shift)
- P&L most prominent
- Screenshots lazy-load
- "Ask AI to Review" button prominent
- Tab state in URL hash
- Print-friendly
- Previous/Next trade navigation

---

### Phase 1.6 - CSV Import & Quick-Log ⏳
**Estimated Time:** 3-4 hours

**Deliverables:**
- `apps/api/src/routes/import.rs`
  - POST /api/v1/trades/import - Bulk CSV import
  - GET /api/v1/trades/import/templates - Download templates

- `apps/web/src/routes/(app)/trades/import/+page.svelte`
  - 4-step import flow
  - Column mapping
  - Preview & validate
  - Import with progress

- `apps/web/src/lib/components/trade/quick-log.svelte`
  - Minimal fields for fast entry
  - Modal dialog
  - Under 10 seconds to log

- `apps/web/src/lib/components/trade/quick-log-trigger.svelte`
  - Prominent button in top bar
  - FAB on mobile

**Import Flow:**
1. Upload CSV
2. Map columns
3. Preview & validate
4. Import with progress

**Critical Requirements:**
- CSV parsing handles edge cases
- Column auto-detection
- Web workers for large files
- Quick-log opens under 100ms
- Auto-focus symbol field
- Remember last-used values

---

## Implementation Order

### Recommended Sequence

1. **Phase 1.1** - Types & Models (foundation)
2. **Phase 1.2** - Rust API Endpoints (backend complete)
3. **Phase 1.3** - Trade Entry Form (can create trades)
4. **Phase 1.4** - Trade List View (can view trades)
5. **Phase 1.5** - Trade Detail View (full trade info)
6. **Phase 1.6** - CSV Import & Quick-Log (bulk & fast entry)

### Dependencies

- Phase 0 must be complete (especially 0.5 database migrations and 0.8 Rust API scaffold)
- UI components from Phase 0.3 needed
- App shell from Phase 0.4 needed
- Auth from Phase 0.7 needed

---

## Key Features Implemented

### Core Functionality
- ✅ Manual trade logging (full form)
- ✅ Quick-log widget (fast entry)
- ✅ CSV bulk import
- ✅ Trade list with filtering/sorting
- ✅ Trade detail view
- ✅ Screenshot upload
- ✅ Voice notes
- ✅ Trade tags
- ✅ Trade scaling (legs)
- ✅ Missed trade logging
- ✅ Trade archiving

### Calculations
- ✅ R-multiple
- ✅ P&L (dollar and percentage)
- ✅ Duration
- ✅ Session type detection
- ✅ Weighted average prices (scaling)

### Data Management
- ✅ CRUD operations
- ✅ Filtering (15+ filter types)
- ✅ Sorting (8 sort fields)
- ✅ Pagination
- ✅ Tag management
- ✅ Media management

---

## Technical Specifications

### Backend (Rust)
- All endpoints scoped by user_id
- Decimal for financial values
- Parameterized SQL queries
- Transaction support
- Comprehensive validation
- Unit tests for calculations

### Frontend (SvelteKit)
- Svelte 5 runes ($state, $derived, $effect)
- TypeScript strict mode
- Zod validation
- Responsive design (375px to 2560px)
- Optimistic updates
- Draft auto-save
- URL state sync

### Performance Targets
- Trade list load: < 200ms for 50 trades
- Quick-log open: < 100ms
- Form auto-calculations: instant (< 16ms)
- CSV import: handle 5000 rows
- Screenshot upload: progress feedback

---

## Testing Checklist

After Phase 1 completion, verify:

### Trade Entry
- [ ] Can create trade with minimal fields (symbol, direction)
- [ ] Can create trade with all fields
- [ ] Auto-calculations work (R-multiple, P&L, duration)
- [ ] Form validation shows errors
- [ ] Draft auto-save works
- [ ] Can upload screenshots
- [ ] Can record voice notes
- [ ] Can add tags
- [ ] Can log missed trade

### Trade List
- [ ] Trades display in table
- [ ] Can filter by date, symbol, tags, etc.
- [ ] Can sort by any column
- [ ] Pagination works
- [ ] Summary stats update with filters
- [ ] Card view works on mobile
- [ ] URL reflects filters/sort

### Trade Detail
- [ ] All trade data displays
- [ ] Screenshots open in lightbox
- [ ] Can edit trade inline
- [ ] Tabs work correctly
- [ ] Previous/Next navigation works

### CSV Import
- [ ] Can upload CSV
- [ ] Column mapping works
- [ ] Preview shows validation errors
- [ ] Import creates trades
- [ ] Duplicates detected

### Quick-Log
- [ ] Opens quickly
- [ ] Can log trade in under 10 seconds
- [ ] "Save & Log Another" works
- [ ] FAB shows on mobile

### API
- [ ] All endpoints return correct data
- [ ] Filtering works
- [ ] Sorting works
- [ ] Pagination works
- [ ] Validation rejects invalid data
- [ ] User can only access own trades

---

## Estimated Total Time

**Phase 1 Total:** 23-31 hours

- Prompt 1.1: 2-3 hours
- Prompt 1.2: 4-6 hours
- Prompt 1.3: 6-8 hours
- Prompt 1.4: 4-5 hours
- Prompt 1.5: 4-5 hours
- Prompt 1.6: 3-4 hours

---

## Next Steps

After Phase 1 completion:
- **Phase 2:** Advanced Analytics & Visualizations
- **Phase 3:** Daily Planning & Watchlist
- **Phase 4:** AI-Powered Trade Reviews

---

**Phase 1 Status:** Ready to Implement  
**Prerequisites:** Phase 0 complete (especially 0.5, 0.8)  
**Architecture:** Full Rust Backend, NO Supabase
