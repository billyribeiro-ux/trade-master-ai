# TradeMaster AI - Component Documentation

## Design System

### Colors
All colors are defined as CSS custom properties in `app.css` and support dark mode automatically.

**Primary Colors:**
- `--primary` - Main brand color (#6366f1 indigo)
- `--secondary` - Secondary accent
- `--accent` - Accent highlights
- `--destructive` - Error/danger states

**Semantic Colors:**
- `--success` - Success states (#10b981 green)
- `--warning` - Warning states (#f59e0b amber)
- `--error` - Error states (#ef4444 red)

### Typography
- **Font Family:** Inter (sans-serif)
- **Headings:** Bold weight, responsive sizing
- **Body:** Regular weight, 16px base size
- **Code:** JetBrains Mono (monospace)

### Spacing
Uses Tailwind's spacing scale (0.25rem increments):
- `xs`: 0.5rem (8px)
- `sm`: 0.75rem (12px)
- `md`: 1rem (16px)
- `lg`: 1.5rem (24px)
- `xl`: 2rem (32px)

---

## UI Components

### Button
**Location:** `src/lib/components/ui/button.svelte`

**Usage:**
```svelte
<script>
  import { Button } from '$components/ui/button';
</script>

<Button variant="default" size="md" onclick={() => console.log('clicked')}>
  Click me
</Button>
```

**Props:**
- `variant`: 'default' | 'destructive' | 'outline' | 'secondary' | 'ghost' | 'link'
- `size`: 'default' | 'sm' | 'lg' | 'icon'
- `disabled`: boolean
- `class`: string (additional CSS classes)

**Accessibility:**
- ✅ Keyboard accessible (Enter/Space)
- ✅ Focus visible styles
- ✅ Disabled state properly communicated
- ✅ ARIA attributes when needed

---

### Input
**Location:** `src/lib/components/ui/input.svelte`

**Usage:**
```svelte
<script>
  import { Input } from '$components/ui/input';
  let value = $state('');
</script>

<Input
  type="text"
  placeholder="Enter text..."
  bind:value={value}
/>
```

**Props:**
- `type`: 'text' | 'email' | 'password' | 'number' | 'date' | 'time'
- `value`: string | number
- `placeholder`: string
- `disabled`: boolean
- `required`: boolean
- `class`: string

**Accessibility:**
- ✅ Always use with `<Label>` component
- ✅ Supports `aria-describedby` for error messages
- ✅ Proper focus styles
- ✅ Disabled state

---

### Label
**Location:** `src/lib/components/ui/label.svelte`

**Usage:**
```svelte
<script>
  import { Label } from '$components/ui/label';
  import { Input } from '$components/ui/input';
</script>

<Label for="email">Email</Label>
<Input id="email" type="email" />
```

**Props:**
- `for`: string (input id)
- `class`: string

**Accessibility:**
- ✅ Always associate with form inputs
- ✅ Use `for` attribute to link to input `id`

---

### Dialog
**Location:** `src/lib/components/ui/dialog.svelte`

**Usage:**
```svelte
<script>
  import { Dialog } from '$components/ui/dialog';
  let open = $state(false);
</script>

<Dialog bind:open>
  {#snippet trigger()}
    <Button>Open Dialog</Button>
  {/snippet}
  
  {#snippet content()}
    <h2>Dialog Title</h2>
    <p>Dialog content goes here</p>
  {/snippet}
</Dialog>
```

**Props:**
- `open`: boolean (bindable)
- `trigger`: Snippet
- `content`: Snippet

**Accessibility:**
- ✅ Focus trap when open
- ✅ Escape key to close
- ✅ Click outside to close
- ✅ `aria-modal="true"`
- ✅ Proper focus management
- ⚠️ **TODO:** Add focus trap implementation

---

### Select
**Location:** `src/lib/components/ui/select.svelte`

**Usage:**
```svelte
<script>
  import { Select } from '$components/ui/select';
  let selected = $state('');
  
  const options = [
    { value: 'option1', label: 'Option 1' },
    { value: 'option2', label: 'Option 2' }
  ];
</script>

<Select bind:value={selected} {options} placeholder="Select an option" />
```

**Props:**
- `value`: string (bindable)
- `options`: Array<{value: string, label: string}>
- `placeholder`: string
- `disabled`: boolean

**Accessibility:**
- ✅ Keyboard navigation (Arrow keys, Enter, Escape)
- ✅ Proper ARIA attributes
- ✅ Focus management

---

### Spinner
**Location:** `src/lib/components/ui/spinner.svelte`

**Usage:**
```svelte
<script>
  import { Spinner } from '$components/ui/spinner';
</script>

<Spinner size="md" label="Loading..." />
```

**Props:**
- `size`: 'sm' | 'md' | 'lg' | 'xl'
- `label`: string (for screen readers)
- `class`: string

**Accessibility:**
- ✅ `role="status"`
- ✅ `aria-live="polite"`
- ✅ `aria-label` for screen readers
- ✅ Hidden text for context

---

## Layout Components

### SeoHead
**Location:** `src/lib/components/layout/seo-head.svelte`

**Usage:**
```svelte
<script>
  import { SeoHead } from '$components/layout/seo-head.svelte';
</script>

<SeoHead
  title="Dashboard | TradeMaster AI"
  description="View your trading performance and analytics"
  keywords={['trading', 'analytics', 'dashboard']}
/>
```

**Props:**
- `title`: string (required)
- `description`: string
- `keywords`: string[]
- `ogImage`: string (URL)
- `canonical`: string (URL)

**Features:**
- ✅ Automatic Open Graph tags
- ✅ Twitter Card tags
- ✅ Canonical URL
- ✅ Meta keywords
- ✅ Responsive viewport

---

## Best Practices

### Component Patterns

**1. Use Svelte 5 Runes:**
```svelte
<script>
  // ✅ Good - Svelte 5
  let count = $state(0);
  let doubled = $derived(count * 2);
  
  // ❌ Bad - Svelte 4
  let count = 0;
  $: doubled = count * 2;
</script>
```

**2. Use Snippets over Slots:**
```svelte
<script>
  import type { Snippet } from 'svelte';
  
  interface Props {
    children: Snippet;
    header?: Snippet;
  }
  
  let { children, header }: Props = $props();
</script>

{#if header}
  {@render header()}
{/if}

{@render children()}
```

**3. Type Your Props:**
```svelte
<script lang="ts">
  interface Props {
    title: string;
    count?: number;
    onclick?: () => void;
  }
  
  let { title, count = 0, onclick }: Props = $props();
</script>
```

### Accessibility Checklist

- ✅ All interactive elements keyboard accessible
- ✅ Proper ARIA labels and roles
- ✅ Focus visible styles
- ✅ Color contrast meets WCAG AA (4.5:1)
- ✅ Touch targets minimum 44x44px
- ✅ Skip navigation link
- ✅ Semantic HTML
- ✅ Form labels associated with inputs
- ✅ Error messages announced to screen readers

### Performance

- ✅ Lazy load heavy components (ECharts)
- ✅ Use `$derived` for computed values
- ✅ Avoid unnecessary reactivity
- ✅ Optimize images (WebP, lazy loading)
- ✅ Code splitting by route

---

## Testing Components

See `src/test/utils.ts` for testing utilities.

**Example:**
```typescript
import { describe, it, expect } from 'vitest';
import { renderComponent } from '$test/utils';
import Button from '$components/ui/button.svelte';

describe('Button', () => {
  it('should render with text', () => {
    const { getByText } = renderComponent(Button, {
      children: 'Click me'
    });
    
    expect(getByText('Click me')).toBeInTheDocument();
  });
});
```

