<script lang="ts">
	import { apiClient } from '$lib/api';
	import Button from '$lib/components/ui/button.svelte';
	import Input from '$lib/components/ui/input.svelte';
	import Label from '$lib/components/ui/label.svelte';
	import Textarea from '$lib/components/ui/textarea.svelte';
	import Card from '$lib/components/ui/card.svelte';
	import CardHeader from '$lib/components/ui/card-header.svelte';
	import CardTitle from '$lib/components/ui/card-title.svelte';
	import CardContent from '$lib/components/ui/card-content.svelte';
	import Badge from '$lib/components/ui/badge.svelte';
	import Dialog from '$lib/components/ui/dialog.svelte';
	import DialogHeader from '$lib/components/ui/dialog-header.svelte';
	import DialogTitle from '$lib/components/ui/dialog-title.svelte';
	import DialogFooter from '$lib/components/ui/dialog-footer.svelte';
	import Spinner from '$lib/components/ui/spinner.svelte';
	import SetupPerformanceChart from '$lib/components/analytics/setup-performance-chart.svelte';
	import { toasts } from '$lib/stores/toast.svelte';
	import { formatCurrency } from '$lib/utils/format';
	import { onMount } from 'svelte';

	interface PlaybookSetup {
		id: string;
		name: string;
		description: string | null;
		criteria: Record<string, unknown> | null;
		expected_r_min: number | null;
		expected_r_max: number | null;
		min_conviction: number | null;
		preferred_timeframe: string | null;
		market_regimes: string[] | null;
		common_mistakes: string | null;
		trade_count: number;
		win_rate: number | null;
		avg_r: number | null;
		profit_factor: number | null;
		total_pnl: number | null;
		is_active: boolean;
	}

	interface Performance {
		setup_name: string;
		trade_count: number;
		win_count: number;
		loss_count: number;
		win_rate: number | null;
		avg_pnl: number | null;
		avg_r_multiple: number | null;
		total_pnl: number | null;
	}

	let entries = $state<PlaybookSetup[]>([]);
	let performance = $state<Performance[]>([]);
	let loading = $state(true);
	let saving = $state(false);
	let dialogOpen = $state(false);
	let activeTab = $state<'setups' | 'performance'>('setups');
	let expandedSetup = $state<string | null>(null);

	let form = $state({
		name: '',
		description: '',
		expected_r_min: '',
		expected_r_max: '',
		min_conviction: '7',
		preferred_timeframe: '',
		market_regimes: '',
		common_mistakes: '',
		criteria_entry: '',
		criteria_exit: '',
		criteria_risk: '',
	});

	onMount(async () => {
		await Promise.all([loadEntries(), loadPerformance()]);
	});

	async function loadEntries() {
		loading = true;
		try {
			entries = await apiClient.get<PlaybookSetup[]>('/api/v1/playbook') ?? [];
		} catch { /* silent */ }
		loading = false;
	}

	async function loadPerformance() {
		try {
			performance = await apiClient.get<Performance[]>('/api/v1/playbook/performance') ?? [];
		} catch { /* silent */ }
	}

	async function createEntry() {
		if (!form.name.trim()) {
			toasts.error('Name is required');
			return;
		}

		saving = true;
		try {
			const criteria: Record<string, string[]> = {};
			const entryLines = form.criteria_entry.split('\n').filter(Boolean);
			const exitLines = form.criteria_exit.split('\n').filter(Boolean);
			const riskLines = form.criteria_risk.split('\n').filter(Boolean);
			if (entryLines.length) criteria.entry = entryLines;
			if (exitLines.length) criteria.exit = exitLines;
			if (riskLines.length) criteria.risk = riskLines;

			const regimes = form.market_regimes
				.split(',')
				.map((r: string) => r.trim())
				.filter((r: string) => r.length > 0);

			await apiClient.post('/api/v1/playbook', {
				name: form.name.trim(),
				description: form.description || null,
				criteria: Object.keys(criteria).length > 0 ? criteria : null,
				expected_r_min: form.expected_r_min ? parseFloat(form.expected_r_min) : null,
				expected_r_max: form.expected_r_max ? parseFloat(form.expected_r_max) : null,
				min_conviction: form.min_conviction ? parseInt(form.min_conviction) : null,
				preferred_timeframe: form.preferred_timeframe || null,
				market_regimes: regimes.length > 0 ? regimes : null,
				common_mistakes: form.common_mistakes || null,
			});
			toasts.success('Setup added to playbook');
			dialogOpen = false;
			form = { name: '', description: '', expected_r_min: '', expected_r_max: '', min_conviction: '7', preferred_timeframe: '', market_regimes: '', common_mistakes: '', criteria_entry: '', criteria_exit: '', criteria_risk: '' };
			await loadEntries();
		} catch (error) {
			toasts.error('Failed to create', error instanceof Error ? error.message : '');
		} finally {
			saving = false;
		}
	}

	async function toggleActive(entry: PlaybookSetup) {
		try {
			await apiClient.put(`/api/v1/playbook/${entry.id}`, {
				name: entry.name,
				is_active: !entry.is_active,
			});
			await loadEntries();
		} catch {
			toasts.error('Failed to update');
		}
	}

	async function deleteEntry(id: string) {
		try {
			await apiClient.delete(`/api/v1/playbook/${id}`);
			toasts.success('Setup removed');
			await loadEntries();
		} catch {
			toasts.error('Failed to delete');
		}
	}

	function getCriteriaList(criteria: Record<string, unknown> | null, key: string): string[] {
		if (!criteria || !criteria[key]) return [];
		const val = criteria[key];
		return Array.isArray(val) ? val : [];
	}

	function winRateColor(rate: number | null): string {
		if (rate == null) return '';
		if (rate >= 60) return 'text-green-500';
		if (rate >= 50) return 'text-yellow-500';
		return 'text-red-500';
	}
