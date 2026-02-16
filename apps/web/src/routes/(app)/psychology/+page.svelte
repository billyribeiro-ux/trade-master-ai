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
	import Checkbox from '$lib/components/ui/checkbox.svelte';
	import Spinner from '$lib/components/ui/spinner.svelte';
	import { toasts } from '$lib/stores/toast.svelte';
	import { formatCurrency, formatDate } from '$lib/utils/format';
	import { onMount } from 'svelte';

	interface MoodLog {
		id: string;
		log_date: string;
		pre_market_mood: number | null;
		during_trading_mood: number | null;
		post_market_mood: number | null;
		emotional_labels: string[] | null;
		sleep_quality: number | null;
		sleep_hours: number | null;
		exercised: boolean | null;
		meditated: boolean | null;
		journal_entry: string | null;
		felt_fomo: boolean;
		felt_revenge: boolean;
		felt_overconfident: boolean;
		felt_fearful: boolean;
	}

	interface Insights {
		avg_pre_market_mood: number | null;
		avg_during_mood: number | null;
		avg_post_mood: number | null;
		avg_sleep_quality: number | null;
		avg_sleep_hours: number | null;
		fomo_count: number;
		revenge_count: number;
		overconfident_count: number;
		fearful_count: number;
		total_logs: number;
		high_mood_avg_pnl: number | null;
		low_mood_avg_pnl: number | null;
		good_sleep_avg_pnl: number | null;
		poor_sleep_avg_pnl: number | null;
	}

	let logs = $state<MoodLog[]>([]);
	let insights = $state<Insights | null>(null);
	let loading = $state(true);
	let saving = $state(false);
	let activeTab = $state<'log' | 'history' | 'insights'>('log');

	let form = $state({
		pre_market_mood: '7',
		during_trading_mood: '',
		post_market_mood: '',
		sleep_quality: '7',
		sleep_hours: '7',
		journal_entry: '',
		felt_fomo: false,
		felt_revenge: false,
		felt_overconfident: false,
		felt_fearful: false,
		exercised: false,
		meditated: false,
	});

	onMount(async () => {
		await Promise.all([loadLogs(), loadInsights()]);
	});

	async function loadLogs() {
		try {
			logs = await apiClient.get<MoodLog[]>('/api/v1/psychology/mood-logs') ?? [];
		} catch { /* silent */ }
		loading = false;
	}

	async function loadInsights() {
		try {
			insights = await apiClient.get<Insights>('/api/v1/psychology/insights') ?? null;
		} catch { /* silent */ }
	}

	async function saveMoodLog() {
		saving = true;
		try {
			await apiClient.post('/api/v1/psychology/mood-logs', {
				log_date: new Date().toISOString(),
				pre_market_mood: form.pre_market_mood ? parseInt(form.pre_market_mood) : null,
				during_trading_mood: form.during_trading_mood ? parseInt(form.during_trading_mood) : null,
				post_market_mood: form.post_market_mood ? parseInt(form.post_market_mood) : null,
				sleep_quality: form.sleep_quality ? parseInt(form.sleep_quality) : null,
				sleep_hours: form.sleep_hours ? parseFloat(form.sleep_hours) : null,
				journal_entry: form.journal_entry || null,
				felt_fomo: form.felt_fomo,
				felt_revenge: form.felt_revenge,
				felt_overconfident: form.felt_overconfident,
				felt_fearful: form.felt_fearful,
				exercised: form.exercised,
				meditated: form.meditated,
			});
			toasts.success('Mood log saved');
			await Promise.all([loadLogs(), loadInsights()]);
		} catch (error) {
			toasts.error('Failed to save', error instanceof Error ? error.message : '');
		} finally {
			saving = false;
		}
	}

	function moodColor(score: number | null): string {
		if (!score) return 'text-muted-foreground';
		if (score >= 8) return 'text-green-600';
		if (score >= 6) return 'text-yellow-600';
		if (score >= 4) return 'text-orange-600';
		return 'text-red-600';
	}

	function moodEmoji(score: number | null): string {
		if (!score) return '-';
		if (score >= 8) return `${score}/10`;
		if (score >= 6) return `${score}/10`;
		if (score >= 4) return `${score}/10`;
		return `${score}/10`;
	}
</script>

<svelte:head>
	<title>Psychology - TradeMaster AI</title>
</svelte:head>

