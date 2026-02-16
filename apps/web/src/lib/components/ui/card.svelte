<script lang="ts">
	import { type Snippet } from 'svelte';

	interface Props {
		class?: string;
		children: Snippet;
	}

	let { class: className = '', children }: Props = $props();

	const classes = `rounded-lg border bg-card text-card-foreground shadow-sm ${className}`;
</script>

<div class={classes}>
	{@render children()}
</div>
