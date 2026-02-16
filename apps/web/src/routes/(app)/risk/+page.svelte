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
	import { toasts } from '$lib/stores/toast.svelte';

	// Position Size Calculator
	let posForm = $state({ account_size: '', risk_percent: '1', entry_price: '', stop_loss: '' });
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
		} catch {
			toasts.error('Please fill in all fields with valid numbers');
		}
	}

	async function calculateRiskReward() {
		try {
			rrResult = await apiClient.post('/api/v1/risk/risk-reward', {
				entry_price: parseFloat(rrForm.entry_price),
				stop_loss: parseFloat(rrForm.stop_loss),
				target_price: parseFloat(rrForm.target_price),
			});
		} catch {
			toasts.error('Please fill in all fields with valid numbers');
		}
	}

	async function calculateKelly() {
		try {
			kellyResult = await apiClient.post('/api/v1/risk/kelly', {
				win_rate: parseFloat(kellyForm.win_rate),
				avg_win: parseFloat(kellyForm.avg_win),
				avg_loss: parseFloat(kellyForm.avg_loss),
			});
		} catch {
			toasts.error('Please fill in all fields with valid numbers');
		}
	}

	const rrBarWidth = $derived(
		rrResult?.risk_reward_ratio
			? Math.min(rrResult.risk_reward_ratio / (1 + rrResult.risk_reward_ratio) * 100, 90)
			: 50
	);

	const rrColor = $derived(
		rrResult?.risk_reward_ratio
			? rrResult.risk_reward_ratio >= 3 ? 'text-green-400' : rrResult.risk_reward_ratio >= 2 ? 'text-green-500' : rrResult.risk_reward_ratio >= 1 ? 'text-yellow-500' : 'text-red-500'
			: ''
	);

	const kellyColor = $derived(
		kellyResult
			? kellyResult.half_kelly >= 5 ? 'text-green-400' : kellyResult.half_kelly >= 2 ? 'text-yellow-500' : kellyResult.half_kelly > 0 ? 'text-orange-500' : 'text-red-500'
			: ''
	);

	const posAccountPercent = $derived(
		posResult && posForm.account_size
			? (posResult.position_value / parseFloat(posForm.account_size) * 100)
			: 0
	);
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
					<div class="mt-4 space-y-3">
						<div class="text-center p-4 bg-primary/10 rounded-xl">
							<div class="text-3xl font-bold">{Math.floor(posResult.position_size)}</div>
							<div class="text-xs text-muted-foreground mt-1">Shares</div>
						</div>
						<div class="grid grid-cols-2 gap-2">
							<div class="p-2 bg-muted/50 rounded-lg text-center">
								<div class="text-sm font-bold text-red-500">{formatCurrency(posResult.risk_amount)}</div>
								<div class="text-[10px] text-muted-foreground">Risk Amount</div>
							</div>
							<div class="p-2 bg-muted/50 rounded-lg text-center">
								<div class="text-sm font-bold">{formatCurrency(posResult.position_value)}</div>
								<div class="text-[10px] text-muted-foreground">Position Value</div>
							</div>
						</div>
						<!-- Account usage bar -->
						<div>
							<div class="flex justify-between text-[10px] text-muted-foreground mb-1">
								<span>Account Usage</span>
								<span>{posAccountPercent.toFixed(1)}%</span>
							</div>
							<div class="h-2 bg-muted rounded-full overflow-hidden">
								<div
									class="h-full rounded-full transition-all {posAccountPercent > 50 ? 'bg-red-500' : posAccountPercent > 25 ? 'bg-yellow-500' : 'bg-green-500'}"
									style="width: {Math.min(posAccountPercent, 100)}%"
								></div>
							</div>
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
					<div class="mt-4 space-y-3">
						<div class="text-center p-4 bg-primary/10 rounded-xl">
							<div class="text-3xl font-bold {rrColor}">
								{rrResult.risk_reward_ratio ? `1:${rrResult.risk_reward_ratio.toFixed(2)}` : 'N/A'}
							</div>
							<div class="text-xs text-muted-foreground mt-1">Risk : Reward</div>
						</div>
						<!-- Visual R:R bar -->
						<div class="relative h-8 rounded-lg overflow-hidden flex">
							<div
								class="bg-red-500/30 flex items-center justify-center text-xs font-medium"
								style="width: {100 - rrBarWidth}%"
							>
								{formatCurrency(rrResult.risk_amount)}
							</div>
							<div
								class="bg-green-500/30 flex items-center justify-center text-xs font-medium"
								style="width: {rrBarWidth}%"
							>
								{formatCurrency(rrResult.reward_amount)}
							</div>
						</div>
						<div class="flex justify-between text-[10px] text-muted-foreground">
							<span>Risk</span>
							<span>Reward</span>
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
					<div class="mt-4 space-y-3">
						<div class="text-center p-4 bg-primary/10 rounded-xl">
							<div class="text-3xl font-bold {kellyColor}">
								{kellyResult.half_kelly.toFixed(2)}%
							</div>
							<div class="text-xs text-muted-foreground mt-1">Half Kelly (Recommended)</div>
						</div>
						<div class="grid grid-cols-2 gap-2">
							<div class="p-2 bg-muted/50 rounded-lg text-center">
								<div class="text-sm font-bold">{kellyResult.kelly_percentage.toFixed(2)}%</div>
								<div class="text-[10px] text-muted-foreground">Full Kelly</div>
							</div>
							<div class="p-2 bg-muted/50 rounded-lg text-center">
								<div class="text-sm font-bold">{kellyResult.quarter_kelly.toFixed(2)}%</div>
								<div class="text-[10px] text-muted-foreground">Quarter Kelly</div>
							</div>
						</div>
						<!-- Kelly gauge bar -->
						<div>
							<div class="flex justify-between text-[10px] text-muted-foreground mb-1">
								<span>Conservative</span>
								<span>Aggressive</span>
							</div>
							<div class="h-3 bg-gradient-to-r from-green-500/30 via-yellow-500/30 to-red-500/30 rounded-full relative">
								<div
									class="absolute top-0 w-3 h-3 bg-white rounded-full border-2 border-primary shadow"
									style="left: clamp(0%, {Math.min(kellyResult.half_kelly / 20 * 100, 100)}%, calc(100% - 12px))"
								></div>
							</div>
						</div>
						<p class="text-xs text-muted-foreground">
							Half Kelly balances growth with drawdown protection. Quarter Kelly is safest.
						</p>
					</div>
				{/if}
			</CardContent>
		</Card>
	</div>
</div>
