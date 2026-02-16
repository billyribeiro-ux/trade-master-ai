<script lang="ts">
	import { apiClient } from '$lib/api';
	import Button from '$lib/components/ui/button.svelte';
	import Input from '$lib/components/ui/input.svelte';
	import Textarea from '$lib/components/ui/textarea.svelte';
	import Card from '$lib/components/ui/card.svelte';
	import CardHeader from '$lib/components/ui/card-header.svelte';
	import CardTitle from '$lib/components/ui/card-title.svelte';
	import CardContent from '$lib/components/ui/card-content.svelte';
	import Spinner from '$lib/components/ui/spinner.svelte';
	import { toasts } from '$lib/stores/toast.svelte';
	import { onMount } from 'svelte';

	interface AiReview {
		id: string;
		review_type: string;
		prompt: string;
		response: string;
		created_at: string;
	}

	interface ChatMessage {
		id: string;
		role: 'user' | 'assistant';
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

	async function startNewReview() {
		if (!prompt.trim()) {
			toasts.error('Please enter a prompt');
			return;
		}

		sending = true;
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
			}
		} catch (error) {
			toasts.error('Failed to start review', error instanceof Error ? error.message : 'Please try again');
		} finally {
			sending = false;
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
		} catch (error) {
			toasts.error('Failed to send message');
			chatMessage = msg;
		} finally {
			sending = false;
		}
	}

	async function openReview(reviewId: string) {
		loading = true;
		try {
			activeReview = await apiClient.get<{ review: AiReview; messages: ChatMessage[] }>(
				`/api/v1/ai/reviews/${reviewId}`
			) ?? null;
			scrollToBottom();
		} catch {
			toasts.error('Failed to load review');
		} finally {
			loading = false;
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
</script>

<svelte:head>
	<title>AI Trade Review - TradeMaster AI</title>
</svelte:head>

<div class="flex h-[calc(100vh-4rem)]">
	<!-- Sidebar: Review History -->
	<div class="w-80 border-r bg-muted/30 flex flex-col">
		<div class="p-4 border-b">
			<h2 class="font-semibold text-lg">AI Reviews</h2>
			<p class="text-sm text-muted-foreground mt-1">Chat with your AI trading coach</p>
		</div>

		<button
			class="m-3 px-4 py-2 bg-primary text-primary-foreground rounded-lg text-sm font-medium hover:bg-primary/90 transition-colors"
			onclick={() => { activeReview = null; }}
		>
			+ New Review
		</button>

		<div class="flex-1 overflow-y-auto">
			{#each reviews as review}
				<button
					class="w-full text-left p-3 border-b hover:bg-muted/50 transition-colors {activeReview?.review.id === review.id ? 'bg-muted' : ''}"
					onclick={() => openReview(review.id)}
				>
					<div class="text-sm font-medium truncate">{review.prompt.slice(0, 60)}...</div>
					<div class="text-xs text-muted-foreground mt-1">
						{review.review_type} · {formatDate(review.created_at)}
					</div>
				</button>
			{/each}

			{#if reviews.length === 0 && !loading}
				<p class="text-center text-muted-foreground text-sm p-6">
					No reviews yet. Start a conversation with your AI coach.
				</p>
			{/if}
		</div>
	</div>

	<!-- Main Chat Area -->
	<div class="flex-1 flex flex-col">
		{#if activeReview}
			<!-- Chat Messages -->
			<div bind:this={messagesContainer} class="flex-1 overflow-y-auto p-6 space-y-4">
				{#each activeReview.messages as message}
					<div class="flex {message.role === 'user' ? 'justify-end' : 'justify-start'}">
						<div class="max-w-[75%] rounded-lg p-4 {message.role === 'user'
							? 'bg-primary text-primary-foreground'
							: 'bg-muted'}">
							<div class="text-sm whitespace-pre-wrap">{message.content}</div>
							<div class="text-xs opacity-60 mt-2">{formatDate(message.created_at)}</div>
						</div>
					</div>
				{/each}

				{#if sending}
					<div class="flex justify-start">
						<div class="bg-muted rounded-lg p-4">
							<Spinner size="sm" />
							<span class="text-sm ml-2">Thinking...</span>
						</div>
					</div>
				{/if}
			</div>

			<!-- Chat Input -->
			<div class="border-t p-4">
				<form
					class="flex gap-3"
					onsubmit={(e) => { e.preventDefault(); sendChatMessage(); }}
				>
					<Input
						bind:value={chatMessage}
						placeholder="Ask a follow-up question..."
						disabled={sending}
					/>
					<Button type="submit" disabled={sending || !chatMessage.trim()}>
						{#snippet children()}
							Send
						{/snippet}
					</Button>
				</form>
			</div>
		{:else}
			<!-- New Review Form -->
			<div class="flex-1 flex items-center justify-center p-6">
				<Card class="w-full max-w-2xl">
					<CardHeader>
						<CardTitle>Start a New AI Review</CardTitle>
					</CardHeader>
					<CardContent class="space-y-4">
						<div class="space-y-2">
							<label class="text-sm font-medium" for="trade-id">Trade ID (optional)</label>
							<Input
								id="trade-id"
								bind:value={tradeId}
								placeholder="Paste a trade ID for specific analysis"
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
								placeholder="What would you like to discuss? e.g., 'Analyze my recent AAPL trade' or 'How can I improve my risk management?'"
								rows={4}
								disabled={sending}
							/>
						</div>

						<Button onclick={startNewReview} disabled={sending || !prompt.trim()}>
							{#snippet children()}
								{#if sending}
									<Spinner size="sm" />
									<span class="ml-2">Analyzing...</span>
								{:else}
									Start Review
								{/if}
							{/snippet}
						</Button>
					</CardContent>
				</Card>
			</div>
		{/if}
	</div>
</div>