<div class="p-6 max-w-6xl mx-auto space-y-6">
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold">Trading Psychology</h1>
			<p class="text-muted-foreground mt-1">Track your emotional state and find patterns</p>
		</div>
		<div class="flex gap-2">
			<Button variant={activeTab === 'log' ? 'default' : 'outline'} onclick={() => activeTab = 'log'}>
				{#snippet children()}Log{/snippet}
			</Button>
			<Button variant={activeTab === 'history' ? 'default' : 'outline'} onclick={() => activeTab = 'history'}>
				{#snippet children()}History{/snippet}
			</Button>
			<Button variant={activeTab === 'insights' ? 'default' : 'outline'} onclick={() => activeTab = 'insights'}>
				{#snippet children()}Insights{/snippet}
			</Button>
		</div>
	</div>

	{#if activeTab === 'log'}
		<div class="grid gap-6 md:grid-cols-2">
			<Card>
				<CardHeader><CardTitle>Mood & Energy</CardTitle></CardHeader>
				<CardContent class="space-y-4">
					<div class="space-y-2">
						<Label for="pre-mood">Pre-Market Mood (1-10)</Label>
						<Input id="pre-mood" type="number" min="1" max="10" bind:value={form.pre_market_mood} />
					</div>
					<div class="space-y-2">
						<Label for="during-mood">During Trading Mood (1-10)</Label>
						<Input id="during-mood" type="number" min="1" max="10" bind:value={form.during_trading_mood} />
					</div>
					<div class="space-y-2">
						<Label for="post-mood">Post-Market Mood (1-10)</Label>
						<Input id="post-mood" type="number" min="1" max="10" bind:value={form.post_market_mood} />
					</div>
					<div class="space-y-2">
						<Label for="sleep-q">Sleep Quality (1-10)</Label>
						<Input id="sleep-q" type="number" min="1" max="10" bind:value={form.sleep_quality} />
					</div>
					<div class="space-y-2">
						<Label for="sleep-h">Sleep Hours</Label>
						<Input id="sleep-h" type="number" step="0.5" bind:value={form.sleep_hours} />
					</div>
				</CardContent>
			</Card>

			<Card>
				<CardHeader><CardTitle>Tilt Indicators & Habits</CardTitle></CardHeader>
				<CardContent class="space-y-4">
					<div class="space-y-3">
						<p class="text-sm font-medium">Did you experience any of these today?</p>
						<Checkbox bind:checked={form.felt_fomo} label="FOMO (Fear of Missing Out)" />
						<Checkbox bind:checked={form.felt_revenge} label="Revenge Trading Urge" />
						<Checkbox bind:checked={form.felt_overconfident} label="Overconfidence" />
						<Checkbox bind:checked={form.felt_fearful} label="Fear / Hesitation" />
					</div>
					<hr />
					<div class="space-y-3">
						<p class="text-sm font-medium">Positive habits</p>
						<Checkbox bind:checked={form.exercised} label="Exercised today" />
						<Checkbox bind:checked={form.meditated} label="Meditated today" />
					</div>
				</CardContent>
			</Card>
		</div>

		<Card>
			<CardHeader><CardTitle>Journal Entry</CardTitle></CardHeader>
			<CardContent>
				<Textarea
					bind:value={form.journal_entry}
					placeholder="How are you feeling about your trading today? What's on your mind?"
					rows={6}
				/>
			</CardContent>
		</Card>

		<Button onclick={saveMoodLog} disabled={saving}>
			{#snippet children()}
				{saving ? 'Saving...' : 'Save Mood Log'}
			{/snippet}
		</Button>

	{:else if activeTab === 'history'}
		{#if loading}
			<div class="flex justify-center py-12"><Spinner /></div>
		{:else if logs.length === 0}
			<p class="text-center text-muted-foreground py-12">No mood logs yet. Start tracking today.</p>
		{:else}
			<div class="space-y-3">
				{#each logs as log}
					<Card>
						<CardContent class="py-4">
							<div class="flex items-center justify-between">
								<span class="font-medium">{formatDate(log.log_date)}</span>
								<div class="flex gap-4 text-sm">
									<span>Pre: <span class={moodColor(log.pre_market_mood)}>{moodEmoji(log.pre_market_mood)}</span></span>
									<span>During: <span class={moodColor(log.during_trading_mood)}>{moodEmoji(log.during_trading_mood)}</span></span>
									<span>Post: <span class={moodColor(log.post_market_mood)}>{moodEmoji(log.post_market_mood)}</span></span>
								</div>
								<div class="flex gap-1">
									{#if log.felt_fomo}<Badge variant="destructive">{#snippet children()}FOMO{/snippet}</Badge>{/if}
									{#if log.felt_revenge}<Badge variant="destructive">{#snippet children()}Revenge{/snippet}</Badge>{/if}
									{#if log.felt_overconfident}<Badge variant="warning">{#snippet children()}Overconfident{/snippet}</Badge>{/if}
									{#if log.felt_fearful}<Badge variant="warning">{#snippet children()}Fearful{/snippet}</Badge>{/if}
								</div>
							</div>
							{#if log.journal_entry}
								<p class="text-sm text-muted-foreground mt-2 line-clamp-2">{log.journal_entry}</p>
							{/if}
						</CardContent>
					</Card>
				{/each}
			</div>
		{/if}

	{:else if activeTab === 'insights'}
		{#if !insights || insights.total_logs === 0}
			<p class="text-center text-muted-foreground py-12">
				Log at least a few days of mood data to see insights.
			</p>
		{:else}
			<div class="grid gap-6 md:grid-cols-2 lg:grid-cols-4">
				<Card>
					<CardContent class="pt-6 text-center">
						<div class="text-3xl font-bold">{insights.avg_pre_market_mood?.toFixed(1) ?? '-'}</div>
						<div class="text-sm text-muted-foreground mt-1">Avg Pre-Market Mood</div>
					</CardContent>
				</Card>
				<Card>
					<CardContent class="pt-6 text-center">
						<div class="text-3xl font-bold">{insights.avg_sleep_quality?.toFixed(1) ?? '-'}</div>
						<div class="text-sm text-muted-foreground mt-1">Avg Sleep Quality</div>
					</CardContent>
				</Card>
				<Card>
					<CardContent class="pt-6 text-center">
						<div class="text-3xl font-bold">{insights.avg_sleep_hours?.toFixed(1) ?? '-'}h</div>
						<div class="text-sm text-muted-foreground mt-1">Avg Sleep Hours</div>
					</CardContent>
				</Card>
				<Card>
					<CardContent class="pt-6 text-center">
						<div class="text-3xl font-bold">{insights.total_logs}</div>
						<div class="text-sm text-muted-foreground mt-1">Total Logs</div>
					</CardContent>
				</Card>
			</div>

			<div class="grid gap-6 md:grid-cols-2">
				<Card>
					<CardHeader><CardTitle>Tilt Frequency</CardTitle></CardHeader>
					<CardContent class="space-y-3">
						<div class="flex justify-between text-sm">
							<span>FOMO episodes</span>
							<span class="font-semibold text-destructive">{insights.fomo_count}</span>
						</div>
						<div class="flex justify-between text-sm">
							<span>Revenge trading urges</span>
							<span class="font-semibold text-destructive">{insights.revenge_count}</span>
						</div>
						<div class="flex justify-between text-sm">
							<span>Overconfidence episodes</span>
							<span class="font-semibold text-orange-600">{insights.overconfident_count}</span>
						</div>
						<div class="flex justify-between text-sm">
							<span>Fear/hesitation episodes</span>
							<span class="font-semibold text-orange-600">{insights.fearful_count}</span>
						</div>
					</CardContent>
				</Card>

				<Card>
					<CardHeader><CardTitle>Mood vs Performance</CardTitle></CardHeader>
					<CardContent class="space-y-3">
						<div class="flex justify-between text-sm">
							<span>Avg P&L on high mood days</span>
							<span class="font-semibold {(insights.high_mood_avg_pnl ?? 0) >= 0 ? 'text-green-600' : 'text-destructive'}">
								{insights.high_mood_avg_pnl != null ? formatCurrency(insights.high_mood_avg_pnl) : 'N/A'}
							</span>
						</div>
						<div class="flex justify-between text-sm">
							<span>Avg P&L on low mood days</span>
							<span class="font-semibold {(insights.low_mood_avg_pnl ?? 0) >= 0 ? 'text-green-600' : 'text-destructive'}">
								{insights.low_mood_avg_pnl != null ? formatCurrency(insights.low_mood_avg_pnl) : 'N/A'}
							</span>
						</div>
						<hr />
						<div class="flex justify-between text-sm">
							<span>Avg P&L on good sleep days</span>
							<span class="font-semibold {(insights.good_sleep_avg_pnl ?? 0) >= 0 ? 'text-green-600' : 'text-destructive'}">
								{insights.good_sleep_avg_pnl != null ? formatCurrency(insights.good_sleep_avg_pnl) : 'N/A'}
							</span>
						</div>
						<div class="flex justify-between text-sm">
							<span>Avg P&L on poor sleep days</span>
							<span class="font-semibold {(insights.poor_sleep_avg_pnl ?? 0) >= 0 ? 'text-green-600' : 'text-destructive'}">
								{insights.poor_sleep_avg_pnl != null ? formatCurrency(insights.poor_sleep_avg_pnl) : 'N/A'}
							</span>
						</div>
					</CardContent>
				</Card>
			</div>
		{/if}
	{/if}
</div>
