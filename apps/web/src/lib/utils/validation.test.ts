import { describe, it, expect } from 'vitest';

/**
 * Email validation tests
 */
describe('Email Validation', () => {
	const validateEmail = (email: string): boolean => {
		const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
		return emailRegex.test(email);
	};

	it('should validate correct email addresses', () => {
		expect(validateEmail('test@example.com')).toBe(true);
		expect(validateEmail('user.name@domain.co.uk')).toBe(true);
		expect(validateEmail('user+tag@example.com')).toBe(true);
	});

	it('should reject invalid email addresses', () => {
		expect(validateEmail('invalid')).toBe(false);
		expect(validateEmail('invalid@')).toBe(false);
		expect(validateEmail('@example.com')).toBe(false);
		expect(validateEmail('test@')).toBe(false);
		expect(validateEmail('test @example.com')).toBe(false);
	});

	it('should reject empty strings', () => {
		expect(validateEmail('')).toBe(false);
	});
});

/**
 * Password validation tests
 */
describe('Password Validation', () => {
	const validatePassword = (password: string): boolean => {
		return password.length >= 8;
	};

	it('should accept passwords with 8+ characters', () => {
		expect(validatePassword('password123')).toBe(true);
		expect(validatePassword('12345678')).toBe(true);
		expect(validatePassword('VeryLongPassword123!')).toBe(true);
	});

	it('should reject passwords with less than 8 characters', () => {
		expect(validatePassword('short')).toBe(false);
		expect(validatePassword('1234567')).toBe(false);
		expect(validatePassword('')).toBe(false);
	});
});

/**
 * Number formatting tests
 */
describe('Number Formatting', () => {
	const formatCurrency = (value: number): string => {
		return new Intl.NumberFormat('en-US', {
			style: 'currency',
			currency: 'USD'
		}).format(value);
	};

	it('should format positive numbers correctly', () => {
		expect(formatCurrency(1234.56)).toBe('$1,234.56');
		expect(formatCurrency(0.99)).toBe('$0.99');
	});

	it('should format negative numbers correctly', () => {
		expect(formatCurrency(-1234.56)).toBe('-$1,234.56');
	});

	it('should format zero correctly', () => {
		expect(formatCurrency(0)).toBe('$0.00');
	});
});

/**
 * Percentage formatting tests
 */
describe('Percentage Formatting', () => {
	const formatPercentage = (value: number, decimals = 2): string => {
		return `${value.toFixed(decimals)}%`;
	};

	it('should format percentages with default decimals', () => {
		expect(formatPercentage(12.345)).toBe('12.35%');
		expect(formatPercentage(0.5)).toBe('0.50%');
	});

	it('should format percentages with custom decimals', () => {
		expect(formatPercentage(12.345, 1)).toBe('12.3%');
		expect(formatPercentage(12.345, 0)).toBe('12%');
	});

	it('should handle negative percentages', () => {
		expect(formatPercentage(-5.67)).toBe('-5.67%');
	});
});

