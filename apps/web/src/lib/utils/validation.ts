export function validateEmail(email: string): boolean {
	const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
	return emailRegex.test(email);
}

export function validatePassword(password: string): { valid: boolean; message?: string } {
	if (password.length < 8) {
		return { valid: false, message: 'Password must be at least 8 characters' };
	}
	return { valid: true };
}

export function validateRequired(value: unknown, fieldName: string): { valid: boolean; message?: string } {
	if (value === null || value === undefined || value === '') {
		return { valid: false, message: `${fieldName} is required` };
	}
	return { valid: true };
}

export function validateNumber(value: unknown, fieldName: string, options?: { min?: number; max?: number }): { valid: boolean; message?: string } {
	const num = Number(value);
	
	if (isNaN(num)) {
		return { valid: false, message: `${fieldName} must be a valid number` };
	}
	
	if (options?.min !== undefined && num < options.min) {
		return { valid: false, message: `${fieldName} must be at least ${options.min}` };
	}
	
	if (options?.max !== undefined && num > options.max) {
		return { valid: false, message: `${fieldName} must be at most ${options.max}` };
	}
	
	return { valid: true };
}
