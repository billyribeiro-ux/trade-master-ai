<script lang="ts">
	import { tradesApi, apiClient } from '$lib/api';
	import Card from '$lib/components/ui/card.svelte';
	import CardHeader from '$lib/components/ui/card-header.svelte';
	import CardTitle from '$lib/components/ui/card-title.svelte';
	import CardContent from '$lib/components/ui/card-content.svelte';
	import Badge from '$lib/components/ui/badge.svelte';
	import Spinner from '$lib/components/ui/spinner.svelte';
	import EquityCurve from '$lib/components/analytics/equity-curve.svelte';
	import SeoHead from '$lib/components/layout/seo-head.svelte';
	import { formatCurrency } from '$lib/utils/format';
	import { onMount } from 'svelte';
	import type { TradeStats } from '$lib/types/trade';

	interface RecentTrade {
		id: string;
		symbol: string;
		direction: string;
		pnl: number | null;
		r_multiple: number | null;
		entry_datetime: string;
		setup_type: string | null;
	}

	interface EquityPoint {
		date: string;
		cumulative_pnl: number;
		trade_count: number;
	}

	let stats = $state<TradeStats | null>(null);
	let recentTrades = $state<RecentTrade[]>([]);
	let equityData = $state<EquityPoint[]>([]);
	let loading = $state(true);

	onMount(async () => {
		try {
			const [s, trades, equity] = await Promise.all([
				tradesApi.getStats(),
				apiClient.get<RecentTrade[]>('/api/v1/trades?limit=5'),
				apiClient.get<EquityPoint[]>('/api/v1/analytics/equity-curve'),
			]);
			stats = s;
			recentTrades = trades ?? [];
			equityData = equity ?? [];
		} catch (error) {
			console.error('Failed to load dashboard:', error);
		} finally {
			loading = false;
		}
	});

	function pnlColor(pnl: number | null): string {
		if (pnl == null) return '';
		return pnl >= 0 ? 'text-green-500' : 'text-red-500';
	}

	function timeAgo(dateStr: string): string {
		const diff = Date.now() - new Date(dateStr).getTime();
		const mins = Math.floor(diff / 60000);
		if (mins < 60) return `${mins}m ago`;
		const hrs = Math.floor(mins / 60);
		if (hrs < 24) return `${hrs}h ago`;
		const days = Math.floor(hrs / 24);
		return `${days}d ago`;
	}

	const quickLinks = [
		{ href: '/trades/new', label: 'Log Trade', icon: '📝' },
		{ href: '/planning', label: 'Daily Plan', icon: '📋' },
		{ href: '/ai-review', label: 'AI Review', icon: '🤖' },
		{ href: '/analytics', label: 'Analytics', icon: '📊' },
		{ href: '/psychology', label: 'Mood Log', icon: '🧠' },
		{ href: '/playbook', label: 'Playbook', icon: '📖' },
	];
</script>

<SeoHead
	title="Dashboard"
	description="View your trading performance metrics, recent trades, and equity curve. Track your win rate, profit factor, and R-multiples in real-time."
	keywords={['trading dashboard', 'trading metrics', 'equity curve', 'win rate', 'profit factor', 'trading journal']}
/>

