<script lang="ts">
	interface Props {
		type?: string;
		value?: string | number;
		placeholder?: string;
		disabled?: boolean;
		required?: boolean;
		class?: string;
		id?: string;
		name?: string;
		min?: string | number;
		max?: string | number;
		step?: string | number;
		oninput?: (e: Event & { currentTarget: HTMLInputElement }) => void;
		onchange?: (e: Event & { currentTarget: HTMLInputElement }) => void;
		onblur?: (e: FocusEvent & { currentTarget: HTMLInputElement }) => void;
	}

	let {
		type = 'text',
		value = $bindable(''),
		placeholder,
		disabled = false,
		required = false,
		class: className = '',
		id,
		name,
		min,
		max,
		step,
		oninput,
		onchange,
		onblur
	}: Props = $props();

	const classes = `flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 ${className}`;
</script>

<input
	{type}
	bind:value
	{placeholder}
	{disabled}
	{required}
	{id}
	{name}
	{min}
	{max}
	{step}
	class={classes}
	{oninput}
	{onchange}
	{onblur}
/>
