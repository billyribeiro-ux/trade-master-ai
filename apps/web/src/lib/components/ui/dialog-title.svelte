<script lang="ts">
	interface Props {
		class?: string;
		children: import('svelte').Snippet;
	}

	let { class: className = '', children }: Props = $props();

	const classes = `text-lg font-semibold leading-none tracking-tight ${className}`;
</script>

<h2 class={classes}>
	{@render children()}
</h2>
