<script lang="ts">
	import { type Snippet } from 'svelte';

	interface Props {
		class?: string;
		children: Snippet;
	}

	let { class: className = '', children }: Props = $props();

	const classes = `text-sm text-muted-foreground ${className}`;
</script>

<p class={classes}>
	{@render children()}
</p>
