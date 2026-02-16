export interface User {
	id: string;
	email: string;
	email_verified: boolean;
	onboarding_completed: boolean;
}

export interface LoginRequest {
	email: string;
	password: string;
}

export interface RegisterRequest {
	email: string;
	password: string;
}

export interface AuthResponse {
	access_token: string;
	refresh_token: string;
	user: User;
}

export interface RefreshTokenRequest {
	refresh_token: string;
}
