/**
 * Production Monitoring and Error Tracking
 * 
 * This module provides error tracking, performance monitoring, and logging
 * for production environments. Integrate with services like Sentry, LogRocket,
 * or Datadog as needed.
 */

interface ErrorContext {
	user_id?: string;
	route?: string;
	component?: string;
	action?: string;
	metadata?: Record<string, any>;
}

interface PerformanceMetric {
	name: string;
	value: number;
	unit: 'ms' | 'bytes' | 'count';
	tags?: Record<string, string>;
}

class MonitoringService {
	private isProduction: boolean;
	private userId: string | null = null;

	constructor() {
		this.isProduction = import.meta.env.PROD;
	}

	/**
	 * Initialize monitoring with user context
	 */
	init(userId?: string) {
		this.userId = userId || null;

		if (this.isProduction) {
			// Initialize error tracking service (e.g., Sentry)
			// Sentry.init({
			//   dsn: import.meta.env.VITE_SENTRY_DSN,
			//   environment: import.meta.env.MODE,
			//   tracesSampleRate: 0.1,
			// });

			// Set user context
			if (userId) {
				this.setUser(userId);
			}
		}
	}

	/**
	 * Set user context for error tracking
	 */
	setUser(userId: string) {
		this.userId = userId;

		if (this.isProduction) {
			// Sentry.setUser({ id: userId });
		}
	}

	/**
	 * Clear user context (on logout)
	 */
	clearUser() {
		this.userId = null;

		if (this.isProduction) {
			// Sentry.setUser(null);
		}
	}

	/**
	 * Log an error with context
	 */
	logError(error: Error, context?: ErrorContext) {
		console.error('[Error]', error, context);

		if (this.isProduction) {
			// Sentry.captureException(error, {
			//   tags: {
			//     route: context?.route,
			//     component: context?.component,
			//     action: context?.action,
			//   },
			//   extra: context?.metadata,
			// });
		}
	}

	/**
	 * Log a warning message
	 */
	logWarning(message: string, context?: ErrorContext) {
		console.warn('[Warning]', message, context);

		if (this.isProduction) {
			// Sentry.captureMessage(message, {
			//   level: 'warning',
			//   tags: context,
			// });
		}
	}

	/**
	 * Log an info message
	 */
	logInfo(message: string, metadata?: Record<string, any>) {
		console.info('[Info]', message, metadata);

		// In production, send to logging service
		if (this.isProduction) {
			// Send to logging service (e.g., Datadog, LogRocket)
		}
	}

	/**
	 * Track a performance metric
	 */
	trackPerformance(metric: PerformanceMetric) {
		console.debug('[Performance]', metric);

		if (this.isProduction) {
			// Send to performance monitoring service
			// Example: Datadog, New Relic, etc.
		}
	}

	/**
	 * Track a custom event
	 */
	trackEvent(eventName: string, properties?: Record<string, any>) {
		console.debug('[Event]', eventName, properties);

		if (this.isProduction) {
			// Send to analytics service
			// Example: Mixpanel, Amplitude, etc.
		}
	}

	/**
	 * Measure and track page load time
	 */
	trackPageLoad(route: string) {
		if (typeof window === 'undefined') return;

		const navigation = performance.getEntriesByType('navigation')[0] as PerformanceNavigationTiming;
		if (!navigation) return;

		const loadTime = navigation.loadEventEnd - navigation.fetchStart;
		const domContentLoaded = navigation.domContentLoadedEventEnd - navigation.fetchStart;
		const firstPaint = performance.getEntriesByName('first-paint')[0]?.startTime || 0;

		this.trackPerformance({
			name: 'page_load',
			value: loadTime,
			unit: 'ms',
			tags: { route }
		});

		this.trackPerformance({
			name: 'dom_content_loaded',
			value: domContentLoaded,
			unit: 'ms',
			tags: { route }
		});

		if (firstPaint > 0) {
			this.trackPerformance({
				name: 'first_paint',
				value: firstPaint,
				unit: 'ms',
				tags: { route }
			});
		}
	}
}

// Export singleton instance
export const monitoring = new MonitoringService();

