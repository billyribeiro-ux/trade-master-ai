<script lang="ts">
	import { apiClient } from '$lib/api';
	import Button from '$lib/components/ui/button.svelte';
	import Card from '$lib/components/ui/card.svelte';
	import CardHeader from '$lib/components/ui/card-header.svelte';
	import CardTitle from '$lib/components/ui/card-title.svelte';
	import CardContent from '$lib/components/ui/card-content.svelte';
	import Spinner from '$lib/components/ui/spinner.svelte';
	import EquityCurve from '$lib/components/analytics/equity-curve.svelte';
	import WinLossChart from '$lib/components/analytics/win-loss-chart.svelte';
	import SetupPerformanceChart from '$lib/components/analytics/setup-performance-chart.svelte';
	import TimeCharts from '$lib/components/analytics/time-charts.svelte';
	import SeoHead from '$lib/components/layout/seo-head.svelte';
	import { formatCurrency } from '$lib/utils/format';
	import { onMount } from 'svelte';
	import type {
		EquityCurveData,
		WinLossDistribution,
		SetupPerformance,
		TimeBasedAnalytics,
		DrawdownData
	} from '$lib/types/analytics';

	let loading = $state(true);
	let equityCurve = $state<EquityCurveData | null>(null);
	let winLoss = $state<WinLossDistribution | null>(null);
	let setupPerf = $state<SetupPerformance[]>([]);
	let timeBased = $state<TimeBasedAnalytics | null>(null);
	let drawdown = $state<DrawdownData | null>(null);
	let activeTab = $state<'overview' | 'setups' | 'time'>('overview');

	onMount(async () => {
		await loadAnalytics();
	});

	async function loadAnalytics(): Promise<void> {
		loading = true;
		try {
			const [equity, distribution, setups, time, dd] = await Promise.all([
				apiClient.get<EquityCurveData>('/api/v1/analytics/equity-curve'),
				apiClient.get<WinLossDistribution>('/api/v1/analytics/win-loss-distribution'),
				apiClient.get<SetupPerformance[]>('/api/v1/analytics/setup-performance'),
				apiClient.get<TimeBasedAnalytics>('/api/v1/analytics/time-based'),
				apiClient.get<DrawdownData>('/api/v1/analytics/drawdown')
			]);

			equityCurve = equity;
			winLoss = distribution;
			setupPerf = setups ?? [];
			timeBased = time;
			drawdown = dd;
		} catch (error) {
			console.error('Failed to load analytics:', error);
		} finally {
			loading = false;
		}
	}

	const totalPnl = $derived(
		equityCurve?.points && equityCurve.points.length > 0
			? Number(equityCurve.points[equityCurve.points.length - 1]?.cumulative_pnl ?? 0)
			: 0
	);
	const totalTrades = $derived(
		(winLoss?.win_count ?? 0) + (winLoss?.loss_count ?? 0)
	);
	const winRate = $derived(
		totalTrades > 0 ? ((winLoss?.win_count ?? 0) / totalTrades * 100) : 0
	);
	const profitFactor = $derived(
		winLoss && Math.abs(winLoss.avg_loss) > 0
			? Math.abs(winLoss.avg_win * winLoss.win_count) /
			  Math.abs(winLoss.avg_loss * winLoss.loss_count)
			: 0
	);
</script>

<SeoHead
	title="Analytics"
	description="Deep dive into your trading analytics. View equity curves, win/loss distributions, setup performance, time-based patterns, and drawdown analysis."
	keywords={['trading analytics', 'equity curve', 'win loss ratio', 'setup performance', 'drawdown analysis', 'trading statistics']}
/>

