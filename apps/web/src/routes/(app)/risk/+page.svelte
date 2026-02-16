<script lang="ts">
	import { apiClient } from '$lib/api';
	import Button from '$lib/components/ui/button.svelte';
	import Input from '$lib/components/ui/input.svelte';
	import Label from '$lib/components/ui/label.svelte';
	import Card from '$lib/components/ui/card.svelte';
	import CardHeader from '$lib/components/ui/card-header.svelte';
	import CardTitle from '$lib/components/ui/card-title.svelte';
	import CardContent from '$lib/components/ui/card-content.svelte';
	import { formatCurrency } from '$lib/utils/format';

	// Position Size Calculator
	let posForm = $state({ account_size: '', risk_percent: '', entry_price: '', stop_loss: '' });
	let posResult = $state<{ position_size: number; risk_amount: number; position_value: number } | null>(null);

	// Risk/Reward Calculator
	let rrForm = $state({ entry_price: '', stop_loss: '', target_price: '' });
	let rrResult = $state<{ risk_reward_ratio: number | null; risk_amount: number; reward_amount: number } | null>(null);

	// Kelly Criterion
	let kellyForm = $state({ win_rate: '', avg_win: '', avg_loss: '' });
	let kellyResult = $state<{ kelly_percentage: number; half_kelly: number; quarter_kelly: number } | null>(null);

	async function calculatePositionSize() {
		try {
			posResult = await apiClient.post('/api/v1/risk/position-size', {
				account_size: parseFloat(posForm.account_size),
				risk_percent: parseFloat(posForm.risk_percent),
				entry_price: parseFloat(posForm.entry_price),
				stop_loss: parseFloat(posForm.stop_loss),
			});
		} catch { /* validation handled by API */ }
	}

	async function calculateRiskReward() {
		try {
			rrResult = await apiClient.post('/api/v1/risk/risk-reward', {
				entry_price: parseFloat(rrForm.entry_price),
				stop_loss: parseFloat(rrForm.stop_loss),
				target_price: parseFloat(rrForm.target_price),
			});
		} catch { /* validation handled by API */ }
	}

	async function calculateKelly() {
		try {
			kellyResult = await apiClient.post('/api/v1/risk/kelly', {
				win_rate: parseFloat(kellyForm.win_rate),
				avg_win: parseFloat(kellyForm.avg_win),
				avg_loss: parseFloat(kellyForm.avg_loss),
			});
		} catch { /* validation handled by API */ }
	}
</script>

<svelte:head>
	<title>Risk Management - TradeMaster AI</title>
</svelte:head>

