<script lang="ts">
	import { goto } from '$app/navigation';
	import { authApi } from '$lib/api';
	import { onMount } from 'svelte';
	import Toaster from '$lib/components/ui/toaster.svelte';
	import type { Snippet } from 'svelte';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();

	let user = $state<{ id: string; email: string } | null>(null);
	let loading = $state(true);
	let currentPath = $state('');

	onMount(() => {
		// Track current path for active nav highlighting
		currentPath = window.location.pathname;

		// Listen for navigation changes
		const handleNavigation = () => {
			currentPath = window.location.pathname;
		};
		window.addEventListener('popstate', handleNavigation);

		// Load user data
		(async () => {
			try {
				const currentUser = await authApi.getCurrentUser();
				user = currentUser;
			} catch {
				goto('/login');
			} finally {
				loading = false;
			}
		})();

		return () => {
			window.removeEventListener('popstate', handleNavigation);
		};
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
	<!-- Skip Navigation Link for Accessibility -->
	<a
		href="#main-content"
		class="sr-only focus:not-sr-only focus:absolute focus:left-4 focus:top-4 focus:z-50 focus:rounded-md focus:bg-primary focus:px-4 focus:py-2 focus:text-primary-foreground focus:outline-none focus:ring-2 focus:ring-ring"
	>
		Skip to main content
	</a>

	<div class="flex h-screen bg-background">
		<!-- Sidebar -->
		<aside class="hidden w-64 border-r bg-card lg:block">
			<div class="flex h-full flex-col">
				<div class="flex h-16 items-center border-b px-6">
					<h1 class="text-xl font-bold">TradeMaster AI</h1>
				</div>
				<nav class="flex-1 space-y-1 p-4" aria-label="Main navigation">
					{#each navItems as item}
						<a
							href={item.href}
							class="flex items-center gap-3 rounded-lg px-3 py-2 text-sm font-medium transition-colors hover:bg-accent hover:text-accent-foreground {currentPath.startsWith(item.href) ? 'bg-accent text-accent-foreground' : 'text-muted-foreground'}"
							aria-current={currentPath.startsWith(item.href) ? 'page' : undefined}
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
		<main id="main-content" class="flex-1 overflow-auto">
			{@render children()}
		</main>
	</div>
	<Toaster />
{/if}