<div class="p-6 max-w-7xl mx-auto space-y-6">
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold">Analytics</h1>
			<p class="text-muted-foreground mt-1">Analyze your trading performance</p>
		</div>
		<div class="flex gap-2">
			<Button variant={activeTab === 'overview' ? 'default' : 'outline'} onclick={() => activeTab = 'overview'}>
				{#snippet children()}Overview{/snippet}
			</Button>
			<Button variant={activeTab === 'setups' ? 'default' : 'outline'} onclick={() => activeTab = 'setups'}>
				{#snippet children()}Setups{/snippet}
			</Button>
			<Button variant={activeTab === 'time' ? 'default' : 'outline'} onclick={() => activeTab = 'time'}>
				{#snippet children()}Time Analysis{/snippet}
			</Button>
		</div>
	</div>

	{#if loading}
		<div class="flex justify-center py-16"><Spinner /></div>
	{:else}
		<!-- KPI Cards -->
		<div class="grid gap-4 md:grid-cols-4">
			<Card>
				<CardContent class="pt-6">
					<p class="text-sm text-muted-foreground">Total P&L</p>
					<p class="text-2xl font-bold {totalPnl >= 0 ? 'text-green-500' : 'text-red-500'}">
						{totalPnl >= 0 ? '+' : ''}{formatCurrency(totalPnl)}
					</p>
				</CardContent>
			</Card>
			<Card>
				<CardContent class="pt-6">
					<p class="text-sm text-muted-foreground">Total Trades</p>
					<p class="text-2xl font-bold">{totalTrades}</p>
				</CardContent>
			</Card>
			<Card>
				<CardContent class="pt-6">
					<p class="text-sm text-muted-foreground">Win Rate</p>
					<p class="text-2xl font-bold {winRate >= 50 ? 'text-green-500' : 'text-red-500'}">
						{winRate.toFixed(1)}%
					</p>
				</CardContent>
			</Card>
			<Card>
				<CardContent class="pt-6">
					<p class="text-sm text-muted-foreground">Profit Factor</p>
					<p class="text-2xl font-bold {profitFactor >= 1 ? 'text-green-500' : 'text-red-500'}">
						{profitFactor > 0 ? profitFactor.toFixed(2) : 'N/A'}
					</p>
				</CardContent>
			</Card>
		</div>

		{#if activeTab === 'overview'}
			<!-- Equity Curve -->
			{#if equityCurve && equityCurve.points.length > 0}
				<Card>
					<CardHeader><CardTitle>Equity Curve & Drawdown</CardTitle></CardHeader>
					<CardContent>
						<EquityCurve points={equityCurve.points} />
					</CardContent>
				</Card>
			{:else}
				<Card>
					<CardContent class="py-12 text-center text-muted-foreground">
						No closed trades yet. Start logging trades to see your equity curve.
					</CardContent>
				</Card>
			{/if}

			<!-- Win/Loss Distribution -->
			{#if winLoss && totalTrades > 0}
				<Card>
					<CardHeader><CardTitle>Win/Loss Distribution</CardTitle></CardHeader>
					<CardContent>
						<WinLossChart
							wins={winLoss.wins}
							losses={winLoss.losses}
							avg_win={winLoss.avg_win}
							avg_loss={winLoss.avg_loss}
						/>
					</CardContent>
				</Card>
			{/if}

			<!-- Drawdown Stats -->
			{#if drawdown}
				<Card>
					<CardHeader><CardTitle>Drawdown Analysis</CardTitle></CardHeader>
					<CardContent>
						<div class="grid gap-4 md:grid-cols-3">
							<div class="space-y-1">
								<p class="text-sm text-muted-foreground">Current Drawdown</p>
								<p class="text-2xl font-bold text-red-500">
									{drawdown.current_drawdown_pct.toFixed(2)}%
								</p>
							</div>
							<div class="space-y-1">
								<p class="text-sm text-muted-foreground">Max Drawdown</p>
								<p class="text-2xl font-bold text-red-500">
									{drawdown.max_drawdown_pct.toFixed(2)}%
								</p>
							</div>
							<div class="space-y-1">
								<p class="text-sm text-muted-foreground">Recovery Factor</p>
								<p class="text-2xl font-bold">
									{drawdown.recovery_factor !== null ? drawdown.recovery_factor.toFixed(2) : 'N/A'}
								</p>
							</div>
						</div>
					</CardContent>
				</Card>
			{/if}

		{:else if activeTab === 'setups'}
			<!-- Setup Performance Chart + Table -->
			{#if setupPerf.length > 0}
				<Card>
					<CardHeader><CardTitle>Setup Performance</CardTitle></CardHeader>
					<CardContent>
						<SetupPerformanceChart setups={setupPerf} />
					</CardContent>
				</Card>

				<Card>
					<CardHeader><CardTitle>Setup Details</CardTitle></CardHeader>
					<CardContent>
						<div class="overflow-x-auto">
							<table class="w-full text-sm">
								<thead class="border-b">
									<tr>
										<th class="px-4 py-3 text-left font-medium">Setup</th>
										<th class="px-4 py-3 text-right font-medium">Trades</th>
										<th class="px-4 py-3 text-right font-medium">Win Rate</th>
										<th class="px-4 py-3 text-right font-medium">Total P&L</th>
										<th class="px-4 py-3 text-right font-medium">Avg P&L</th>
										<th class="px-4 py-3 text-right font-medium">Avg R</th>
									</tr>
								</thead>
								<tbody>
									{#each setupPerf as setup}
										<tr class="border-b hover:bg-muted/50">
											<td class="px-4 py-3 font-medium">{setup.setup_name}</td>
											<td class="px-4 py-3 text-right">{setup.trade_count}</td>
											<td class="px-4 py-3 text-right">{Number(setup.win_rate).toFixed(1)}%</td>
											<td class="px-4 py-3 text-right font-medium {Number(setup.total_pnl) >= 0 ? 'text-green-500' : 'text-red-500'}">
												{formatCurrency(Number(setup.total_pnl))}
											</td>
											<td class="px-4 py-3 text-right">{formatCurrency(Number(setup.avg_pnl))}</td>
											<td class="px-4 py-3 text-right">
												{setup.avg_r_multiple ? `${Number(setup.avg_r_multiple).toFixed(2)}R` : 'N/A'}
											</td>
										</tr>
									{/each}
								</tbody>
							</table>
						</div>
					</CardContent>
				</Card>
			{:else}
				<Card>
					<CardContent class="py-12 text-center text-muted-foreground">
						No setup data yet. Log at least 3 trades with the same setup name to see performance.
					</CardContent>
				</Card>
			{/if}

		{:else if activeTab === 'time'}
			<!-- Time-Based Analytics -->
			{#if timeBased}
				<Card>
					<CardHeader><CardTitle>Time-Based Performance</CardTitle></CardHeader>
					<CardContent>
						<TimeCharts
							hourly={timeBased.hourly ?? []}
							daily={timeBased.daily ?? []}
							monthly={timeBased.monthly ?? []}
						/>
					</CardContent>
				</Card>
			{:else}
				<Card>
					<CardContent class="py-12 text-center text-muted-foreground">
						No time-based data yet. Close some trades to see performance by time.
					</CardContent>
				</Card>
			{/if}
		{/if}
	{/if}
</div>
