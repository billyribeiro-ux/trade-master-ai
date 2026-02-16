<script lang="ts">
	import { type Snippet } from 'svelte';

	interface Props {
		class?: string;
		children: Snippet;
	}

	let { class: className = '', children }: Props = $props();

	const classes = `flex flex-col space-y-1.5 text-center sm:text-left ${className}`;
</script>

<div class={classes}>
	{@render children()}
</div>
