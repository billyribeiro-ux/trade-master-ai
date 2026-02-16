<script lang="ts">
	import { type Snippet } from 'svelte';

	interface Props {
		class?: string;
		children: Snippet;
	}

	let { class: className = '', children }: Props = $props();

	const classes = `flex items-center p-6 pt-0 ${className}`;
</script>

<div class={classes}>
	{@render children()}
</div>
