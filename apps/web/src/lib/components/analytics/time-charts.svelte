<script lang="ts">
	import Chart from './chart.svelte';

	interface HourlyPerf {
		hour: number;
		trade_count: number;
		win_rate: number;
		avg_pnl: number;
	}

	interface DailyPerf {
		day_of_week: number;
		day_name: string;
		trade_count: number;
		win_rate: number;
		avg_pnl: number;
	}

	interface MonthlyPerf {
		month: string;
		trade_count: number;
		total_pnl: number;
		win_rate: number;
	}

	interface Props {
		hourly: HourlyPerf[];
		daily: DailyPerf[];
		monthly: MonthlyPerf[];
	}

	let { hourly, daily, monthly }: Props = $props();

	function buildHourlyOptions() {
		const hours = hourly.map(h => `${h.hour}:00`);
		const pnl = hourly.map(h => Number(h.avg_pnl));
		const counts = hourly.map(h => h.trade_count);

		return {
			backgroundColor: 'transparent',
			tooltip: {
				trigger: 'axis',
				backgroundColor: 'rgba(30,30,30,0.95)',
				borderColor: '#444',
				textStyle: { color: '#eee', fontSize: 12 },
			},
			legend: { data: ['Avg P&L', 'Trades'], textStyle: { color: '#999' }, top: 0 },
			grid: { left: 50, right: 50, top: 40, bottom: 30 },
			xAxis: {
				type: 'category',
				data: hours,
				axisLabel: { color: '#888', fontSize: 10 },
				axisLine: { lineStyle: { color: '#333' } },
			},
			yAxis: [
				{
					type: 'value',
					name: 'Avg P&L',
					nameTextStyle: { color: '#888' },
					axisLabel: { color: '#888' },
					splitLine: { lineStyle: { color: '#222' } }
				},
				{
					type: 'value',
					name: 'Trades',
					nameTextStyle: { color: '#888' },
					axisLabel: { color: '#888' },
					splitLine: { show: false }
				}
			],
			series: [
				{
					name: 'Avg P&L',
					type: 'bar',
					data: pnl.map(v => ({
						value: v,
						itemStyle: { color: v >= 0 ? '#22c55e' : '#ef4444', borderRadius: [4, 4, 0, 0] }
					})),
				},
				{
					name: 'Trades',
					type: 'line',
					yAxisIndex: 1,
					data: counts,
					symbol: 'circle',
					symbolSize: 6,
					lineStyle: { width: 2, color: '#8b5cf6' },
					itemStyle: { color: '#8b5cf6' }
				}
			]
		};
	}

	function buildDailyOptions() {
		const days = daily.map(d => d.day_name.trim());
		const pnl = daily.map(d => Number(d.avg_pnl));
		const winRates = daily.map(d => Number(d.win_rate));

		return {
			backgroundColor: 'transparent',
			tooltip: {
				trigger: 'axis',
				backgroundColor: 'rgba(30,30,30,0.95)',
				borderColor: '#444',
				textStyle: { color: '#eee', fontSize: 12 },
			},
			legend: { data: ['Avg P&L', 'Win Rate'], textStyle: { color: '#999' }, top: 0 },
			grid: { left: 50, right: 50, top: 40, bottom: 30 },
			xAxis: {
				type: 'category',
				data: days,
				axisLabel: { color: '#888', fontSize: 11 },
				axisLine: { lineStyle: { color: '#333' } },
			},
			yAxis: [
				{
					type: 'value',
					name: 'Avg P&L',
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
					name: 'Avg P&L',
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

	function buildMonthlyOptions() {
		const months = monthly.map(m => m.month);
		const pnl = monthly.map(m => Number(m.total_pnl));

		return {
			backgroundColor: 'transparent',
			tooltip: {
				trigger: 'axis',
				backgroundColor: 'rgba(30,30,30,0.95)',
				borderColor: '#444',
				textStyle: { color: '#eee', fontSize: 12 },
			},
			grid: { left: 50, right: 20, top: 20, bottom: 30 },
			xAxis: {
				type: 'category',
				data: months,
				axisLabel: { color: '#888', fontSize: 10 },
				axisLine: { lineStyle: { color: '#333' } },
			},
			yAxis: {
				type: 'value',
				name: 'P&L ($)',
				nameTextStyle: { color: '#888' },
				axisLabel: { color: '#888' },
				splitLine: { lineStyle: { color: '#222' } }
			},
			series: [
				{
					type: 'bar',
					data: pnl.map(v => ({
						value: v,
						itemStyle: { color: v >= 0 ? '#22c55e' : '#ef4444', borderRadius: [4, 4, 0, 0] }
					})),
				}
			]
		};
	}

	const hourlyOpts = $derived(buildHourlyOptions());
	const dailyOpts = $derived(buildDailyOptions());
	const monthlyOpts = $derived(buildMonthlyOptions());
</script>

<div class="space-y-6">
	{#if hourly.length > 0}
		<div>
			<h3 class="text-sm font-semibold text-muted-foreground mb-2">Performance by Hour</h3>
			<Chart options={hourlyOpts} height="280px" />
		</div>
	{/if}

	<div class="grid gap-6 md:grid-cols-2">
		{#if daily.length > 0}
			<div>
				<h3 class="text-sm font-semibold text-muted-foreground mb-2">Performance by Day of Week</h3>
				<Chart options={dailyOpts} height="260px" />
			</div>
		{/if}

		{#if monthly.length > 0}
			<div>
				<h3 class="text-sm font-semibold text-muted-foreground mb-2">Monthly P&L</h3>
				<Chart options={monthlyOpts} height="260px" />
			</div>
		{/if}
	</div>
</div>
