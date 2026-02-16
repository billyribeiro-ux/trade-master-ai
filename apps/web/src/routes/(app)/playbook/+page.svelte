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
	import { toasts } from '$lib/stores/toast.svelte';
	import { formatCurrency, formatPercent } from '$lib/utils/format';
	import { onMount } from 'svelte';

	interface PlaybookEntry {
		id: string;
		name: string;
		description: string | null;
		entry_criteria: string[] | null;
		exit_criteria: string[] | null;
		risk_rules: string[] | null;
		asset_classes: string[] | null;
		timeframes: string[] | null;
		is_active: boolean;
	}

	interface Performance {
		setup_name: string;
		total_trades: number;
		winning_trades: number;
		win_rate: number;
		total_pnl: number;
		avg_pnl: number;
		avg_r_multiple: number | null;
		largest_win: number;
		largest_loss: number;
	}

	let entries = $state<PlaybookEntry[]>([]);
	let performance = $state<Performance[]>([]);
	let loading = $state(true);
	let saving = $state(false);
	let dialogOpen = $state(false);
	let activeTab = $state<'setups' | 'performance'>('setups');

	let form = $state({
		name: '',
		description: '',
		entry_criteria: '',
		exit_criteria: '',
		risk_rules: '',
	});

	onMount(async () => {
		await Promise.all([loadEntries(), loadPerformance()]);
	});

	async function loadEntries() {
		loading = true;
		try {
			entries = await apiClient.get<PlaybookEntry[]>('/api/v1/playbook') ?? [];
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
			await apiClient.post('/api/v1/playbook', {
				name: form.name.trim(),
				description: form.description || null,
				entry_criteria: form.entry_criteria ? form.entry_criteria.split('\n').filter(Boolean) : null,
				exit_criteria: form.exit_criteria ? form.exit_criteria.split('\n').filter(Boolean) : null,
				risk_rules: form.risk_rules ? form.risk_rules.split('\n').filter(Boolean) : null,
			});
			toasts.success('Setup added to playbook');
			dialogOpen = false;
			form = { name: '', description: '', entry_criteria: '', exit_criteria: '', risk_rules: '' };
			await loadEntries();
		} catch (error) {
			toasts.error('Failed to create', error instanceof Error ? error.message : '');
		} finally {
			saving = false;
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
					<Card>
						<CardHeader>
							<div class="flex items-center justify-between">
								<CardTitle>{entry.name}</CardTitle>
								<div class="flex items-center gap-2">
									{#if entry.is_active}
										<Badge variant="success">{#snippet children()}Active{/snippet}</Badge>
									{:else}
										<Badge variant="outline">{#snippet children()}Inactive{/snippet}</Badge>
									{/if}
									<Button variant="ghost" size="sm" onclick={() => deleteEntry(entry.id)}>
										{#snippet children()}Delete{/snippet}
									</Button>
								</div>
							</div>
						</CardHeader>
						<CardContent class="space-y-3">
							{#if entry.description}
								<p class="text-sm text-muted-foreground">{entry.description}</p>
							{/if}

							{#if entry.entry_criteria && entry.entry_criteria.length > 0}
								<div>
									<p class="text-xs font-semibold uppercase text-muted-foreground mb-1">Entry Criteria</p>
									<ul class="text-sm space-y-1">
										{#each entry.entry_criteria as criterion}
											<li class="flex items-start gap-2">
												<span class="text-green-600 mt-0.5">&#10003;</span>
												<span>{criterion}</span>
											</li>
										{/each}
									</ul>
								</div>
							{/if}

							{#if entry.exit_criteria && entry.exit_criteria.length > 0}
								<div>
									<p class="text-xs font-semibold uppercase text-muted-foreground mb-1">Exit Criteria</p>
									<ul class="text-sm space-y-1">
										{#each entry.exit_criteria as criterion}
											<li class="flex items-start gap-2">
												<span class="text-blue-600 mt-0.5">&#8594;</span>
												<span>{criterion}</span>
											</li>
										{/each}
									</ul>
								</div>
							{/if}

							{#if entry.risk_rules && entry.risk_rules.length > 0}
								<div>
									<p class="text-xs font-semibold uppercase text-muted-foreground mb-1">Risk Rules</p>
									<ul class="text-sm space-y-1">
										{#each entry.risk_rules as rule}
											<li class="flex items-start gap-2">
												<span class="text-orange-600 mt-0.5">&#9888;</span>
												<span>{rule}</span>
											</li>
										{/each}
									</ul>
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
			<div class="overflow-x-auto">
				<table class="w-full text-sm">
					<thead>
						<tr class="border-b">
							<th class="text-left p-3 font-medium">Setup</th>
							<th class="text-right p-3 font-medium">Trades</th>
							<th class="text-right p-3 font-medium">Win Rate</th>
							<th class="text-right p-3 font-medium">Total P&L</th>
							<th class="text-right p-3 font-medium">Avg P&L</th>
							<th class="text-right p-3 font-medium">Avg R</th>
							<th class="text-right p-3 font-medium">Best</th>
							<th class="text-right p-3 font-medium">Worst</th>
						</tr>
					</thead>
					<tbody>
						{#each performance as p}
							<tr class="border-b hover:bg-muted/50">
								<td class="p-3 font-medium">{p.setup_name}</td>
								<td class="p-3 text-right">{p.total_trades}</td>
								<td class="p-3 text-right">{p.win_rate.toFixed(1)}%</td>
								<td class="p-3 text-right {p.total_pnl >= 0 ? 'text-green-600' : 'text-destructive'}">
									{formatCurrency(p.total_pnl)}
								</td>
								<td class="p-3 text-right">{formatCurrency(p.avg_pnl)}</td>
								<td class="p-3 text-right">{p.avg_r_multiple?.toFixed(2) ?? '-'}R</td>
								<td class="p-3 text-right text-green-600">{formatCurrency(p.largest_win)}</td>
								<td class="p-3 text-right text-destructive">{formatCurrency(p.largest_loss)}</td>
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
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
				<Textarea id="setup-desc" bind:value={form.description} placeholder="Describe this setup..." rows={3} />
			</div>
			<div class="space-y-2">
				<Label for="entry-criteria">Entry Criteria (one per line)</Label>
				<Textarea id="entry-criteria" bind:value={form.entry_criteria} placeholder="Price above VWAP&#10;Volume increasing&#10;Clean flag pattern" rows={4} />
			</div>
			<div class="space-y-2">
				<Label for="exit-criteria">Exit Criteria (one per line)</Label>
				<Textarea id="exit-criteria" bind:value={form.exit_criteria} placeholder="Target hit&#10;Trailing stop triggered&#10;End of day" rows={3} />
			</div>
			<div class="space-y-2">
				<Label for="risk-rules">Risk Rules (one per line)</Label>
				<Textarea id="risk-rules" bind:value={form.risk_rules} placeholder="Max 1% risk&#10;Stop below flag low&#10;No averaging down" rows={3} />
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
