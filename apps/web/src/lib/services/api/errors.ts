export type ErrorCode =
	| 'UNAUTHORIZED'
	| 'FORBIDDEN'
	| 'NOT_FOUND'
	| 'VALIDATION'
	| 'RATE_LIMITED'
	| 'SERVER_ERROR'
	| 'NETWORK_ERROR'
	| 'TIMEOUT'
	| 'UNKNOWN';

export class ApiError extends Error {
	constructor(
		public status: number,
		public code: ErrorCode,
		message: string,
		public details: unknown = null,
	) {
		super(message);
		this.name = 'ApiError';
	}
}

export async function parseApiError(response: Response): Promise<ApiError> {
	let errorData: { code?: string; message?: string; details?: unknown } = {};

	try {
		errorData = await response.json();
	} catch {
		// Response body is not JSON
	}

	const status = response.status;
	const code = mapStatusToCode(status);
	const message = errorData.message || getDefaultMessage(status);
	const details = errorData.details || null;

	return new ApiError(status, code, message, details);
}

function mapStatusToCode(status: number): ErrorCode {
	switch (status) {
		case 401:
			return 'UNAUTHORIZED';
		case 403:
			return 'FORBIDDEN';
		case 404:
			return 'NOT_FOUND';
		case 422:
			return 'VALIDATION';
		case 429:
			return 'RATE_LIMITED';
		case 408:
			return 'TIMEOUT';
		default:
			if (status >= 500) return 'SERVER_ERROR';
			if (status === 0) return 'NETWORK_ERROR';
			return 'UNKNOWN';
	}
}

function getDefaultMessage(status: number): string {
	switch (status) {
		case 401:
			return 'You must be logged in to perform this action';
		case 403:
			return 'You do not have permission to perform this action';
		case 404:
			return 'The requested resource was not found';
		case 422:
			return 'The provided data is invalid';
		case 429:
			return 'Too many requests. Please try again later';
		case 408:
			return 'Request timeout. Please try again';
		case 0:
			return 'Network error. Please check your connection';
		default:
			if (status >= 500) return 'Server error. Please try again later';
			return 'An unexpected error occurred';
	}
}

export function isApiError(error: unknown): error is ApiError {
	return error instanceof ApiError;
}

export function getUserFriendlyMessage(error: unknown): string {
	if (isApiError(error)) {
		return error.message;
	}

	if (error instanceof Error) {
		return error.message;
	}

	return 'An unexpected error occurred';
}
