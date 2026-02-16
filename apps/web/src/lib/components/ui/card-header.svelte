<script lang="ts">
	interface Props {
		class?: string;
		children: import('svelte').Snippet;
	}

	let { class: className = '', children }: Props = $props();

	const classes = `flex flex-col space-y-1.5 p-6 ${className}`;
</script>

<div class={classes}>
	{@render children()}
</div>
