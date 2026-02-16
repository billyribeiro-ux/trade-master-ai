# TradeMaster AI — Phase Prompts for Claude AI (Windsurf)
# Apple Principal Engineer ICT Level 7+ Standards

---

## HOW TO USE THIS DOCUMENT

Each prompt below is a self-contained instruction set designed to be pasted directly into Claude AI within Windsurf. Execute them in exact order — each prompt builds on the output of the previous one. Do not skip prompts. Do not reorder prompts.

Before starting, paste this context block at the beginning of EVERY prompt session:

---

### GLOBAL CONTEXT BLOCK (Paste before every prompt)

```
YOU ARE A PRINCIPAL ENGINEER (ICT LEVEL 7+) AT APPLE/MICROSOFT.

PROJECT: TradeMaster AI — The ultimate all-in-one AI-powered trading journal app.
DESCRIPTION: A trading journal that combines trade logging, deep analytics, daily planning, AI-powered trade reviews with chart screenshot analysis, risk management tools, and psychological tracking. The AI is the central nervous system — not a bolt-on feature.

TECH STACK (NON-NEGOTIABLE):
- Frontend: SvelteKit 5 with Svelte 5 runes ($state, $derived, $effect, $props)
- Language: TypeScript strict mode — zero warnings, zero errors, zero type-safety shortcuts
- Styling: Tailwind CSS 4, dark-mode-first design system
- UI Foundation: shadcn-svelte for base primitives + custom trading component library
- Backend API: Rust / Axum
- Database: Supabase (PostgreSQL 15+) with Row Level Security
- Auth: Supabase Auth (OAuth + email/password)
- File Storage: Supabase Storage (screenshots, media)
- AI Engine: Anthropic Claude API (multimodal — text + image analysis)
- Charts (Financial): Lightweight Charts by TradingView (candlestick rendering, trade replay)
- Charts (Analytics): Apache ECharts (dashboards, heatmaps, equity curves, distributions, all analytics viz)
- Graphics (Dashboard UI): shadcn-svelte + custom Svelte 5 component library (gauges, meters, KPI cards, score displays, progress indicators, status badges, sparklines)
- Package Manager: pnpm (all JS/TS projects)
- Icons: Iconify with Phosphor and Carbon icon sets — NEVER use Lucide icons
- Deployment: Vercel (frontend) + Fly.io (Rust API)
- CI/CD: GitHub Actions

CODING STANDARDS:
- Svelte 5 runes ONLY: $state, $derived, $effect, $props — NEVER use Svelte 4 stores, reactive declarations ($:), or slots
- Use snippets over slots in all components
- TypeScript strict mode in every file — no 'any' types, no type assertions unless absolutely necessary with a comment explaining why
- Every component must have typed props using $props()
- Every async operation must have loading, error, and empty states
- Every form must have validation
- Every API call must have error handling with user-friendly messages
- All colors, spacing, typography must come from the design system tokens — no hardcoded values
- Responsive: works on 375px (mobile) through 2560px (ultrawide)
- Keyboard accessible: tab navigation, focus management, aria labels
- All code must be production-ready — no TODOs, no placeholders, no "add more here" comments, no simplified examples
- Build for 10-year longevity — no hacky workarounds, no shortcuts

FILE STRUCTURE:
trademaster-ai/
├── apps/
│   ├── web/                          # SvelteKit 5 frontend
│   │   ├── src/
│   │   │   ├── lib/
│   │   │   │   ├── components/
│   │   │   │   │   ├── ui/           # shadcn-svelte + design system primitives
│   │   │   │   │   ├── trade/        # Trade-specific components
│   │   │   │   │   ├── analytics/    # Chart & visualization components
│   │   │   │   │   ├── planning/     # Daily plan components
│   │   │   │   │   ├── ai/           # AI review components
│   │   │   │   │   ├── risk/         # Risk management components
│   │   │   │   │   ├── psychology/   # Mood, tilt, goals components
│   │   │   │   │   ├── dashboard/    # Dashboard KPI widgets, gauges, meters
│   │   │   │   │   └── layout/       # Shell, nav, sidebar
│   │   │   │   ├── stores/           # Svelte 5 rune-based state management
│   │   │   │   ├── services/         # API client, AI service, broker adapters
│   │   │   │   ├── utils/            # Calculations, formatters, validators
│   │   │   │   ├── types/            # Shared TypeScript types/interfaces
│   │   │   │   └── config/           # App configuration, constants
│   │   │   ├── routes/
│   │   │   │   ├── (auth)/           # Login, register, onboarding
│   │   │   │   ├── (app)/            # Authenticated app routes
│   │   │   │   │   ├── dashboard/
│   │   │   │   │   ├── trades/
│   │   │   │   │   ├── plan/
│   │   │   │   │   ├── review/
│   │   │   │   │   ├── analytics/
│   │   │   │   │   ├── risk/
│   │   │   │   │   ├── playbook/
│   │   │   │   │   ├── psychology/
│   │   │   │   │   ├── settings/
│   │   │   │   │   └── account/
│   │   │   │   └── api/              # SvelteKit API routes
│   │   │   ├── hooks.server.ts
│   │   │   └── app.d.ts
│   │   ├── static/
│   │   ├── svelte.config.js
│   │   ├── tailwind.config.ts
│   │   ├── tsconfig.json
│   │   └── package.json
│   │
│   └── api/                          # Rust/Axum backend
│       ├── src/
│       │   ├── routes/
│       │   ├── handlers/
│       │   ├── models/
│       │   ├── services/
│       │   ├── middleware/
│       │   ├── config.rs
│       │   ├── error.rs
│       │   └── main.rs
│       ├── migrations/
│       └── Cargo.toml
│
├── packages/
│   ├── types/                        # Shared TS types
│   └── calculations/                 # Shared trading math
│
├── supabase/
│   ├── migrations/
│   └── config.toml
│
├── pnpm-workspace.yaml
└── turbo.json

WHEN I SAY "COMPLETE" I MEAN:
- Every line of code written out in full
- Every import statement included
- Every type defined
- Every edge case handled
- Every error state covered
- Production-ready as written — copy-paste and it works
```

---

## ============================================================
## PHASE 0 — FOUNDATION & INFRASTRUCTURE
## ============================================================

### PROMPT 0.1 — Project Scaffolding & Monorepo Setup

```
[PASTE GLOBAL CONTEXT BLOCK ABOVE FIRST]

PHASE 0 — FOUNDATION & INFRASTRUCTURE
PROMPT 0.1 — Project Scaffolding & Monorepo Setup

OBJECTIVE: Create the complete monorepo structure for TradeMaster AI with SvelteKit 5, TypeScript strict mode, Tailwind CSS 4, and pnpm workspace configuration.

DELIVERABLES:

1. Root monorepo configuration:
   - pnpm-workspace.yaml defining workspace packages: apps/web, packages/types, packages/calculations
   - turbo.json with build, dev, lint, type-check, and test pipelines
   - Root package.json with workspace scripts
   - Root .gitignore (comprehensive: node_modules, .env, .svelte-kit, dist, build, .turbo, .DS_Store, coverage, *.local)
   - Root .npmrc with shamefully-hoist=true for pnpm compatibility
   - .env.example with all required environment variables documented:
     - PUBLIC_SUPABASE_URL
     - PUBLIC_SUPABASE_ANON_KEY
     - SUPABASE_SERVICE_ROLE_KEY
     - ANTHROPIC_API_KEY
     - PUBLIC_APP_URL
     - DATABASE_URL

2. apps/web — SvelteKit 5 project:
   - Initialize with: pnpm create svelte@latest (skeleton project, TypeScript)
   - package.json with these exact dependencies:
     - svelte (latest 5.x)
     - @sveltejs/kit (latest 2.x)
     - @sveltejs/adapter-vercel
     - typescript
     - tailwindcss (v4)
     - @tailwindcss/vite
     - @iconify/svelte
     - bits-ui (shadcn-svelte foundation)
     - clsx
     - tailwind-merge
     - echarts
     - lightweight-charts
     - @supabase/supabase-js
     - @supabase/ssr
     - zod (form validation)
     - date-fns
     - mode-watcher (dark mode)
   - tsconfig.json with strict: true, noUncheckedIndexedAccess: true, exactOptionalPropertyTypes: true, noImplicitReturns: true, noFallthroughCasesInSwitch: true, forceConsistentCasingInFileNames: true
   - svelte.config.js with Vercel adapter, TypeScript preprocessing, and alias configuration ($lib, $components, $stores, $services, $utils, $types, $config)
   - vite.config.ts with Tailwind CSS v4 plugin
   - app.d.ts with Supabase session types in App.Locals
   - app.css with Tailwind v4 imports and CSS custom properties for the design system (defined in next prompt)
   - Create the complete folder structure as defined in the global context block — every directory with a .gitkeep or index.ts barrel file

3. packages/types — Shared TypeScript types:
   - package.json with name "@trademaster/types", typescript as dev dependency
   - tsconfig.json extending root strict config
   - src/index.ts as barrel export

4. packages/calculations — Shared trading math:
   - package.json with name "@trademaster/calculations", typescript as dev dependency
   - tsconfig.json extending root strict config
   - src/index.ts as barrel export

5. ESLint configuration:
   - eslint.config.js at root with TypeScript + Svelte rules
   - Rules: no-unused-vars (error), no-explicit-any (error), consistent-type-imports (warn)
   - Integration with Svelte 5 (eslint-plugin-svelte)

6. Prettier configuration:
   - .prettierrc with: tabWidth 2, useTabs true, singleQuote true, trailingComma 'all', printWidth 100, plugins for Svelte and Tailwind class sorting
   - .prettierignore

CRITICAL REQUIREMENTS:
- pnpm ONLY — no npm, no yarn commands anywhere
- Every tsconfig must have strict: true
- SvelteKit 5 with Svelte 5 — verify the versions are correct
- Tailwind CSS v4 (NOT v3 — v4 uses @import "tailwindcss" not @tailwind directives)
- All path aliases must work: $lib, $components, $stores, $services, $utils, $types, $config
- The project must compile clean with zero errors after setup: pnpm type-check && pnpm lint

OUTPUT: Every file with complete contents. I will copy these directly into my project.
```

