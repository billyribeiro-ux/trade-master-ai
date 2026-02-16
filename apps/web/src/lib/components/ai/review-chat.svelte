<script lang="ts">
	import Button from '$lib/components/ui/button.svelte';
	import Textarea from '$lib/components/ui/textarea.svelte';

	interface Message {
		id: string;
		role: string;
		content: string;
		created_at: string;
	}

	interface Props {
		messages: Message[];
		sending: boolean;
		onSend: (message: string) => void;
	}

	let { messages, sending, onSend }: Props = $props();
	let input = $state('');

	function handleSend() {
		const msg = input.trim();
		if (!msg || sending) return;
		input = '';
		onSend(msg);
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.key === 'Enter' && !e.shiftKey) {
			e.preventDefault();
			handleSend();
		}
	}

	const suggestedQuestions = [
		'What could I have done differently?',
		'How can I improve my risk management?',
		'Was my entry timing optimal?',
		'What patterns should I watch for next time?',
	];
</script>

<div class="flex flex-col h-full">
	<div class="flex-1 overflow-y-auto space-y-4 p-4 min-h-[300px] max-h-[500px]">
		{#each messages as msg}
			<div class="flex {msg.role === 'user' ? 'justify-end' : 'justify-start'}">
				<div class="max-w-[80%] rounded-2xl px-4 py-3 text-sm {msg.role === 'user'
					? 'bg-primary text-primary-foreground rounded-br-sm'
					: 'bg-muted rounded-bl-sm'}">
					<div class="whitespace-pre-wrap">{msg.content}</div>
					<div class="text-[10px] mt-1 opacity-60">
						{new Date(msg.created_at).toLocaleTimeString()}
					</div>
				</div>
			</div>
		{/each}

		{#if sending}
			<div class="flex justify-start">
				<div class="bg-muted rounded-2xl rounded-bl-sm px-4 py-3">
					<div class="flex items-center gap-2 text-sm text-muted-foreground">
						<div class="flex gap-1">
							<span class="w-2 h-2 bg-current rounded-full animate-bounce" style="animation-delay: 0ms"></span>
							<span class="w-2 h-2 bg-current rounded-full animate-bounce" style="animation-delay: 150ms"></span>
							<span class="w-2 h-2 bg-current rounded-full animate-bounce" style="animation-delay: 300ms"></span>
						</div>
						AI is thinking...
					</div>
				</div>
			</div>
		{/if}
	</div>

	{#if messages.length <= 2}
		<div class="px-4 pb-2">
			<p class="text-xs text-muted-foreground mb-2">Suggested follow-ups:</p>
			<div class="flex flex-wrap gap-2">
				{#each suggestedQuestions as q}
					<button
						class="text-xs px-3 py-1.5 rounded-full border border-border hover:bg-muted transition-colors"
						onclick={() => { input = q; }}
					>
						{q}
					</button>
				{/each}
			</div>
		</div>
	{/if}

	<div class="border-t p-4">
		<div class="flex gap-2">
			<div class="flex-1">
				<Textarea
					bind:value={input}
					placeholder="Ask a follow-up question..."
					rows={2}
					onkeydown={handleKeydown}
				/>
			</div>
			<Button onclick={handleSend} disabled={sending || !input.trim()}>
				{#snippet children()}Send{/snippet}
			</Button>
		</div>
	</div>
</div>