</script>

<svelte:head>
	<title>Playbook - TradeMaster AI</title>
</svelte:head>

<div class="p-6 max-w-6xl mx-auto space-y-6">
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold">Trading Playbook</h1>
			<p class="text-muted-foreground mt-1">Define your setups, rules, and track their performance</p>
		</div>
		<div class="flex gap-2">
			<Button variant={activeTab === 'setups' ? 'default' : 'outline'} onclick={() => activeTab = 'setups'}>
				{#snippet children()}Setups{/snippet}
			</Button>
			<Button variant={activeTab === 'performance' ? 'default' : 'outline'} onclick={() => activeTab = 'performance'}>
				{#snippet children()}Performance{/snippet}
			</Button>
			<Button onclick={() => dialogOpen = true}>
				{#snippet children()}+ Add Setup{/snippet}
			</Button>
		</div>
	</div>

	{#if activeTab === 'setups'}
		{#if loading}
			<div class="flex justify-center py-12"><Spinner /></div>
		{:else if entries.length === 0}
			<div class="text-center py-16">
				<h3 class="text-lg font-medium">No setups in your playbook yet</h3>
				<p class="text-muted-foreground mt-2">Define the trading setups you use so you can track their performance over time.</p>
				<Button class="mt-4" onclick={() => dialogOpen = true}>
					{#snippet children()}Add Your First Setup{/snippet}
				</Button>
			</div>
		{:else}
			<div class="grid gap-4 md:grid-cols-2">
				{#each entries as entry}
					<Card class="{entry.is_active ? '' : 'opacity-60'}">
						<CardHeader>
							<div class="flex items-center justify-between">
								<div class="flex items-center gap-2">
									<CardTitle>{entry.name}</CardTitle>
									<button
										class="w-10 h-5 rounded-full transition-colors {entry.is_active ? 'bg-green-500' : 'bg-muted-foreground/30'}"
										onclick={() => toggleActive(entry)}
										title={entry.is_active ? 'Active - click to deactivate' : 'Inactive - click to activate'}
									>
										<div class="w-4 h-4 bg-white rounded-full transition-transform {entry.is_active ? 'translate-x-5' : 'translate-x-0.5'}"></div>
									</button>
								</div>
								<Button variant="ghost" size="sm" onclick={() => deleteEntry(entry.id)}>
									{#snippet children()}<span class="text-destructive text-xs">Delete</span>{/snippet}
								</Button>
							</div>
						</CardHeader>
						<CardContent class="space-y-3">
							{#if entry.description}
								<p class="text-sm text-muted-foreground">{entry.description}</p>
							{/if}

							<!-- Stats Bar -->
							{#if entry.trade_count > 0}
								<div class="grid grid-cols-4 gap-2 p-3 bg-muted/30 rounded-lg">
									<div class="text-center">
										<div class="text-lg font-bold">{entry.trade_count}</div>
										<div class="text-[10px] text-muted-foreground">Trades</div>
									</div>
									<div class="text-center">
										<div class="text-lg font-bold {winRateColor(entry.win_rate != null ? Number(entry.win_rate) : null)}">
											{entry.win_rate != null ? Number(entry.win_rate).toFixed(0) + '%' : '-'}
										</div>
										<div class="text-[10px] text-muted-foreground">Win Rate</div>
									</div>
									<div class="text-center">
										<div class="text-lg font-bold">
											{entry.avg_r != null ? Number(entry.avg_r).toFixed(1) + 'R' : '-'}
										</div>
										<div class="text-[10px] text-muted-foreground">Avg R</div>
									</div>
									<div class="text-center">
										<div class="text-lg font-bold {(entry.total_pnl ?? 0) >= 0 ? 'text-green-500' : 'text-red-500'}">
											{entry.total_pnl != null ? formatCurrency(Number(entry.total_pnl)) : '-'}
										</div>
										<div class="text-[10px] text-muted-foreground">P&L</div>
									</div>
								</div>
							{/if}

							<!-- Tags -->
							<div class="flex flex-wrap gap-1.5">
								{#if entry.preferred_timeframe}
									<Badge variant="outline">{#snippet children()}{entry.preferred_timeframe}{/snippet}</Badge>
								{/if}
								{#if entry.expected_r_min != null || entry.expected_r_max != null}
									<Badge variant="outline">{#snippet children()}R: {entry.expected_r_min ?? '?'}-{entry.expected_r_max ?? '?'}{/snippet}</Badge>
								{/if}
								{#if entry.min_conviction != null}
									<Badge variant="outline">{#snippet children()}Conv ≥{entry.min_conviction}{/snippet}</Badge>
								{/if}
								{#if entry.market_regimes}
									{#each entry.market_regimes as regime}
										<Badge variant="secondary">{#snippet children()}{regime}{/snippet}</Badge>
									{/each}
								{/if}
							</div>

							<!-- Expandable Criteria -->
							<button
								class="w-full text-left text-xs text-muted-foreground hover:text-foreground transition-colors"
								onclick={() => expandedSetup = expandedSetup === entry.id ? null : entry.id}
							>
								{expandedSetup === entry.id ? '▼' : '▶'} Criteria & Rules
							</button>

							{#if expandedSetup === entry.id}
								<div class="space-y-3 pl-2 border-l-2 border-muted">
									{#if getCriteriaList(entry.criteria, 'entry').length > 0}
										<div>
											<p class="text-xs font-semibold uppercase text-green-400 mb-1">Entry Criteria</p>
											<ul class="text-sm space-y-0.5">
												{#each getCriteriaList(entry.criteria, 'entry') as criterion}
													<li class="flex items-start gap-2">
														<span class="text-green-500 mt-0.5">✓</span>
														<span>{criterion}</span>
													</li>
												{/each}
											</ul>
										</div>
									{/if}

									{#if getCriteriaList(entry.criteria, 'exit').length > 0}
										<div>
											<p class="text-xs font-semibold uppercase text-blue-400 mb-1">Exit Criteria</p>
											<ul class="text-sm space-y-0.5">
												{#each getCriteriaList(entry.criteria, 'exit') as criterion}
													<li class="flex items-start gap-2">
														<span class="text-blue-500 mt-0.5">→</span>
														<span>{criterion}</span>
													</li>
												{/each}
											</ul>
										</div>
									{/if}

									{#if getCriteriaList(entry.criteria, 'risk').length > 0}
										<div>
											<p class="text-xs font-semibold uppercase text-orange-400 mb-1">Risk Rules</p>
											<ul class="text-sm space-y-0.5">
												{#each getCriteriaList(entry.criteria, 'risk') as rule}
													<li class="flex items-start gap-2">
														<span class="text-orange-500 mt-0.5">⚠</span>
														<span>{rule}</span>
													</li>
												{/each}
											</ul>
										</div>
									{/if}

									{#if entry.common_mistakes}
										<div>
											<p class="text-xs font-semibold uppercase text-red-400 mb-1">Common Mistakes</p>
											<p class="text-sm text-muted-foreground">{entry.common_mistakes}</p>
										</div>
									{/if}
								</div>
							{/if}
						</CardContent>
					</Card>
				{/each}
			</div>
		{/if}

	{:else if activeTab === 'performance'}
		{#if performance.length === 0}
			<p class="text-center text-muted-foreground py-12">
				No performance data yet. Add setups and log trades with matching setup names.
			</p>
		{:else}
			<!-- Performance Chart -->
			<Card>
				<CardHeader><CardTitle>Setup Comparison</CardTitle></CardHeader>
				<CardContent>
					<SetupPerformanceChart setups={performance.map(p => ({
					setup_name: p.setup_name,
					trade_count: p.trade_count,
					win_rate: Number(p.win_rate ?? 0),
					total_pnl: Number(p.total_pnl ?? 0),
					avg_pnl: Number(p.avg_pnl ?? 0),
					avg_r_multiple: p.avg_r_multiple != null ? Number(p.avg_r_multiple) : null,
				}))} />
				</CardContent>
			</Card>

			<!-- Performance Table -->
			<Card>
				<CardHeader><CardTitle>Detailed Stats</CardTitle></CardHeader>
				<CardContent>
					<div class="overflow-x-auto">
						<table class="w-full text-sm">
							<thead>
								<tr class="border-b">
									<th class="text-left p-3 font-medium">Setup</th>
									<th class="text-right p-3 font-medium">Trades</th>
									<th class="text-right p-3 font-medium">W/L</th>
									<th class="text-right p-3 font-medium">Win Rate</th>
									<th class="text-right p-3 font-medium">Avg R</th>
									<th class="text-right p-3 font-medium">Total P&L</th>
								</tr>
							</thead>
							<tbody>
								{#each performance as p}
									<tr class="border-b hover:bg-muted/50">
										<td class="p-3 font-medium">{p.setup_name}</td>
										<td class="p-3 text-right">{p.trade_count}</td>
										<td class="p-3 text-right">
											<span class="text-green-500">{p.win_count}</span>/<span class="text-red-500">{p.loss_count}</span>
										</td>
										<td class="p-3 text-right {winRateColor(p.win_rate != null ? Number(p.win_rate) : null)}">
											{p.win_rate != null ? Number(p.win_rate).toFixed(1) + '%' : '-'}
										</td>
										<td class="p-3 text-right">
											{p.avg_r_multiple != null ? Number(p.avg_r_multiple).toFixed(2) + 'R' : '-'}
										</td>
										<td class="p-3 text-right font-medium {(p.total_pnl ?? 0) >= 0 ? 'text-green-500' : 'text-red-500'}">
											{p.total_pnl != null ? formatCurrency(Number(p.total_pnl)) : '-'}
										</td>
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
				</CardContent>
			</Card>
		{/if}
	{/if}
</div>

<!-- Add Setup Dialog -->
<Dialog bind:open={dialogOpen}>
	{#snippet children()}
		<DialogHeader>
			<DialogTitle>Add Trading Setup</DialogTitle>
		</DialogHeader>
		<div class="space-y-4 py-4">
			<div class="space-y-2">
				<Label for="setup-name" required>Setup Name</Label>
				<Input id="setup-name" bind:value={form.name} placeholder="Bull Flag Breakout" required />
			</div>
			<div class="space-y-2">
				<Label for="setup-desc">Description</Label>
				<Textarea id="setup-desc" bind:value={form.description} placeholder="Describe this setup..." rows={2} />
			</div>
			<div class="grid grid-cols-2 gap-4">
				<div class="space-y-2">
					<Label for="r-min">Expected R Min</Label>
					<Input id="r-min" type="number" step="0.1" bind:value={form.expected_r_min} placeholder="1.5" />
				</div>
				<div class="space-y-2">
					<Label for="r-max">Expected R Max</Label>
					<Input id="r-max" type="number" step="0.1" bind:value={form.expected_r_max} placeholder="3.0" />
				</div>
			</div>
			<div class="grid grid-cols-2 gap-4">
				<div class="space-y-2">
					<Label for="conviction">Min Conviction (1-10)</Label>
					<Input id="conviction" type="number" min="1" max="10" bind:value={form.min_conviction} />
				</div>
				<div class="space-y-2">
					<Label for="timeframe">Preferred Timeframe</Label>
					<Input id="timeframe" bind:value={form.preferred_timeframe} placeholder="5m, 15m, 1h..." />
				</div>
			</div>
			<div class="space-y-2">
				<Label for="regimes">Market Regimes (comma-separated)</Label>
				<Input id="regimes" bind:value={form.market_regimes} placeholder="trending, range-bound, volatile..." />
			</div>
			<div class="space-y-2">
				<Label for="entry-criteria">Entry Criteria (one per line)</Label>
				<Textarea id="entry-criteria" bind:value={form.criteria_entry} placeholder="Price above VWAP&#10;Volume increasing&#10;Clean flag pattern" rows={3} />
			</div>
			<div class="space-y-2">
				<Label for="exit-criteria">Exit Criteria (one per line)</Label>
				<Textarea id="exit-criteria" bind:value={form.criteria_exit} placeholder="Target hit&#10;Trailing stop triggered" rows={2} />
			</div>
			<div class="space-y-2">
				<Label for="risk-criteria">Risk Rules (one per line)</Label>
				<Textarea id="risk-criteria" bind:value={form.criteria_risk} placeholder="Max 1% risk&#10;Stop below flag low" rows={2} />
			</div>
			<div class="space-y-2">
				<Label for="mistakes">Common Mistakes</Label>
				<Textarea id="mistakes" bind:value={form.common_mistakes} placeholder="Things to watch out for with this setup..." rows={2} />
			</div>
		</div>
		<DialogFooter>
			<Button variant="outline" onclick={() => dialogOpen = false}>
				{#snippet children()}Cancel{/snippet}
			</Button>
			<Button onclick={createEntry} disabled={saving}>
				{#snippet children()}{saving ? 'Saving...' : 'Add Setup'}{/snippet}
			</Button>
		</DialogFooter>
	{/snippet}
</Dialog>
