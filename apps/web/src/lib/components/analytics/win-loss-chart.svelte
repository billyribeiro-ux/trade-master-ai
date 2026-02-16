<script lang="ts">
	import Chart from './chart.svelte';
	import type { EChartsOption } from 'echarts';

	interface Props {
		wins: number[];
		losses: number[];
		avg_win: number;
		avg_loss: number;
	}

	let { wins, losses, avg_win, avg_loss }: Props = $props();

	function buildOptions(): EChartsOption {
		const allValues = [...wins, ...losses.map(v => Math.abs(v))];
		const maxVal = Math.max(...allValues, 1);
		const bucketSize = Math.ceil(maxVal / 15);
		const buckets: number[] = [];
		const winCounts: number[] = [];
		const lossCounts: number[] = [];

		for (let i = 0; i <= 15; i++) {
			buckets.push(i * bucketSize);
			winCounts.push(0);
			lossCounts.push(0);
		}

		for (const w of wins) {
			const idx = Math.min(Math.floor(w / bucketSize), 15);
			winCounts[idx] = (winCounts[idx] ?? 0) + 1;
		}
		for (const l of losses) {
			const idx = Math.min(Math.floor(Math.abs(l) / bucketSize), 15);
			lossCounts[idx] = (lossCounts[idx] ?? 0) + 1;
		}

		const labels = buckets.map(b => `$${b}`);

		return {
			backgroundColor: 'transparent',
			tooltip: {
				trigger: 'axis',
				backgroundColor: 'rgba(30,30,30,0.95)',
				borderColor: '#444',
				textStyle: { color: '#eee', fontSize: 12 },
			},
			legend: {
				data: ['Wins', 'Losses'],
				textStyle: { color: '#999' },
				top: 0
			},
			grid: { left: 50, right: 20, top: 40, bottom: 30 },
			xAxis: {
				type: 'category',
				data: labels,
				axisLabel: { color: '#888', fontSize: 10, rotate: 30 },
				axisLine: { lineStyle: { color: '#333' } },
			},
			yAxis: {
				type: 'value',
				name: 'Trades',
				nameTextStyle: { color: '#888' },
				axisLabel: { color: '#888' },
				splitLine: { lineStyle: { color: '#222' } }
			},
			series: [
				{
					name: 'Wins',
					type: 'bar',
					data: winCounts,
					itemStyle: { color: '#22c55e', borderRadius: [4, 4, 0, 0] },
					barGap: '10%'
				},
				{
					name: 'Losses',
					type: 'bar',
					data: lossCounts,
					itemStyle: { color: '#ef4444', borderRadius: [4, 4, 0, 0] }
				}
			]
		};
	}

	const options = $derived(buildOptions());
</script>

<div class="space-y-3">
	<Chart {options} height="280px" />
	<div class="flex justify-center gap-8 text-sm">
		<div class="text-center">
			<span class="text-muted-foreground">Avg Win</span>
			<p class="font-bold text-green-500">+${Number(avg_win).toFixed(2)}</p>
		</div>
		<div class="text-center">
			<span class="text-muted-foreground">Avg Loss</span>
			<p class="font-bold text-red-500">${Number(avg_loss).toFixed(2)}</p>
		</div>
		<div class="text-center">
			<span class="text-muted-foreground">Win/Loss Ratio</span>
			<p class="font-bold">{avg_loss !== 0 ? (Math.abs(Number(avg_win) / Number(avg_loss))).toFixed(2) : 'N/A'}</p>
		</div>
	</div>
</div>
