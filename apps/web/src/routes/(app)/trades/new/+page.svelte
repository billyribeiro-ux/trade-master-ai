<script lang="ts">
	import { goto } from '$app/navigation';
	import { tradesApi, tagsApi } from '$lib/api';
	import Button from '$lib/components/ui/button.svelte';
	import Input from '$lib/components/ui/input.svelte';
	import Label from '$lib/components/ui/label.svelte';
	import Select from '$lib/components/ui/select.svelte';
	import Textarea from '$lib/components/ui/textarea.svelte';
	import Card from '$lib/components/ui/card.svelte';
	import CardHeader from '$lib/components/ui/card-header.svelte';
	import CardTitle from '$lib/components/ui/card-title.svelte';
	import CardContent from '$lib/components/ui/card-content.svelte';
	import CardFooter from '$lib/components/ui/card-footer.svelte';
	import { toasts } from '$lib/stores/toast.svelte';
	import { onMount } from 'svelte';
	import type { TagWithCount } from '$lib/types/tag';

	let loading = $state(false);
	let tags = $state<TagWithCount[]>([]);
	let selectedTags = $state<string[]>([]);

	let formData = $state({
		symbol: '',
		direction: 'long' as 'long' | 'short',
		asset_class: 'stocks' as 'stocks' | 'options' | 'futures' | 'forex' | 'crypto',
		entry_date: new Date().toISOString().slice(0, 16),
		entry_price: '',
		quantity: '',
		stop_loss: '',
		take_profit: '',
		risk_percent: '',
		conviction: '' as '' | 'low' | 'medium' | 'high',
		setup_name: '',
		timeframe: '',
		thesis: '',
		emotional_state: '',
		market_condition: '',
		is_paper_trade: false,
		commissions: ''
	});

	let calculatedValues = $derived({
		positionValue: formData.entry_price && formData.quantity 
			? parseFloat(formData.entry_price) * parseFloat(formData.quantity)
			: 0,
		riskAmount: formData.entry_price && formData.quantity && formData.stop_loss
			? Math.abs(parseFloat(formData.entry_price) - parseFloat(formData.stop_loss)) * parseFloat(formData.quantity)
			: 0
	});

	onMount(async () => {
		try {
			tags = await tagsApi.list();
		} catch (error) {
			console.error('Failed to load tags:', error);
		}
	});

	async function handleSubmit(e: Event) {
		e.preventDefault();
		
		if (!formData.symbol || !formData.entry_price || !formData.quantity) {
			toasts.error('Missing required fields', 'Please fill in symbol, entry price, and quantity');
			return;
		}

		loading = true;

		try {
			const trade = await tradesApi.create({
				symbol: formData.symbol.toUpperCase(),
				direction: formData.direction,
				asset_class: formData.asset_class,
				entry_date: new Date(formData.entry_date).toISOString(),
				entry_price: parseFloat(formData.entry_price),
				quantity: parseFloat(formData.quantity),
				stop_loss: formData.stop_loss ? parseFloat(formData.stop_loss) : undefined,
				take_profit: formData.take_profit ? parseFloat(formData.take_profit) : undefined,
				risk_percent: formData.risk_percent ? parseFloat(formData.risk_percent) : undefined,
				conviction: formData.conviction || undefined,
				setup_name: formData.setup_name || undefined,
				timeframe: formData.timeframe || undefined,
				thesis: formData.thesis || undefined,
				emotional_state: formData.emotional_state || undefined,
				market_condition: formData.market_condition || undefined,
				is_paper_trade: formData.is_paper_trade,
				commissions: formData.commissions ? parseFloat(formData.commissions) : undefined
			});

			// Add tags to trade
			for (const tagId of selectedTags) {
				await tagsApi.addToTrade(trade.id, tagId);
			}

			toasts.success('Trade logged successfully!');
			goto(`/trades/${trade.id}`);
		} catch (error) {
			toasts.error('Failed to log trade', error instanceof Error ? error.message : 'Please try again');
		} finally {
			loading = false;
		}
	}

	function toggleTag(tagId: string) {
		if (selectedTags.includes(tagId)) {
			selectedTags = selectedTags.filter(id => id !== tagId);
		} else {
			selectedTags = [...selectedTags, tagId];
		}
	}
</script>

