<script lang="ts">
	import { page } from '$app/stores';
	import { goto } from '$app/navigation';
	import { tradesApi } from '$lib/api';
	import Button from '$lib/components/ui/button.svelte';
	import Card from '$lib/components/ui/card.svelte';
	import CardHeader from '$lib/components/ui/card-header.svelte';
	import CardTitle from '$lib/components/ui/card-title.svelte';
	import CardContent from '$lib/components/ui/card-content.svelte';
	import Badge from '$lib/components/ui/badge.svelte';
	import Dialog from '$lib/components/ui/dialog.svelte';
	import DialogHeader from '$lib/components/ui/dialog-header.svelte';
	import DialogTitle from '$lib/components/ui/dialog-title.svelte';
	import DialogFooter from '$lib/components/ui/dialog-footer.svelte';
	import Input from '$lib/components/ui/input.svelte';
	import Label from '$lib/components/ui/label.svelte';
	import Textarea from '$lib/components/ui/textarea.svelte';
	// Select unused for now
	import { toasts } from '$lib/stores/toast.svelte';
	import { formatCurrency, formatPercent, formatDateTime, formatDuration } from '$lib/utils/format';
	import { onMount } from 'svelte';
	import type { TradeWithDetails } from '$lib/types/trade';

	let trade = $state<TradeWithDetails | null>(null);
	let loading = $state(true);
	let closeDialogOpen = $state(false);
	let deleteDialogOpen = $state(false);

	let closeFormData = $state({
		exit_date: new Date().toISOString().slice(0, 16),
		exit_price: '',
		actual_exit_price: '',
		mistakes: '',
		lessons: '',
		execution_grade: '',
		patience_grade: '',
		discipline_grade: '',
		overall_grade: '',
		broke_rules: false,
		followed_plan: false
	});

	const tradeId = $derived($page.params.id ?? '');

	onMount(async () => {
		await loadTrade();
	});

	async function loadTrade() {
		loading = true;
		try {
			trade = await tradesApi.get(tradeId);
		} catch (error) {
			toasts.error('Failed to load trade', error instanceof Error ? error.message : 'Trade not found');
			goto('/trades');
		} finally {
			loading = false;
		}
	}

	async function handleCloseTrade() {
		if (!closeFormData.exit_price) {
			toasts.error('Exit price is required');
			return;
		}

		try {
			await tradesApi.close(tradeId, {
				exit_date: new Date(closeFormData.exit_date).toISOString(),
				exit_price: parseFloat(closeFormData.exit_price),
				actual_exit_price: closeFormData.actual_exit_price ? parseFloat(closeFormData.actual_exit_price) : undefined,
				mistakes: closeFormData.mistakes || undefined,
				lessons: closeFormData.lessons || undefined,
				execution_grade: closeFormData.execution_grade || undefined,
				patience_grade: closeFormData.patience_grade || undefined,
				discipline_grade: closeFormData.discipline_grade || undefined,
				overall_grade: closeFormData.overall_grade || undefined,
				broke_rules: closeFormData.broke_rules,
				followed_plan: closeFormData.followed_plan
			});

			toasts.success('Trade closed successfully');
			closeDialogOpen = false;
			await loadTrade();
		} catch (error) {
			toasts.error('Failed to close trade', error instanceof Error ? error.message : 'Please try again');
		}
	}

	async function handleDeleteTrade() {
		try {
			await tradesApi.delete(tradeId);
			toasts.success('Trade deleted successfully');
			goto('/trades');
		} catch (error) {
			toasts.error('Failed to delete trade', error instanceof Error ? error.message : 'Please try again');
		}
	}

	function getStatusVariant(status: string): 'default' | 'success' | 'secondary' {
		switch (status) {
			case 'open': return 'default';
			case 'closed': return 'success';
			default: return 'secondary';
		}
	}

	function getPnlColor(pnl: number | undefined): string {
		if (!pnl) return '';
		return pnl > 0 ? 'text-success' : 'text-destructive';
	}
</script>

<svelte:head>
	<title>{trade ? `${trade.trade.symbol} Trade` : 'Trade'} - TradeMaster AI</title>
</svelte:head>

