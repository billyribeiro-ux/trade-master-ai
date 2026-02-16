import { test, expect } from '@playwright/test';

test.describe('Navigation and Accessibility', () => {
	test('should have skip navigation link', async ({ page }) => {
		await page.goto('/login');
		
		// Focus on the page
		await page.keyboard.press('Tab');
		
		// Check if skip link becomes visible on focus
		const skipLink = page.locator('a[href="#main-content"]');
		await expect(skipLink).toBeVisible();
	});

	test('should navigate with keyboard', async ({ page }) => {
		await page.goto('/login');
		
		// Tab through interactive elements
		await page.keyboard.press('Tab');
		await page.keyboard.press('Tab');
		
		// Verify focus is on an interactive element
		const focusedElement = page.locator(':focus');
		await expect(focusedElement).toBeVisible();
	});

	test('should have proper heading hierarchy', async ({ page }) => {
		await page.goto('/login');
		
		// Check for h1
		const h1 = page.locator('h1');
		await expect(h1).toBeVisible();
		
		// Verify only one h1 per page
		const h1Count = await h1.count();
		expect(h1Count).toBeLessThanOrEqual(1);
	});

	test('should have proper ARIA labels on navigation', async ({ page }) => {
		await page.goto('/login');
		
		// Check for navigation landmarks
		const nav = page.locator('nav');
		if (await nav.count() > 0) {
			const ariaLabel = await nav.first().getAttribute('aria-label');
			expect(ariaLabel).toBeTruthy();
		}
	});

	test('should have proper page titles', async ({ page }) => {
		await page.goto('/login');
		await expect(page).toHaveTitle(/login|sign in/i);
		
		await page.goto('/register');
		await expect(page).toHaveTitle(/register|sign up/i);
	});

	test('should have proper meta descriptions', async ({ page }) => {
		await page.goto('/login');
		
		const metaDescription = page.locator('meta[name="description"]');
		const content = await metaDescription.getAttribute('content');
		expect(content).toBeTruthy();
		expect(content!.length).toBeGreaterThan(0);
	});
});

test.describe('Responsive Design', () => {
	test('should be responsive on mobile', async ({ page }) => {
		await page.setViewportSize({ width: 375, height: 667 });
		await page.goto('/login');
		
		// Check that form is visible and usable
		const emailInput = page.locator('input[type="email"]');
		const passwordInput = page.locator('input[type="password"]');
		const submitButton = page.locator('button[type="submit"]');
		
		await expect(emailInput).toBeVisible();
		await expect(passwordInput).toBeVisible();
		await expect(submitButton).toBeVisible();
	});

	test('should be responsive on tablet', async ({ page }) => {
		await page.setViewportSize({ width: 768, height: 1024 });
		await page.goto('/login');
		
		const emailInput = page.locator('input[type="email"]');
		await expect(emailInput).toBeVisible();
	});

	test('should be responsive on desktop', async ({ page }) => {
		await page.setViewportSize({ width: 1920, height: 1080 });
		await page.goto('/login');
		
		const emailInput = page.locator('input[type="email"]');
		await expect(emailInput).toBeVisible();
	});
});

