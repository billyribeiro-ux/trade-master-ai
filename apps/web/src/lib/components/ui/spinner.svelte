<script lang="ts">
	interface Props {
		size?: 'sm' | 'md' | 'lg' | 'xl';
		class?: string;
		label?: string;
	}

	let { size = 'md', class: className = '', label = 'Loading' }: Props = $props();

	const sizes = {
		sm: 'h-4 w-4 border-2',
		md: 'h-8 w-8 border-2',
		lg: 'h-12 w-12 border-3',
		xl: 'h-16 w-16 border-4'
	};

	const classes = $derived(`animate-spin rounded-full border-current border-t-transparent ${sizes[size]} ${className}`);
</script>

<div class={classes} role="status" aria-live="polite" aria-label={label}>
	<span class="sr-only">{label}...</span>
</div>
