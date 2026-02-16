<script lang="ts">
	import { tradesApi } from '$lib/api';
	import Button from '$lib/components/ui/button.svelte';
	import Card from '$lib/components/ui/card.svelte';
	import CardContent from '$lib/components/ui/card-content.svelte';
	import Badge from '$lib/components/ui/badge.svelte';
	import Select from '$lib/components/ui/select.svelte';
	import Input from '$lib/components/ui/input.svelte';
	import { formatCurrency, formatPercent, formatDate } from '$lib/utils/format';
	import { onMount } from 'svelte';
	import type { Trade, TradeStatus } from '$lib/types/trade';

	let trades = $state<Trade[]>([]);
	let total = $state(0);
	let page = $state(1);
	let perPage = $state(50);
	let totalPages = $state(0);
	let loading = $state(true);

	let filters = $state({
		status: '',
		direction: '',
		asset_class: '',
		symbol: '',
		setup_name: ''
	});

	let sortBy = $state('entry_date');
	let sortOrder = $state<'asc' | 'desc'>('desc');

	async function loadTrades() {
		loading = true;
		try {
			const response = await tradesApi.list({
				page,
				per_page: perPage,
				sort_by: sortBy,
				sort_order: sortOrder,
				filters: {
					status: (filters.status || undefined) as TradeStatus | undefined,
					direction: filters.direction as any,
					asset_class: filters.asset_class as any,
					symbol: filters.symbol || undefined,
					setup_name: filters.setup_name || undefined
				}
			});
			trades = response.trades;
			total = response.total;
			totalPages = response.total_pages;
		} catch (error) {
			console.error('Failed to load trades:', error);
		} finally {
			loading = false;
		}
	}

	onMount(() => {
		loadTrades();
	});

	function handleFilterChange() {
		page = 1;
		loadTrades();
	}

	function handleSort(field: string) {
		if (sortBy === field) {
			sortOrder = sortOrder === 'asc' ? 'desc' : 'asc';
		} else {
			sortBy = field;
			sortOrder = 'desc';
		}
		loadTrades();
	}

	function getStatusVariant(status: string): 'default' | 'success' | 'secondary' {
		switch (status) {
			case 'open':
				return 'default';
			case 'closed':
				return 'success';
			default:
				return 'secondary';
		}
	}

	function getDirectionColor(direction: string): string {
		return direction === 'long' ? 'text-success' : 'text-destructive';
	}

	function getPnlColor(pnl: number | undefined): string {
		if (!pnl) return '';
		return pnl > 0 ? 'text-success' : 'text-destructive';
	}
</script>

<svelte:head>
	<title>Trades - TradeMaster AI</title>
</svelte:head>