---

### PROMPT 0.2 — Design System Tokens & Tailwind Configuration

```
[PASTE GLOBAL CONTEXT BLOCK]

PHASE 0 — FOUNDATION & INFRASTRUCTURE
PROMPT 0.2 — Design System Tokens & Tailwind Configuration

OBJECTIVE: Create the complete design system for a dark-mode-first professional trading application. This design system is the visual foundation of the entire app — every color, font, spacing value, shadow, and border radius used anywhere in the app comes from these tokens.

DEPENDENCIES: Prompt 0.1 (project scaffolding exists)

DELIVERABLES:

1. apps/web/src/app.css — Complete CSS custom properties and Tailwind v4 configuration:

   COLOR SYSTEM (dark-mode-first, inspired by Bloomberg Terminal meets modern fintech):
   
   Backgrounds:
   - --bg-primary: Deep dark background for the main canvas (near-black with slight blue undertone, NOT pure black)
   - --bg-secondary: Slightly lighter surface for cards and panels
   - --bg-tertiary: Elevated surface for modals, dropdowns, popovers
   - --bg-hover: Subtle hover state for interactive elements
   - --bg-active: Active/selected state background
   - --bg-input: Input field background (slightly different from card background for visual distinction)
   
   Text:
   - --text-primary: High contrast white (not pure #fff, slightly warm)
   - --text-secondary: Muted text for labels, descriptions, secondary info
   - --text-tertiary: Even more muted for placeholders, disabled text
   - --text-inverse: Dark text for use on light backgrounds (buttons, badges)
   
   Trading Colors (CRITICAL — these are the most important colors in the app):
   - --profit: Green for winning trades, positive P&L, bullish signals (vibrant but not neon)
   - --profit-muted: Softer green for backgrounds, subtle indicators
   - --profit-bg: Very subtle green tint for profit row backgrounds
   - --loss: Red for losing trades, negative P&L, bearish signals (clear but not alarming)
   - --loss-muted: Softer red for backgrounds
   - --loss-bg: Very subtle red tint for loss row backgrounds
   - --neutral: For breakeven, flat, unchanged values
   
   Brand/Accent:
   - --accent-primary: Primary brand accent (professional blue — not generic Bootstrap blue)
   - --accent-primary-hover: Hover state
   - --accent-primary-active: Active state
   - --accent-secondary: Secondary accent for variety
   
   Semantic:
   - --warning: Amber/orange for warnings, tilt alerts, caution states
   - --warning-bg: Subtle warning background
   - --info: Blue for informational elements
   - --info-bg: Subtle info background
   - --danger: Red for destructive actions (distinct from --loss)
   - --danger-bg: Subtle danger background
   - --success: Green for success states (can alias --profit)
   
   Borders:
   - --border-primary: Default border color (subtle, low contrast)
   - --border-secondary: Even more subtle for nested elements
   - --border-focus: Focus ring color (accent-primary based)
   - --border-hover: Border on hover
   
   Grades (for trade grading A/B/C/D):
   - --grade-a: Gold/premium color
   - --grade-b: Blue/good color
   - --grade-c: Orange/average color
   - --grade-d: Red/poor color
   
   Score Gradient (for 0-100 scores like Personal Edge Score):
   - CSS gradient or stepped colors from red (0) through orange (25) through yellow (50) through green-yellow (75) to green (100)
   
   Conviction Colors (for 1-5 conviction scoring):
   - 5 distinct colors from low conviction (gray/blue) to high conviction (gold/bright)

   TYPOGRAPHY:
   - Font family: Inter for UI, JetBrains Mono for numbers/prices/code
   - Size scale: xs (11px), sm (12px), base (14px), lg (16px), xl (18px), 2xl (20px), 3xl (24px), 4xl (30px), 5xl (36px)
   - Font weights: normal (400), medium (500), semibold (600), bold (700)
   - Line heights: tight (1.2), normal (1.5), relaxed (1.75)
   - Letter spacing: tight (-0.01em for headings), normal (0), wide (0.025em for labels)

   SPACING:
   - Consistent scale: 0, 1 (4px), 2 (8px), 3 (12px), 4 (16px), 5 (20px), 6 (24px), 8 (32px), 10 (40px), 12 (48px), 16 (64px), 20 (80px), 24 (96px)

   BORDERS:
   - Border radius: none (0), sm (4px), md (6px), lg (8px), xl (12px), 2xl (16px), full (9999px)
   - Border widths: 1px default, 2px for focus rings

   SHADOWS:
   - shadow-sm: Subtle card shadow
   - shadow-md: Elevated elements (dropdowns, popovers)
   - shadow-lg: Modals, major overlays
   - shadow-glow-profit: Subtle green glow for profit highlights
   - shadow-glow-loss: Subtle red glow for loss highlights
   - shadow-glow-accent: Accent color glow for primary actions

   TRANSITIONS:
   - Default: 150ms ease-in-out
   - Slow: 300ms ease-in-out
   - Fast: 75ms ease-in-out

   Z-INDEX SCALE:
   - base: 0
   - dropdown: 50
   - sticky: 100
   - overlay: 200
   - modal: 300
   - popover: 400
   - toast: 500
   - tooltip: 600

2. tailwind.config.ts — Extended Tailwind configuration:
   - Map ALL CSS custom properties to Tailwind utility classes
   - Custom colors: bg-surface-primary, text-profit, text-loss, border-default, etc.
   - Custom font families: font-sans (Inter), font-mono (JetBrains Mono)
   - Extended spacing if needed beyond Tailwind defaults
   - Container queries plugin if available
   - All custom shadows mapped
   - Animation keyframes: fade-in, slide-up, slide-down, scale-in, pulse-subtle

3. apps/web/src/lib/utils/cn.ts — Class merging utility:
   - Export a cn() function using clsx + tailwind-merge
   - This is used in EVERY component for conditional class application

4. apps/web/src/lib/config/theme.ts — Theme constants:
   - Export typed constants for colors, spacing, etc. that are used in JavaScript (ECharts themes, Lightweight Charts themes)
   - ECharts dark theme configuration object matching the design system exactly
   - Lightweight Charts theme configuration object matching the design system exactly

CRITICAL REQUIREMENTS:
- This is Tailwind CSS v4 — use @import "tailwindcss" syntax, NOT @tailwind base/components/utilities
- All colors must have sufficient contrast ratios for accessibility (WCAG AA minimum)
- The profit green and loss red must be clearly distinguishable (consider colorblind users — test with deuteranopia simulation)
- No hardcoded color values anywhere in the app after this — everything references the design system
- The design must look professional and premium — think Bloomberg Terminal meets Stripe Dashboard, not generic Bootstrap
- Include CSS for scrollbar styling (dark scrollbars matching the theme)
- Include CSS for selection/highlight colors
- Include CSS for focus-visible outlines

OUTPUT: Complete file contents for every file listed above.
```

---

### PROMPT 0.3 — shadcn-svelte Base Components

