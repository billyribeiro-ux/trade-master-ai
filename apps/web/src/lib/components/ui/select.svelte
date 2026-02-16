<script lang="ts">
	interface Props {
		value?: string | number;
		placeholder?: string;
		disabled?: boolean;
		required?: boolean;
		class?: string;
		id?: string;
		name?: string;
		options: Array<{ value: string | number; label: string; disabled?: boolean }>;
		onchange?: (e: Event & { currentTarget: HTMLSelectElement }) => void;
	}

	let {
		value = $bindable(''),
		placeholder = 'Select an option',
		disabled = false,
		required = false,
		class: className = '',
		id,
		name,
		options,
		onchange
	}: Props = $props();

	const classes = `flex h-10 w-full items-center justify-between rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 ${className}`;
</script>

<select bind:value {disabled} {required} {id} {name} class={classes} {onchange}>
	{#if placeholder}
		<option value="" disabled selected={!value}>{placeholder}</option>
	{/if}
	{#each options as option}
		<option value={option.value} disabled={option.disabled}>
			{option.label}
		</option>
	{/each}
</select>
