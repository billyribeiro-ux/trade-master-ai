<script lang="ts">
	interface Props {
		checked?: boolean;
		disabled?: boolean;
		required?: boolean;
		class?: string;
		id?: string;
		name?: string;
		value?: string;
		onchange?: (e: Event & { currentTarget: HTMLInputElement }) => void;
	}

	let {
		checked = $bindable(false),
		disabled = false,
		required = false,
		class: className = '',
		id,
		name,
		value,
		onchange
	}: Props = $props();

	const classes = `peer h-4 w-4 shrink-0 rounded-sm border border-primary ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground ${className}`;
</script>

<input
	type="checkbox"
	bind:checked
	{disabled}
	{required}
	{id}
	{name}
	{value}
	class={classes}
	data-state={checked ? 'checked' : 'unchecked'}
	{onchange}
/>