```
[PASTE GLOBAL CONTEXT BLOCK]

PHASE 0 — FOUNDATION & INFRASTRUCTURE
PROMPT 0.3 — shadcn-svelte Base Components

OBJECTIVE: Set up shadcn-svelte and create the foundational UI primitive components that every other component in the app will be built on top of. These are the atoms of the design system.

DEPENDENCIES: Prompt 0.1 (project structure), Prompt 0.2 (design system tokens)

DELIVERABLES:

Initialize shadcn-svelte in the project and create these base components in apps/web/src/lib/components/ui/. Every component must use Svelte 5 runes, TypeScript strict, the design system tokens from Prompt 0.2, and the cn() utility for class merging.

1. Button (button.svelte):
   - Variants: primary (accent color), secondary (subtle), outline, ghost, danger, profit (green), loss (red)
   - Sizes: sm, md, lg
   - States: default, hover, active, disabled, loading (with spinner)
   - Props: variant, size, disabled, loading, type (button/submit/reset), href (renders as <a> if provided), full-width option
   - Icon support: optional leading and trailing icon slots via snippets
   - Keyboard: Enter/Space activation, proper focus ring

2. Input (input.svelte):
   - Variants: default, error, success
   - Types: text, number, email, password, search, tel, url
   - Props: value (bindable), placeholder, disabled, readonly, label, error message, helper text, prefix text, suffix text
   - Number input: optional min, max, step with increment/decrement buttons
   - Currency/price input: formats with commas, handles decimal precision
   - Keyboard: proper tab indexing

3. Select (select.svelte):
   - Single select dropdown with search/filter
   - Props: options (array of {value, label, icon?, disabled?}), value (bindable), placeholder, disabled, error, label
   - Keyboard: arrow keys, type-ahead, Enter to select, Escape to close

4. Textarea (textarea.svelte):
   - Auto-resize option
   - Character count display option
   - Props: value (bindable), placeholder, rows, maxLength, disabled, label, error

5. Checkbox (checkbox.svelte):
   - Props: checked (bindable), label, disabled, indeterminate
   - Styled with design system colors

6. Toggle/Switch (toggle.svelte):
   - Props: checked (bindable), label, disabled, size (sm/md)
   - Smooth animation on toggle

7. Badge (badge.svelte):
   - Variants: default, profit, loss, warning, info, danger, grade-a, grade-b, grade-c, grade-d, outline
   - Sizes: sm, md
   - Props: variant, size, dot (shows a colored dot indicator)

8. Card (card.svelte):
   - Base container component for all dashboard cards
   - Props: padding (sm/md/lg/none), hover effect, clickable, border option
   - Snippets: header, content, footer
   - Subtle border with hover state

9. Dialog/Modal (dialog.svelte):
   - Props: open (bindable), title, description, size (sm/md/lg/xl/full)
   - Close on escape, close on overlay click (configurable)
   - Focus trap when open
   - Smooth enter/exit animations
   - Snippets: trigger, content, footer

10. Tooltip (tooltip.svelte):
    - Props: content (text), side (top/right/bottom/left), delay (ms)
    - Shows on hover/focus
    - Arrow pointer

11. Tabs (tabs.svelte + tab-item.svelte):
    - Props: items (array of {value, label, icon?, count?}), value (bindable)
    - Underline style indicator with smooth animation
    - Overflow scroll on mobile when many tabs

12. Dropdown Menu (dropdown.svelte):
    - Trigger + menu items
    - Items support: label, icon, shortcut hint, danger styling, disabled, separator
    - Keyboard: arrow keys, Enter to select, Escape to close

13. Toast/Notification (toast.svelte + toast-provider.svelte):
    - Variants: success, error, warning, info
    - Position: top-right
    - Auto-dismiss with configurable duration
    - Manual dismiss button
    - Stack multiple toasts
    - Provide a global toast() function callable from anywhere: toast.success("Trade saved"), toast.error("Failed to connect")

14. Skeleton Loader (skeleton.svelte):
    - Props: width, height, variant (text/circle/rect/card)
    - Shimmer animation
    - Used for every loading state in the app

15. Empty State (empty-state.svelte):
    - Props: icon, title, description, action button (optional)
    - Used when lists/tables have no data

16. Avatar (avatar.svelte):
    - Props: src, alt, size (sm/md/lg), fallback (initials)
    - Circular with border

17. Separator (separator.svelte):
    - Horizontal or vertical divider line
    - Uses border color from design system

18. Progress Bar (progress.svelte):
    - Props: value (0-100), variant (default/profit/loss/warning/accent), size (sm/md), showLabel
    - Smooth animation on value change

19. Spinner (spinner.svelte):
    - Props: size (sm/md/lg), color
    - CSS animation, no JS

20. Icon wrapper (icon.svelte):
    - Wraps @iconify/svelte with consistent sizing and color application
    - Props: name (Iconify icon name), size (sm/md/lg/xl or number), color (uses currentColor by default), class
    - Default to Phosphor icons: ph:house, ph:chart-line, ph:clipboard-text, etc.

CRITICAL REQUIREMENTS:
- Every component uses Svelte 5 runes — $props(), $state(), $derived(), $effect()
- Every component uses the cn() utility for class merging
- Every component accepts a class prop for external styling overrides
- Every component has proper TypeScript types for all props
- Use snippets (not slots) for composable content areas
- All interactive elements have focus-visible outlines
- All animations use the transition timing from the design system
- Components must render correctly on both 375px and 2560px screens
- Test each component mentally for edge cases: empty strings, undefined values, very long text, zero values

OUTPUT: Complete file contents for every component. Each file must be complete and production-ready.
```

---

### PROMPT 0.4 — App Shell, Layout & Navigation

```
[PASTE GLOBAL CONTEXT BLOCK]

PHASE 0 — FOUNDATION & INFRASTRUCTURE
PROMPT 0.4 — App Shell, Layout & Navigation

OBJECTIVE: Create the complete application shell — the persistent layout that wraps every authenticated page. This includes the sidebar navigation, top bar, mobile bottom navigation, and the responsive layout system.

DEPENDENCIES: Prompt 0.1 (project structure), Prompt 0.2 (design system), Prompt 0.3 (UI components)

DELIVERABLES:

1. apps/web/src/routes/(app)/+layout.svelte — Authenticated layout wrapper:
   - Checks for authenticated session, redirects to /login if not authenticated
   - Renders the app shell with sidebar + topbar + content area
   - Passes user session data to child routes via layout data

2. apps/web/src/routes/(app)/+layout.server.ts — Server-side auth check:
   - Loads Supabase session
   - Redirects to /login if no session
   - Returns user profile data for the layout

3. apps/web/src/lib/components/layout/app-shell.svelte — Main shell component:
   - Three-zone layout: sidebar (left) + main content (center-right)
   - Sidebar is collapsible (icon-only mode) on desktop, hidden on mobile
   - Sidebar collapse state persists in localStorage
   - Main content area scrolls independently from sidebar
   - Top bar is fixed/sticky at top of content area
   - On mobile (<768px): sidebar hidden, bottom nav appears instead
   - Smooth transition animations for sidebar collapse/expand

4. apps/web/src/lib/components/layout/sidebar.svelte — Desktop sidebar navigation:
   - App logo/brand at top (collapses to icon)
   - Navigation items with Phosphor icons:
     - Dashboard (ph:squares-four)
     - Trades (ph:list-bullets)
     - Plan (ph:calendar-check)
     - Review (ph:brain)
     - Analytics (ph:chart-line-up)
     - Risk (ph:shield-check)
     - Playbook (ph:book-open)
     - Psychology (ph:heartbeat)
   - Separator
   - Secondary nav:
     - Settings (ph:gear-six)
     - Account (ph:user-circle)
   - Active state highlighting with accent color left border indicator
   - Hover states
   - Tooltip showing full label when sidebar is collapsed (icon-only mode)
   - User avatar + name at bottom (collapses to avatar only)
   - Collapse toggle button at bottom

5. apps/web/src/lib/components/layout/mobile-nav.svelte — Mobile bottom navigation:
   - Fixed bottom bar visible only on mobile (<768px)
   - 5 primary items with icons + labels:
     - Dashboard
     - Trades
     - Plan
     - Review
     - More (opens a sheet/drawer with remaining nav items)
   - Active state with accent color
   - Safe area padding for iOS notch/home indicator
   - The "More" button opens a drawer/sheet from bottom with: Analytics, Risk, Playbook, Psychology, Settings, Account

6. apps/web/src/lib/components/layout/top-bar.svelte — Top bar:
   - Page title (dynamic based on current route)
   - Right side: Quick-log trade button (prominent CTA), notification bell, user avatar dropdown
   - On mobile: hamburger menu (if needed), page title, quick-log button
   - The quick-log button is the most prominent interactive element — it should be visually distinct (accent color, slightly larger)
   - Breadcrumb support for nested pages (e.g., Trades > Trade Detail)

7. apps/web/src/lib/components/layout/mobile-drawer.svelte — Mobile "More" drawer:
   - Bottom sheet that slides up
   - Contains remaining nav items not in the bottom bar
   - Overlay backdrop, dismissible by swiping down or tapping backdrop
   - Smooth spring animation

8. apps/web/src/lib/stores/navigation.svelte.ts — Navigation state:
   - Use Svelte 5 runes (class-based store or module-level $state)
   - Tracks: current route, sidebar collapsed state, mobile drawer open state
   - Sidebar state persists to localStorage
   - Export functions: toggleSidebar(), openMobileDrawer(), closeMobileDrawer()

9. apps/web/src/lib/stores/page.svelte.ts — Page metadata state:
   - Tracks: page title, breadcrumbs, page actions
   - Child pages set their title/breadcrumbs on mount via $effect
   - Top bar reads from this store

10. apps/web/src/routes/(app)/dashboard/+page.svelte — Placeholder dashboard:
    - Simple page that says "Dashboard" with the correct layout rendering
    - Proves the shell works end-to-end
    - Sets page title via the page store

CRITICAL REQUIREMENTS:
- The sidebar must feel premium — smooth animations, subtle hover effects, professional spacing
- Navigation must work flawlessly via keyboard (Tab through items, Enter to navigate)
- The mobile experience must feel native — no janky transitions, proper safe area handling
- Sidebar width: expanded ~240px, collapsed ~64px
- The layout must not have any content shift when sidebar toggles
- All navigation items must use SvelteKit's enhanced link navigation (use <a> tags with href, not programmatic navigation)
- The quick-log trade button should be visually prominent enough that a trader instinctively reaches for it
- Z-indexes must follow the design system scale
- Scrolling behavior: sidebar nav scrolls independently if items overflow, main content scrolls independently

OUTPUT: Complete file contents for every file.
```