<div class="p-6 max-w-6xl mx-auto space-y-6">
	<div>
		<h1 class="text-3xl font-bold">Risk Management</h1>
		<p class="text-muted-foreground mt-1">Position sizing, risk/reward analysis, and Kelly criterion</p>
	</div>

	<div class="grid gap-6 lg:grid-cols-3">
		<!-- Position Size Calculator -->
		<Card>
			<CardHeader>
				<CardTitle>Position Size Calculator</CardTitle>
			</CardHeader>
			<CardContent class="space-y-3">
				<div class="space-y-1">
					<Label for="ps-account">Account Size ($)</Label>
					<Input id="ps-account" type="number" step="0.01" bind:value={posForm.account_size} placeholder="10000" />
				</div>
				<div class="space-y-1">
					<Label for="ps-risk">Risk Per Trade (%)</Label>
					<Input id="ps-risk" type="number" step="0.1" bind:value={posForm.risk_percent} placeholder="1" />
				</div>
				<div class="space-y-1">
					<Label for="ps-entry">Entry Price ($)</Label>
					<Input id="ps-entry" type="number" step="0.01" bind:value={posForm.entry_price} placeholder="150.00" />
				</div>
				<div class="space-y-1">
					<Label for="ps-stop">Stop Loss ($)</Label>
					<Input id="ps-stop" type="number" step="0.01" bind:value={posForm.stop_loss} placeholder="148.00" />
				</div>
				<Button onclick={calculatePositionSize}>
					{#snippet children()}Calculate{/snippet}
				</Button>

				{#if posResult}
					<div class="mt-4 p-3 bg-muted rounded-lg space-y-2">
						<div class="flex justify-between text-sm">
							<span class="text-muted-foreground">Shares:</span>
							<span class="font-semibold">{Math.floor(posResult.position_size)}</span>
						</div>
						<div class="flex justify-between text-sm">
							<span class="text-muted-foreground">Risk Amount:</span>
							<span class="font-semibold">{formatCurrency(posResult.risk_amount)}</span>
						</div>
						<div class="flex justify-between text-sm">
							<span class="text-muted-foreground">Position Value:</span>
							<span class="font-semibold">{formatCurrency(posResult.position_value)}</span>
						</div>
					</div>
				{/if}
			</CardContent>
		</Card>

		<!-- Risk/Reward Calculator -->
		<Card>
			<CardHeader>
				<CardTitle>Risk/Reward Ratio</CardTitle>
			</CardHeader>
			<CardContent class="space-y-3">
				<div class="space-y-1">
					<Label for="rr-entry">Entry Price ($)</Label>
					<Input id="rr-entry" type="number" step="0.01" bind:value={rrForm.entry_price} placeholder="150.00" />
				</div>
				<div class="space-y-1">
					<Label for="rr-stop">Stop Loss ($)</Label>
					<Input id="rr-stop" type="number" step="0.01" bind:value={rrForm.stop_loss} placeholder="148.00" />
				</div>
				<div class="space-y-1">
					<Label for="rr-target">Target Price ($)</Label>
					<Input id="rr-target" type="number" step="0.01" bind:value={rrForm.target_price} placeholder="156.00" />
				</div>
				<Button onclick={calculateRiskReward}>
					{#snippet children()}Calculate{/snippet}
				</Button>

				{#if rrResult}
					<div class="mt-4 p-3 bg-muted rounded-lg space-y-2">
						<div class="flex justify-between text-sm">
							<span class="text-muted-foreground">R:R Ratio:</span>
							<span class="font-semibold text-lg">
								{rrResult.risk_reward_ratio ? `1:${rrResult.risk_reward_ratio.toFixed(2)}` : 'N/A'}
							</span>
						</div>
						<div class="flex justify-between text-sm">
							<span class="text-muted-foreground">Risk:</span>
							<span class="font-semibold text-destructive">{formatCurrency(rrResult.risk_amount)}</span>
						</div>
						<div class="flex justify-between text-sm">
							<span class="text-muted-foreground">Reward:</span>
							<span class="font-semibold text-green-600">{formatCurrency(rrResult.reward_amount)}</span>
						</div>
					</div>
				{/if}
			</CardContent>
		</Card>

		<!-- Kelly Criterion -->
		<Card>
			<CardHeader>
				<CardTitle>Kelly Criterion</CardTitle>
			</CardHeader>
			<CardContent class="space-y-3">
				<div class="space-y-1">
					<Label for="k-wr">Win Rate (%)</Label>
					<Input id="k-wr" type="number" step="0.1" bind:value={kellyForm.win_rate} placeholder="55" />
				</div>
				<div class="space-y-1">
					<Label for="k-aw">Average Win ($)</Label>
					<Input id="k-aw" type="number" step="0.01" bind:value={kellyForm.avg_win} placeholder="200" />
				</div>
				<div class="space-y-1">
					<Label for="k-al">Average Loss ($)</Label>
					<Input id="k-al" type="number" step="0.01" bind:value={kellyForm.avg_loss} placeholder="100" />
				</div>
				<Button onclick={calculateKelly}>
					{#snippet children()}Calculate{/snippet}
				</Button>

				{#if kellyResult}
					<div class="mt-4 p-3 bg-muted rounded-lg space-y-2">
						<div class="flex justify-between text-sm">
							<span class="text-muted-foreground">Full Kelly:</span>
							<span class="font-semibold">{kellyResult.kelly_percentage.toFixed(2)}%</span>
						</div>
						<div class="flex justify-between text-sm">
							<span class="text-muted-foreground">Half Kelly:</span>
							<span class="font-semibold text-green-600">{kellyResult.half_kelly.toFixed(2)}%</span>
						</div>
						<div class="flex justify-between text-sm">
							<span class="text-muted-foreground">Quarter Kelly:</span>
							<span class="font-semibold">{kellyResult.quarter_kelly.toFixed(2)}%</span>
						</div>
						<p class="text-xs text-muted-foreground mt-2">
							Half Kelly is recommended for most traders as it balances growth with drawdown protection.
						</p>
					</div>
				{/if}
			</CardContent>
		</Card>
	</div>
</div>
