<script lang="ts">
	import Chart from './chart.svelte';

	interface Props {
		points: Array<{ date: string; cumulative_pnl: number; trade_count: number }>;
	}

	let { points }: Props = $props();

	function buildOptions() {
		const dates = points.map(p => new Date(p.date).toLocaleDateString());
		const pnl = points.map(p => Number(p.cumulative_pnl));

		let peak = 0;
		const drawdown = pnl.map(v => {
			if (v > peak) peak = v;
			return peak - v;
		});

		return {
			backgroundColor: 'transparent',
			tooltip: {
				trigger: 'axis',
				backgroundColor: 'rgba(30,30,30,0.95)',
				borderColor: '#444',
				textStyle: { color: '#eee', fontSize: 12 },
			},
			legend: {
				data: ['Cumulative P&L', 'Drawdown'],
				textStyle: { color: '#999' },
				top: 0
			},
			grid: { left: 60, right: 60, top: 40, bottom: 30 },
			xAxis: {
				type: 'category',
				data: dates,
				axisLabel: { color: '#888', fontSize: 10 },
				axisLine: { lineStyle: { color: '#333' } },
				splitLine: { show: false }
			},
			yAxis: [
				{
					type: 'value',
					name: 'P&L ($)',
					nameTextStyle: { color: '#888' },
					axisLabel: { color: '#888' },
					splitLine: { lineStyle: { color: '#222' } }
				},
				{
					type: 'value',
					name: 'Drawdown',
					nameTextStyle: { color: '#888' },
					axisLabel: { color: '#888' },
					inverse: true,
					splitLine: { show: false }
				}
			],
			series: [
				{
					name: 'Cumulative P&L',
					type: 'line',
					data: pnl,
					smooth: true,
					symbol: 'none',
					lineStyle: { width: 2, color: '#3b82f6' },
					areaStyle: {
						color: {
							type: 'linear',
							x: 0, y: 0, x2: 0, y2: 1,
							colorStops: [
								{ offset: 0, color: 'rgba(59,130,246,0.3)' },
								{ offset: 1, color: 'rgba(59,130,246,0.02)' }
							]
						} as any
					}
				},
				{
					name: 'Drawdown',
					type: 'line',
					yAxisIndex: 1,
					data: drawdown,
					smooth: true,
					symbol: 'none',
					lineStyle: { width: 1, color: '#ef4444', type: 'dashed' },
					areaStyle: {
						color: {
							type: 'linear',
							x: 0, y: 0, x2: 0, y2: 1,
							colorStops: [
								{ offset: 0, color: 'rgba(239,68,68,0.15)' },
								{ offset: 1, color: 'rgba(239,68,68,0.02)' }
							]
						} as any
					}
				}
			]
		};
	}

	const options = $derived(buildOptions());
</script>

<Chart {options} height="360px" />