---

### PROMPT 0.5 — Supabase Configuration & Auth Setup

```
[PASTE GLOBAL CONTEXT BLOCK]

PHASE 0 — FOUNDATION & INFRASTRUCTURE
PROMPT 0.5 — Supabase Configuration & Auth Setup

OBJECTIVE: Configure Supabase client (browser + server), set up authentication hooks for SvelteKit, configure storage buckets, and create the auth helper utilities that every authenticated feature depends on.

DEPENDENCIES: Prompt 0.1 (project structure), Prompt 0.4 (layout with auth checks)

DELIVERABLES:

1. apps/web/src/lib/services/supabase/client.ts — Browser Supabase client:
   - Create browser client using @supabase/ssr createBrowserClient
   - Uses PUBLIC_SUPABASE_URL and PUBLIC_SUPABASE_ANON_KEY from environment
   - Singleton pattern — one client instance for the browser
   - Typed with Database generic from generated types

2. apps/web/src/lib/services/supabase/server.ts — Server Supabase client:
   - Factory function createServerClient(event: RequestEvent) using @supabase/ssr
   - Handles cookie-based session management with SvelteKit's cookies API
   - Returns typed Supabase client
   - Used in +page.server.ts and +layout.server.ts files

3. apps/web/src/lib/services/supabase/admin.ts — Admin/Service Role client:
   - Uses SUPABASE_SERVICE_ROLE_KEY (server-side only, NEVER exposed to browser)
   - For operations that need to bypass RLS (admin tasks, background jobs)
   - Includes runtime check that this is only used server-side

4. apps/web/src/hooks.server.ts — SvelteKit server hooks:
   - Creates Supabase server client for every request
   - Attaches client to event.locals.supabase
   - Loads and attaches session to event.locals.session
   - Handles auth token refresh automatically
   - Protects (app) routes — redirects to /login if no session
   - Allows (auth) routes without session
   - Sets proper cache headers for authenticated responses

5. apps/web/src/app.d.ts — Global type declarations:
   - Extend App.Locals with supabase client and session
   - Extend App.PageData with session
   - Full TypeScript types so every +page.server.ts knows about locals.supabase

6. apps/web/src/routes/(app)/+layout.server.ts — Authenticated layout loader:
   - Gets session from locals
   - Fetches user profile from user_profiles table
   - Returns session + profile to layout
   - If no profile exists (new user), redirects to onboarding

7. apps/web/src/routes/(app)/+layout.ts — Client-side layout:
   - Creates browser Supabase client
   - Sets up auth state change listener
   - Invalidates data on auth state change (login/logout)
   - Returns supabase client to page data

8. apps/web/src/lib/services/supabase/storage.ts — Storage utilities:
   - Helper functions for trade media uploads:
     - uploadTradeMedia(file: File, tradeId: string): Promise<string> — returns public URL
     - uploadAvatar(file: File, userId: string): Promise<string>
     - deleteFile(path: string): Promise<void>
     - getPublicUrl(path: string): string
   - File validation: max size (10MB for images, 50MB for recordings), allowed types (image/png, image/jpeg, image/webp, video/mp4)
   - Generates unique file paths: {userId}/{tradeId}/{timestamp}_{filename}
   - Error handling with user-friendly messages

9. apps/web/src/lib/stores/auth.svelte.ts — Auth state store:
   - Svelte 5 runes-based auth state
   - $state for: user, session, profile, isLoading, isAuthenticated
   - $derived for: userId, displayName, email, avatarUrl, tradingStyle, experienceLevel
   - Functions: signOut(), refreshSession()
   - Initializes from layout data, updates on auth state changes

10. apps/web/src/lib/types/database.ts — Database type definitions:
    - For now, manually define the types that match our schema (we'll generate these from Supabase later)
    - Define types for: User, UserProfile (with all fields from the features spec: trading_style, experience_level, account_size_range, default_risk_pct, timezone, session_times, goals, ai_personality, onboarding_completed)
    - Export Database type following Supabase conventions

CRITICAL REQUIREMENTS:
- Auth must handle edge cases: expired tokens, network failures, concurrent tab sessions
- The cookie-based auth must work with SvelteKit's SSR properly — no flashing of unauthenticated state
- Storage uploads must validate file types and sizes BEFORE uploading
- All Supabase operations must have proper error handling — never expose raw Supabase errors to users
- The auth store must be reactive — when session changes, every component reading auth state updates automatically
- Server-side client MUST use event.cookies for session management, not localStorage
- Environment variables must be validated at startup — if missing, throw clear error messages

OUTPUT: Complete file contents for every file.
```

---

### PROMPT 0.6 — Database Migrations: Core Schema (Users, Trades, Media, Tags)

