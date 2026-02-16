<script lang="ts">
	interface Props {
		value?: string;
		placeholder?: string;
		disabled?: boolean;
		required?: boolean;
		class?: string;
		id?: string;
		name?: string;
		rows?: number;
		maxlength?: number;
		oninput?: (e: Event & { currentTarget: HTMLTextAreaElement }) => void;
		onchange?: (e: Event & { currentTarget: HTMLTextAreaElement }) => void;
	}

	let {
		value = $bindable(''),
		placeholder,
		disabled = false,
		required = false,
		class: className = '',
		id,
		name,
		rows = 3,
		maxlength,
		oninput,
		onchange
	}: Props = $props();

	const classes = `flex min-h-[80px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 ${className}`;
</script>

<textarea
	bind:value
	{placeholder}
	{disabled}
	{required}
	{id}
	{name}
	{rows}
	{maxlength}
	class={classes}
	{oninput}
	{onchange}
></textarea>
