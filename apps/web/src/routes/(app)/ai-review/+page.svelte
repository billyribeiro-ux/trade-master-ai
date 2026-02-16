<script lang="ts">
	import { apiClient } from '$lib/api';
	import Button from '$lib/components/ui/button.svelte';
	import Input from '$lib/components/ui/input.svelte';
	import Textarea from '$lib/components/ui/textarea.svelte';
	import Card from '$lib/components/ui/card.svelte';
	import CardHeader from '$lib/components/ui/card-header.svelte';
	import CardTitle from '$lib/components/ui/card-title.svelte';
	import CardContent from '$lib/components/ui/card-content.svelte';
	import Badge from '$lib/components/ui/badge.svelte';
	import Spinner from '$lib/components/ui/spinner.svelte';
	import ScoreGauge from '$lib/components/ai/score-gauge.svelte';
	import { toasts } from '$lib/stores/toast.svelte';
	import { onMount } from 'svelte';

	interface AiReview {
		id: string;
		trade_id: string | null;
		review_type: string | null;
		overall_score: number | null;
		execution_quality_score: number | null;
		risk_management_score: number | null;
		plan_adherence_score: number | null;
		thesis_alignment_score: number | null;
		strengths: string[] | null;
		weaknesses: string[] | null;
		key_lesson: string | null;
		actionable_fixes: string[] | null;
		alternative_scenario: string | null;
		emotional_state_detected: string | null;
		raw_response: string | null;
		tokens_used: number | null;
		model_used: string | null;
		created_at: string;
	}

	interface ChatMessage {
		id: string;
		role: string;
		content: string;
		created_at: string;
	}

	let reviews = $state<AiReview[]>([]);
	let activeReview = $state<{ review: AiReview; messages: ChatMessage[] } | null>(null);
	let loading = $state(true);
	let sending = $state(false);
	let prompt = $state('');
	let chatMessage = $state('');
	let tradeId = $state('');
	let messagesContainer: HTMLDivElement | undefined = $state();
	let thinkingStage = $state(0);
	let thinkingInterval: ReturnType<typeof setInterval> | null = null;

	const thinkingStages = [
		'Reading trade data...',
		'Analyzing execution...',
		'Evaluating risk management...',
		'Generating insights...',
	];

	onMount(async () => {
		await loadReviews();
	});

	async function loadReviews() {
		loading = true;
		try {
			reviews = await apiClient.get<AiReview[]>('/api/v1/ai/reviews') ?? [];
		} catch {
			toasts.error('Failed to load reviews');
		} finally {
			loading = false;
		}
	}

	function startThinking() {
		thinkingStage = 0;
		thinkingInterval = setInterval(() => {
			thinkingStage = (thinkingStage + 1) % thinkingStages.length;
		}, 3000);
	}

	function stopThinking() {
		if (thinkingInterval) {
			clearInterval(thinkingInterval);
			thinkingInterval = null;
		}
	}

	async function startNewReview() {
		if (!prompt.trim()) {
			toasts.error('Please enter a prompt');
			return;
		}

		sending = true;
		startThinking();
		try {
			const body: Record<string, unknown> = {
				review_type: tradeId ? 'trade_analysis' : 'general',
				prompt: prompt.trim(),
			};
			if (tradeId.trim()) {
				body.trade_id = tradeId.trim();
			}

			const result = await apiClient.post<{ review: AiReview; messages: ChatMessage[] }>(
				'/api/v1/ai/reviews',
				body
			);

			if (result) {
				activeReview = result;
				prompt = '';
				tradeId = '';
				await loadReviews();
				scrollToBottom();
			}
		} catch (error) {
			toasts.error('Failed to start review', error instanceof Error ? error.message : 'Please try again');
		} finally {
			sending = false;
			stopThinking();
		}
	}

	async function sendChatMessage() {
		if (!chatMessage.trim() || !activeReview) return;

		sending = true;
		const msg = chatMessage.trim();
		chatMessage = '';

		try {
			const result = await apiClient.post<{ review: AiReview; messages: ChatMessage[] }>(
				`/api/v1/ai/reviews/${activeReview.review.id}/chat`,
				{ message: msg }
			);

			if (result) {
				activeReview = result;
				scrollToBottom();
			}
		} catch {
			toasts.error('Failed to send message');
			chatMessage = msg;
		} finally {
			sending = false;
		}
	}

	async function openReview(reviewId: string) {
		try {
			activeReview = await apiClient.get<{ review: AiReview; messages: ChatMessage[] }>(
				`/api/v1/ai/reviews/${reviewId}`
			) ?? null;
			scrollToBottom();
		} catch {
			toasts.error('Failed to load review');
		}
	}

	async function deleteReview(reviewId: string) {
		try {
			await apiClient.delete(`/api/v1/ai/reviews/${reviewId}`);
			if (activeReview?.review.id === reviewId) activeReview = null;
			await loadReviews();
			toasts.success('Review deleted');
		} catch {
			toasts.error('Failed to delete review');
		}
	}

	function scrollToBottom() {
		setTimeout(() => {
			if (messagesContainer) {
				messagesContainer.scrollTop = messagesContainer.scrollHeight;
			}
		}, 50);
	}

	function formatDate(dateStr: string): string {
		return new Date(dateStr).toLocaleDateString('en-US', {
			month: 'short',
			day: 'numeric',
			hour: '2-digit',
			minute: '2-digit',
		});
	}

	function handleChatKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
			sendChatMessage();
		}
	}

	const hasScores = $derived(
		activeReview?.review?.overall_score != null
	);

	const suggestedQuestions = [
		'What could I have done differently?',
		'How can I improve my risk management?',
		'Was my entry timing optimal?',
		'What patterns should I watch for?',
	];
