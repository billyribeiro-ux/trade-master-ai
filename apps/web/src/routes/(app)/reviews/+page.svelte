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

	interface Review {
		id: string;
		review_type: string;
		period_start: string;
		period_end: string;
		total_trades: number | null;
		winning_trades: number | null;
		win_rate: number | null;
		total_pnl: number | null;
		what_went_well: string | null;
		what_to_improve: string | null;
		key_lessons: string[] | null;
		goals_next_period: string[] | null;
		discipline_rating: number | null;
		patience_rating: number | null;
		execution_rating: number | null;
		overall_rating: number | null;
		created_at: string;
	}

	let reviews = $state<Review[]>([]);
	let loading = $state(true);
	let saving = $state(false);
	let showForm = $state(false);

	let form = $state({
		review_type: 'weekly',
		period_start: '',
		period_end: '',
		what_went_well: '',
		what_to_improve: '',
		key_lessons: '',
		goals_next_period: '',
		discipline_rating: '7',
		patience_rating: '7',
		execution_rating: '7',
		overall_rating: '7',
	});

	onMount(async () => {
		await loadReviews();

		// Default period to last 7 days
		const end = new Date();
		const start = new Date();
		start.setDate(start.getDate() - 7);
		form.period_start = start.toISOString().split('T')[0] ?? '';
		form.period_end = end.toISOString().split('T')[0] ?? '';
	});

	async function loadReviews() {
		loading = true;
		try {
			reviews = await apiClient.get<Review[]>('/api/v1/reviews') ?? [];
		} catch { /* silent */ }
		loading = false;
	}

	async function createReview() {
		if (!form.period_start || !form.period_end) {
			toasts.error('Period start and end dates are required');
			return;
		}

		saving = true;
		try {
			await apiClient.post('/api/v1/reviews', {
				review_type: form.review_type,
				period_start: new Date(form.period_start).toISOString(),
				period_end: new Date(form.period_end).toISOString(),
				what_went_well: form.what_went_well || null,
				what_to_improve: form.what_to_improve || null,
				key_lessons: form.key_lessons ? form.key_lessons.split('\n').filter(Boolean) : null,
				goals_next_period: form.goals_next_period ? form.goals_next_period.split('\n').filter(Boolean) : null,
				discipline_rating: form.discipline_rating ? parseInt(form.discipline_rating) : null,
				patience_rating: form.patience_rating ? parseInt(form.patience_rating) : null,
				execution_rating: form.execution_rating ? parseInt(form.execution_rating) : null,
				overall_rating: form.overall_rating ? parseInt(form.overall_rating) : null,
			});
			toasts.success('Review created');
			showForm = false;
			await loadReviews();
		} catch (error) {
			toasts.error('Failed to create review', error instanceof Error ? error.message : '');
		} finally {
			saving = false;
		}
	}

	function ratingColor(rating: number | null): string {
		if (!rating) return '';
		if (rating >= 8) return 'text-green-600';
		if (rating >= 6) return 'text-yellow-600';
		return 'text-red-600';
	}
</script>

<svelte:head>
	<title>Reviews - TradeMaster AI</title>
</svelte:head>

