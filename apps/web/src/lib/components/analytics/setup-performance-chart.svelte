<script lang="ts">
	import Chart from './chart.svelte';
	import type { EChartsOption } from 'echarts';

	interface SetupPerf {
		setup_name: string;
		trade_count: number;
		win_rate: number;
		total_pnl: number;
		avg_pnl: number;
		avg_r_multiple: number | null;
	}

	interface Props {
		setups: SetupPerf[];
	}

	let { setups }: Props = $props();

	function buildOptions(): EChartsOption {
		const names = setups.map(s => s.setup_name);
		const pnl = setups.map(s => Number(s.total_pnl));
		const winRates = setups.map(s => Number(s.win_rate));

		return {
			backgroundColor: 'transparent',
			tooltip: {
				trigger: 'axis',
				backgroundColor: 'rgba(30,30,30,0.95)',
				borderColor: '#444',
				textStyle: { color: '#eee', fontSize: 12 },
			},
			legend: {
				data: ['Total P&L', 'Win Rate'],
				textStyle: { color: '#999' },
				top: 0
			},
			grid: { left: 60, right: 60, top: 40, bottom: 60 },
			xAxis: {
				type: 'category',
				data: names,
				axisLabel: { color: '#888', fontSize: 10, rotate: 20, interval: 0 },
				axisLine: { lineStyle: { color: '#333' } },
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
					name: 'Win Rate %',
					nameTextStyle: { color: '#888' },
					axisLabel: { color: '#888' },
					min: 0,
					max: 100,
					splitLine: { show: false }
				}
			],
			series: [
				{
					name: 'Total P&L',
					type: 'bar',
					data: pnl.map(v => ({
						value: v,
						itemStyle: { color: v >= 0 ? '#22c55e' : '#ef4444', borderRadius: [4, 4, 0, 0] }
					})),
				},
				{
					name: 'Win Rate',
					type: 'line',
					yAxisIndex: 1,
					data: winRates,
					symbol: 'circle',
					symbolSize: 8,
					lineStyle: { width: 2, color: '#f59e0b' },
					itemStyle: { color: '#f59e0b' }
				}
			]
		};
	}

	const options = $derived(buildOptions());
</script>

<Chart {options} height="320px" />