<div class="p-6 space-y-6">
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold">Trades</h1>
			<p class="text-muted-foreground mt-1">{total} total trades</p>
		</div>
		<div class="flex gap-2">
			<Button variant="outline">
				{#snippet children()}
					<a href="/trades/import">Import CSV</a>
				{/snippet}
			</Button>
			<Button>
				{#snippet children()}
					<a href="/trades/new">Log New Trade</a>
				{/snippet}
			</Button>
		</div>
	</div>

	<Card>
		<CardContent class="p-6">
			<div class="grid gap-4 md:grid-cols-5">
				<div>
					<Select
						bind:value={filters.status}
						placeholder="All Statuses"
						options={[
							{ value: '', label: 'All Statuses' },
							{ value: 'open', label: 'Open' },
							{ value: 'closed', label: 'Closed' },
							{ value: 'cancelled', label: 'Cancelled' }
						]}
						onchange={handleFilterChange}
					/>
				</div>
				<div>
					<Select
						bind:value={filters.direction}
						placeholder="All Directions"
						options={[
							{ value: '', label: 'All Directions' },
							{ value: 'long', label: 'Long' },
							{ value: 'short', label: 'Short' }
						]}
						onchange={handleFilterChange}
					/>
				</div>
				<div>
					<Select
						bind:value={filters.asset_class}
						placeholder="All Assets"
						options={[
							{ value: '', label: 'All Assets' },
							{ value: 'stocks', label: 'Stocks' },
							{ value: 'options', label: 'Options' },
							{ value: 'futures', label: 'Futures' },
							{ value: 'forex', label: 'Forex' },
							{ value: 'crypto', label: 'Crypto' }
						]}
						onchange={handleFilterChange}
					/>
				</div>
				<div>
					<Input
						bind:value={filters.symbol}
						placeholder="Search symbol..."
						oninput={handleFilterChange}
					/>
				</div>
				<div>
					<Input
						bind:value={filters.setup_name}
						placeholder="Search setup..."
						oninput={handleFilterChange}
					/>
				</div>
			</div>
		</CardContent>
	</Card>

	{#if loading}
		<div class="text-center py-12 text-muted-foreground">Loading trades...</div>
	{:else if trades.length === 0}
		<Card>
			<CardContent class="py-12 text-center">
				<p class="text-muted-foreground mb-4">No trades found</p>
				<Button>
					{#snippet children()}
						<a href="/trades/new">Log Your First Trade</a>
					{/snippet}
				</Button>
			</CardContent>
		</Card>
	{:else}
		<div class="rounded-md border">
			<div class="overflow-x-auto">
				<table class="w-full">
					<thead class="border-b bg-muted/50">
						<tr>
							<th class="px-4 py-3 text-left text-sm font-medium">
								<button onclick={() => handleSort('symbol')} class="hover:text-foreground">
									Symbol
								</button>
							</th>
							<th class="px-4 py-3 text-left text-sm font-medium">
								<button onclick={() => handleSort('direction')} class="hover:text-foreground">
									Direction
								</button>
							</th>
							<th class="px-4 py-3 text-left text-sm font-medium">
								<button onclick={() => handleSort('entry_date')} class="hover:text-foreground">
									Entry Date
								</button>
							</th>
							<th class="px-4 py-3 text-right text-sm font-medium">
								<button onclick={() => handleSort('entry_price')} class="hover:text-foreground">
									Entry
								</button>
							</th>
							<th class="px-4 py-3 text-right text-sm font-medium">
								<button onclick={() => handleSort('exit_price')} class="hover:text-foreground">
									Exit
								</button>
							</th>
							<th class="px-4 py-3 text-right text-sm font-medium">
								<button onclick={() => handleSort('pnl')} class="hover:text-foreground">
									P&L
								</button>
							</th>
							<th class="px-4 py-3 text-right text-sm font-medium">
								<button onclick={() => handleSort('pnl_percent')} class="hover:text-foreground">
									P&L %
								</button>
							</th>
							<th class="px-4 py-3 text-right text-sm font-medium">R</th>
							<th class="px-4 py-3 text-left text-sm font-medium">Status</th>
							<th class="px-4 py-3 text-left text-sm font-medium">Setup</th>
						</tr>
					</thead>
					<tbody>
						{#each trades as trade}
							<tr class="border-b hover:bg-muted/50 cursor-pointer" onclick={() => window.location.href = `/trades/${trade.id}`}>
								<td class="px-4 py-3 font-medium">{trade.symbol}</td>
								<td class="px-4 py-3">
									<span class={getDirectionColor(trade.direction)}>
										{trade.direction.toUpperCase()}
									</span>
								</td>
								<td class="px-4 py-3 text-sm text-muted-foreground">
									{formatDate(trade.entry_date)}
								</td>
								<td class="px-4 py-3 text-right text-sm">
									{formatCurrency(trade.entry_price)}
								</td>
								<td class="px-4 py-3 text-right text-sm">
									{trade.exit_price ? formatCurrency(trade.exit_price) : '-'}
								</td>
								<td class="px-4 py-3 text-right font-medium {getPnlColor(trade.net_pnl)}">
									{trade.net_pnl !== null && trade.net_pnl !== undefined ? formatCurrency(trade.net_pnl) : '-'}
								</td>
								<td class="px-4 py-3 text-right font-medium {getPnlColor(trade.pnl_percent)}">
									{trade.pnl_percent !== null && trade.pnl_percent !== undefined ? formatPercent(trade.pnl_percent) : '-'}
								</td>
								<td class="px-4 py-3 text-right text-sm">
									{trade.r_multiple !== null && trade.r_multiple !== undefined ? `${trade.r_multiple.toFixed(2)}R` : '-'}
								</td>
								<td class="px-4 py-3">
									<Badge variant={getStatusVariant(trade.status)}>
										{#snippet children()}
											{trade.status}
										{/snippet}
									</Badge>
								</td>
								<td class="px-4 py-3 text-sm text-muted-foreground">
									{trade.setup_name || '-'}
								</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</div>

		{#if totalPages > 1}
			<div class="flex items-center justify-between">
				<p class="text-sm text-muted-foreground">
					Showing {(page - 1) * perPage + 1} to {Math.min(page * perPage, total)} of {total} trades
				</p>
				<div class="flex gap-2">
					<Button
						variant="outline"
						disabled={page === 1}
						onclick={() => {
							page--;
							loadTrades();
						}}
					>
						{#snippet children()}
							Previous
						{/snippet}
					</Button>
					<Button
						variant="outline"
						disabled={page === totalPages}
						onclick={() => {
							page++;
							loadTrades();
						}}
					>
						{#snippet children()}
							Next
						{/snippet}
					</Button>
				</div>
			</div>
		{/if}
	{/if}
</div>
