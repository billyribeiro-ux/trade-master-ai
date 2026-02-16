<script lang="ts">
	import { apiClient } from '$lib/api';
	import Card from '$lib/components/ui/card.svelte';
	import CardHeader from '$lib/components/ui/card-header.svelte';
	import CardTitle from '$lib/components/ui/card-title.svelte';
	import CardContent from '$lib/components/ui/card-content.svelte';
	import { formatCurrency, formatPercent } from '$lib/utils/format';
	import { onMount } from 'svelte';

	let loading = $state(true);
	let equityCurve = $state<any>(null);
	let winLoss = $state<any>(null);
	let setupPerf = $state<any[]>([]);
	let timeBased = $state<any>(null);
	let drawdown = $state<any>(null);

	onMount(async () => {
		await loadAnalytics();
	});

	async function loadAnalytics() {
		loading = true;
		try {
			const [equity, distribution, setups, time, dd] = await Promise.all([
				apiClient.get('/api/v1/analytics/equity-curve'),
				apiClient.get('/api/v1/analytics/win-loss-distribution'),
				apiClient.get('/api/v1/analytics/setup-performance'),
				apiClient.get('/api/v1/analytics/time-based'),
				apiClient.get('/api/v1/analytics/drawdown')
			]);

			equityCurve = equity;
			winLoss = distribution;
			setupPerf = setups;
			timeBased = time;
			drawdown = dd;
		} catch (error) {
			console.error('Failed to load analytics:', error);
		} finally {
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>Analytics - TradeMaster AI</title>
</svelte:head>

<div class="p-6 space-y-6">
	<div>
		<h1 class="text-3xl font-bold">Analytics</h1>
		<p class="text-muted-foreground mt-1">Analyze your trading performance</p>
	</div>

	{#if loading}
		<div class="text-center py-12 text-muted-foreground">Loading analytics...</div>
	{:else}
		<!-- Equity Curve -->
		{#if equityCurve && equityCurve.points.length > 0}
			<Card>
				<CardHeader>
					<CardTitle>Equity Curve</CardTitle>
				</CardHeader>
				<CardContent>
					<div class="h-64 flex items-center justify-center border rounded-lg">
						<div class="text-center">
							<p class="text-sm text-muted-foreground mb-2">Chart visualization</p>
							<p class="text-xs text-muted-foreground">
								{equityCurve.points.length} trades tracked
							</p>
							<p class="text-lg font-bold mt-2">
								Final P&L: {formatCurrency(equityCurve.points[equityCurve.points.length - 1].cumulative_pnl)}
							</p>
						</div>
					</div>
				</CardContent>
			</Card>
		{/if}

		<!-- Win/Loss Distribution -->
		{#if winLoss}
			<div class="grid gap-6 md:grid-cols-2">
				<Card>
					<CardHeader>
						<CardTitle>Win Distribution</CardTitle>
					</CardHeader>
					<CardContent class="space-y-3">
						<div class="flex justify-between">
							<span class="text-sm text-muted-foreground">Total Wins</span>
							<span class="text-sm font-medium text-success">{winLoss.wins.length}</span>
						</div>
						<div class="flex justify-between">
							<span class="text-sm text-muted-foreground">Average Win</span>
							<span class="text-sm font-medium text-success">{formatCurrency(winLoss.avg_win)}</span>
						</div>
						<div class="flex justify-between">
							<span class="text-sm text-muted-foreground">Largest Win</span>
							<span class="text-sm font-medium text-success">{formatCurrency(winLoss.largest_win)}</span>
						</div>
					</CardContent>
				</Card>

				<Card>
					<CardHeader>
						<CardTitle>Loss Distribution</CardTitle>
					</CardHeader>
					<CardContent class="space-y-3">
						<div class="flex justify-between">
							<span class="text-sm text-muted-foreground">Total Losses</span>
							<span class="text-sm font-medium text-destructive">{winLoss.losses.length}</span>
						</div>
						<div class="flex justify-between">
							<span class="text-sm text-muted-foreground">Average Loss</span>
							<span class="text-sm font-medium text-destructive">{formatCurrency(winLoss.avg_loss)}</span>
						</div>
						<div class="flex justify-between">
							<span class="text-sm text-muted-foreground">Largest Loss</span>
							<span class="text-sm font-medium text-destructive">{formatCurrency(winLoss.largest_loss)}</span>
						</div>
					</CardContent>
				</Card>
			</div>
		{/if}

		<!-- Setup Performance -->
		{#if setupPerf.length > 0}
			<Card>
				<CardHeader>
					<CardTitle>Setup Performance</CardTitle>
				</CardHeader>
				<CardContent>
					<div class="overflow-x-auto">
						<table class="w-full">
							<thead class="border-b">
								<tr>
									<th class="px-4 py-3 text-left text-sm font-medium">Setup</th>
									<th class="px-4 py-3 text-right text-sm font-medium">Trades</th>
									<th class="px-4 py-3 text-right text-sm font-medium">Win Rate</th>
									<th class="px-4 py-3 text-right text-sm font-medium">Total P&L</th>
									<th class="px-4 py-3 text-right text-sm font-medium">Avg P&L</th>
									<th class="px-4 py-3 text-right text-sm font-medium">Avg R</th>
								</tr>
							</thead>
							<tbody>
								{#each setupPerf as setup}
									<tr class="border-b hover:bg-muted/50">
										<td class="px-4 py-3 font-medium">{setup.setup_name}</td>
										<td class="px-4 py-3 text-right text-sm">{setup.trade_count}</td>
										<td class="px-4 py-3 text-right text-sm">{setup.win_rate.toFixed(1)}%</td>
										<td class="px-4 py-3 text-right font-medium {setup.total_pnl > 0 ? 'text-success' : 'text-destructive'}">
											{formatCurrency(setup.total_pnl)}
										</td>
										<td class="px-4 py-3 text-right text-sm">{formatCurrency(setup.avg_pnl)}</td>
										<td class="px-4 py-3 text-right text-sm">
											{setup.avg_r_multiple ? `${setup.avg_r_multiple.toFixed(2)}R` : 'N/A'}
										</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
				</CardContent>
			</Card>
		{/if}

		<!-- Time-Based Analytics -->
		{#if timeBased}
			<div class="grid gap-6 md:grid-cols-2">
				<Card>
					<CardHeader>
						<CardTitle>Performance by Day of Week</CardTitle>
					</CardHeader>
					<CardContent>
						<div class="space-y-2">
							{#each timeBased.daily as day}
								<div class="flex justify-between items-center">
									<span class="text-sm font-medium">{day.day_name.trim()}</span>
									<div class="flex gap-4 text-sm">
										<span class="text-muted-foreground">{day.trade_count} trades</span>
										<span class="font-medium">{day.win_rate.toFixed(0)}% WR</span>
										<span class="font-medium {day.avg_pnl > 0 ? 'text-success' : 'text-destructive'}">
											{formatCurrency(day.avg_pnl)}
										</span>
									</div>
								</div>
							{/each}
						</div>
					</CardContent>
				</Card>

				<Card>
					<CardHeader>
						<CardTitle>Monthly Performance</CardTitle>
					</CardHeader>
					<CardContent>
						<div class="space-y-2">
							{#each timeBased.monthly as month}
								<div class="flex justify-between items-center">
									<span class="text-sm font-medium">{month.month}</span>
									<div class="flex gap-4 text-sm">
										<span class="text-muted-foreground">{month.trade_count} trades</span>
										<span class="font-medium">{month.win_rate.toFixed(0)}% WR</span>
										<span class="font-medium {month.total_pnl > 0 ? 'text-success' : 'text-destructive'}">
											{formatCurrency(month.total_pnl)}
										</span>
									</div>
								</div>
							{/each}
						</div>
					</CardContent>
				</Card>
			</div>
		{/if}

		<!-- Drawdown Analysis -->
		{#if drawdown}
			<Card>
				<CardHeader>
					<CardTitle>Drawdown Analysis</CardTitle>
				</CardHeader>
				<CardContent>
					<div class="grid gap-4 md:grid-cols-3">
						<div class="space-y-1">
							<p class="text-sm text-muted-foreground">Current Drawdown</p>
							<p class="text-2xl font-bold text-destructive">
								{formatCurrency(drawdown.current_drawdown)}
							</p>
						</div>
						<div class="space-y-1">
							<p class="text-sm text-muted-foreground">Max Drawdown</p>
							<p class="text-2xl font-bold text-destructive">
								{formatCurrency(drawdown.max_drawdown)}
							</p>
						</div>
						<div class="space-y-1">
							<p class="text-sm text-muted-foreground">Recovery Factor</p>
							<p class="text-2xl font-bold">
								{drawdown.recovery_factor ? drawdown.recovery_factor.toFixed(2) : 'N/A'}
							</p>
						</div>
					</div>
				</CardContent>
			</Card>
		{/if}
	{/if}
</div>
