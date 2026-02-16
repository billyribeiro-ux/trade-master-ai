<script lang="ts">
	interface Props {
		score: number;
		max?: number;
		size?: number;
		label?: string;
	}

	let { score, max = 10, size = 100, label = '' }: Props = $props();

	const percentage = $derived(Math.min(score / max, 1));
	const circumference = $derived(2 * Math.PI * 40);
	const dashOffset = $derived(circumference * (1 - percentage));
	const color = $derived(
		score >= max * 0.8 ? '#22c55e' :
		score >= max * 0.6 ? '#f59e0b' :
		score >= max * 0.4 ? '#f97316' : '#ef4444'
	);
</script>

<div class="flex flex-col items-center gap-1">
	<svg width={size} height={size} viewBox="0 0 100 100">
		<circle cx="50" cy="50" r="40" fill="none" stroke="#333" stroke-width="8" />
		<circle
			cx="50" cy="50" r="40"
			fill="none"
			stroke={color}
			stroke-width="8"
			stroke-linecap="round"
			stroke-dasharray={circumference}
			stroke-dashoffset={dashOffset}
			transform="rotate(-90 50 50)"
			style="transition: stroke-dashoffset 0.8s ease-out, stroke 0.3s;"
		/>
		<text x="50" y="50" text-anchor="middle" dominant-baseline="central"
			fill={color} font-size="24" font-weight="bold">
			{score.toFixed(1)}
		</text>
	</svg>
	{#if label}
		<span class="text-xs text-muted-foreground font-medium">{label}</span>
	{/if}
</div>
