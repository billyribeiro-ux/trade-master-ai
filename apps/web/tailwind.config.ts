import type { Config } from 'tailwindcss';

export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			colors: {
				bg: {
					primary: 'rgb(var(--bg-primary) / <alpha-value>)',
					secondary: 'rgb(var(--bg-secondary) / <alpha-value>)',
					tertiary: 'rgb(var(--bg-tertiary) / <alpha-value>)',
					hover: 'rgb(var(--bg-hover) / <alpha-value>)',
					active: 'rgb(var(--bg-active) / <alpha-value>)',
					input: 'rgb(var(--bg-input) / <alpha-value>)'
				},
				text: {
					primary: 'rgb(var(--text-primary) / <alpha-value>)',
					secondary: 'rgb(var(--text-secondary) / <alpha-value>)',
					tertiary: 'rgb(var(--text-tertiary) / <alpha-value>)',
					inverse: 'rgb(var(--text-inverse) / <alpha-value>)'
				},
				profit: {
					DEFAULT: 'rgb(var(--profit) / <alpha-value>)',
					muted: 'rgb(var(--profit-muted) / <alpha-value>)',
					bg: 'rgb(var(--profit-bg) / <alpha-value>)'
				},
				loss: {
					DEFAULT: 'rgb(var(--loss) / <alpha-value>)',
					muted: 'rgb(var(--loss-muted) / <alpha-value>)',
					bg: 'rgb(var(--loss-bg) / <alpha-value>)'
				},
				neutral: 'rgb(var(--neutral) / <alpha-value>)',
				accent: {
					primary: 'rgb(var(--accent-primary) / <alpha-value>)',
					'primary-hover': 'rgb(var(--accent-primary-hover) / <alpha-value>)',
					'primary-active': 'rgb(var(--accent-primary-active) / <alpha-value>)',
					secondary: 'rgb(var(--accent-secondary) / <alpha-value>)'
				},
				warning: {
					DEFAULT: 'rgb(var(--warning) / <alpha-value>)',
					bg: 'rgb(var(--warning-bg) / <alpha-value>)'
				},
				info: {
					DEFAULT: 'rgb(var(--info) / <alpha-value>)',
					bg: 'rgb(var(--info-bg) / <alpha-value>)'
				},
				danger: {
					DEFAULT: 'rgb(var(--danger) / <alpha-value>)',
					bg: 'rgb(var(--danger-bg) / <alpha-value>)'
				},
				success: 'rgb(var(--success) / <alpha-value>)',
				border: {
					primary: 'rgb(var(--border-primary) / <alpha-value>)',
					secondary: 'rgb(var(--border-secondary) / <alpha-value>)',
					focus: 'rgb(var(--border-focus) / <alpha-value>)',
					hover: 'rgb(var(--border-hover) / <alpha-value>)'
				},
				grade: {
					a: 'var(--grade-a)',
					'a-bg': 'var(--grade-a-bg)',
					b: 'var(--grade-b)',
					'b-bg': 'var(--grade-b-bg)',
					c: 'var(--grade-c)',
					'c-bg': 'var(--grade-c-bg)',
					d: 'var(--grade-d)',
					'd-bg': 'var(--grade-d-bg)'
				},
				conviction: {
					1: 'var(--conviction-1)',
					2: 'var(--conviction-2)',
					3: 'var(--conviction-3)',
					4: 'var(--conviction-4)',
					5: 'var(--conviction-5)'
				},
				score: {
					0: 'var(--score-0)',
					25: 'var(--score-25)',
					50: 'var(--score-50)',
					75: 'var(--score-75)',
					100: 'var(--score-100)'
				}
			},
			fontFamily: {
				sans: ['Inter', 'system-ui', 'sans-serif'],
				mono: ['JetBrains Mono', 'monospace']
			},
			fontSize: {
				xs: ['0.6875rem', { lineHeight: '1rem' }],
				sm: ['0.75rem', { lineHeight: '1.25rem' }],
				base: ['0.875rem', { lineHeight: '1.5rem' }],
				lg: ['1rem', { lineHeight: '1.75rem' }],
				xl: ['1.125rem', { lineHeight: '1.75rem' }],
				'2xl': ['1.25rem', { lineHeight: '2rem' }],
				'3xl': ['1.5rem', { lineHeight: '2rem' }],
				'4xl': ['1.875rem', { lineHeight: '2.25rem' }],
				'5xl': ['2.25rem', { lineHeight: '2.5rem' }]
			},
			borderRadius: {
				none: '0',
				sm: '0.25rem',
				DEFAULT: '0.375rem',
				md: '0.375rem',
				lg: '0.5rem',
				xl: '0.75rem',
				'2xl': '1rem',
				full: '9999px'
			},
			boxShadow: {
				sm: '0 1px 2px 0 rgb(0 0 0 / 0.05)',
				DEFAULT: '0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1)',
				md: '0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)',
				lg: '0 10px 15px -3px rgb(0 0 0 / 0.1), 0 4px 6px -4px rgb(0 0 0 / 0.1)',
				'glow-profit': '0 0 20px rgb(var(--profit) / 0.3)',
				'glow-loss': '0 0 20px rgb(var(--loss) / 0.3)',
				'glow-accent': '0 0 20px rgb(var(--accent-primary) / 0.3)'
			},
			transitionDuration: {
				DEFAULT: '150ms',
				fast: '75ms',
				slow: '300ms'
			},
			transitionTimingFunction: {
				DEFAULT: 'ease-in-out'
			},
			zIndex: {
				base: '0',
				dropdown: '50',
				sticky: '100',
				overlay: '200',
				modal: '300',
				popover: '400',
				toast: '500',
				tooltip: '600'
			},
			animation: {
				'fade-in': 'fadeIn 150ms ease-in-out',
				'slide-up': 'slideUp 150ms ease-in-out',
				'slide-down': 'slideDown 150ms ease-in-out',
				'scale-in': 'scaleIn 150ms ease-in-out',
				'pulse-subtle': 'pulseSubtle 2s ease-in-out infinite'
			},
			keyframes: {
				fadeIn: {
					'0%': { opacity: '0' },
					'100%': { opacity: '1' }
				},
				slideUp: {
					'0%': { transform: 'translateY(10px)', opacity: '0' },
					'100%': { transform: 'translateY(0)', opacity: '1' }
				},
				slideDown: {
					'0%': { transform: 'translateY(-10px)', opacity: '0' },
					'100%': { transform: 'translateY(0)', opacity: '1' }
				},
				scaleIn: {
					'0%': { transform: 'scale(0.95)', opacity: '0' },
					'100%': { transform: 'scale(1)', opacity: '1' }
				},
				pulseSubtle: {
					'0%, 100%': { opacity: '1' },
					'50%': { opacity: '0.8' }
				}
			}
		}
	}
} satisfies Config;