</script>

<svelte:head>
	<title>AI Trade Review - TradeMaster AI</title>
</svelte:head>

<div class="flex h-[calc(100vh-4rem)]">
	<!-- Sidebar: Review History -->
	<div class="w-80 border-r bg-muted/30 flex flex-col shrink-0">
		<div class="p-4 border-b">
			<h2 class="font-semibold text-lg">AI Reviews</h2>
			<p class="text-sm text-muted-foreground mt-1">Chat with your AI trading coach</p>
		</div>

		<button
			class="m-3 px-4 py-2.5 bg-primary text-primary-foreground rounded-lg text-sm font-medium hover:bg-primary/90 transition-colors"
			onclick={() => { activeReview = null; }}
		>
			+ New Review
		</button>

		<div class="flex-1 overflow-y-auto">
			{#if loading}
				<div class="flex justify-center py-6"><Spinner /></div>
			{:else}
				{#each reviews as review}
					<button
						class="w-full text-left p-3 border-b hover:bg-muted/50 transition-colors {activeReview?.review.id === review.id ? 'bg-muted' : ''}"
						onclick={() => openReview(review.id)}
					>
						<div class="flex items-center justify-between">
							<Badge variant={review.review_type === 'trade_analysis' ? 'default' : 'secondary'}>
								{#snippet children()}{review.review_type ?? 'general'}{/snippet}
							</Badge>
							{#if review.overall_score != null}
								<span class="text-xs font-bold {review.overall_score >= 7 ? 'text-green-500' : review.overall_score >= 5 ? 'text-yellow-500' : 'text-red-500'}">
									{review.overall_score}/10
								</span>
							{/if}
						</div>
						<div class="text-xs text-muted-foreground mt-1.5">
							{formatDate(review.created_at)}
						</div>
					</button>
				{/each}

				{#if reviews.length === 0}
					<p class="text-center text-muted-foreground text-sm p-6">
						No reviews yet. Start a conversation with your AI coach.
					</p>
				{/if}
			{/if}
		</div>
	</div>

	<!-- Main Content Area -->
	<div class="flex-1 flex flex-col min-w-0">
		{#if activeReview}
			<!-- Score Header (if scores exist) -->
			{#if hasScores}
				<div class="border-b p-4 bg-muted/20">
					<div class="flex items-center gap-6 justify-center flex-wrap">
						{#if activeReview.review.overall_score != null}
							<ScoreGauge score={activeReview.review.overall_score} label="Overall" size={80} />
						{/if}
						{#if activeReview.review.execution_quality_score != null}
							<ScoreGauge score={activeReview.review.execution_quality_score} label="Execution" size={64} />
						{/if}
						{#if activeReview.review.risk_management_score != null}
							<ScoreGauge score={activeReview.review.risk_management_score} label="Risk Mgmt" size={64} />
						{/if}
						{#if activeReview.review.plan_adherence_score != null}
							<ScoreGauge score={activeReview.review.plan_adherence_score} label="Plan" size={64} />
						{/if}
					</div>

					<!-- Strengths / Weaknesses / Key Lesson -->
					<div class="grid gap-3 md:grid-cols-3 mt-4">
						{#if activeReview.review.strengths && activeReview.review.strengths.length > 0}
							<div class="p-3 bg-green-950/20 rounded-lg">
								<p class="text-xs font-semibold uppercase text-green-400 mb-1.5">Strengths</p>
								<ul class="text-xs space-y-1">
									{#each activeReview.review.strengths as s}
										<li class="flex gap-1.5"><span class="text-green-400">+</span> {s}</li>
									{/each}
								</ul>
							</div>
						{/if}
						{#if activeReview.review.weaknesses && activeReview.review.weaknesses.length > 0}
							<div class="p-3 bg-red-950/20 rounded-lg">
								<p class="text-xs font-semibold uppercase text-red-400 mb-1.5">Weaknesses</p>
								<ul class="text-xs space-y-1">
									{#each activeReview.review.weaknesses as w}
										<li class="flex gap-1.5"><span class="text-red-400">-</span> {w}</li>
									{/each}
								</ul>
							</div>
						{/if}
						{#if activeReview.review.key_lesson}
							<div class="p-3 bg-blue-950/20 rounded-lg">
								<p class="text-xs font-semibold uppercase text-blue-400 mb-1.5">Key Lesson</p>
								<p class="text-xs">{activeReview.review.key_lesson}</p>
								{#if activeReview.review.emotional_state_detected}
									<div class="mt-2">
										<Badge variant="outline">
											{#snippet children()}Emotional state: {activeReview!.review.emotional_state_detected}{/snippet}
										</Badge>
									</div>
								{/if}
							</div>
						{/if}
					</div>

					{#if activeReview.review.actionable_fixes && activeReview.review.actionable_fixes.length > 0}
						<div class="mt-3 p-3 bg-yellow-950/20 rounded-lg">
							<p class="text-xs font-semibold uppercase text-yellow-400 mb-1.5">Action Items</p>
							<ol class="text-xs space-y-1 list-decimal list-inside">
								{#each activeReview.review.actionable_fixes as fix}
									<li>{fix}</li>
								{/each}
							</ol>
						</div>
					{/if}
				</div>
			{/if}

			<!-- Chat Messages -->
			<div bind:this={messagesContainer} class="flex-1 overflow-y-auto p-6 space-y-4">
				{#each activeReview.messages as message}
					<div class="flex {message.role === 'user' ? 'justify-end' : 'justify-start'}">
						<div class="max-w-[75%] rounded-2xl px-4 py-3 {message.role === 'user'
							? 'bg-primary text-primary-foreground rounded-br-sm'
							: 'bg-muted rounded-bl-sm'}">
							<div class="text-sm whitespace-pre-wrap">{message.content}</div>
							<div class="text-[10px] opacity-50 mt-1.5">{formatDate(message.created_at)}</div>
						</div>
					</div>
				{/each}

				{#if sending}
					<div class="flex justify-start">
						<div class="bg-muted rounded-2xl rounded-bl-sm px-4 py-3">
							<div class="flex items-center gap-2 text-sm text-muted-foreground">
								<div class="flex gap-1">
									<span class="w-1.5 h-1.5 bg-current rounded-full animate-bounce" style="animation-delay: 0ms"></span>
									<span class="w-1.5 h-1.5 bg-current rounded-full animate-bounce" style="animation-delay: 150ms"></span>
									<span class="w-1.5 h-1.5 bg-current rounded-full animate-bounce" style="animation-delay: 300ms"></span>
								</div>
								{thinkingStages[thinkingStage] ?? 'Thinking...'}
							</div>
						</div>
					</div>
				{/if}
			</div>

			<!-- Suggested Questions (only on first exchange) -->
			{#if activeReview.messages.length <= 2 && !sending}
				<div class="px-6 pb-2">
					<div class="flex flex-wrap gap-2">
						{#each suggestedQuestions as q}
							<button
								class="text-xs px-3 py-1.5 rounded-full border border-border hover:bg-muted transition-colors"
								onclick={() => { chatMessage = q; }}
							>
								{q}
							</button>
						{/each}
					</div>
				</div>
			{/if}

			<!-- Chat Input -->
			<div class="border-t p-4">
				<div class="flex gap-3">
					<div class="flex-1">
						<Textarea
							bind:value={chatMessage}
							placeholder="Ask a follow-up question..."
							rows={2}
							disabled={sending}
							onkeydown={handleChatKeydown}
						/>
					</div>
					<div class="flex flex-col gap-2">
						<Button onclick={sendChatMessage} disabled={sending || !chatMessage.trim()}>
							{#snippet children()}Send{/snippet}
						</Button>
						<Button variant="ghost" onclick={() => deleteReview(activeReview!.review.id)}>
							{#snippet children()}<span class="text-xs text-destructive">Delete</span>{/snippet}
						</Button>
					</div>
				</div>
			</div>
		{:else}
			<!-- New Review Form -->
			<div class="flex-1 flex items-center justify-center p-6">
				<Card class="w-full max-w-2xl">
					<CardHeader>
						<CardTitle>Start a New AI Review</CardTitle>
					</CardHeader>
					<CardContent class="space-y-5">
						<div class="space-y-2">
							<label class="text-sm font-medium" for="trade-id">Trade ID (optional)</label>
							<Input
								id="trade-id"
								bind:value={tradeId}
								placeholder="Paste a trade ID for specific trade analysis"
								disabled={sending}
							/>
							<p class="text-xs text-muted-foreground">
								Leave empty for general trading advice
							</p>
						</div>

						<div class="space-y-2">
							<label class="text-sm font-medium" for="prompt">Your Question</label>
							<Textarea
								id="prompt"
								bind:value={prompt}
								placeholder="e.g., 'Analyze my recent AAPL trade' or 'How can I improve my risk management?'"
								rows={4}
								disabled={sending}
							/>
						</div>

						<div class="flex flex-wrap gap-2">
							{#each ['Review my last trade', 'What are my biggest mistakes?', 'Help me build a trading plan', 'Analyze my win rate by setup'] as suggestion}
								<button
									class="text-xs px-3 py-1.5 rounded-full border border-border hover:bg-muted transition-colors"
									onclick={() => { prompt = suggestion; }}
								>
									{suggestion}
								</button>
							{/each}
						</div>

						<Button onclick={startNewReview} disabled={sending || !prompt.trim()}>
							{#snippet children()}
								{#if sending}
									<span class="flex items-center gap-2">
										<Spinner />
										{thinkingStages[thinkingStage] ?? 'Analyzing...'}
									</span>
								{:else}
									Start AI Review
								{/if}
							{/snippet}
						</Button>
					</CardContent>
				</Card>
			</div>
		{/if}
	</div>
</div>
