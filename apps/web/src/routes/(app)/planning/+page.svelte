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
	import Dialog from '$lib/components/ui/dialog.svelte';
	import DialogHeader from '$lib/components/ui/dialog-header.svelte';
	import DialogTitle from '$lib/components/ui/dialog-title.svelte';
	import DialogFooter from '$lib/components/ui/dialog-footer.svelte';
	import Badge from '$lib/components/ui/badge.svelte';
	import { toasts } from '$lib/stores/toast.svelte';
	// formatDate unused - using todayStr instead
	import { onMount } from 'svelte';

	const biasOptions = [
		{ value: 'bullish', label: 'Bullish', emoji: '🟢', color: 'bg-green-500/20 border-green-500/50 text-green-400' },
		{ value: 'bearish', label: 'Bearish', emoji: '🔴', color: 'bg-red-500/20 border-red-500/50 text-red-400' },
		{ value: 'neutral', label: 'Neutral', emoji: '⚪', color: 'bg-gray-500/20 border-gray-500/50 text-gray-400' },
		{ value: 'range-bound', label: 'Range', emoji: '↔️', color: 'bg-yellow-500/20 border-yellow-500/50 text-yellow-400' },
	];

	let loading = $state(true);
	let saving = $state(false);
	let todaysPlan = $state<any>(null);
	let watchlistDialogOpen = $state(false);

	let planForm = $state({
		market_bias: '',
		bias_reasoning: '',
		session_goals: [] as string[],
		max_trades: '',
		max_daily_loss: '',
		notes: '',
	});

	let watchlistForm = $state({
		symbol: '',
		direction: 'long',
		setup_description: '',
		catalysts: '',
	});

	let newGoal = $state('');

	const todayStr = new Date().toLocaleDateString('en-US', {
		weekday: 'long',
		month: 'long',
		day: 'numeric',
		year: 'numeric',
	});

	onMount(async () => {
		await loadTodaysPlan();
	});

	async function loadTodaysPlan() {
		loading = true;
		try {
			const today = new Date().toISOString().split('T')[0];
			const response = await apiClient.get<any>(`/api/v1/plans/by-date?date=${today}`);

			if (response) {
				todaysPlan = response;
				if (response.plan) {
					planForm = {
						market_bias: response.plan.market_bias || '',
						bias_reasoning: response.plan.bias_reasoning || '',
						session_goals: response.plan.session_goals || [],
						max_trades: response.plan.max_trades?.toString() || '',
						max_daily_loss: response.plan.max_daily_loss?.toString() || '',
						notes: response.plan.notes || '',
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
		saving = true;
		try {
			const today = new Date().toISOString().split('T')[0];
			const data = {
				plan_date: today,
				market_bias: planForm.market_bias || null,
				bias_reasoning: planForm.bias_reasoning || null,
				session_goals: planForm.session_goals.length > 0 ? planForm.session_goals : null,
				max_trades: planForm.max_trades ? parseInt(planForm.max_trades) : null,
				max_daily_loss: planForm.max_daily_loss ? parseFloat(planForm.max_daily_loss) : null,
				notes: planForm.notes || null,
			};

			if (todaysPlan?.plan) {
				await apiClient.put(`/api/v1/plans/${todaysPlan.plan.id}`, data);
				toasts.success('Plan updated');
			} else {
				await apiClient.post('/api/v1/plans', data);
				toasts.success('Plan created');
			}

			await loadTodaysPlan();
		} catch (error) {
			toasts.error('Failed to save plan', error instanceof Error ? error.message : '');
		} finally {
			saving = false;
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
				direction: watchlistForm.direction || null,
				setup_description: watchlistForm.setup_description || null,
				catalysts: watchlistForm.catalysts || null,
			});

			toasts.success('Added to watchlist');
			watchlistDialogOpen = false;
			watchlistForm = { symbol: '', direction: 'long', setup_description: '', catalysts: '' };
			await loadTodaysPlan();
		} catch {
			toasts.error('Failed to add watchlist item');
		}
	}

	async function removeWatchlistItem(itemId: string) {
		if (!todaysPlan?.plan) return;
		try {
			await apiClient.delete(`/api/v1/plans/${todaysPlan.plan.id}/watchlist/${itemId}`);
			toasts.success('Removed from watchlist');
			await loadTodaysPlan();
		} catch {
			toasts.error('Failed to remove item');
		}
	}

	function addGoal() {
		if (newGoal.trim()) {
			planForm.session_goals = [...planForm.session_goals, newGoal.trim()];
			newGoal = '';
		}
	}

	function removeGoal(index: number) {
		planForm.session_goals = planForm.session_goals.filter((_: string, i: number) => i !== index);
	}

	const watchlistCount = $derived(todaysPlan?.watchlist?.length ?? 0);
	const goalsCount = $derived(planForm.session_goals.length);
</script>

<svelte:head>
	<title>Daily Planning - TradeMaster AI</title>
</svelte:head>

<div class="p-6 max-w-6xl mx-auto space-y-6">
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold">Daily Plan</h1>
			<p class="text-muted-foreground mt-1">{todayStr}</p>
		</div>
		<div class="flex items-center gap-3">
			{#if todaysPlan?.plan?.completed}
				<Badge variant="success">{#snippet children()}Day Complete ✓{/snippet}</Badge>
			{/if}
			<Button onclick={savePlan} disabled={saving || loading}>
				{#snippet children()}
					{saving ? 'Saving...' : todaysPlan?.plan ? 'Update Plan' : 'Create Plan'}
				{/snippet}
			</Button>
		</div>
	</div>

	<!-- Quick Stats -->
	{#if todaysPlan?.plan}
		<div class="grid grid-cols-3 gap-3">
			<div class="bg-muted/30 rounded-lg p-3 text-center">
				<div class="text-lg font-bold">{goalsCount}</div>
				<div class="text-xs text-muted-foreground">Goals Set</div>
			</div>
			<div class="bg-muted/30 rounded-lg p-3 text-center">
				<div class="text-lg font-bold">{watchlistCount}</div>
				<div class="text-xs text-muted-foreground">Watching</div>
			</div>
			<div class="bg-muted/30 rounded-lg p-3 text-center">
				<div class="text-lg font-bold">{planForm.max_trades || '∞'}</div>
				<div class="text-xs text-muted-foreground">Max Trades</div>
			</div>
		</div>
	{/if}

	<div class="grid gap-6 md:grid-cols-2">
		<!-- Market Analysis -->
		<Card>
			<CardHeader><CardTitle>Market Analysis</CardTitle></CardHeader>
			<CardContent class="space-y-4">
				<div class="space-y-2">
					<Label>Market Bias</Label>
					<div class="grid grid-cols-4 gap-2">
						{#each biasOptions as opt}
							<button
								class="p-2 rounded-lg border text-center text-sm transition-all {planForm.market_bias === opt.value
									? opt.color + ' border-2'
									: 'border-border hover:bg-muted'}"
								onclick={() => planForm.market_bias = planForm.market_bias === opt.value ? '' : opt.value}
							>
								<div class="text-lg">{opt.emoji}</div>
								<div class="text-xs mt-0.5">{opt.label}</div>
							</button>
						{/each}
					</div>
				</div>
				<div class="space-y-2">
					<Label for="bias_reasoning">Bias Reasoning</Label>
					<Textarea
						id="bias_reasoning"
						bind:value={planForm.bias_reasoning}
						placeholder="Why do you have this bias? Key factors, levels, news..."
						rows={4}
						disabled={loading}
					/>
				</div>
			</CardContent>
		</Card>

		<!-- Risk & Goals -->
		<Card>
			<CardHeader><CardTitle>Risk & Goals</CardTitle></CardHeader>
			<CardContent class="space-y-4">
				<div class="grid grid-cols-2 gap-3">
					<div class="space-y-2">
						<Label for="max_trades">Max Trades</Label>
						<Input
							id="max_trades"
							type="number"
							bind:value={planForm.max_trades}
							placeholder="3"
							disabled={loading}
						/>
					</div>
					<div class="space-y-2">
						<Label for="max_daily_loss">Max Loss ($)</Label>
						<Input
							id="max_daily_loss"
							type="number"
							step="0.01"
							bind:value={planForm.max_daily_loss}
							placeholder="500"
							disabled={loading}
						/>
					</div>
				</div>
				<div class="space-y-2">
					<Label>Session Goals</Label>
					<div class="flex gap-2">
						<Input
							bind:value={newGoal}
							placeholder="Add a goal..."
							onkeydown={(e: KeyboardEvent) => e.key === 'Enter' && addGoal()}
						/>
						<Button type="button" onclick={addGoal} size="sm">
							{#snippet children()}+{/snippet}
						</Button>
					</div>
					{#if planForm.session_goals.length > 0}
						<div class="space-y-1.5 mt-2">
							{#each planForm.session_goals as goal, i}
								<div class="flex items-center gap-2 p-2 bg-muted/30 rounded-lg group">
									<div class="w-5 h-5 rounded border-2 border-muted-foreground/30 flex items-center justify-center text-xs shrink-0">
										{i + 1}
									</div>
									<span class="text-sm flex-1">{goal}</span>
									<button
										onclick={() => removeGoal(i)}
										class="text-destructive opacity-0 group-hover:opacity-100 transition-opacity text-sm"
									>✕</button>
								</div>
							{/each}
						</div>
					{/if}
				</div>
			</CardContent>
		</Card>
	</div>

	<!-- Notes -->
	<Card>
		<CardHeader><CardTitle>Notes & Reminders</CardTitle></CardHeader>
		<CardContent>
			<Textarea
				bind:value={planForm.notes}
				placeholder="Market conditions, catalysts, reminders, anything else..."
				rows={3}
				disabled={loading}
			/>
		</CardContent>
	</Card>

	<!-- AI Plan of Attack -->
	{#if todaysPlan?.plan?.ai_plan_of_attack}
		<Card class="border-blue-800/30">
			<CardHeader>
				<CardTitle>
					<span class="flex items-center gap-2">🤖 AI Plan of Attack</span>
				</CardTitle>
			</CardHeader>
			<CardContent>
				<p class="text-sm whitespace-pre-wrap">{todaysPlan.plan.ai_plan_of_attack}</p>
			</CardContent>
		</Card>
	{/if}

	<!-- Watchlist -->
	<Card>
		<CardHeader>
			<div class="flex items-center justify-between">
				<CardTitle>Watchlist ({watchlistCount})</CardTitle>
				<Button onclick={() => watchlistDialogOpen = true} disabled={!todaysPlan?.plan} size="sm">
					{#snippet children()}+ Add Symbol{/snippet}
				</Button>
			</div>
		</CardHeader>
		<CardContent>
			{#if todaysPlan?.watchlist && todaysPlan.watchlist.length > 0}
				<div class="grid gap-3 md:grid-cols-2 lg:grid-cols-3">
					{#each todaysPlan.watchlist as item}
						<div class="p-3 border rounded-lg hover:bg-muted/30 transition-colors {item.was_traded ? 'border-blue-500/30 bg-blue-950/10' : ''}">
							<div class="flex items-center justify-between">
								<div class="flex items-center gap-2">
									<span class="font-bold text-lg">{item.symbol}</span>
									{#if item.direction}
										<span class="text-xs px-1.5 py-0.5 rounded {item.direction === 'long'
											? 'bg-green-500/20 text-green-400'
											: 'bg-red-500/20 text-red-400'}">
											{item.direction.toUpperCase()}
										</span>
									{/if}
								</div>
								<div class="flex items-center gap-1">
									{#if item.was_traded}
										<Badge variant="outline">{#snippet children()}Traded{/snippet}</Badge>
									{/if}
									<button
										onclick={() => removeWatchlistItem(item.id)}
										class="text-muted-foreground hover:text-destructive text-sm p-1"
									>✕</button>
								</div>
							</div>
							{#if item.setup_description}
								<p class="text-sm text-muted-foreground mt-1.5">{item.setup_description}</p>
							{/if}
							{#if item.catalysts}
								<p class="text-xs text-muted-foreground mt-1">💡 {item.catalysts}</p>
							{/if}
							{#if item.outcome}
								<p class="text-xs mt-1 font-medium">📊 {item.outcome}</p>
							{/if}
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

<!-- Add to Watchlist Dialog -->
<Dialog bind:open={watchlistDialogOpen}>
	{#snippet children()}
		<DialogHeader>
			<DialogTitle>Add to Watchlist</DialogTitle>
		</DialogHeader>
		<div class="space-y-4 py-4">
			<div class="space-y-2">
				<Label for="symbol" required>Symbol</Label>
				<Input id="symbol" bind:value={watchlistForm.symbol} placeholder="AAPL" required />
			</div>
			<div class="space-y-2">
				<Label>Direction</Label>
				<div class="flex gap-2">
					{#each ['long', 'short'] as dir}
						<button
							class="flex-1 py-2 rounded-lg border text-sm transition-all {watchlistForm.direction === dir
								? (dir === 'long' ? 'bg-green-500/20 border-green-500/50 text-green-400' : 'bg-red-500/20 border-red-500/50 text-red-400')
								: 'border-border hover:bg-muted'}"
							onclick={() => watchlistForm.direction = dir}
						>
							{dir === 'long' ? '🟢 Long' : '🔴 Short'}
						</button>
					{/each}
				</div>
			</div>
			<div class="space-y-2">
				<Label for="setup_desc">Setup Description</Label>
				<Textarea id="setup_desc" bind:value={watchlistForm.setup_description} placeholder="Bull flag near resistance..." rows={3} />
			</div>
			<div class="space-y-2">
				<Label for="catalysts">Catalysts</Label>
				<Input id="catalysts" bind:value={watchlistForm.catalysts} placeholder="Earnings, FDA approval..." />
			</div>
		</div>
		<DialogFooter>
			<Button variant="outline" onclick={() => watchlistDialogOpen = false}>
				{#snippet children()}Cancel{/snippet}
			</Button>
			<Button onclick={addWatchlistItem}>
				{#snippet children()}Add to Watchlist{/snippet}
			</Button>
		</DialogFooter>
	{/snippet}
</Dialog>