```
[PASTE GLOBAL CONTEXT BLOCK]

PHASE 0 — FOUNDATION & INFRASTRUCTURE
PROMPT 0.6 — Database Migrations: Core Schema

OBJECTIVE: Create all Supabase SQL migrations for the core database tables. These tables are the foundation that every feature builds on. Every table must have Row Level Security policies, proper indexes, and correct foreign key relationships.

DEPENDENCIES: Prompt 0.5 (Supabase configuration)

DELIVERABLES:

Create SQL migration files in supabase/migrations/ with timestamps. Each migration is a separate file.

MIGRATION 1 — User Profiles:
File: supabase/migrations/001_user_profiles.sql

- user_profiles table:
  - id uuid PRIMARY KEY DEFAULT gen_random_uuid()
  - user_id uuid REFERENCES auth.users(id) ON DELETE CASCADE NOT NULL UNIQUE
  - display_name text
  - trading_style text CHECK (trading_style IN ('day_trading', 'swing_trading', 'scalping', 'position_trading'))
  - experience_level text CHECK (experience_level IN ('beginner', 'intermediate', 'advanced'))
  - account_size_range text CHECK (account_size_range IN ('under_5k', '5k_25k', '25k_50k', '50k_100k', '100k_500k', 'over_500k'))
  - default_risk_pct decimal(5,2) DEFAULT 1.00
  - timezone text DEFAULT 'America/New_York'
  - session_times jsonb DEFAULT '{"ny_open": true, "london": false, "asia": false}'::jsonb
  - goals text[] DEFAULT '{}'
  - ai_personality text DEFAULT 'balanced' CHECK (ai_personality IN ('strict_coach', 'encouraging_mentor', 'balanced'))
  - onboarding_completed boolean DEFAULT false
  - avatar_url text
  - created_at timestamptz DEFAULT now()
  - updated_at timestamptz DEFAULT now()

- RLS Policies:
  - Enable RLS
  - SELECT: users can only read their own profile (auth.uid() = user_id)
  - INSERT: users can only insert their own profile (auth.uid() = user_id)
  - UPDATE: users can only update their own profile (auth.uid() = user_id)
  - DELETE: users can only delete their own profile (auth.uid() = user_id)

- Trigger: auto-create profile on auth.users insert (with user_id = NEW.id)
- Trigger: auto-update updated_at on any row change
- Index: user_id (unique, already via constraint)

MIGRATION 2 — Tags:
File: supabase/migrations/002_tags.sql

- tags table:
  - id uuid PRIMARY KEY DEFAULT gen_random_uuid()
  - user_id uuid REFERENCES auth.users(id) ON DELETE CASCADE NOT NULL
  - name text NOT NULL
  - category text NOT NULL CHECK (category IN ('strategy', 'mistake', 'emotion', 'session', 'market_condition', 'custom'))
  - color text DEFAULT '#6366f1'
  - icon text
  - sort_order smallint DEFAULT 0
  - created_at timestamptz DEFAULT now()
  - UNIQUE(user_id, name)

- RLS: users access only their own tags
- Index: (user_id, category)
- Seed default tags for each category (created via a function called on profile creation):
  Strategies: Breakout, Mean Reversion, Scalping, Swing Pullback, VWAP Bounce, Trend Following, Gap Fill, Reversal
  Mistakes: FOMO Entry, Revenge Trading, Overtrading, Moving Stop, Early Exit, Late Entry, No Plan, Oversized
  Emotions: Confident, Anxious, Frustrated, Calm, Overexcited, Bored, Focused, Fearful
  Sessions: Pre-Market, First 30 Min, Mid-Morning, Midday, Afternoon, Power Hour, After Hours
  Market Conditions: Trending Up, Trending Down, Range Bound, High Volatility, Low Volume, News Driven, Choppy

MIGRATION 3 — Trades:
File: supabase/migrations/003_trades.sql

- trades table:
  - id uuid PRIMARY KEY DEFAULT gen_random_uuid()
  - user_id uuid REFERENCES auth.users(id) ON DELETE CASCADE NOT NULL
  - created_at timestamptz DEFAULT now()
  - updated_at timestamptz DEFAULT now()
  
  -- Instrument
  - symbol text NOT NULL
  - asset_class text NOT NULL CHECK (asset_class IN ('stock', 'option', 'forex', 'futures', 'crypto'))
  - direction text NOT NULL CHECK (direction IN ('long', 'short'))
  
  -- Entry
  - entry_price decimal(20,8)
  - entry_time timestamptz
  - entry_size decimal(20,8)
  
  -- Exit
  - exit_price decimal(20,8)
  - exit_time timestamptz
  - exit_size decimal(20,8)
  
  -- Risk
  - stop_loss decimal(20,8)
  - take_profit decimal(20,8)
  - r_multiple decimal(10,4)
  - planned_risk_pct decimal(5,2)
  - actual_risk_pct decimal(5,2)
  
  -- Financials
  - pnl decimal(20,2)
  - pnl_pct decimal(10,4)
  - commissions decimal(10,2) DEFAULT 0
  - slippage decimal(10,4)
  
  -- Metadata
  - status text DEFAULT 'open' CHECK (status IN ('open', 'closed', 'partial'))
  - duration_minutes integer
  - conviction_score smallint CHECK (conviction_score BETWEEN 1 AND 5)
  - trade_grade text CHECK (trade_grade IN ('A', 'B', 'C', 'D'))
  - grade_score decimal(5,2)
  - setup_name text
  - is_missed_trade boolean DEFAULT false
  
  -- Thesis
  - pre_trade_thesis text
  - thesis_alignment_score smallint CHECK (thesis_alignment_score BETWEEN 1 AND 5)
  
  -- Context
  - market_regime text CHECK (market_regime IN ('trending_up', 'trending_down', 'ranging', 'volatile', 'choppy', 'low_volume'))
  - sector text
  - vix_at_entry decimal(10,2)
  - volume_at_entry bigint
  - bid_ask_spread decimal(10,6)
  
  -- Scaling
  - is_scaled boolean DEFAULT false
  
  -- Overnight
  - held_overnight boolean DEFAULT false
  - gap_pct decimal(10,4)
  
  -- Session
  - session_type text CHECK (session_type IN ('premarket', 'first_30', 'mid_morning', 'midday', 'afternoon', 'power_hour', 'after_hours'))
  - hour_into_session smallint
  
  -- Source
  - entry_source text DEFAULT 'manual' CHECK (entry_source IN ('manual', 'broker_sync', 'csv_import'))
  - broker_connection_id uuid
  
  -- Notes
  - notes text
  - voice_note_url text
  - voice_note_sentiment text
  
  -- Soft delete
  - archived boolean DEFAULT false

- RLS: users access only their own trades
- Indexes:
  - (user_id, created_at DESC)
  - (user_id, symbol)
  - (user_id, status)
  - (user_id, asset_class)
  - (user_id, setup_name)
  - (user_id, entry_time DESC)
  - (user_id, archived)
  - (user_id, session_type)
  - (user_id, market_regime)
  - (user_id, trade_grade)
  - (user_id, conviction_score)
- Trigger: auto-update updated_at

MIGRATION 4 — Trade Tags (junction table):
File: supabase/migrations/004_trade_tags.sql

- trade_tags table:
  - trade_id uuid REFERENCES trades(id) ON DELETE CASCADE
  - tag_id uuid REFERENCES tags(id) ON DELETE CASCADE
  - PRIMARY KEY (trade_id, tag_id)

- RLS: users can only access trade_tags where they own the trade
- Index: (tag_id) for reverse lookups

MIGRATION 5 — Trade Legs (scaling):
File: supabase/migrations/005_trade_legs.sql

- trade_legs table:
  - id uuid PRIMARY KEY DEFAULT gen_random_uuid()
  - trade_id uuid REFERENCES trades(id) ON DELETE CASCADE NOT NULL
  - user_id uuid REFERENCES auth.users(id) ON DELETE CASCADE NOT NULL
  - leg_type text NOT NULL CHECK (leg_type IN ('entry', 'exit', 'add', 'trim'))
  - price decimal(20,8) NOT NULL
  - size decimal(20,8) NOT NULL
  - timestamp timestamptz NOT NULL
  - fees decimal(10,2) DEFAULT 0
  - notes text
  - sort_order smallint DEFAULT 0
  - created_at timestamptz DEFAULT now()

- RLS: users access only their own legs
- Index: (trade_id, sort_order)

MIGRATION 6 — Trade Media:
File: supabase/migrations/006_trade_media.sql

- trade_media table:
  - id uuid PRIMARY KEY DEFAULT gen_random_uuid()
  - trade_id uuid REFERENCES trades(id) ON DELETE CASCADE NOT NULL
  - user_id uuid REFERENCES auth.users(id) ON DELETE CASCADE NOT NULL
  - media_type text NOT NULL CHECK (media_type IN ('screenshot', 'recording', 'order_confirmation'))
  - storage_path text NOT NULL
  - thumbnail_path text
  - file_name text
  - file_size integer
  - mime_type text
  - ai_analysis jsonb
  - annotations jsonb
  - captured_at timestamptz DEFAULT now()
  - sort_order smallint DEFAULT 0
  - created_at timestamptz DEFAULT now()

- RLS: users access only their own media
- Index: (trade_id, sort_order)

ALSO DELIVER:

7. packages/types/src/database.ts — TypeScript types matching EVERY table above exactly:
   - Type for each table's row, insert, and update shapes
   - Following Supabase generated type conventions
   - Export a master Database type
   - Every field must match the SQL schema precisely — same names, same nullability, same defaults

8. apps/web/src/lib/types/index.ts — Re-export from @trademaster/types for convenience

CRITICAL REQUIREMENTS:
- Every table MUST have RLS enabled and policies defined — no exceptions
- Every foreign key MUST have ON DELETE CASCADE
- Use gen_random_uuid() for all UUIDs (built into PostgreSQL)
- Decimal precision must be sufficient for all asset classes (crypto needs 8 decimal places for prices like 0.00001234)
- All timestamptz columns — never use timestamp without timezone
- The auto-create profile trigger must fire on auth.users insert
- The updated_at trigger must be a reusable function applied to all tables that have updated_at
- Index selection must optimize for the most common query patterns: listing trades by date, filtering by tag/setup/asset, and analytics aggregations by user

OUTPUT: Complete SQL for every migration file, complete TypeScript for the types file.
```

---

### PROMPT 0.7 — Database Migrations: Planning, AI, Psychology, Risk Tables