{#if loading}
	<div class="flex h-screen items-center justify-center">
		<div class="text-muted-foreground">Loading trade...</div>
	</div>
{:else if trade}
	<div class="p-6 max-w-6xl mx-auto space-y-6">
		<!-- Header -->
		<div class="flex items-center justify-between">
			<div>
				<div class="flex items-center gap-3">
					<h1 class="text-3xl font-bold">{trade!.trade.symbol}</h1>
					<Badge variant={getStatusVariant(trade!.trade.status)}>
						{#snippet children()}
							{trade!.trade.status}
						{/snippet}
					</Badge>
					<Badge variant="outline">
						{#snippet children()}
							{trade!.trade.direction.toUpperCase()}
						{/snippet}
					</Badge>
				</div>
				<p class="text-muted-foreground mt-1">
					{formatDateTime(trade.trade.entry_date)}
				</p>
			</div>
			<div class="flex gap-2">
				{#if trade.trade.status === 'open'}
					<Button variant="default" onclick={() => closeDialogOpen = true}>
						{#snippet children()}
							Close Trade
						{/snippet}
					</Button>
				{/if}
				<Button variant="outline" onclick={() => goto(`/trades/${tradeId}/edit`)}>
					{#snippet children()}
						Edit
					{/snippet}
				</Button>
				<Button variant="destructive" onclick={() => deleteDialogOpen = true}>
					{#snippet children()}
						Delete
					{/snippet}
				</Button>
			</div>
		</div>

		<!-- P&L Summary (if closed) -->
		{#if trade.trade.status === 'closed'}
			<div class="grid gap-4 md:grid-cols-4">
				<Card>
					<CardContent class="pt-6">
						<div class="text-sm text-muted-foreground">Net P&L</div>
						<div class="text-2xl font-bold {getPnlColor(trade.trade.net_pnl)}">
							{trade.trade.net_pnl !== null && trade.trade.net_pnl !== undefined ? formatCurrency(trade.trade.net_pnl) : 'N/A'}
						</div>
					</CardContent>
				</Card>
				<Card>
					<CardContent class="pt-6">
						<div class="text-sm text-muted-foreground">P&L %</div>
						<div class="text-2xl font-bold {getPnlColor(trade.trade.pnl_percent)}">
							{trade.trade.pnl_percent !== null && trade.trade.pnl_percent !== undefined ? formatPercent(trade.trade.pnl_percent) : 'N/A'}
						</div>
					</CardContent>
				</Card>
				<Card>
					<CardContent class="pt-6">
						<div class="text-sm text-muted-foreground">R-Multiple</div>
						<div class="text-2xl font-bold">
							{trade.trade.r_multiple !== null && trade.trade.r_multiple !== undefined ? `${trade.trade.r_multiple.toFixed(2)}R` : 'N/A'}
						</div>
					</CardContent>
				</Card>
				<Card>
					<CardContent class="pt-6">
						<div class="text-sm text-muted-foreground">Hold Time</div>
						<div class="text-2xl font-bold">
							{trade.trade.hold_time_minutes ? formatDuration(trade.trade.hold_time_minutes) : 'N/A'}
						</div>
					</CardContent>
				</Card>
			</div>
		{/if}

		<div class="grid gap-6 md:grid-cols-2">
			<!-- Position Details -->
			<Card>
				<CardHeader>
					<CardTitle>Position Details</CardTitle>
				</CardHeader>
				<CardContent class="space-y-3">
					<div class="flex justify-between">
						<span class="text-muted-foreground">Entry Price</span>
						<span class="font-medium">{formatCurrency(trade.trade.entry_price)}</span>
					</div>
					<div class="flex justify-between">
						<span class="text-muted-foreground">Quantity</span>
						<span class="font-medium">{trade.trade.quantity}</span>
					</div>
					{#if trade.trade.exit_price}
						<div class="flex justify-between">
							<span class="text-muted-foreground">Exit Price</span>
							<span class="font-medium">{formatCurrency(trade.trade.exit_price)}</span>
						</div>
					{/if}
					{#if trade.trade.stop_loss}
						<div class="flex justify-between">
							<span class="text-muted-foreground">Stop Loss</span>
							<span class="font-medium">{formatCurrency(trade.trade.stop_loss)}</span>
						</div>
					{/if}
					{#if trade.trade.take_profit}
						<div class="flex justify-between">
							<span class="text-muted-foreground">Take Profit</span>
							<span class="font-medium">{formatCurrency(trade.trade.take_profit)}</span>
						</div>
					{/if}
					{#if trade.trade.commissions}
						<div class="flex justify-between">
							<span class="text-muted-foreground">Commissions</span>
							<span class="font-medium">{formatCurrency(trade.trade.commissions)}</span>
						</div>
					{/if}
				</CardContent>
			</Card>

			<!-- Setup & Strategy -->
			<Card>
				<CardHeader>
					<CardTitle>Setup & Strategy</CardTitle>
				</CardHeader>
				<CardContent class="space-y-3">
					{#if trade.trade.setup_name}
						<div class="flex justify-between">
							<span class="text-muted-foreground">Setup</span>
							<span class="font-medium">{trade.trade.setup_name}</span>
						</div>
					{/if}
					{#if trade.trade.timeframe}
						<div class="flex justify-between">
							<span class="text-muted-foreground">Timeframe</span>
							<span class="font-medium">{trade.trade.timeframe}</span>
						</div>
					{/if}
					{#if trade.trade.conviction}
						<div class="flex justify-between">
							<span class="text-muted-foreground">Conviction</span>
							<span class="font-medium capitalize">{trade.trade.conviction}</span>
						</div>
					{/if}
					<div class="flex justify-between">
						<span class="text-muted-foreground">Asset Class</span>
						<span class="font-medium capitalize">{trade.trade.asset_class}</span>
					</div>
					{#if trade.trade.is_paper_trade}
						<div class="flex justify-between">
							<span class="text-muted-foreground">Type</span>
							<Badge variant="secondary">
								{#snippet children()}
									Paper Trade
								{/snippet}
							</Badge>
						</div>
					{/if}
				</CardContent>
			</Card>
		</div>

		<!-- Thesis & Notes -->
		{#if trade.trade.thesis}
			<Card>
				<CardHeader>
					<CardTitle>Trade Thesis</CardTitle>
				</CardHeader>
				<CardContent>
					<p class="text-sm whitespace-pre-wrap">{trade.trade.thesis}</p>
				</CardContent>
			</Card>
		{/if}

		{#if trade.trade.mistakes || trade.trade.lessons}
			<div class="grid gap-6 md:grid-cols-2">
				{#if trade.trade.mistakes}
					<Card>
						<CardHeader>
							<CardTitle>Mistakes</CardTitle>
						</CardHeader>
						<CardContent>
							<p class="text-sm whitespace-pre-wrap">{trade.trade.mistakes}</p>
						</CardContent>
					</Card>
				{/if}
				{#if trade.trade.lessons}
					<Card>
						<CardHeader>
							<CardTitle>Lessons Learned</CardTitle>
						</CardHeader>
						<CardContent>
							<p class="text-sm whitespace-pre-wrap">{trade.trade.lessons}</p>
						</CardContent>
					</Card>
				{/if}
			</div>
		{/if}

		<!-- Tags -->
		{#if trade.tags.length > 0}
			<Card>
				<CardHeader>
					<CardTitle>Tags</CardTitle>
				</CardHeader>
				<CardContent>
					<div class="flex flex-wrap gap-2">
						{#each trade.tags as tag}
							<Badge variant="outline">
								{#snippet children()}
									{tag.name}
								{/snippet}
							</Badge>
						{/each}
					</div>
				</CardContent>
			</Card>
		{/if}
	</div>

	<!-- Close Trade Dialog -->
	<Dialog bind:open={closeDialogOpen}>
		{#snippet children()}
			<DialogHeader>
				<DialogTitle>Close Trade</DialogTitle>
			</DialogHeader>
			<div class="space-y-4 py-4">
				<div class="grid gap-4 md:grid-cols-2">
					<div class="space-y-2">
						<Label for="exit_date" required>Exit Date & Time</Label>
						<Input
							id="exit_date"
							type="datetime-local"
							bind:value={closeFormData.exit_date}
							required
						/>
					</div>
					<div class="space-y-2">
						<Label for="exit_price" required>Exit Price</Label>
						<Input
							id="exit_price"
							type="number"
							step="0.01"
							bind:value={closeFormData.exit_price}
							required
						/>
					</div>
				</div>
				<div class="space-y-2">
					<Label for="mistakes">Mistakes</Label>
					<Textarea
						id="mistakes"
						bind:value={closeFormData.mistakes}
						placeholder="What went wrong?"
						rows={3}
					/>
				</div>
				<div class="space-y-2">
					<Label for="lessons">Lessons Learned</Label>
					<Textarea
						id="lessons"
						bind:value={closeFormData.lessons}
						placeholder="What did you learn?"
						rows={3}
					/>
				</div>
				<div class="grid gap-4 md:grid-cols-2">
					<div class="flex items-center space-x-2">
						<input
							type="checkbox"
							id="broke_rules"
							bind:checked={closeFormData.broke_rules}
							class="h-4 w-4 rounded"
						/>
						<Label for="broke_rules">Broke trading rules</Label>
					</div>
					<div class="flex items-center space-x-2">
						<input
							type="checkbox"
							id="followed_plan"
							bind:checked={closeFormData.followed_plan}
							class="h-4 w-4 rounded"
						/>
						<Label for="followed_plan">Followed trading plan</Label>
					</div>
				</div>
			</div>
			<DialogFooter>
				<Button variant="outline" onclick={() => closeDialogOpen = false}>
					{#snippet children()}
						Cancel
					{/snippet}
				</Button>
				<Button onclick={handleCloseTrade}>
					{#snippet children()}
						Close Trade
					{/snippet}
				</Button>
			</DialogFooter>
		{/snippet}
	</Dialog>

	<!-- Delete Confirmation Dialog -->
	<Dialog bind:open={deleteDialogOpen}>
		{#snippet children()}
			<DialogHeader>
				<DialogTitle>Delete Trade</DialogTitle>
			</DialogHeader>
			<p class="py-4">Are you sure you want to delete this trade? This action cannot be undone.</p>
			<DialogFooter>
				<Button variant="outline" onclick={() => deleteDialogOpen = false}>
					{#snippet children()}
						Cancel
					{/snippet}
				</Button>
				<Button variant="destructive" onclick={handleDeleteTrade}>
					{#snippet children()}
						Delete
					{/snippet}
				</Button>
			</DialogFooter>
		{/snippet}
	</Dialog>
{/if}
