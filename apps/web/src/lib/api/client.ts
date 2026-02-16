import { browser } from '$app/environment';
import { goto } from '$app/navigation';

const API_BASE_URL = import.meta.env.VITE_API_URL || 'http://localhost:3000';

interface RequestOptions extends RequestInit {
	requiresAuth?: boolean;
}

class ApiClient {
	private accessToken: string | null = null;
	private refreshToken: string | null = null;

	constructor() {
		if (browser) {
			this.accessToken = localStorage.getItem('access_token');
			this.refreshToken = localStorage.getItem('refresh_token');
		}
	}

	setTokens(accessToken: string, refreshToken: string) {
		this.accessToken = accessToken;
		this.refreshToken = refreshToken;
		if (browser) {
			localStorage.setItem('access_token', accessToken);
			localStorage.setItem('refresh_token', refreshToken);
		}
	}

	clearTokens() {
		this.accessToken = null;
		this.refreshToken = null;
		if (browser) {
			localStorage.removeItem('access_token');
			localStorage.removeItem('refresh_token');
		}
	}

	getAccessToken(): string | null {
		return this.accessToken;
	}

	async request<T>(endpoint: string, options: RequestOptions = {}): Promise<T> {
		const { requiresAuth = true, ...fetchOptions } = options;

		const headers: Record<string, string> = {
			'Content-Type': 'application/json',
			...(fetchOptions.headers as Record<string, string>)
		};

		if (requiresAuth && this.accessToken) {
			headers['Authorization'] = `Bearer ${this.accessToken}`;
		}

		const url = `${API_BASE_URL}${endpoint}`;

		try {
			let response = await fetch(url, {
				...fetchOptions,
				headers
			});

			if (response.status === 401 && requiresAuth && this.refreshToken) {
				const refreshed = await this.refreshAccessToken();
				if (refreshed) {
					headers['Authorization'] = `Bearer ${this.accessToken}`;
					response = await fetch(url, {
						...fetchOptions,
						headers
					});
				} else {
					this.clearTokens();
					if (browser) {
						goto('/login');
					}
					throw new Error('Session expired. Please log in again.');
				}
			}

			if (!response.ok) {
				const error = await response.json().catch(() => ({ message: response.statusText }));
				throw new Error(error.message || `HTTP ${response.status}: ${response.statusText}`);
			}

			const contentType = response.headers.get('content-type');
			if (contentType && contentType.includes('application/json')) {
				return await response.json();
			}

			return {} as T;
		} catch (error) {
			if (error instanceof Error) {
				throw error;
			}
			throw new Error('An unexpected error occurred');
		}
	}

	private async refreshAccessToken(): Promise<boolean> {
		if (!this.refreshToken) {
			return false;
		}

		try {
			const response = await fetch(`${API_BASE_URL}/api/v1/auth/refresh`, {
				method: 'POST',
				headers: {
					'Content-Type': 'application/json'
				},
				body: JSON.stringify({ refresh_token: this.refreshToken })
			});

			if (!response.ok) {
				return false;
			}

			const data = await response.json();
			this.setTokens(data.access_token, data.refresh_token);
			return true;
		} catch {
			return false;
		}
	}

	async get<T>(endpoint: string, options?: RequestOptions): Promise<T> {
		return this.request<T>(endpoint, { ...options, method: 'GET' });
	}

	async post<T>(endpoint: string, data?: unknown, options?: RequestOptions): Promise<T> {
		return this.request<T>(endpoint, {
			...options,
			method: 'POST',
			body: data ? JSON.stringify(data) : undefined
		});
	}

	async put<T>(endpoint: string, data?: unknown, options?: RequestOptions): Promise<T> {
		return this.request<T>(endpoint, {
			...options,
			method: 'PUT',
			body: data ? JSON.stringify(data) : undefined
		});
	}

	async delete<T>(endpoint: string, options?: RequestOptions): Promise<T> {
		return this.request<T>(endpoint, { ...options, method: 'DELETE' });
	}
}

export const apiClient = new ApiClient();