```
[PASTE GLOBAL CONTEXT BLOCK]

PHASE 0 — FOUNDATION & INFRASTRUCTURE
PROMPT 0.7 — Database Migrations: Planning, AI, Psychology, Risk Tables

OBJECTIVE: Create all remaining database tables for daily planning, AI reviews, psychology tracking, risk management, playbook, gamification, accountability, and system features.

DEPENDENCIES: Prompt 0.6 (core tables exist)

DELIVERABLES:

Create SQL migration files continuing the numbering from Prompt 0.6.

MIGRATION 7 — Daily Plans & Watchlist:
File: supabase/migrations/007_daily_plans.sql

- daily_plans table:
  - id, user_id, plan_date (date, UNIQUE per user), market_bias, bias_reasoning
  - session_goals (jsonb array), max_trades (integer), max_daily_loss (decimal)
  - checklist_items (jsonb array of {text, required, checked})
  - ai_plan_of_attack (text — AI-generated summary)
  - notes (text)
  - carried_forward_from (uuid FK to daily_plans, nullable)
  - adherence_score (decimal — end of day AI assessment)
  - completed (boolean)
  - created_at, updated_at
  - UNIQUE(user_id, plan_date)
  - RLS, indexes on (user_id, plan_date DESC)

- watchlist_items table:
  - id, plan_id FK, user_id FK
  - symbol, key_levels (jsonb array of {price, type, notes})
  - catalysts (text), setup_description (text)
  - ai_setup_probability (decimal), risk_reward_ratio (decimal)
  - position_size_suggested (decimal)
  - was_traded (boolean), outcome (text: winner/loser/missed/no_setup)
  - sort_order, created_at
  - RLS, index on (plan_id)

MIGRATION 8 — AI Reviews & Chat:
File: supabase/migrations/008_ai_reviews.sql

- ai_reviews table:
  - id, trade_id FK (nullable — for weekly/monthly reviews not tied to single trade), user_id FK
  - review_type (trade/weekly/monthly/quarterly)
  - review_period_start (date, nullable), review_period_end (date, nullable)
  - overall_score (decimal 1-10)
  - strengths (jsonb array), weaknesses (jsonb array)
  - key_lesson (text), actionable_fixes (jsonb array)
  - alternative_scenario (text)
  - execution_quality_score (decimal), risk_management_score (decimal)
  - plan_adherence_score (decimal), emotional_state_detected (text)
  - prompt_version (text), model_used (text)
  - raw_response (text), tokens_used (integer)
  - cost_usd (decimal)
  - created_at
  - RLS, indexes on (user_id, created_at DESC), (trade_id)

- ai_review_messages table:
  - id, review_id FK, role (user/assistant), content (text)
  - tokens_used (integer), created_at
  - RLS, index on (review_id, created_at ASC)

MIGRATION 9 — Psychology & Mood:
File: supabase/migrations/009_psychology.sql

- mood_logs table:
  - id, user_id FK, log_date (date, UNIQUE per user)
  - pre_market_mood (smallint 1-10), post_market_mood (smallint 1-10)
  - stress_level (smallint 1-10), confidence_level (smallint 1-10)
  - sleep_quality (smallint 1-10)
  - emotions (text array), notes (text)
  - created_at, updated_at
  - RLS, index on (user_id, log_date DESC)

- trading_goals table:
  - id, user_id FK
  - goal_type (win_rate/profit_factor/consistency/risk_mgmt/journal/custom)
  - title, description, target_value (decimal), current_value (decimal)
  - target_date (date), status (active/achieved/abandoned)
  - created_at, updated_at
  - RLS, index on (user_id, status)

- tilt_events table:
  - id, user_id FK, detected_at (timestamptz)
  - trigger_type (rapid_trades/size_escalation/loss_chasing/daily_limit/watchlist_deviation)
  - severity (warning/alert/critical)
  - trade_ids (uuid array), details (jsonb)
  - was_acknowledged (boolean), user_action (text)
  - created_at
  - RLS, index on (user_id, detected_at DESC)

- alert_rules table:
  - id, user_id FK, rule_name (text)
  - condition (jsonb — {trigger_type, operator, value, window_minutes})
  - action (notify/cooldown/lockout/journal_required)
  - cooldown_minutes (integer), message (text)
  - is_active (boolean), times_triggered (integer)
  - created_at, updated_at
  - RLS, index on (user_id, is_active)

MIGRATION 10 — Playbook & Grading:
File: supabase/migrations/010_playbook.sql

- playbook_setups table:
  - id, user_id FK, name (text, UNIQUE per user)
  - description (text)
  - criteria (jsonb array of {criterion, weight, required, description})
  - example_screenshots (text array — storage paths)
  - expected_r_multiple (decimal), min_conviction (smallint)
  - timeframe (text), market_regime_filter (text array)
  - is_active (boolean)
  - total_trades (integer DEFAULT 0), win_rate (decimal), avg_r (decimal), profit_factor (decimal)
  - created_at, updated_at
  - RLS, index on (user_id, is_active)

- grading_rubrics table:
  - id, user_id FK, name (text)
  - criteria (jsonb array of {name, weight, scale_min, scale_max, description})
  - is_default (boolean DEFAULT false)
  - created_at, updated_at
  - RLS, index on (user_id, is_default)
  - Constraint: only one default per user

MIGRATION 11 — Risk, Market Context, Edge Score:
File: supabase/migrations/011_risk_and_scoring.sql

- market_snapshots table:
  - id, trade_id FK, user_id FK
  - tick (decimal), add_value (decimal), vold (decimal)
  - vix (decimal), vix_change (decimal)
  - spy_price (decimal), qqq_price (decimal)
  - sector_performance (jsonb), breadth (jsonb)
  - captured_at (timestamptz)
  - RLS, index on (trade_id)

- edge_score_history table:
  - id, user_id FK, score_date (date, UNIQUE per user)
  - composite_score (decimal 0-100)
  - plan_adherence_component (decimal)
  - grade_distribution_component (decimal)
  - risk_mgmt_component (decimal)
  - journal_quality_component (decimal)
  - emotional_stability_component (decimal)
  - pnl_normalized_component (decimal)
  - ai_explanation (text)
  - created_at
  - RLS, index on (user_id, score_date DESC)

MIGRATION 12 — Gamification, Accountability, Broker Connections:
File: supabase/migrations/012_social_and_system.sql

- user_streaks table:
  - id, user_id FK
  - streak_type (journal/planning/plan_adherence/review)
  - current_count (integer), decay_value (decimal)
  - best_count (integer), last_logged_at (timestamptz)
  - created_at, updated_at
  - RLS, UNIQUE(user_id, streak_type)

- shared_rulesets table:
  - id, creator_id FK (auth.users)
  - name, description, type (checklist/rubric/playbook)
  - data (jsonb), is_public (boolean)
  - import_count (integer DEFAULT 0)
  - created_at, updated_at
  - RLS (public ones readable by all, private by creator only)

- accountability_links table:
  - id, trader_id FK, coach_id FK
  - permissions (jsonb — {view_plan, view_grades, view_tilt, view_pnl, view_mood, view_reviews})
  - is_active (boolean), created_at, expires_at
  - RLS (accessible by both trader and coach)

- broker_connections table:
  - id, user_id FK
  - broker_name (text), display_name (text)
  - api_key_encrypted (text), api_secret_encrypted (text)
  - oauth_token_encrypted (text), oauth_refresh_token_encrypted (text)
  - last_sync_at (timestamptz), sync_status (text: active/error/paused)
  - sync_error_message (text)
  - is_active (boolean)
  - created_at, updated_at
  - RLS, index on (user_id, is_active)

- analytics_cache table:
  - id, user_id FK
  - cache_key (text), data (jsonb)
  - computed_at (timestamptz), expires_at (timestamptz)
  - RLS, UNIQUE(user_id, cache_key)
  - Index on (user_id, cache_key)
  - Index on (expires_at) for cleanup

- economic_events table:
  - id, event_date (timestamptz)
  - title (text), impact (high/medium/low)
  - currency (text), forecast (text), actual (text), previous (text)
  - ai_summary (text)
  - created_at
  - NO RLS (shared data, read-only for users)
  - Index on (event_date)

ALSO DELIVER:

Updated packages/types/src/database.ts with TypeScript types for ALL tables from this migration AND the previous migration (Prompt 0.6). This file should contain the COMPLETE database type definition — not just the additions.

CRITICAL REQUIREMENTS:
- Every table has RLS enabled
- Every foreign key has ON DELETE CASCADE
- Every jsonb column has a comment describing its expected structure
- UNIQUE constraints where specified (user_id + plan_date, user_id + streak_type, etc.)
- The updated_at trigger function from Prompt 0.6 is reused on all tables that have updated_at
- All CHECK constraints must be present as defined
- All indexes must be present as defined

OUTPUT: Complete SQL for every migration file, complete updated TypeScript types file.
```

---

### PROMPT 0.8 — API Client Layer & Service Architecture

