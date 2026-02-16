<script lang="ts">
	import { type Snippet } from 'svelte';

	interface Props {
		for?: string;
		class?: string;
		required?: boolean;
		children: Snippet;
	}

	let { for: htmlFor, class: className = '', required = false, children }: Props = $props();

	const classes = `text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70 ${className}`;
</script>

<label for={htmlFor} class={classes}>
	{@render children()}
	{#if required}
		<span class="text-destructive ml-1">*</span>
	{/if}
</label>
