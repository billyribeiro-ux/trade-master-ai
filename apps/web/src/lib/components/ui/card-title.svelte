<script lang="ts">
	import { type Snippet } from 'svelte';

	interface Props {
		class?: string;
		children: Snippet;
	}

	let { class: className = '', children }: Props = $props();

	const classes = `text-2xl font-semibold leading-none tracking-tight ${className}`;
</script>

<h3 class={classes}>
	{@render children()}
</h3>