```
[PASTE GLOBAL CONTEXT BLOCK]

PHASE 0 — FOUNDATION & INFRASTRUCTURE
PROMPT 0.8 — API Client Layer & Service Architecture

OBJECTIVE: Create the typed API client layer that all frontend features use to communicate with both Supabase directly and the Rust/Axum backend. This includes the base fetch wrapper, error handling, request/response typing, and the service modules for each domain.

DEPENDENCIES: Prompt 0.5 (Supabase client), Prompt 0.6 & 0.7 (database types)

DELIVERABLES:

1. apps/web/src/lib/services/api/client.ts — Base API client:
   - Typed fetch wrapper for the Rust/Axum backend
   - Automatically attaches Supabase auth token as Bearer header
   - Base URL from environment variable (PUBLIC_API_URL)
   - Methods: get<T>, post<T>, put<T>, patch<T>, delete<T>
   - Automatic JSON serialization/deserialization
   - Error handling: parse API errors into typed ApiError objects
   - Request timeout (30 seconds default, configurable)
   - Retry logic: 1 retry on 5xx errors with exponential backoff
   - Request/response interceptors for logging in development
   - Abort controller support for cancellable requests

2. apps/web/src/lib/services/api/errors.ts — Error types and handling:
   - ApiError class with: status, code, message, details
   - Error code enum: UNAUTHORIZED, FORBIDDEN, NOT_FOUND, VALIDATION, RATE_LIMITED, SERVER_ERROR, NETWORK_ERROR, TIMEOUT
   - parseApiError(response: Response): ApiError function
   - isApiError(error: unknown): error is ApiError type guard
   - User-friendly error message mapping (don't show raw error messages to users)

3. apps/web/src/lib/services/api/types.ts — Shared API types:
   - PaginatedResponse<T> { data: T[], total: number, page: number, pageSize: number, hasMore: boolean }
   - ApiResponse<T> { data: T, message?: string }
   - SortDirection = 'asc' | 'desc'
   - DateRange { from: string, to: string }
   - FilterOperator = 'eq' | 'neq' | 'gt' | 'gte' | 'lt' | 'lte' | 'in' | 'contains'

4. apps/web/src/lib/services/trades.ts — Trade service:
   - All trade CRUD operations via Supabase client (direct DB access with RLS)
   - listTrades(filters, sort, pagination): paginated, filterable trade list
   - getTrade(id): single trade with tags, media, legs
   - createTrade(data): insert trade + tags + return created
   - updateTrade(id, data): partial update
   - archiveTrade(id): soft delete (set archived = true)
   - restoreTrade(id): unarchive
   - getTradeWithRelations(id): trade + tags + media + legs + ai_reviews in one query
   - Filter types: by date range, symbol, asset_class, direction, status, tags, setup_name, conviction, grade, market_regime, session_type, archived
   - Sort options: created_at, entry_time, pnl, r_multiple, symbol

5. apps/web/src/lib/services/tags.ts — Tag service:
   - listTags(category?): all user tags, optionally filtered by category
   - createTag(data): create custom tag
   - updateTag(id, data): update tag name/color/icon
   - deleteTag(id): remove tag (cascade removes trade_tags entries)
   - addTagToTrade(tradeId, tagId): create junction record
   - removeTagFromTrade(tradeId, tagId): remove junction record
   - getTagsForTrade(tradeId): all tags on a specific trade
   - getTradesForTag(tagId): all trades with a specific tag

6. apps/web/src/lib/services/plans.ts — Daily plan service:
   - getPlan(date): get plan for a specific date
   - getTodayPlan(): shorthand for today
   - createPlan(data): create new daily plan
   - updatePlan(id, data): partial update
   - completePlan(id): mark as completed
   - getRecentPlans(limit): last N plans for review
   - Watchlist item CRUD within a plan
   - carryForward(fromPlanId): create new plan from previous day's unfinished items

7. apps/web/src/lib/services/media.ts — Trade media service:
   - uploadMedia(tradeId, file, mediaType): upload to Supabase Storage + create record
   - deleteMedia(id): remove record + delete from storage
   - getMediaForTrade(tradeId): all media for a trade
   - updateAnnotations(id, annotations): save user drawings/annotations
   - reorderMedia(tradeId, orderedIds): update sort_order

8. apps/web/src/lib/services/psychology.ts — Psychology service:
   - getMoodLog(date): get mood for specific date
   - saveMoodLog(data): create or update mood for a date
   - getGoals(status?): list goals
   - createGoal(data): new goal
   - updateGoal(id, data): update goal progress/status
   - getTiltEvents(dateRange): tilt events in range
   - getAlertRules(): active alert rules
   - createAlertRule(data): new rule
   - toggleAlertRule(id, active): enable/disable

9. apps/web/src/lib/services/ai.ts — AI service (for Rust backend calls):
   - requestTradeReview(tradeId, screenshotUrls): POST to Rust API, returns review
   - sendReviewMessage(reviewId, message): chat follow-up
   - requestDailyPlan(context): AI plan of attack generation
   - requestWeeklySummary(dateRange): weekly AI report
   - requestMonthlySummary(dateRange): monthly AI report
   - getReviewForTrade(tradeId): fetch existing review
   - getReviewMessages(reviewId): chat history
   - All methods return typed responses
   - Loading state management (AI calls can take 10-30 seconds)

10. apps/web/src/lib/stores/trades.svelte.ts — Trade state management:
    - Svelte 5 runes-based store
    - $state for: trades list, current trade, filters, sort, pagination, isLoading, error
    - $derived for: filtered count, has more pages
    - Methods: loadTrades(), loadMore(), setFilter(), clearFilters(), setSort()
    - Optimistic updates: when creating/updating a trade, update local state immediately, then sync with server
    - Error recovery: if server call fails, revert optimistic update and show toast

CRITICAL REQUIREMENTS:
- Every service function must have complete TypeScript types — input and output
- Every service function must handle errors and return typed results
- Supabase queries must select only needed columns (no select('*') — explicitly list columns)
- Pagination must work consistently across all list operations
- The API client must automatically handle auth token refresh if a 401 is returned
- All date/time handling must account for the user's timezone setting
- Service functions should be pure — no side effects, no direct DOM manipulation, no toast calls (the calling component handles UI feedback)

OUTPUT: Complete file contents for every file.
```

---

### PROMPT 0.9 — Auth Pages (Login, Register, Onboarding)

```
[PASTE GLOBAL CONTEXT BLOCK]

PHASE 0 — FOUNDATION & INFRASTRUCTURE
PROMPT 0.9 — Auth Pages (Login, Register, Onboarding)

OBJECTIVE: Create the complete authentication flow — login page, registration page, password reset, and the multi-step onboarding quiz. These are the first screens every user sees and must be polished, fast, and professional.

DEPENDENCIES: Prompt 0.3 (UI components), Prompt 0.4 (layout), Prompt 0.5 (Supabase auth)

DELIVERABLES:

1. apps/web/src/routes/(auth)/+layout.svelte — Auth layout:
   - Centered content layout, no sidebar
   - Dark background with subtle gradient or pattern
   - App logo prominently displayed
   - Clean, minimal design — no clutter

2. apps/web/src/routes/(auth)/login/+page.svelte — Login page:
   - Email + password form with validation (Zod schema)
   - "Sign in with Google" button (Supabase OAuth)
   - "Sign in with Apple" button (Supabase OAuth)
   - "Forgot password?" link
   - "Don't have an account? Sign up" link
   - Loading state on submit
   - Error display (invalid credentials, network error, etc.)
   - Redirect to /dashboard on success (or to onboarding if profile not completed)
   - Remember email option (localStorage)

3. apps/web/src/routes/(auth)/login/+page.server.ts — Login server action:
   - Handle form submission
   - Supabase signInWithPassword
   - Set session cookies
   - Redirect on success

4. apps/web/src/routes/(auth)/register/+page.svelte — Registration page:
   - Email + password + confirm password form
   - Password strength indicator (min 8 chars, uppercase, lowercase, number, special)
   - Google/Apple OAuth options
   - "Already have an account? Sign in" link
   - Terms acceptance checkbox (links to terms page)
   - Form validation with Zod
   - Loading state, error display
   - On success: redirect to onboarding

5. apps/web/src/routes/(auth)/register/+page.server.ts — Register server action

6. apps/web/src/routes/(auth)/forgot-password/+page.svelte — Password reset request:
   - Email input
   - Submit sends reset email via Supabase
   - Success message: "Check your email for a reset link"
   - Error handling

7. apps/web/src/routes/(auth)/reset-password/+page.svelte — Password reset form:
   - New password + confirm password
   - Password strength indicator
   - Handles the reset token from email link
   - Redirect to login on success

8. apps/web/src/routes/(auth)/onboarding/+page.svelte — Multi-step onboarding quiz:
   
   This is a multi-step wizard with progress indicator. 5 steps:
   
   STEP 1 — Trading Style:
   - Large, clickable cards for each option: Day Trading, Swing Trading, Scalping, Position Trading
   - Each card has an icon (Phosphor), title, and short description
   - Single select — clicking one deselects others
   - Visual feedback on selection (border highlight, subtle scale animation)
   
   STEP 2 — Primary Assets:
   - Multi-select cards: Stocks, Options, Forex, Futures, Crypto
   - Can select multiple
   - Same card style as Step 1
   
   STEP 3 — Experience Level:
   - Three cards: Beginner (< 1 year), Intermediate (1-3 years), Advanced (3+ years)
   - Each with appropriate description
   - Single select
   
   STEP 4 — Account & Risk:
   - Account size range dropdown: Under $5K, $5K-$25K, $25K-$50K, $50K-$100K, $100K-$500K, Over $500K
   - Default risk percentage slider or input (0.25% to 5%, default 1%)
   - Timezone auto-detected, editable dropdown
   - Active session times checkboxes: NY Open, London, Asia
   
   STEP 5 — Goals & AI Personality:
   - Goals multi-select: Improve consistency, Scale to full-time, Reduce losses, Build discipline, Grow account, Learn risk management
   - AI Personality selection (3 cards):
     - Strict Coach: "No sugarcoating. I'll tell you exactly what you did wrong."
     - Encouraging Mentor: "We'll focus on progress and build on your strengths."
     - Balanced: "Honest feedback with encouragement. Best of both worlds."
   
   Navigation:
   - Back/Next buttons
   - Progress bar showing current step
   - "Skip for now" option (sets defaults and completes onboarding)
   - Final step has "Start Trading" button that saves profile and redirects to /dashboard
   
   On completion:
   - Save all selections to user_profiles via Supabase
   - Set onboarding_completed = true
   - Create default tags for the user (calls the seed function)
   - Create a default grading rubric
   - Redirect to /dashboard with a welcome toast

9. apps/web/src/routes/(auth)/onboarding/+page.server.ts — Onboarding server logic:
   - Check if user already completed onboarding, redirect to /dashboard if so
   - Handle form submission to update user_profiles

10. apps/web/src/lib/components/auth/password-strength.svelte — Password strength indicator:
    - Visual bar that fills and changes color based on strength
    - Shows requirements checklist: length, uppercase, lowercase, number, special char
    - Each requirement shows check or X

11. apps/web/src/lib/components/auth/oauth-buttons.svelte — OAuth button group:
    - Google and Apple sign-in buttons with proper branding
    - Consistent styling, loading states
    - Handles OAuth redirect flow

CRITICAL REQUIREMENTS:
- The onboarding must feel smooth and premium — not like a boring form
- Card selections should have satisfying animations (scale, border glow, checkmark appear)
- The progress indicator must clearly show where the user is in the flow
- All form validation must show errors inline (not just at the top of the form)
- OAuth buttons must follow platform branding guidelines (Google's brand colors, Apple's black/white)
- The login page should auto-focus the email input on mount
- Password fields must have show/hide toggle
- All auth errors must be user-friendly: "Invalid email or password" not "AuthError: invalid_credentials"
- The onboarding quiz must save progress — if user refreshes mid-quiz, they resume where they left off (save to localStorage until final submit)
- Mobile experience must be perfect — cards should stack vertically, inputs should be full-width

OUTPUT: Complete file contents for every file.
```

