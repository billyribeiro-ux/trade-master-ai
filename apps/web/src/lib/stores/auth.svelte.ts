import type { User } from '$services/auth';

interface AuthState {
	user: User | null;
	accessToken: string | null;
	isLoading: boolean;
	isAuthenticated: boolean;
}

class AuthStore {
	private state = $state<AuthState>({
		user: null,
		accessToken: null,
		isLoading: true,
		isAuthenticated: false,
	});

	get user(): User | null {
		return this.state.user;
	}

	get accessToken(): string | null {
		return this.state.accessToken;
	}

	get isLoading(): boolean {
		return this.state.isLoading;
	}

	get isAuthenticated(): boolean {
		return this.state.isAuthenticated;
	}

	get userId(): string | null {
		return this.state.user?.id ?? null;
	}

	get email(): string | null {
		return this.state.user?.email ?? null;
	}

	get displayName(): string | null {
		return this.state.user?.displayName ?? null;
	}

	setUser(user: User | null): void {
		this.state.user = user;
		this.state.isAuthenticated = user !== null;
	}

	setAccessToken(token: string | null): void {
		this.state.accessToken = token;
	}

	setLoading(loading: boolean): void {
		this.state.isLoading = loading;
	}

	login(user: User, accessToken: string): void {
		this.state.user = user;
		this.state.accessToken = accessToken;
		this.state.isAuthenticated = true;
		this.state.isLoading = false;
	}

	logout(): void {
		this.state.user = null;
		this.state.accessToken = null;
		this.state.isAuthenticated = false;
		this.state.isLoading = false;
	}

	updateUser(updates: Partial<User>): void {
		if (this.state.user) {
			this.state.user = { ...this.state.user, ...updates };
		}
	}
}

export const authStore = new AuthStore();
