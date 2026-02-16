<script lang="ts">
	import { goto } from '$app/navigation';
	import { page } from '$app/stores';
	import { authApi } from '$lib/api';
	import { onMount } from 'svelte';
	import Toaster from '$lib/components/ui/toaster.svelte';

	let user = $state<{ id: string; email: string } | null>(null);
	let loading = $state(true);

	onMount(async () => {
		try {
			const currentUser = await authApi.getCurrentUser();
			user = currentUser;
		} catch {
			goto('/login');
		} finally {
			loading = false;
		}
	});

	async function handleLogout() {
		await authApi.logout();
		goto('/login');
	}

	const navItems = [
		{ href: '/dashboard', label: 'Dashboard', icon: 'LayoutDashboard' },
		{ href: '/trades', label: 'Trades', icon: 'TrendingUp' },
		{ href: '/analytics', label: 'Analytics', icon: 'BarChart3' },
		{ href: '/planning', label: 'Planning', icon: 'Calendar' },
		{ href: '/ai-review', label: 'AI Coach', icon: 'Bot' },
		{ href: '/risk', label: 'Risk Tools', icon: 'Shield' },
		{ href: '/psychology', label: 'Psychology', icon: 'Brain' },
		{ href: '/playbook', label: 'Playbook', icon: 'BookOpen' },
		{ href: '/reviews', label: 'Reviews', icon: 'ClipboardCheck' },
	];
</script>

{#if loading}
	<div class="flex h-screen items-center justify-center">
		<div class="text-muted-foreground">Loading...</div>
	</div>
{:else if user}
	<div class="flex h-screen bg-background">
		<!-- Sidebar -->
		<aside class="hidden w-64 border-r bg-card lg:block">
			<div class="flex h-full flex-col">
				<div class="flex h-16 items-center border-b px-6">
					<h1 class="text-xl font-bold">TradeMaster AI</h1>
				</div>
				<nav class="flex-1 space-y-1 p-4">
					{#each navItems as item}
						<a
							href={item.href}
							class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground {$page.url.pathname.startsWith(item.href) ? 'bg-accent text-accent-foreground' : 'text-muted-foreground'}"
						>
							{item.label}
						</a>
					{/each}
				</nav>
				<div class="border-t p-4">
					<div class="flex items-center gap-3 rounded-lg px-3 py-2">
						<div class="flex-1">
							<p class="text-sm font-medium">{user.email}</p>
						</div>
					</div>
					<button
						onclick={handleLogout}
						class="mt-2 w-full rounded-lg px-3 py-2 text-sm font-medium text-muted-foreground transition-colors hover:bg-accent hover:text-accent-foreground"
					>
						Logout
					</button>
				</div>
			</div>
		</aside>

		<!-- Main content -->
		<main class="flex-1 overflow-auto">
			{@render children()}
		</main>
	</div>
	<Toaster />
{/if}