<svelte:head>
	<title>Log New Trade - TradeMaster AI</title>
</svelte:head>

<div class="p-6 max-w-4xl mx-auto space-y-6">
	<div>
		<h1 class="text-3xl font-bold">Log New Trade</h1>
		<p class="text-muted-foreground mt-1">Record your trade details</p>
	</div>

	<form onsubmit={handleSubmit} class="space-y-6">
		<!-- Basic Information -->
		<Card>
			<CardHeader>
				<CardTitle>Basic Information</CardTitle>
			</CardHeader>
			<CardContent class="space-y-4">
				<div class="grid gap-4 md:grid-cols-2">
					<div class="space-y-2">
						<Label for="symbol" required>Symbol</Label>
						<Input
							id="symbol"
							bind:value={formData.symbol}
							placeholder="AAPL"
							required
							disabled={loading}
						/>
					</div>
					<div class="space-y-2">
						<Label for="asset_class" required>Asset Class</Label>
						<Select
							id="asset_class"
							bind:value={formData.asset_class}
							required
							disabled={loading}
							options={[
								{ value: 'stocks', label: 'Stocks' },
								{ value: 'options', label: 'Options' },
								{ value: 'futures', label: 'Futures' },
								{ value: 'forex', label: 'Forex' },
								{ value: 'crypto', label: 'Crypto' }
							]}
						/>
					</div>
				</div>

				<div class="grid gap-4 md:grid-cols-2">
					<div class="space-y-2">
						<Label for="direction" required>Direction</Label>
						<Select
							id="direction"
							bind:value={formData.direction}
							required
							disabled={loading}
							options={[
								{ value: 'long', label: 'Long' },
								{ value: 'short', label: 'Short' }
							]}
						/>
					</div>
					<div class="space-y-2">
						<Label for="entry_date" required>Entry Date & Time</Label>
						<Input
							id="entry_date"
							type="datetime-local"
							bind:value={formData.entry_date}
							required
							disabled={loading}
						/>
					</div>
				</div>
			</CardContent>
		</Card>

		<!-- Position Details -->
		<Card>
			<CardHeader>
				<CardTitle>Position Details</CardTitle>
			</CardHeader>
			<CardContent class="space-y-4">
				<div class="grid gap-4 md:grid-cols-3">
					<div class="space-y-2">
						<Label for="entry_price" required>Entry Price</Label>
						<Input
							id="entry_price"
							type="number"
							step="0.01"
							bind:value={formData.entry_price}
							placeholder="100.00"
							required
							disabled={loading}
						/>
					</div>
					<div class="space-y-2">
						<Label for="quantity" required>Quantity</Label>
						<Input
							id="quantity"
							type="number"
							step="0.01"
							bind:value={formData.quantity}
							placeholder="100"
							required
							disabled={loading}
						/>
					</div>
					<div class="space-y-2">
						<Label for="commissions">Commissions</Label>
						<Input
							id="commissions"
							type="number"
							step="0.01"
							bind:value={formData.commissions}
							placeholder="0.00"
							disabled={loading}
						/>
					</div>
				</div>

				{#if calculatedValues.positionValue > 0}
					<div class="rounded-lg bg-muted p-4">
						<p class="text-sm text-muted-foreground">
							Position Value: <span class="font-medium text-foreground">${calculatedValues.positionValue.toFixed(2)}</span>
						</p>
					</div>
				{/if}
			</CardContent>
		</Card>

		<!-- Risk Management -->
		<Card>
			<CardHeader>
				<CardTitle>Risk Management</CardTitle>
			</CardHeader>
			<CardContent class="space-y-4">
				<div class="grid gap-4 md:grid-cols-3">
					<div class="space-y-2">
						<Label for="stop_loss">Stop Loss</Label>
						<Input
							id="stop_loss"
							type="number"
							step="0.01"
							bind:value={formData.stop_loss}
							placeholder="95.00"
							disabled={loading}
						/>
					</div>
					<div class="space-y-2">
						<Label for="take_profit">Take Profit</Label>
						<Input
							id="take_profit"
							type="number"
							step="0.01"
							bind:value={formData.take_profit}
							placeholder="110.00"
							disabled={loading}
						/>
					</div>
					<div class="space-y-2">
						<Label for="risk_percent">Risk %</Label>
						<Input
							id="risk_percent"
							type="number"
							step="0.1"
							bind:value={formData.risk_percent}
							placeholder="1.0"
							disabled={loading}
						/>
					</div>
				</div>

				{#if calculatedValues.riskAmount > 0}
					<div class="rounded-lg bg-muted p-4">
						<p class="text-sm text-muted-foreground">
							Risk Amount: <span class="font-medium text-foreground">${calculatedValues.riskAmount.toFixed(2)}</span>
						</p>
					</div>
				{/if}
			</CardContent>
		</Card>

		<!-- Setup & Strategy -->
		<Card>
			<CardHeader>
				<CardTitle>Setup & Strategy</CardTitle>
			</CardHeader>
			<CardContent class="space-y-4">
				<div class="grid gap-4 md:grid-cols-3">
					<div class="space-y-2">
						<Label for="setup_name">Setup Name</Label>
						<Input
							id="setup_name"
							bind:value={formData.setup_name}
							placeholder="Bull Flag Breakout"
							disabled={loading}
						/>
					</div>
					<div class="space-y-2">
						<Label for="timeframe">Timeframe</Label>
						<Input
							id="timeframe"
							bind:value={formData.timeframe}
							placeholder="5m, 15m, 1h, 4h, 1D"
							disabled={loading}
						/>
					</div>
					<div class="space-y-2">
						<Label for="conviction">Conviction</Label>
						<Select
							id="conviction"
							bind:value={formData.conviction}
							disabled={loading}
							options={[
								{ value: '', label: 'Select conviction' },
								{ value: 'low', label: 'Low' },
								{ value: 'medium', label: 'Medium' },
								{ value: 'high', label: 'High' }
							]}
						/>
					</div>
				</div>

				<div class="space-y-2">
					<Label for="thesis">Trade Thesis</Label>
					<Textarea
						id="thesis"
						bind:value={formData.thesis}
						placeholder="Why are you taking this trade? What's your analysis?"
						rows={4}
						disabled={loading}
					/>
				</div>
			</CardContent>
		</Card>

		<!-- Context -->
		<Card>
			<CardHeader>
				<CardTitle>Context & Psychology</CardTitle>
			</CardHeader>
			<CardContent class="space-y-4">
				<div class="grid gap-4 md:grid-cols-2">
					<div class="space-y-2">
						<Label for="emotional_state">Emotional State</Label>
						<Input
							id="emotional_state"
							bind:value={formData.emotional_state}
							placeholder="Calm, Confident, Anxious, etc."
							disabled={loading}
						/>
					</div>
					<div class="space-y-2">
						<Label for="market_condition">Market Condition</Label>
						<Input
							id="market_condition"
							bind:value={formData.market_condition}
							placeholder="Trending, Ranging, Volatile, etc."
							disabled={loading}
						/>
					</div>
				</div>

				<div class="flex items-center space-x-2">
					<input
						type="checkbox"
						id="is_paper_trade"
						bind:checked={formData.is_paper_trade}
						disabled={loading}
						class="h-4 w-4 rounded border-gray-300"
					/>
					<Label for="is_paper_trade">This is a paper trade (simulation)</Label>
				</div>
			</CardContent>
		</Card>

		<!-- Tags -->
		{#if tags.length > 0}
			<Card>
				<CardHeader>
					<CardTitle>Tags</CardTitle>
				</CardHeader>
				<CardContent>
					<div class="flex flex-wrap gap-2">
						{#each tags as tag}
							<button
								type="button"
								onclick={() => toggleTag(tag.id)}
								class="px-3 py-1 rounded-full text-sm border transition-colors {selectedTags.includes(tag.id) ? 'bg-primary text-primary-foreground border-primary' : 'bg-background border-input hover:bg-accent'}"
								disabled={loading}
							>
								{tag.name}
							</button>
						{/each}
					</div>
				</CardContent>
			</Card>
		{/if}

		<!-- Actions -->
		<CardFooter class="flex justify-between">
			<Button type="button" variant="outline" onclick={() => goto('/trades')} disabled={loading}>
				{#snippet children()}
					Cancel
				{/snippet}
			</Button>
			<Button type="submit" disabled={loading}>
				{#snippet children()}
					{loading ? 'Logging Trade...' : 'Log Trade'}
				{/snippet}
			</Button>
		</CardFooter>
	</form>
</div>
