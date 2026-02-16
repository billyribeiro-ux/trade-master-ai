<script lang="ts">
	import { goto } from '$app/navigation';
	import { apiClient } from '$lib/api';
	import Button from '$lib/components/ui/button.svelte';
	import Input from '$lib/components/ui/input.svelte';
	import Label from '$lib/components/ui/label.svelte';
	import Textarea from '$lib/components/ui/textarea.svelte';
	import Card from '$lib/components/ui/card.svelte';
	import CardHeader from '$lib/components/ui/card-header.svelte';
	import CardTitle from '$lib/components/ui/card-title.svelte';
	import CardContent from '$lib/components/ui/card-content.svelte';
	import Dialog from '$lib/components/ui/dialog.svelte';
	import DialogHeader from '$lib/components/ui/dialog-header.svelte';
	import DialogTitle from '$lib/components/ui/dialog-title.svelte';
	import DialogFooter from '$lib/components/ui/dialog-footer.svelte';
	import Badge from '$lib/components/ui/badge.svelte';
	import { toasts } from '$lib/stores/toast.svelte';
	import { formatCurrency, formatDate } from '$lib/utils/format';
	import { onMount } from 'svelte';

	let loading = $state(true);
	let todaysPlan = $state<any>(null);
	let watchlistDialogOpen = $state(false);

	let planForm = $state({
		market_bias: '',
		trade_plan: '',
		max_trades: '',
		max_loss: '',
		pre_market_notes: '',
		emotional_state: '',
		key_levels: [] as string[],
		focus_setups: [] as string[],
		goals_for_day: [] as string[]
	});

	let watchlistForm = $state({
		symbol: '',
		entry_price: '',
		stop_loss: '',
		target_price: '',
		setup_type: '',
		notes: ''
	});

	let newKeyLevel = $state('');
	let newSetup = $state('');
	let newGoal = $state('');

	onMount(async () => {
		await loadTodaysPlan();
	});

	async function loadTodaysPlan() {
		loading = true;
		try {
			const today = new Date().toISOString();
			const response = await apiClient.get<any>(`/api/v1/plans/by-date?date=${today}`);
			
			if (response) {
				todaysPlan = response;
				if (response.plan) {
					planForm = {
						market_bias: response.plan.market_bias || '',
						trade_plan: response.plan.trade_plan || '',
						max_trades: response.plan.max_trades?.toString() || '',
						max_loss: response.plan.max_loss?.toString() || '',
						pre_market_notes: response.plan.pre_market_notes || '',
						emotional_state: response.plan.emotional_state || '',
						key_levels: response.plan.key_levels || [],
						focus_setups: response.plan.focus_setups || [],
						goals_for_day: response.plan.goals_for_day || []
					};
				}
			}
		} catch (error) {
			console.error('Failed to load plan:', error);
		} finally {
			loading = false;
		}
	}

	async function savePlan() {
		loading = true;
		try {
			const data = {
				plan_date: new Date().toISOString(),
				market_bias: planForm.market_bias || null,
				trade_plan: planForm.trade_plan || null,
				max_trades: planForm.max_trades ? parseInt(planForm.max_trades) : null,
				max_loss: planForm.max_loss ? parseFloat(planForm.max_loss) : null,
				pre_market_notes: planForm.pre_market_notes || null,
				emotional_state: planForm.emotional_state || null,
				key_levels: planForm.key_levels.length > 0 ? planForm.key_levels : null,
				focus_setups: planForm.focus_setups.length > 0 ? planForm.focus_setups : null,
				goals_for_day: planForm.goals_for_day.length > 0 ? planForm.goals_for_day : null
			};

			if (todaysPlan?.plan) {
				await apiClient.put(`/api/v1/plans/${todaysPlan.plan.id}`, data);
				toasts.success('Plan updated successfully');
			} else {
				await apiClient.post('/api/v1/plans', data);
				toasts.success('Plan created successfully');
			}

			await loadTodaysPlan();
		} catch (error) {
			toasts.error('Failed to save plan', error instanceof Error ? error.message : 'Please try again');
		} finally {
			loading = false;
		}
	}

	async function addWatchlistItem() {
		if (!todaysPlan?.plan) {
			toasts.error('Please create a plan first');
			return;
		}

		if (!watchlistForm.symbol) {
			toasts.error('Symbol is required');
			return;
		}

		try {
			await apiClient.post(`/api/v1/plans/${todaysPlan.plan.id}/watchlist`, {
				symbol: watchlistForm.symbol.toUpperCase(),
				entry_price: watchlistForm.entry_price ? parseFloat(watchlistForm.entry_price) : null,
				stop_loss: watchlistForm.stop_loss ? parseFloat(watchlistForm.stop_loss) : null,
				target_price: watchlistForm.target_price ? parseFloat(watchlistForm.target_price) : null,
				setup_type: watchlistForm.setup_type || null,
				notes: watchlistForm.notes || null
			});

			toasts.success('Added to watchlist');
			watchlistDialogOpen = false;
			watchlistForm = {
				symbol: '',
				entry_price: '',
				stop_loss: '',
				target_price: '',
				setup_type: '',
				notes: ''
			};
			await loadTodaysPlan();
		} catch (error) {
			toasts.error('Failed to add watchlist item');
		}
	}

	async function removeWatchlistItem(itemId: string) {
		if (!todaysPlan?.plan) return;

		try {
			await apiClient.delete(`/api/v1/plans/${todaysPlan.plan.id}/watchlist/${itemId}`);
			toasts.success('Removed from watchlist');
			await loadTodaysPlan();
		} catch (error) {
			toasts.error('Failed to remove item');
		}
	}

	function addKeyLevel() {
		if (newKeyLevel.trim()) {
			planForm.key_levels = [...planForm.key_levels, newKeyLevel.trim()];
			newKeyLevel = '';
		}
	}

	function removeKeyLevel(index: number) {
		planForm.key_levels = planForm.key_levels.filter((_, i) => i !== index);
	}

	function addSetup() {
		if (newSetup.trim()) {
			planForm.focus_setups = [...planForm.focus_setups, newSetup.trim()];
			newSetup = '';
		}
	}

	function removeSetup(index: number) {
		planForm.focus_setups = planForm.focus_setups.filter((_, i) => i !== index);
	}

	function addGoal() {
		if (newGoal.trim()) {
			planForm.goals_for_day = [...planForm.goals_for_day, newGoal.trim()];
			newGoal = '';
		}
	}

	function removeGoal(index: number) {
		planForm.goals_for_day = planForm.goals_for_day.filter((_, i) => i !== index);
	}