<div class="p-6 space-y-6 max-w-7xl mx-auto">
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold">Dashboard</h1>
			<p class="text-muted-foreground mt-1">
				{new Date().toLocaleDateString('en-US', { weekday: 'long', month: 'long', day: 'numeric' })}
			</p>
		</div>
		<a href="/trades/new" class="px-4 py-2 bg-primary text-primary-foreground rounded-lg text-sm font-medium hover:bg-primary/90 transition-colors">
			+ Log Trade
		</a>
	</div>

	{#if loading}
		<div class="flex justify-center py-12"><Spinner /></div>
	{:else if stats}
		<!-- KPI Cards -->
		<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-4">
			<Card>
				<CardContent class="pt-6">
					<div class="flex items-center justify-between">
						<div>
							<p class="text-xs text-muted-foreground uppercase font-medium">Total P&L</p>
							<div class="text-2xl font-bold mt-1 {stats.total_pnl >= 0 ? 'text-green-500' : 'text-red-500'}">
								{formatCurrency(stats.total_pnl)}
							</div>
						</div>
						<div class="w-12 h-12 rounded-full flex items-center justify-center {stats.total_pnl >= 0 ? 'bg-green-500/10' : 'bg-red-500/10'}">
							<span class="text-xl">{stats.total_pnl >= 0 ? '📈' : '📉'}</span>
						</div>
					</div>
					<p class="text-xs text-muted-foreground mt-2">{stats.total_trades} total trades</p>
				</CardContent>
			</Card>

			<Card>
				<CardContent class="pt-6">
					<div class="flex items-center justify-between">
						<div>
							<p class="text-xs text-muted-foreground uppercase font-medium">Win Rate</p>
							<div class="text-2xl font-bold mt-1">{stats.win_rate.toFixed(1)}%</div>
						</div>
						<div class="w-12 h-12 rounded-full bg-blue-500/10 flex items-center justify-center">
							<span class="text-xl">🎯</span>
						</div>
					</div>
					<div class="flex items-center gap-2 mt-2">
						<span class="text-xs text-green-500">{stats.winning_trades}W</span>
						<span class="text-xs text-muted-foreground">/</span>
						<span class="text-xs text-red-500">{stats.losing_trades}L</span>
						<!-- Mini win rate bar -->
						<div class="flex-1 h-1.5 bg-red-500/30 rounded-full overflow-hidden">
							<div class="h-full bg-green-500 rounded-full" style="width: {stats.win_rate}%"></div>
						</div>
					</div>
				</CardContent>
			</Card>

			<Card>
				<CardContent class="pt-6">
					<div class="flex items-center justify-between">
						<div>
							<p class="text-xs text-muted-foreground uppercase font-medium">Profit Factor</p>
							<div class="text-2xl font-bold mt-1">{stats.profit_factor?.toFixed(2) ?? 'N/A'}</div>
						</div>
						<div class="w-12 h-12 rounded-full bg-purple-500/10 flex items-center justify-center">
							<span class="text-xl">⚖️</span>
						</div>
					</div>
					<p class="text-xs text-muted-foreground mt-2">
						Avg Win: <span class="text-green-500">{formatCurrency(stats.avg_win)}</span>
					</p>
				</CardContent>
			</Card>

			<Card>
				<CardContent class="pt-6">
					<div class="flex items-center justify-between">
						<div>
							<p class="text-xs text-muted-foreground uppercase font-medium">Avg R-Multiple</p>
							<div class="text-2xl font-bold mt-1 {(stats.avg_r_multiple ?? 0) >= 0 ? 'text-green-500' : 'text-red-500'}">
								{stats.avg_r_multiple?.toFixed(2) ?? 'N/A'}R
							</div>
						</div>
						<div class="w-12 h-12 rounded-full bg-yellow-500/10 flex items-center justify-center">
							<span class="text-xl">💰</span>
						</div>
					</div>
					<p class="text-xs text-muted-foreground mt-2">
						Hold: {stats.avg_hold_time_minutes ? `${Math.floor(stats.avg_hold_time_minutes / 60)}h ${Math.round(stats.avg_hold_time_minutes % 60)}m` : 'N/A'}
					</p>
				</CardContent>
			</Card>
		</div>

		<div class="grid gap-6 lg:grid-cols-3">
			<!-- Equity Curve (takes 2 cols) -->
			<div class="lg:col-span-2">
				<Card>
					<CardHeader><CardTitle>Equity Curve</CardTitle></CardHeader>
					<CardContent>
						{#if equityData.length >= 2}
							<EquityCurve points={equityData} />
						{:else}
							<p class="text-center text-muted-foreground py-8">Need at least 2 trades to show equity curve</p>
						{/if}
					</CardContent>
				</Card>
			</div>

			<!-- Performance Summary -->
			<Card>
				<CardHeader><CardTitle>Performance</CardTitle></CardHeader>
				<CardContent class="space-y-4">
					<div class="space-y-3">
						<div class="flex justify-between items-center">
							<span class="text-sm text-muted-foreground">Largest Win</span>
							<span class="text-sm font-bold text-green-500">{formatCurrency(stats.largest_win)}</span>
						</div>
						<div class="flex justify-between items-center">
							<span class="text-sm text-muted-foreground">Largest Loss</span>
							<span class="text-sm font-bold text-red-500">{formatCurrency(stats.largest_loss)}</span>
						</div>
						<div class="flex justify-between items-center">
							<span class="text-sm text-muted-foreground">Avg Loss</span>
							<span class="text-sm font-medium">{formatCurrency(stats.avg_loss)}</span>
						</div>
					</div>

					<hr class="border-muted" />

					<!-- Quick Links Grid -->
					<div class="grid grid-cols-3 gap-2">
						{#each quickLinks as link}
							<a
								href={link.href}
								class="p-2 rounded-lg border border-border hover:bg-muted/50 transition-colors text-center"
							>
								<div class="text-lg">{link.icon}</div>
								<div class="text-[10px] text-muted-foreground mt-0.5">{link.label}</div>
							</a>
						{/each}
					</div>
				</CardContent>
			</Card>
		</div>

		<!-- Recent Trades -->
		{#if recentTrades.length > 0}
			<Card>
				<CardHeader>
					<div class="flex items-center justify-between">
						<CardTitle>Recent Trades</CardTitle>
						<a href="/trades" class="text-xs text-muted-foreground hover:text-foreground transition-colors">
							View all →
						</a>
					</div>
				</CardHeader>
				<CardContent>
					<div class="space-y-2">
						{#each recentTrades as trade}
							<a
								href="/trades/{trade.id}"
								class="flex items-center justify-between p-3 rounded-lg hover:bg-muted/50 transition-colors"
							>
								<div class="flex items-center gap-3">
									<div class="w-10 h-10 rounded-lg flex items-center justify-center {trade.direction === 'long' ? 'bg-green-500/10' : 'bg-red-500/10'}">
										<span class="text-xs font-bold {trade.direction === 'long' ? 'text-green-500' : 'text-red-500'}">
											{trade.direction === 'long' ? '▲' : '▼'}
										</span>
									</div>
									<div>
										<div class="flex items-center gap-2">
											<span class="font-bold">{trade.symbol}</span>
											{#if trade.setup_type}
												<Badge variant="secondary">{#snippet children()}{trade.setup_type}{/snippet}</Badge>
											{/if}
										</div>
										<div class="text-xs text-muted-foreground">{timeAgo(trade.entry_datetime)}</div>
									</div>
								</div>
								<div class="text-right">
									<div class="font-bold {pnlColor(trade.pnl)}">
										{trade.pnl != null ? formatCurrency(trade.pnl) : '-'}
									</div>
									{#if trade.r_multiple != null}
										<div class="text-xs text-muted-foreground">{Number(trade.r_multiple).toFixed(2)}R</div>
									{/if}
								</div>
							</a>
						{/each}
					</div>
				</CardContent>
			</Card>
		{/if}
	{:else}
		<!-- Empty State -->
		<div class="text-center py-16">
			<div class="text-6xl mb-4">📊</div>
			<h2 class="text-2xl font-bold">Welcome to TradeMaster AI</h2>
			<p class="text-muted-foreground mt-2 max-w-md mx-auto">
				Start by logging your first trade. Track your performance, get AI-powered insights, and improve your trading.
			</p>
			<div class="flex justify-center gap-3 mt-6">
				<a href="/trades/new" class="px-4 py-2 bg-primary text-primary-foreground rounded-lg text-sm font-medium hover:bg-primary/90 transition-colors">
					Log Your First Trade
				</a>
				<a href="/planning" class="px-4 py-2 border border-border rounded-lg text-sm font-medium hover:bg-muted transition-colors">
					Create a Plan
				</a>
			</div>
		</div>
	{/if}
</div>
