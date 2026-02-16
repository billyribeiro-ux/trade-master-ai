import { ApiError, parseApiError } from './errors';
import { authStore } from '$stores/auth.svelte';

const API_BASE_URL = import.meta.env.PUBLIC_API_URL || 'http://localhost:3001';
const API_PREFIX = '/api/v1';
const DEFAULT_TIMEOUT = 30000;

interface RequestConfig extends RequestInit {
	timeout?: number;
}

class ApiClient {
	private baseUrl: string;
	private isRefreshing = false;
	private refreshPromise: Promise<void> | null = null;

	constructor(baseUrl: string) {
		this.baseUrl = baseUrl;
	}

	private async request<T>(
		endpoint: string,
		config: RequestConfig = {},
	): Promise<T> {
		const { timeout = DEFAULT_TIMEOUT, ...fetchConfig } = config;

		const controller = new AbortController();
		const timeoutId = setTimeout(() => controller.abort(), timeout);

		try {
			const url = `${this.baseUrl}${API_PREFIX}${endpoint}`;
			const headers: Record<string, string> = {
				'Content-Type': 'application/json',
				...(fetchConfig.headers as Record<string, string>),
			};

			const accessToken = authStore.accessToken;
			if (accessToken) {
				headers['Authorization'] = `Bearer ${accessToken}`;
			}

			const response = await fetch(url, {
				...fetchConfig,
				headers,
				credentials: 'include',
				signal: controller.signal,
			});

			clearTimeout(timeoutId);

			if (response.status === 401 && !endpoint.includes('/auth/')) {
				if (this.isRefreshing) {
					await this.refreshPromise;
					return this.request<T>(endpoint, config);
				}

				this.isRefreshing = true;
				this.refreshPromise = this.handleTokenRefresh();

				try {
					await this.refreshPromise;
					return this.request<T>(endpoint, config);
				} finally {
					this.isRefreshing = false;
					this.refreshPromise = null;
				}
			}

			if (!response.ok) {
				throw await parseApiError(response);
			}

			if (response.status === 204) {
				return undefined as T;
			}

			return await response.json();
		} catch (error) {
			clearTimeout(timeoutId);

			if (error instanceof ApiError) {
				throw error;
			}

			if ((error as Error).name === 'AbortError') {
				throw new ApiError(408, 'TIMEOUT', 'Request timeout', null);
			}

			throw new ApiError(0, 'NETWORK_ERROR', 'Network error occurred', error);
		}
	}

	private async handleTokenRefresh(): Promise<void> {
		try {
			const response = await fetch(`${this.baseUrl}${API_PREFIX}/auth/refresh`, {
				method: 'POST',
				credentials: 'include',
			});

			if (!response.ok) {
				throw new Error('Token refresh failed');
			}

			const data = await response.json();
			authStore.setAccessToken(data.accessToken);
		} catch (error) {
			authStore.logout();
			if (typeof window !== 'undefined') {
				window.location.href = '/login';
			}
			throw error;
		}
	}

	async get<T>(endpoint: string, config?: RequestConfig): Promise<T> {
		return this.request<T>(endpoint, { ...config, method: 'GET' });
	}

	async post<T>(endpoint: string, data: unknown, config?: RequestConfig): Promise<T> {
		return this.request<T>(endpoint, {
			...config,
			method: 'POST',
			body: JSON.stringify(data),
		});
	}

	async put<T>(endpoint: string, data: unknown, config?: RequestConfig): Promise<T> {
		return this.request<T>(endpoint, {
			...config,
			method: 'PUT',
			body: JSON.stringify(data),
		});
	}

	async patch<T>(endpoint: string, data: unknown, config?: RequestConfig): Promise<T> {
		return this.request<T>(endpoint, {
			...config,
			method: 'PATCH',
			body: JSON.stringify(data),
		});
	}

	async delete<T>(endpoint: string, config?: RequestConfig): Promise<T> {
		return this.request<T>(endpoint, { ...config, method: 'DELETE' });
	}

	async uploadFile<T>(
		endpoint: string,
		formData: FormData,
		config?: RequestConfig,
	): Promise<T> {
		const { timeout = DEFAULT_TIMEOUT, ...fetchConfig } = config || {};
		const controller = new AbortController();
		const timeoutId = setTimeout(() => controller.abort(), timeout);

		try {
			const url = `${this.baseUrl}${API_PREFIX}${endpoint}`;
			const headers: Record<string, string> = {
				...(fetchConfig.headers as Record<string, string>),
			};

			const accessToken = authStore.accessToken;
			if (accessToken) {
				headers['Authorization'] = `Bearer ${accessToken}`;
			}

			const response = await fetch(url, {
				...fetchConfig,
				method: 'POST',
				headers,
				body: formData,
				credentials: 'include',
				signal: controller.signal,
			});

			clearTimeout(timeoutId);

			if (!response.ok) {
				throw await parseApiError(response);
			}

			return await response.json();
		} catch (error) {
			clearTimeout(timeoutId);
			throw error;
		}
	}
}

export const apiClient = new ApiClient(API_BASE_URL);
