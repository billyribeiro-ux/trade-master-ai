<script lang="ts">
	import { goto } from '$app/navigation';
	import { authApi } from '$lib/api';
	import Button from '$lib/components/ui/button.svelte';
	import Input from '$lib/components/ui/input.svelte';
	import Label from '$lib/components/ui/label.svelte';
	import Card from '$lib/components/ui/card.svelte';
	import CardHeader from '$lib/components/ui/card-header.svelte';
	import CardTitle from '$lib/components/ui/card-title.svelte';
	import CardDescription from '$lib/components/ui/card-description.svelte';
	import CardContent from '$lib/components/ui/card-content.svelte';
	import CardFooter from '$lib/components/ui/card-footer.svelte';
	import { toasts } from '$lib/stores/toast.svelte';
	import { validateEmail, validatePassword } from '$lib/utils/validation';

	let email = $state('');
	let password = $state('');
	let confirmPassword = $state('');
	let loading = $state(false);
	let errors = $state<{ email?: string; password?: string; confirmPassword?: string }>({});

	async function handleSubmit(e: Event) {
		e.preventDefault();
		errors = {};

		if (!validateEmail(email)) {
			errors.email = 'Please enter a valid email address';
			return;
		}

		const passwordValidation = validatePassword(password);
		if (!passwordValidation.valid) {
			errors.password = passwordValidation.message;
			return;
		}

		if (password !== confirmPassword) {
			errors.confirmPassword = 'Passwords do not match';
			return;
		}

		loading = true;

		try {
			await authApi.register({ email, password });
			toasts.success('Account created successfully!');
			goto('/onboarding');
		} catch (error) {
			toasts.error('Registration failed', error instanceof Error ? error.message : 'Please try again');
		} finally {
			loading = false;
		}
	}
</script>

<svelte:head>
	<title>Sign Up - TradeMaster AI</title>
</svelte:head>

<div class="flex min-h-screen items-center justify-center bg-background p-4">
	<Card class="w-full max-w-md">
		<CardHeader>
			<CardTitle>Create an account</CardTitle>
			<CardDescription>Start your trading journal with TradeMaster AI</CardDescription>
		</CardHeader>
		<form onsubmit={handleSubmit}>
			<CardContent class="space-y-4">
				<div class="space-y-2">
					<Label for="email" required>Email</Label>
					<Input
						id="email"
						type="email"
						bind:value={email}
						placeholder="you@example.com"
						required
						disabled={loading}
					/>
					{#if errors.email}
						<p class="text-sm text-destructive">{errors.email}</p>
					{/if}
				</div>
				<div class="space-y-2">
					<Label for="password" required>Password</Label>
					<Input
						id="password"
						type="password"
						bind:value={password}
						placeholder="••••••••"
						required
						disabled={loading}
					/>
					{#if errors.password}
						<p class="text-sm text-destructive">{errors.password}</p>
					{/if}
				</div>
				<div class="space-y-2">
					<Label for="confirmPassword" required>Confirm Password</Label>
					<Input
						id="confirmPassword"
						type="password"
						bind:value={confirmPassword}
						placeholder="••••••••"
						required
						disabled={loading}
					/>
					{#if errors.confirmPassword}
						<p class="text-sm text-destructive">{errors.confirmPassword}</p>
					{/if}
				</div>
			</CardContent>
			<CardFooter class="flex-col space-y-4">
				<Button type="submit" class="w-full" disabled={loading}>
					{#snippet children()}
						{loading ? 'Creating account...' : 'Create account'}
					{/snippet}
				</Button>
				<p class="text-sm text-muted-foreground text-center">
					Already have an account?
					<a href="/login" class="text-primary hover:underline">Sign in</a>
				</p>
			</CardFooter>
		</form>
	</Card>
</div>