---

### PROMPT 0.10 — Rust/Axum API Scaffolding

```
[PASTE GLOBAL CONTEXT BLOCK]

PHASE 0 — FOUNDATION & INFRASTRUCTURE
PROMPT 0.10 — Rust/Axum API Scaffolding

OBJECTIVE: Create the complete Rust/Axum backend API scaffold. This API handles computationally intensive operations (analytics calculations, Monte Carlo simulations), AI integration (Claude API calls), and broker sync operations. It does NOT duplicate what Supabase handles directly (basic CRUD with RLS).

DEPENDENCIES: Prompt 0.6 & 0.7 (database schema for model definitions)

DELIVERABLES:

1. apps/api/Cargo.toml — Dependencies:
   - axum (latest) — web framework
   - tokio (full features) — async runtime
   - serde, serde_json — serialization
   - sqlx (features: runtime-tokio, postgres, uuid, chrono, json, decimal) — database
   - tower, tower-http (cors, trace, timeout, compression) — middleware
   - tracing, tracing-subscriber — structured logging
   - reqwest — HTTP client (for Claude API, broker APIs)
   - jsonwebtoken — JWT validation (Supabase tokens)
   - uuid — UUID handling
   - chrono — date/time
   - rust_decimal — precise decimal math for financial calculations
   - dotenvy — environment variables
   - anyhow, thiserror — error handling
   - base64 — encoding for media

2. apps/api/src/main.rs — Application entry point:
   - Load environment variables
   - Initialize tracing/logging
   - Create database connection pool (sqlx::PgPool)
   - Build Axum router with all route groups
   - Apply middleware: CORS, request tracing, timeout, compression
   - Bind to configurable port
   - Graceful shutdown handling

3. apps/api/src/config.rs — Configuration:
   - Struct with all config fields loaded from env vars:
     - database_url, port, cors_origins
     - supabase_jwt_secret (for validating auth tokens)
     - anthropic_api_key
     - max_pool_connections
   - Validation on load — panic with clear messages if required vars missing

4. apps/api/src/error.rs — Error handling:
   - AppError enum with variants: BadRequest, Unauthorized, Forbidden, NotFound, Internal, Validation, RateLimited, AiError, BrokerError
   - Implement IntoResponse for AppError (returns proper HTTP status + JSON error body)
   - Error body format: { "error": { "code": "...", "message": "...", "details": ... } }
   - From<sqlx::Error>, From<reqwest::Error>, From<anyhow::Error> implementations

5. apps/api/src/middleware/auth.rs — Auth middleware:
   - Extract and validate Supabase JWT from Authorization: Bearer header
   - Decode JWT using supabase_jwt_secret
   - Extract user_id (sub claim) and attach to request extensions
   - Reject with 401 if no token, expired token, or invalid token
   - AuthUser extractor that handlers can use: async fn handler(user: AuthUser)

6. apps/api/src/middleware/mod.rs — Middleware barrel exports

7. apps/api/src/routes/mod.rs — Route registration:
   - Mount route groups:
     - /api/health — health check (no auth)
     - /api/v1/analytics — analytics endpoints (auth required)
     - /api/v1/ai — AI review endpoints (auth required)
     - /api/v1/risk — risk calculations (auth required)
     - /api/v1/brokers — broker sync (auth required)
   - Apply auth middleware to /api/v1/* routes

8. apps/api/src/routes/health.rs — Health check:
   - GET /api/health — returns { "status": "healthy", "version": "0.1.0", "timestamp": "..." }
   - Also checks database connectivity

9. apps/api/src/routes/analytics.rs — Analytics route stubs:
   - GET /api/v1/analytics/summary — core metrics (stub with types defined)
   - GET /api/v1/analytics/equity-curve — equity curve data (stub)
   - GET /api/v1/analytics/heatmap — calendar heatmap (stub)
   - GET /api/v1/analytics/monte-carlo — Monte Carlo simulation (stub)
   - All stubs return proper types but with TODO implementation
   - Define all request query param types and response types NOW even though implementation comes later

10. apps/api/src/routes/ai.rs — AI route stubs:
    - POST /api/v1/ai/review-trade — request trade review (stub)
    - POST /api/v1/ai/review-chat — follow-up chat (stub)
    - POST /api/v1/ai/plan-of-attack — generate daily plan (stub)
    - POST /api/v1/ai/weekly-summary — weekly report (stub)
    - All with types defined

11. apps/api/src/routes/risk.rs — Risk calculation stubs:
    - POST /api/v1/risk/position-size — calculate position size (stub)
    - GET /api/v1/risk/portfolio-heat — portfolio risk overview (stub)
    - POST /api/v1/risk/what-if — scenario simulation (stub)

12. apps/api/src/models/ — Data models:
    - mod.rs with barrel exports
    - user.rs — AuthUser struct
    - trade.rs — Trade struct matching DB schema
    - analytics.rs — Response types for analytics endpoints
    - ai.rs — Request/response types for AI endpoints
    - risk.rs — Request/response types for risk endpoints
    - All models derive Serialize, Deserialize, and use proper Rust types (Decimal, Uuid, DateTime<Utc>)

13. apps/api/src/services/ — Service layer stubs:
    - mod.rs
    - ai_engine.rs — Claude API integration (struct with methods, not implemented yet)
    - analytics_engine.rs — Analytics computation (struct with methods, not implemented yet)
    - Each service struct takes PgPool and config in constructor

14. Dockerfile for the Rust API:
    - Multi-stage build (builder + runtime)
    - Based on rust:slim for builder, debian:slim for runtime
    - Copies only the compiled binary
    - Runs as non-root user
    - Health check configured

15. fly.toml — Fly.io deployment configuration:
    - App name, region selection
    - Health check path
    - Environment variable references

CRITICAL REQUIREMENTS:
- All financial calculations MUST use rust_decimal (Decimal type), NEVER f64 for money
- The auth middleware must properly validate Supabase JWTs — this is the security boundary
- CORS must be configured to allow only the frontend origin
- Every route handler must use the AuthUser extractor — no unauthenticated access to /api/v1/*
- All request/response types must be fully defined even for stub endpoints
- Logging must include request_id, user_id, endpoint, duration for every request
- The database pool must have connection limits configured
- Error responses must never leak internal details (no stack traces, no SQL errors)

OUTPUT: Complete file contents for every file.
```

---

## PHASE SUMMARY

Phase 0 delivers 10 prompts that produce:

- Complete monorepo with SvelteKit 5 + TypeScript strict + Tailwind CSS 4
- Full design system (dark-mode-first, trading-optimized color palette)
- 20 production-ready shadcn-svelte base components
- Responsive app shell with sidebar + mobile bottom nav
- Supabase auth + storage + RLS configuration
- Complete database schema (30+ tables with migrations, RLS, indexes)
- Typed API client layer with service modules for every domain
- Auth flow (login, register, OAuth, password reset, multi-step onboarding)
- Rust/Axum API scaffolding with auth middleware, routes, and models

After Phase 0, the app compiles, the auth flow works end-to-end, the database is fully structured, and every subsequent phase can focus purely on features.

---

## NEXT: PHASE 1 — CORE TRADE LOGGING

Phase 1 prompts will be delivered in the next document. Phase 1 builds the trade entry form, trade list, trade detail view, screenshot upload, tagging, voice notes, CSV import, missed trade logging, and all related UI.