</script>

<svelte:head>
	<title>Daily Planning - TradeMaster AI</title>
</svelte:head>

<div class="p-6 max-w-6xl mx-auto space-y-6">
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold">Daily Planning</h1>
			<p class="text-muted-foreground mt-1">{formatDate(new Date().toISOString())}</p>
		</div>
		<Button onclick={savePlan} disabled={loading}>
			{#snippet children()}
				{loading ? 'Saving...' : 'Save Plan'}
			{/snippet}
		</Button>
	</div>

	<div class="grid gap-6 md:grid-cols-2">
		<!-- Pre-Market Plan -->
		<Card>
			<CardHeader>
				<CardTitle>Pre-Market Plan</CardTitle>
			</CardHeader>
			<CardContent class="space-y-4">
				<div class="space-y-2">
					<Label for="market_bias">Market Bias</Label>
					<Input
						id="market_bias"
						bind:value={planForm.market_bias}
						placeholder="Bullish, Bearish, Neutral, Range-bound"
						disabled={loading}
					/>
				</div>

				<div class="space-y-2">
					<Label for="emotional_state">Emotional State</Label>
					<Input
						id="emotional_state"
						bind:value={planForm.emotional_state}
						placeholder="Calm, Confident, Anxious, etc."
						disabled={loading}
					/>
				</div>

				<div class="space-y-2">
					<Label>Key Levels</Label>
					<div class="flex gap-2">
						<Input
							bind:value={newKeyLevel}
							placeholder="Add support/resistance level"
							onkeydown={(e) => e.key === 'Enter' && addKeyLevel()}
						/>
						<Button type="button" onclick={addKeyLevel} size="sm">
							{#snippet children()}
								Add
							{/snippet}
						</Button>
					</div>
					{#if planForm.key_levels.length > 0}
						<div class="flex flex-wrap gap-2 mt-2">
							{#each planForm.key_levels as level, i}
								<Badge variant="outline">
									{#snippet children()}
										{level}
										<button onclick={() => removeKeyLevel(i)} class="ml-2">×</button>
									{/snippet}
								</Badge>
							{/each}
						</div>
					{/if}
				</div>

				<div class="space-y-2">
					<Label>Focus Setups</Label>
					<div class="flex gap-2">
						<Input
							bind:value={newSetup}
							placeholder="Add setup to focus on"
							onkeydown={(e) => e.key === 'Enter' && addSetup()}
						/>
						<Button type="button" onclick={addSetup} size="sm">
							{#snippet children()}
								Add
							{/snippet}
						</Button>
					</div>
					{#if planForm.focus_setups.length > 0}
						<div class="flex flex-wrap gap-2 mt-2">
							{#each planForm.focus_setups as setup, i}
								<Badge variant="outline">
									{#snippet children()}
										{setup}
										<button onclick={() => removeSetup(i)} class="ml-2">×</button>
									{/snippet}
								</Badge>
							{/each}
						</div>
					{/if}
				</div>
			</CardContent>
		</Card>

		<!-- Risk Management -->
		<Card>
			<CardHeader>
				<CardTitle>Risk Management</CardTitle>
			</CardHeader>
			<CardContent class="space-y-4">
				<div class="space-y-2">
					<Label for="max_trades">Max Trades Today</Label>
					<Input
						id="max_trades"
						type="number"
						bind:value={planForm.max_trades}
						placeholder="3"
						disabled={loading}
					/>
				</div>

				<div class="space-y-2">
					<Label for="max_loss">Max Loss Today ($)</Label>
					<Input
						id="max_loss"
						type="number"
						step="0.01"
						bind:value={planForm.max_loss}
						placeholder="500.00"
						disabled={loading}
					/>
				</div>

				<div class="space-y-2">
					<Label>Goals for Today</Label>
					<div class="flex gap-2">
						<Input
							bind:value={newGoal}
							placeholder="Add a goal"
							onkeydown={(e) => e.key === 'Enter' && addGoal()}
						/>
						<Button type="button" onclick={addGoal} size="sm">
							{#snippet children()}
								Add
							{/snippet}
						</Button>
					</div>
					{#if planForm.goals_for_day.length > 0}
						<div class="space-y-2 mt-2">
							{#each planForm.goals_for_day as goal, i}
								<div class="flex items-center gap-2 text-sm">
									<span>• {goal}</span>
									<button onclick={() => removeGoal(i)} class="text-destructive">×</button>
								</div>
							{/each}
						</div>
					{/if}
				</div>
			</CardContent>
		</Card>
	</div>

	<!-- Trade Plan & Notes -->
	<Card>
		<CardHeader>
			<CardTitle>Trade Plan & Notes</CardTitle>
		</CardHeader>
		<CardContent class="space-y-4">
			<div class="space-y-2">
				<Label for="trade_plan">Trade Plan</Label>
				<Textarea
					id="trade_plan"
					bind:value={planForm.trade_plan}
					placeholder="What's your strategy for today? What setups are you looking for?"
					rows={4}
					disabled={loading}
				/>
			</div>

			<div class="space-y-2">
				<Label for="pre_market_notes">Pre-Market Notes</Label>
				<Textarea
					id="pre_market_notes"
					bind:value={planForm.pre_market_notes}
					placeholder="Market conditions, news, catalysts, etc."
					rows={4}
					disabled={loading}
				/>
			</div>
		</CardContent>
	</Card>

	<!-- Watchlist -->
	<Card>
		<CardHeader>
			<div class="flex items-center justify-between">
				<CardTitle>Watchlist</CardTitle>
				<Button onclick={() => watchlistDialogOpen = true} disabled={!todaysPlan?.plan}>
					{#snippet children()}
						Add Symbol
					{/snippet}
				</Button>
			</div>
		</CardHeader>
		<CardContent>
			{#if todaysPlan?.watchlist && todaysPlan.watchlist.length > 0}
				<div class="space-y-3">
					{#each todaysPlan.watchlist as item}
						<div class="flex items-center justify-between p-3 border rounded-lg">
							<div class="flex-1">
								<div class="flex items-center gap-3">
									<span class="font-bold text-lg">{item.symbol}</span>
									{#if item.setup_type}
										<Badge variant="outline">
											{#snippet children()}
												{item.setup_type}
											{/snippet}
										</Badge>
									{/if}
									{#if item.triggered}
										<Badge variant="success">
											{#snippet children()}
												Triggered
											{/snippet}
										</Badge>
									{/if}
								</div>
								<div class="flex gap-4 text-sm text-muted-foreground mt-1">
									{#if item.entry_price}
										<span>Entry: {formatCurrency(item.entry_price)}</span>
									{/if}
									{#if item.stop_loss}
										<span>Stop: {formatCurrency(item.stop_loss)}</span>
									{/if}
									{#if item.target_price}
										<span>Target: {formatCurrency(item.target_price)}</span>
									{/if}
								</div>
								{#if item.notes}
									<p class="text-sm text-muted-foreground mt-1">{item.notes}</p>
								{/if}
							</div>
							<Button variant="ghost" size="sm" onclick={() => removeWatchlistItem(item.id)}>
								{#snippet children()}
									Remove
								{/snippet}
							</Button>
						</div>
					{/each}
				</div>
			{:else}
				<p class="text-center text-muted-foreground py-8">
					No symbols on watchlist. Add symbols you're watching today.
				</p>
			{/if}
		</CardContent>
	</Card>
</div>

<!-- Add Watchlist Item Dialog -->
<Dialog bind:open={watchlistDialogOpen}>
	{#snippet children()}
		<DialogHeader>
			<DialogTitle>Add to Watchlist</DialogTitle>
		</DialogHeader>
		<div class="space-y-4 py-4">
			<div class="space-y-2">
				<Label for="symbol" required>Symbol</Label>
				<Input
					id="symbol"
					bind:value={watchlistForm.symbol}
					placeholder="AAPL"
					required
				/>
			</div>
			<div class="grid gap-4 md:grid-cols-3">
				<div class="space-y-2">
					<Label for="entry_price">Entry Price</Label>
					<Input
						id="entry_price"
						type="number"
						step="0.01"
						bind:value={watchlistForm.entry_price}
					/>
				</div>
				<div class="space-y-2">
					<Label for="stop_loss">Stop Loss</Label>
					<Input
						id="stop_loss"
						type="number"
						step="0.01"
						bind:value={watchlistForm.stop_loss}
					/>
				</div>
				<div class="space-y-2">
					<Label for="target_price">Target</Label>
					<Input
						id="target_price"
						type="number"
						step="0.01"
						bind:value={watchlistForm.target_price}
					/>
				</div>
			</div>
			<div class="space-y-2">
				<Label for="setup_type">Setup Type</Label>
				<Input
					id="setup_type"
					bind:value={watchlistForm.setup_type}
					placeholder="Bull Flag, Support Bounce, etc."
				/>
			</div>
			<div class="space-y-2">
				<Label for="notes">Notes</Label>
				<Textarea
					id="notes"
					bind:value={watchlistForm.notes}
					placeholder="Why are you watching this symbol?"
					rows={3}
				/>
			</div>
		</div>
		<DialogFooter>
			<Button variant="outline" onclick={() => watchlistDialogOpen = false}>
				{#snippet children()}
					Cancel
				{/snippet}
			</Button>
			<Button onclick={addWatchlistItem}>
				{#snippet children()}
					Add to Watchlist
				{/snippet}
			</Button>
		</DialogFooter>
	{/snippet}
</Dialog>
