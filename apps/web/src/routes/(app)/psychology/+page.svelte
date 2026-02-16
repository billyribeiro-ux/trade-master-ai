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
	import Spinner from '$lib/components/ui/spinner.svelte';
	import { toasts } from '$lib/stores/toast.svelte';
	import { formatCurrency, formatDate } from '$lib/utils/format';
	import { onMount } from 'svelte';

	interface MoodLog {
		id: string;
		log_date: string;
		pre_market_mood: number | null;
		post_market_mood: number | null;
		stress_level: number | null;
		confidence_level: number | null;
		sleep_quality: number | null;
		emotions: string[] | null;
		notes: string | null;
	}

	interface Insights {
		avg_pre_market_mood: number | null;
		avg_post_market_mood: number | null;
		avg_stress_level: number | null;
		avg_confidence_level: number | null;
		avg_sleep_quality: number | null;
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
		post_market_mood: '',
		stress_level: '5',
		confidence_level: '7',
		sleep_quality: '7',
		emotions: '',
		notes: '',
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
			const today = new Date().toISOString().split('T')[0];
			const emotionList = form.emotions
				.split(',')
				.map((e: string) => e.trim())
				.filter((e: string) => e.length > 0);

			await apiClient.post('/api/v1/psychology/mood-logs', {
				log_date: today,
				pre_market_mood: form.pre_market_mood ? parseInt(form.pre_market_mood) : null,
				post_market_mood: form.post_market_mood ? parseInt(form.post_market_mood) : null,
				stress_level: form.stress_level ? parseInt(form.stress_level) : null,
				confidence_level: form.confidence_level ? parseInt(form.confidence_level) : null,
				sleep_quality: form.sleep_quality ? parseInt(form.sleep_quality) : null,
				emotions: emotionList.length > 0 ? emotionList : null,
				notes: form.notes || null,
			});
			toasts.success('Mood log saved');
			await Promise.all([loadLogs(), loadInsights()]);
		} catch (error) {
			toasts.error('Failed to save', error instanceof Error ? error.message : '');
		} finally {
			saving = false;
		}
	}

	function scoreColor(score: number | null): string {
		if (!score) return 'text-muted-foreground';
		if (score >= 8) return 'text-green-600';
		if (score >= 6) return 'text-yellow-600';
		if (score >= 4) return 'text-orange-600';
		return 'text-red-600';
	}

	function scoreDisplay(score: number | null): string {
		return score != null ? `${score}/10` : '-';
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
				<CardHeader><CardTitle>Mood & State</CardTitle></CardHeader>
				<CardContent class="space-y-4">
					<div class="space-y-2">
						<Label for="pre-mood">Pre-Market Mood (1-10)</Label>
						<Input id="pre-mood" type="number" min="1" max="10" bind:value={form.pre_market_mood} />
					</div>
					<div class="space-y-2">
						<Label for="post-mood">Post-Market Mood (1-10)</Label>
						<Input id="post-mood" type="number" min="1" max="10" bind:value={form.post_market_mood} />
					</div>
					<div class="space-y-2">
						<Label for="stress">Stress Level (1-10)</Label>
						<Input id="stress" type="number" min="1" max="10" bind:value={form.stress_level} />
					</div>
					<div class="space-y-2">
						<Label for="confidence">Confidence Level (1-10)</Label>
						<Input id="confidence" type="number" min="1" max="10" bind:value={form.confidence_level} />
					</div>
					<div class="space-y-2">
						<Label for="sleep-q">Sleep Quality (1-10)</Label>
						<Input id="sleep-q" type="number" min="1" max="10" bind:value={form.sleep_quality} />
					</div>
				</CardContent>
			</Card>

			<Card>
				<CardHeader><CardTitle>Emotions & Notes</CardTitle></CardHeader>
				<CardContent class="space-y-4">
					<div class="space-y-2">
						<Label for="emotions">Emotions (comma-separated)</Label>
						<Input id="emotions" bind:value={form.emotions} placeholder="calm, focused, anxious..." />
						<p class="text-xs text-muted-foreground">e.g. calm, focused, anxious, excited, frustrated</p>
					</div>
					<div class="space-y-2">
						<Label for="notes">Notes</Label>
						<Textarea
							id="notes"
							bind:value={form.notes}
							placeholder="How are you feeling about your trading today?"
							rows={8}
						/>
					</div>
				</CardContent>
			</Card>
		</div>

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
							<div class="flex items-center justify-between flex-wrap gap-2">
								<span class="font-medium">{formatDate(log.log_date)}</span>
								<div class="flex gap-4 text-sm">
									<span>Pre: <span class={scoreColor(log.pre_market_mood)}>{scoreDisplay(log.pre_market_mood)}</span></span>
									<span>Post: <span class={scoreColor(log.post_market_mood)}>{scoreDisplay(log.post_market_mood)}</span></span>
									<span>Stress: <span class={scoreColor(log.stress_level)}>{scoreDisplay(log.stress_level)}</span></span>
									<span>Confidence: <span class={scoreColor(log.confidence_level)}>{scoreDisplay(log.confidence_level)}</span></span>
								</div>
								<div class="flex gap-1 flex-wrap">
									{#if log.emotions}
										{#each log.emotions as emotion}
											<Badge variant="secondary">{#snippet children()}{emotion}{/snippet}</Badge>
										{/each}
									{/if}
								</div>
							</div>
							{#if log.notes}
								<p class="text-sm text-muted-foreground mt-2 line-clamp-2">{log.notes}</p>
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
			<div class="grid gap-6 md:grid-cols-2 lg:grid-cols-5">
				<Card>
					<CardContent class="pt-6 text-center">
						<div class="text-3xl font-bold">{insights.avg_pre_market_mood?.toFixed(1) ?? '-'}</div>
						<div class="text-sm text-muted-foreground mt-1">Avg Pre-Market Mood</div>
					</CardContent>
				</Card>
				<Card>
					<CardContent class="pt-6 text-center">
						<div class="text-3xl font-bold">{insights.avg_post_market_mood?.toFixed(1) ?? '-'}</div>
						<div class="text-sm text-muted-foreground mt-1">Avg Post-Market Mood</div>
					</CardContent>
				</Card>
				<Card>
					<CardContent class="pt-6 text-center">
						<div class="text-3xl font-bold">{insights.avg_stress_level?.toFixed(1) ?? '-'}</div>
						<div class="text-sm text-muted-foreground mt-1">Avg Stress Level</div>
					</CardContent>
				</Card>
				<Card>
					<CardContent class="pt-6 text-center">
						<div class="text-3xl font-bold">{insights.avg_confidence_level?.toFixed(1) ?? '-'}</div>
						<div class="text-sm text-muted-foreground mt-1">Avg Confidence</div>
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
					<CardHeader><CardTitle>Mood vs Performance</CardTitle></CardHeader>
					<CardContent class="space-y-3">
						<div class="flex justify-between text-sm">
							<span>Avg P&L on high mood days (≥7)</span>
							<span class="font-semibold {(insights.high_mood_avg_pnl ?? 0) >= 0 ? 'text-green-600' : 'text-destructive'}">
								{insights.high_mood_avg_pnl != null ? formatCurrency(insights.high_mood_avg_pnl) : 'N/A'}
							</span>
						</div>
						<div class="flex justify-between text-sm">
							<span>Avg P&L on low mood days (&lt;7)</span>
							<span class="font-semibold {(insights.low_mood_avg_pnl ?? 0) >= 0 ? 'text-green-600' : 'text-destructive'}">
								{insights.low_mood_avg_pnl != null ? formatCurrency(insights.low_mood_avg_pnl) : 'N/A'}
							</span>
						</div>
					</CardContent>
				</Card>

				<Card>
					<CardHeader><CardTitle>Sleep vs Performance</CardTitle></CardHeader>
					<CardContent class="space-y-3">
						<div class="flex justify-between text-sm">
							<span>Avg P&L on good sleep days (≥7)</span>
							<span class="font-semibold {(insights.good_sleep_avg_pnl ?? 0) >= 0 ? 'text-green-600' : 'text-destructive'}">
								{insights.good_sleep_avg_pnl != null ? formatCurrency(insights.good_sleep_avg_pnl) : 'N/A'}
							</span>
						</div>
						<div class="flex justify-between text-sm">
							<span>Avg P&L on poor sleep days (&lt;7)</span>
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
