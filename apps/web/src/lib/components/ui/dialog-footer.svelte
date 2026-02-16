<script lang="ts">
	import { type Snippet } from 'svelte';

	interface Props {
		class?: string;
		children: Snippet;
	}

	let { class: className = '', children }: Props = $props();

	const classes = `flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2 ${className}`;
</script>

<div class={classes}>
	{@render children()}
</div>
