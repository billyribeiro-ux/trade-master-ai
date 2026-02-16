<script lang="ts">
	import { type Snippet } from 'svelte';
	import { fly } from 'svelte/transition';

	interface Props {
		variant?: 'default' | 'destructive' | 'success';
		title?: string;
		description?: string;
		duration?: number;
		onClose?: () => void;
		children?: Snippet;
	}

	let {
		variant = 'default',
		title,
		description,
		duration = 5000,
		onClose,
		children
	}: Props = $props();

	let visible = $state(true);

	$effect(() => {
		if (duration > 0) {
			const timer = setTimeout(() => {
				visible = false;
				setTimeout(() => onClose?.(), 300);
			}, duration);

			return () => clearTimeout(timer);
		}
	});

	const variants = {
		default: 'border bg-background text-foreground',
		destructive: 'destructive group border-destructive bg-destructive text-destructive-foreground',
		success: 'border-success bg-success text-success-foreground'
	};

	const classes = `pointer-events-auto relative flex w-full items-center justify-between space-x-4 overflow-hidden rounded-md border p-6 pr-8 shadow-lg transition-all ${variants[variant]}`;
</script>

{#if visible}
	<div class={classes} transition:fly={{ y: -100, duration: 300 }}>
		<div class="grid gap-1">
			{#if title}
				<div class="text-sm font-semibold">{title}</div>
			{/if}
			{#if description}
				<div class="text-sm opacity-90">{description}</div>
			{/if}
			{#if children}
				{@render children()}
			{/if}
		</div>
		<button
			type="button"
			class="absolute right-2 top-2 rounded-md p-1 text-foreground/50 opacity-0 transition-opacity hover:text-foreground focus:opacity-100 focus:outline-none focus:ring-2 group-hover:opacity-100"
			onclick={() => {
				visible = false;
				setTimeout(() => onClose?.(), 300);
			}}
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				width="16"
				height="16"
				viewBox="0 0 24 24"
				fill="none"
				stroke="currentColor"
				stroke-width="2"
				stroke-linecap="round"
				stroke-linejoin="round"
			>
				<path d="M18 6 6 18" />
				<path d="m6 6 12 12" />
			</svg>
		</button>
	</div>
{/if}
