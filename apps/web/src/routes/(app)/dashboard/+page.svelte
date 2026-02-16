<script lang="ts">
	import { tradesApi } from '$lib/api';
	import Card from '$lib/components/ui/card.svelte';
	import CardHeader from '$lib/components/ui/card-header.svelte';
	import CardTitle from '$lib/components/ui/card-title.svelte';
	import CardContent from '$lib/components/ui/card-content.svelte';
	import Badge from '$lib/components/ui/badge.svelte';
	import Button from '$lib/components/ui/button.svelte';
	import { formatCurrency, formatPercent } from '$lib/utils/format';
	import { onMount } from 'svelte';
	import type { TradeStats } from '$lib/types/trade';

	let stats = $state<TradeStats | null>(null);
	let loading = $state(true);

	onMount(async () => {
		try {
			stats = await tradesApi.getStats();
		} catch (error) {
			console.error('Failed to load stats:', error);
		} finally {
			loading = false;
		}
	});
</script>

<svelte:head>
	<title>Dashboard - TradeMaster AI</title>
</svelte:head>

<div class="p-6 space-y-6">
	<div class="flex items-center justify-between">
		<h1 class="text-3xl font-bold">Dashboard</h1>
		<Button>
			{#snippet children()}
				<a href="/trades/new">Log Trade</a>
			{/snippet}
		</Button>
	</div>

	{#if loading}
		<div class="text-muted-foreground">Loading stats...</div>
	{:else if stats}
		<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
			<Card>
				<CardHeader>
					<CardTitle class="text-sm font-medium text-muted-foreground">Total P&L</CardTitle>
				</CardHeader>
				<CardContent>
					<div class="text-2xl font-bold" class:text-success={stats.total_pnl > 0} class:text-destructive={stats.total_pnl < 0}>
						{formatCurrency(stats.total_pnl)}
					</div>
					<p class="text-xs text-muted-foreground mt-1">
						{stats.total_trades} trades
					</p>
				</CardContent>
			</Card>

			<Card>
				<CardHeader>
					<CardTitle class="text-sm font-medium text-muted-foreground">Win Rate</CardTitle>
				</CardHeader>
				<CardContent>
					<div class="text-2xl font-bold">
						{stats.win_rate.toFixed(1)}%
					</div>
					<p class="text-xs text-muted-foreground mt-1">
						{stats.winning_trades}W / {stats.losing_trades}L
					</p>
				</CardContent>
			</Card>

			<Card>
				<CardHeader>
					<CardTitle class="text-sm font-medium text-muted-foreground">Profit Factor</CardTitle>
				</CardHeader>
				<CardContent>
					<div class="text-2xl font-bold">
						{stats.profit_factor?.toFixed(2) ?? 'N/A'}
					</div>
					<p class="text-xs text-muted-foreground mt-1">
						Avg Win: {formatCurrency(stats.avg_win)}
					</p>
				</CardContent>
			</Card>

			<Card>
				<CardHeader>
					<CardTitle class="text-sm font-medium text-muted-foreground">Avg R-Multiple</CardTitle>
				</CardHeader>
				<CardContent>
					<div class="text-2xl font-bold" class:text-success={stats.avg_r_multiple && stats.avg_r_multiple > 0} class:text-destructive={stats.avg_r_multiple && stats.avg_r_multiple < 0}>
						{stats.avg_r_multiple?.toFixed(2) ?? 'N/A'}R
					</div>
					<p class="text-xs text-muted-foreground mt-1">
						Risk/Reward ratio
					</p>
				</CardContent>
			</Card>
		</div>

		<div class="grid gap-4 md:grid-cols-2">
			<Card>
				<CardHeader>
					<CardTitle>Performance Summary</CardTitle>
				</CardHeader>
				<CardContent class="space-y-3">
					<div class="flex justify-between">
						<span class="text-sm text-muted-foreground">Largest Win</span>
						<span class="text-sm font-medium text-success">{formatCurrency(stats.largest_win)}</span>
					</div>
					<div class="flex justify-between">
						<span class="text-sm text-muted-foreground">Largest Loss</span>
						<span class="text-sm font-medium text-destructive">{formatCurrency(stats.largest_loss)}</span>
					</div>
					<div class="flex justify-between">
						<span class="text-sm text-muted-foreground">Avg Loss</span>
						<span class="text-sm font-medium">{formatCurrency(stats.avg_loss)}</span>
					</div>
					<div class="flex justify-between">
						<span class="text-sm text-muted-foreground">Avg Hold Time</span>
						<span class="text-sm font-medium">
							{stats.avg_hold_time_minutes ? `${Math.floor(stats.avg_hold_time_minutes / 60)}h ${stats.avg_hold_time_minutes % 60}m` : 'N/A'}
						</span>
					</div>
				</CardContent>
			</Card>

			<Card>
				<CardHeader>
					<CardTitle>Quick Actions</CardTitle>
				</CardHeader>
				<CardContent class="space-y-2">
					<Button variant="outline" class="w-full justify-start">
						{#snippet children()}
							<a href="/trades/new" class="w-full text-left">Log New Trade</a>
						{/snippet}
					</Button>
					<Button variant="outline" class="w-full justify-start">
						{#snippet children()}
							<a href="/trades" class="w-full text-left">View All Trades</a>
						{/snippet}
					</Button>
					<Button variant="outline" class="w-full justify-start">
						{#snippet children()}
							<a href="/analytics" class="w-full text-left">View Analytics</a>
						{/snippet}
					</Button>
					<Button variant="outline" class="w-full justify-start">
						{#snippet children()}
							<a href="/planning" class="w-full text-left">Create Daily Plan</a>
						{/snippet}
					</Button>
				</CardContent>
			</Card>
		</div>
	{:else}
		<Card>
			<CardContent class="py-12 text-center">
				<p class="text-muted-foreground mb-4">No trades yet. Start by logging your first trade!</p>
				<Button>
					{#snippet children()}
						<a href="/trades/new">Log Your First Trade</a>
					{/snippet}
				</Button>
			</CardContent>
		</Card>
	{/if}
</div>