<div class="p-6 max-w-6xl mx-auto space-y-6">
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold">Periodic Reviews</h1>
			<p class="text-muted-foreground mt-1">Weekly and monthly performance reviews</p>
		</div>
		<Button onclick={() => showForm = !showForm}>
			{#snippet children()}{showForm ? 'Cancel' : '+ New Review'}{/snippet}
		</Button>
	</div>

	{#if showForm}
		<Card>
			<CardHeader><CardTitle>Create Review</CardTitle></CardHeader>
			<CardContent class="space-y-4">
				<div class="grid gap-4 md:grid-cols-3">
					<div class="space-y-2">
						<Label for="review-type">Review Type</Label>
						<select
							id="review-type"
							bind:value={form.review_type}
							class="w-full rounded-md border px-3 py-2 text-sm"
						>
							<option value="weekly">Weekly</option>
							<option value="monthly">Monthly</option>
							<option value="quarterly">Quarterly</option>
						</select>
					</div>
					<div class="space-y-2">
						<Label for="period-start">Period Start</Label>
						<Input id="period-start" type="date" bind:value={form.period_start} />
					</div>
					<div class="space-y-2">
						<Label for="period-end">Period End</Label>
						<Input id="period-end" type="date" bind:value={form.period_end} />
					</div>
				</div>

				<div class="grid gap-4 md:grid-cols-2">
					<div class="space-y-2">
						<Label for="went-well">What Went Well</Label>
						<Textarea id="went-well" bind:value={form.what_went_well} rows={4} placeholder="What did you do well this period?" />
					</div>
					<div class="space-y-2">
						<Label for="improve">What to Improve</Label>
						<Textarea id="improve" bind:value={form.what_to_improve} rows={4} placeholder="What needs improvement?" />
					</div>
				</div>

				<div class="grid gap-4 md:grid-cols-2">
					<div class="space-y-2">
						<Label for="lessons">Key Lessons (one per line)</Label>
						<Textarea id="lessons" bind:value={form.key_lessons} rows={3} placeholder="Patience pays off&#10;Stick to the plan" />
					</div>
					<div class="space-y-2">
						<Label for="goals">Goals for Next Period (one per line)</Label>
						<Textarea id="goals" bind:value={form.goals_next_period} rows={3} placeholder="Max 3 trades per day&#10;Always use stop loss" />
					</div>
				</div>

				<div class="grid gap-4 md:grid-cols-4">
					<div class="space-y-2">
						<Label for="r-disc">Discipline (1-10)</Label>
						<Input id="r-disc" type="number" min="1" max="10" bind:value={form.discipline_rating} />
					</div>
					<div class="space-y-2">
						<Label for="r-pat">Patience (1-10)</Label>
						<Input id="r-pat" type="number" min="1" max="10" bind:value={form.patience_rating} />
					</div>
					<div class="space-y-2">
						<Label for="r-exec">Execution (1-10)</Label>
						<Input id="r-exec" type="number" min="1" max="10" bind:value={form.execution_rating} />
					</div>
					<div class="space-y-2">
						<Label for="r-overall">Overall (1-10)</Label>
						<Input id="r-overall" type="number" min="1" max="10" bind:value={form.overall_rating} />
					</div>
				</div>

				<Button onclick={createReview} disabled={saving}>
					{#snippet children()}{saving ? 'Creating...' : 'Create Review'}{/snippet}
				</Button>
			</CardContent>
		</Card>
	{/if}

	{#if loading}
		<div class="flex justify-center py-12"><Spinner /></div>
	{:else if reviews.length === 0}
		<div class="text-center py-16">
			<h3 class="text-lg font-medium">No reviews yet</h3>
			<p class="text-muted-foreground mt-2">Create your first weekly or monthly review to track your progress over time.</p>
		</div>
	{:else}
		<div class="space-y-4">
			{#each reviews as review}
				<Card>
					<CardContent class="py-5">
						<div class="flex items-start justify-between">
							<div>
								<div class="flex items-center gap-3">
									<h3 class="font-semibold text-lg capitalize">{review.review_type} Review</h3>
									<Badge variant="outline">
										{#snippet children()}
											{formatDate(review.period_start)} — {formatDate(review.period_end)}
										{/snippet}
									</Badge>
								</div>

								{#if review.total_trades != null}
									<div class="flex gap-6 mt-3 text-sm">
										<span>Trades: <strong>{review.total_trades}</strong></span>
										<span>Win Rate: <strong>{review.win_rate?.toFixed(1) ?? '-'}%</strong></span>
										<span>
											P&L: <strong class="{(review.total_pnl ?? 0) >= 0 ? 'text-green-600' : 'text-destructive'}">
												{review.total_pnl != null ? formatCurrency(review.total_pnl) : '-'}
											</strong>
										</span>
									</div>
								{/if}
							</div>

							<div class="flex gap-3 text-center">
								{#if review.discipline_rating}
									<div>
										<div class="text-lg font-bold {ratingColor(review.discipline_rating)}">{review.discipline_rating}</div>
										<div class="text-xs text-muted-foreground">Discipline</div>
									</div>
								{/if}
								{#if review.patience_rating}
									<div>
										<div class="text-lg font-bold {ratingColor(review.patience_rating)}">{review.patience_rating}</div>
										<div class="text-xs text-muted-foreground">Patience</div>
									</div>
								{/if}
								{#if review.execution_rating}
									<div>
										<div class="text-lg font-bold {ratingColor(review.execution_rating)}">{review.execution_rating}</div>
										<div class="text-xs text-muted-foreground">Execution</div>
									</div>
								{/if}
								{#if review.overall_rating}
									<div>
										<div class="text-lg font-bold {ratingColor(review.overall_rating)}">{review.overall_rating}</div>
										<div class="text-xs text-muted-foreground">Overall</div>
									</div>
								{/if}
							</div>
						</div>

						{#if review.what_went_well || review.what_to_improve}
							<div class="grid gap-4 md:grid-cols-2 mt-4">
								{#if review.what_went_well}
									<div class="p-3 bg-green-50 dark:bg-green-950/20 rounded-lg">
										<p class="text-xs font-semibold uppercase text-green-700 dark:text-green-400 mb-1">What Went Well</p>
										<p class="text-sm">{review.what_went_well}</p>
									</div>
								{/if}
								{#if review.what_to_improve}
									<div class="p-3 bg-orange-50 dark:bg-orange-950/20 rounded-lg">
										<p class="text-xs font-semibold uppercase text-orange-700 dark:text-orange-400 mb-1">To Improve</p>
										<p class="text-sm">{review.what_to_improve}</p>
									</div>
								{/if}
							</div>
						{/if}

						{#if review.key_lessons && review.key_lessons.length > 0}
							<div class="mt-3">
								<p class="text-xs font-semibold uppercase text-muted-foreground mb-1">Key Lessons</p>
								<div class="flex flex-wrap gap-2">
									{#each review.key_lessons as lesson}
										<Badge variant="outline">{#snippet children()}{lesson}{/snippet}</Badge>
									{/each}
								</div>
							</div>
						{/if}
					</CardContent>
				</Card>
			{/each}
		</div>
	{/if}
</div>
