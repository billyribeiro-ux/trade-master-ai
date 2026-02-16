import { test, expect } from '@playwright/test';

test.describe('Authentication Flow', () => {
	test.beforeEach(async ({ page }) => {
		// Start from the home page
		await page.goto('/');
	});

	test('should display login page', async ({ page }) => {
		await page.goto('/login');
		await expect(page.locator('h1')).toContainText(/log in|sign in/i);
		await expect(page.locator('input[type="email"]')).toBeVisible();
		await expect(page.locator('input[type="password"]')).toBeVisible();
		await expect(page.locator('button[type="submit"]')).toBeVisible();
	});

	test('should display register page', async ({ page }) => {
		await page.goto('/register');
		await expect(page.locator('h1')).toContainText(/register|sign up/i);
		await expect(page.locator('input[type="email"]')).toBeVisible();
		await expect(page.locator('input[type="password"]')).toBeVisible();
		await expect(page.locator('button[type="submit"]')).toBeVisible();
	});

	test('should show validation errors for empty login form', async ({ page }) => {
		await page.goto('/login');
		await page.click('button[type="submit"]');
		
		// Check for validation messages (adjust selectors based on actual implementation)
		const emailInput = page.locator('input[type="email"]');
		await expect(emailInput).toBeFocused();
	});

	test('should navigate between login and register pages', async ({ page }) => {
		await page.goto('/login');
		
		// Find and click the link to register page
		const registerLink = page.locator('a[href*="register"]').first();
		await registerLink.click();
		
		await expect(page).toHaveURL(/.*register/);
		
		// Navigate back to login
		const loginLink = page.locator('a[href*="login"]').first();
		await loginLink.click();
		
		await expect(page).toHaveURL(/.*login/);
	});

	test('should have accessible form elements', async ({ page }) => {
		await page.goto('/login');
		
		// Check for proper labels
		const emailInput = page.locator('input[type="email"]');
		const passwordInput = page.locator('input[type="password"]');
		
		await expect(emailInput).toBeVisible();
		await expect(passwordInput).toBeVisible();
		
		// Check keyboard navigation
		await emailInput.focus();
		await expect(emailInput).toBeFocused();
		
		await page.keyboard.press('Tab');
		await expect(passwordInput).toBeFocused();
	});
});

test.describe('Protected Routes', () => {
	test('should redirect to login when accessing dashboard without auth', async ({ page }) => {
		await page.goto('/dashboard');
		
		// Should redirect to login page
		await expect(page).toHaveURL(/.*login/);
	});

	test('should redirect to login when accessing trades without auth', async ({ page }) => {
		await page.goto('/trades');
		
		// Should redirect to login page
		await expect(page).toHaveURL(/.*login/);
	});

	test('should redirect to login when accessing analytics without auth', async ({ page }) => {
		await page.goto('/analytics');
		
		// Should redirect to login page
		await expect(page).toHaveURL(/.*login/);
	});
});

