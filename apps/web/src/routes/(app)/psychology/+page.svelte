<script lang="ts">
	import { apiClient } from '$lib/api';
	import Button from '$lib/components/ui/button.svelte';
	import Label from '$lib/components/ui/label.svelte';
	import Textarea from '$lib/components/ui/textarea.svelte';
	import Card from '$lib/components/ui/card.svelte';
	import CardHeader from '$lib/components/ui/card-header.svelte';
	import CardTitle from '$lib/components/ui/card-title.svelte';
	import CardContent from '$lib/components/ui/card-content.svelte';
	import Badge from '$lib/components/ui/badge.svelte';
	import Spinner from '$lib/components/ui/spinner.svelte';
	import MoodTrendChart from '$lib/components/psychology/mood-trend-chart.svelte';
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
		pre_market_mood: 7,
		post_market_mood: 5,
		stress_level: 5,
		confidence_level: 7,
		sleep_quality: 7,
		emotions: [] as string[],
		notes: '',
	});

	const emotionOptions = [
		{ label: 'Calm', emoji: '😌' },
		{ label: 'Focused', emoji: '🎯' },
		{ label: 'Confident', emoji: '💪' },
		{ label: 'Excited', emoji: '🔥' },
		{ label: 'Anxious', emoji: '😰' },
		{ label: 'Fearful', emoji: '😨' },
		{ label: 'Frustrated', emoji: '😤' },
		{ label: 'FOMO', emoji: '😬' },
		{ label: 'Revenge', emoji: '😡' },
		{ label: 'Overconfident', emoji: '🤑' },
		{ label: 'Impatient', emoji: '⏰' },
		{ label: 'Disciplined', emoji: '🧘' },
	];

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

	function toggleEmotion(label: string) {
		if (form.emotions.includes(label)) {
			form.emotions = form.emotions.filter(e => e !== label);
		} else {
			form.emotions = [...form.emotions, label];
		}
	}

	async function saveMoodLog() {
		saving = true;
		try {
			const today = new Date().toISOString().split('T')[0];

			await apiClient.post('/api/v1/psychology/mood-logs', {
				log_date: today,
				pre_market_mood: form.pre_market_mood,
				post_market_mood: form.post_market_mood,
				stress_level: form.stress_level,
				confidence_level: form.confidence_level,
				sleep_quality: form.sleep_quality,
				emotions: form.emotions.length > 0 ? form.emotions : null,
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

	function sliderColor(value: number, invert = false): string {
		const v = invert ? 11 - value : value;
		if (v >= 8) return '#22c55e';
		if (v >= 6) return '#f59e0b';
		if (v >= 4) return '#f97316';
		return '#ef4444';
	}

	function scoreColor(score: number | null): string {
		if (score == null) return 'text-muted-foreground';
		if (score >= 8) return 'text-green-500';
		if (score >= 6) return 'text-yellow-500';
		if (score >= 4) return 'text-orange-500';
		return 'text-red-500';
	}

	function moodEmoji(score: number | null): string {
		if (score == null) return '—';
		if (score >= 9) return '🤩';
		if (score >= 7) return '😊';
		if (score >= 5) return '😐';
		if (score >= 3) return '😟';
		return '😫';
	}

	function calcStreak(): number {
		let streak = 0;
		const today = new Date();
		for (let i = 0; i < logs.length; i++) {
			const logDate = new Date(logs[i]?.log_date ?? '');
			const expected = new Date(today);
			expected.setDate(expected.getDate() - i);
			if (logDate.toDateString() === expected.toDateString()) {
				streak++;
			} else {
				break;
			}
		}
		return streak;
	}

	const currentStreak = $derived(calcStreak());
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

	<!-- Streak Banner -->
	{#if currentStreak > 0}
		<div class="bg-gradient-to-r from-blue-950/40 to-purple-950/40 border border-blue-800/30 rounded-lg p-4 flex items-center justify-between">
			<div>
				<span class="text-sm font-medium">Logging Streak</span>
				<p class="text-2xl font-bold">{currentStreak} day{currentStreak > 1 ? 's' : ''} 🔥</p>
			</div>
			<p class="text-sm text-muted-foreground">Keep it up! Consistency is key to self-awareness.</p>
		</div>
	{/if}

	{#if activeTab === 'log'}
		<div class="grid gap-6 md:grid-cols-2">
			<Card>
				<CardHeader><CardTitle>Mood & State</CardTitle></CardHeader>
				<CardContent class="space-y-6">
					<!-- Pre-Market Mood Slider -->
					<div class="space-y-2">
						<div class="flex justify-between items-center">
							<Label>Pre-Market Mood</Label>
							<span class="text-lg font-bold" style="color: {sliderColor(form.pre_market_mood)}">
								{moodEmoji(form.pre_market_mood)} {form.pre_market_mood}
							</span>
						</div>
						<input type="range" min="1" max="10" bind:value={form.pre_market_mood}
							class="w-full h-2 rounded-full appearance-none cursor-pointer"
							style="accent-color: {sliderColor(form.pre_market_mood)}" />
						<div class="flex justify-between text-[10px] text-muted-foreground">
							<span>Terrible</span><span>Great</span>
						</div>
					</div>

					<!-- Post-Market Mood Slider -->
					<div class="space-y-2">
						<div class="flex justify-between items-center">
							<Label>Post-Market Mood</Label>
							<span class="text-lg font-bold" style="color: {sliderColor(form.post_market_mood)}">
								{moodEmoji(form.post_market_mood)} {form.post_market_mood}
							</span>
						</div>
						<input type="range" min="1" max="10" bind:value={form.post_market_mood}
							class="w-full h-2 rounded-full appearance-none cursor-pointer"
							style="accent-color: {sliderColor(form.post_market_mood)}" />
						<div class="flex justify-between text-[10px] text-muted-foreground">
							<span>Terrible</span><span>Great</span>
						</div>
					</div>

					<!-- Stress Level Slider (inverted: lower is better) -->
					<div class="space-y-2">
						<div class="flex justify-between items-center">
							<Label>Stress Level</Label>
							<span class="text-lg font-bold" style="color: {sliderColor(form.stress_level, true)}">
								{form.stress_level}
							</span>
						</div>
						<input type="range" min="1" max="10" bind:value={form.stress_level}
							class="w-full h-2 rounded-full appearance-none cursor-pointer"
							style="accent-color: {sliderColor(form.stress_level, true)}" />
						<div class="flex justify-between text-[10px] text-muted-foreground">
							<span>Relaxed</span><span>Very Stressed</span>
						</div>
					</div>

					<!-- Confidence Slider -->
					<div class="space-y-2">
						<div class="flex justify-between items-center">
							<Label>Confidence Level</Label>
							<span class="text-lg font-bold" style="color: {sliderColor(form.confidence_level)}">
								{form.confidence_level}
							</span>
						</div>
						<input type="range" min="1" max="10" bind:value={form.confidence_level}
							class="w-full h-2 rounded-full appearance-none cursor-pointer"
							style="accent-color: {sliderColor(form.confidence_level)}" />
						<div class="flex justify-between text-[10px] text-muted-foreground">
							<span>No Confidence</span><span>Very Confident</span>
						</div>
					</div>

					<!-- Sleep Quality Slider -->
					<div class="space-y-2">
						<div class="flex justify-between items-center">
							<Label>Sleep Quality</Label>
							<span class="text-lg font-bold" style="color: {sliderColor(form.sleep_quality)}">
								{form.sleep_quality}
							</span>
						</div>
						<input type="range" min="1" max="10" bind:value={form.sleep_quality}
							class="w-full h-2 rounded-full appearance-none cursor-pointer"
							style="accent-color: {sliderColor(form.sleep_quality)}" />
						<div class="flex justify-between text-[10px] text-muted-foreground">
							<span>Terrible</span><span>Excellent</span>
						</div>
					</div>
				</CardContent>
			</Card>

			<div class="space-y-6">
				<Card>
					<CardHeader><CardTitle>How are you feeling?</CardTitle></CardHeader>
					<CardContent>
						<div class="flex flex-wrap gap-2">
							{#each emotionOptions as opt}
								<button
									class="px-3 py-1.5 rounded-full border text-sm transition-all {form.emotions.includes(opt.label)
										? 'bg-primary text-primary-foreground border-primary'
										: 'border-border hover:bg-muted'}"
									onclick={() => toggleEmotion(opt.label)}
								>
									{opt.emoji} {opt.label}
								</button>
							{/each}
						</div>
						{#if form.emotions.length > 0}
							<p class="text-xs text-muted-foreground mt-3">
								Selected: {form.emotions.join(', ')}
							</p>
						{/if}
					</CardContent>
				</Card>

				<Card>
					<CardHeader><CardTitle>Journal Notes</CardTitle></CardHeader>
					<CardContent>
						<Textarea
							bind:value={form.notes}
							placeholder="How are you feeling about your trading today? Any triggers, observations, or reflections..."
							rows={8}
						/>
					</CardContent>
				</Card>

				<Button onclick={saveMoodLog} disabled={saving}>
					{#snippet children()}
						{saving ? 'Saving...' : 'Save Mood Log'}
					{/snippet}
				</Button>
			</div>
		</div>

	{:else if activeTab === 'history'}
		{#if loading}
			<div class="flex justify-center py-12"><Spinner /></div>
		{:else if logs.length === 0}
			<p class="text-center text-muted-foreground py-12">No mood logs yet. Start tracking today.</p>
		{:else}
			<!-- Mood Trend Chart -->
			{#if logs.length >= 3}
				<Card>
					<CardHeader><CardTitle>Mood Trends</CardTitle></CardHeader>
					<CardContent>
						<MoodTrendChart {logs} />
					</CardContent>
				</Card>
			{/if}

			<div class="space-y-3">
				{#each logs as log}
					<Card>
						<CardContent class="py-4">
							<div class="flex items-start justify-between gap-4">
								<div class="flex items-center gap-3">
									<div class="text-center">
										<div class="text-2xl">{moodEmoji(log.pre_market_mood)}</div>
										<div class="text-[10px] text-muted-foreground">Pre</div>
									</div>
									<div class="text-muted-foreground">→</div>
									<div class="text-center">
										<div class="text-2xl">{moodEmoji(log.post_market_mood)}</div>
										<div class="text-[10px] text-muted-foreground">Post</div>
									</div>
									<div class="ml-2">
										<span class="font-medium">{formatDate(log.log_date)}</span>
									</div>
								</div>
								<div class="flex gap-4 text-sm shrink-0">
									<div class="text-center">
										<div class="font-bold {scoreColor(log.stress_level)}">{log.stress_level ?? '-'}</div>
										<div class="text-[10px] text-muted-foreground">Stress</div>
									</div>
									<div class="text-center">
										<div class="font-bold {scoreColor(log.confidence_level)}">{log.confidence_level ?? '-'}</div>
										<div class="text-[10px] text-muted-foreground">Confidence</div>
									</div>
									<div class="text-center">
										<div class="font-bold {scoreColor(log.sleep_quality)}">{log.sleep_quality ?? '-'}</div>
										<div class="text-[10px] text-muted-foreground">Sleep</div>
									</div>
								</div>
							</div>
							{#if log.emotions && log.emotions.length > 0}
								<div class="flex gap-1 flex-wrap mt-2">
									{#each log.emotions as emotion}
										<Badge variant="secondary">{#snippet children()}{emotion}{/snippet}</Badge>
									{/each}
								</div>
							{/if}
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
			<div class="grid gap-4 md:grid-cols-2 lg:grid-cols-5">
				<Card>
					<CardContent class="pt-6 text-center">
						<div class="text-3xl font-bold {scoreColor(insights.avg_pre_market_mood ? Math.round(insights.avg_pre_market_mood) : null)}">
							{insights.avg_pre_market_mood?.toFixed(1) ?? '-'}
						</div>
						<div class="text-sm text-muted-foreground mt-1">Avg Pre-Market</div>
					</CardContent>
				</Card>
				<Card>
					<CardContent class="pt-6 text-center">
						<div class="text-3xl font-bold {scoreColor(insights.avg_post_market_mood ? Math.round(insights.avg_post_market_mood) : null)}">
							{insights.avg_post_market_mood?.toFixed(1) ?? '-'}
						</div>
						<div class="text-sm text-muted-foreground mt-1">Avg Post-Market</div>
					</CardContent>
				</Card>
				<Card>
					<CardContent class="pt-6 text-center">
						<div class="text-3xl font-bold {scoreColor(insights.avg_stress_level ? Math.round(11 - insights.avg_stress_level) : null)}">
							{insights.avg_stress_level?.toFixed(1) ?? '-'}
						</div>
						<div class="text-sm text-muted-foreground mt-1">Avg Stress</div>
					</CardContent>
				</Card>
				<Card>
					<CardContent class="pt-6 text-center">
						<div class="text-3xl font-bold {scoreColor(insights.avg_confidence_level ? Math.round(insights.avg_confidence_level) : null)}">
							{insights.avg_confidence_level?.toFixed(1) ?? '-'}
						</div>
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

			<!-- Mood Trend Chart in Insights -->
			{#if logs.length >= 3}
				<Card>
					<CardHeader><CardTitle>Mood Over Time</CardTitle></CardHeader>
					<CardContent>
						<MoodTrendChart {logs} />
					</CardContent>
				</Card>
			{/if}

			<div class="grid gap-6 md:grid-cols-2">
				<Card>
					<CardHeader><CardTitle>Mood vs Performance</CardTitle></CardHeader>
					<CardContent class="space-y-4">
						<div class="flex justify-between items-center">
							<span class="text-sm">High mood days (≥7)</span>
							<span class="font-bold text-lg {(insights.high_mood_avg_pnl ?? 0) >= 0 ? 'text-green-500' : 'text-red-500'}">
								{insights.high_mood_avg_pnl != null ? formatCurrency(insights.high_mood_avg_pnl) : 'N/A'}
							</span>
						</div>
						<div class="flex justify-between items-center">
							<span class="text-sm">Low mood days (&lt;7)</span>
							<span class="font-bold text-lg {(insights.low_mood_avg_pnl ?? 0) >= 0 ? 'text-green-500' : 'text-red-500'}">
								{insights.low_mood_avg_pnl != null ? formatCurrency(insights.low_mood_avg_pnl) : 'N/A'}
							</span>
						</div>
					</CardContent>
				</Card>

				<Card>
					<CardHeader><CardTitle>Sleep vs Performance</CardTitle></CardHeader>
					<CardContent class="space-y-4">
						<div class="flex justify-between items-center">
							<span class="text-sm">Good sleep days (≥7)</span>
							<span class="font-bold text-lg {(insights.good_sleep_avg_pnl ?? 0) >= 0 ? 'text-green-500' : 'text-red-500'}">
								{insights.good_sleep_avg_pnl != null ? formatCurrency(insights.good_sleep_avg_pnl) : 'N/A'}
							</span>
						</div>
						<div class="flex justify-between items-center">
							<span class="text-sm">Poor sleep days (&lt;7)</span>
							<span class="font-bold text-lg {(insights.poor_sleep_avg_pnl ?? 0) >= 0 ? 'text-green-500' : 'text-red-500'}">
								{insights.poor_sleep_avg_pnl != null ? formatCurrency(insights.poor_sleep_avg_pnl) : 'N/A'}
							</span>
						</div>
					</CardContent>
				</Card>
			</div>
		{/if}
	{/if}
</div>
