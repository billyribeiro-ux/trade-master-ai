<script lang="ts">
	import { toasts } from '$lib/stores/toast.svelte';
	import Toast from './toast.svelte';
</script>

<div
	class="fixed top-0 z-[100] flex max-h-screen w-full flex-col-reverse p-4 sm:right-0 sm:top-0 sm:flex-col md:max-w-[420px]"
>
	{#each toasts.items as toast (toast.id)}
		<Toast
			variant={toast.variant}
			title={toast.title}
			description={toast.description}
			duration={toast.duration}
			onClose={() => toasts.remove(toast.id)}
		/>
	{/each}
</div>
