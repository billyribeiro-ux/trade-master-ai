// Theme configuration for chart libraries (ECharts, Lightweight Charts)
// These values match the design system tokens defined in app.css

export const colors = {
	bg: {
		primary: '#0a0e14',
		secondary: '#111621',
		tertiary: '#1a1f2e',
		hover: '#1e2433',
		active: '#252b3b',
		input: '#13171f',
	},
	text: {
		primary: '#f0f2f5',
		secondary: '#a0a8b8',
		tertiary: '#6b7280',
		inverse: '#0f1419',
	},
	trading: {
		profit: '#10b981',
		profitMuted: '#059669',
		profitBg: 'rgba(16, 185, 129, 0.1)',
		loss: '#ef4444',
		lossMuted: '#dc2626',
		lossBg: 'rgba(239, 68, 68, 0.1)',
		neutral: '#6b7280',
	},
	accent: {
		primary: '#3b82f6',
		primaryHover: '#2563eb',
		primaryActive: '#1d4ed8',
		secondary: '#8b5cf6',
	},
	semantic: {
		warning: '#f59e0b',
		warningBg: 'rgba(245, 158, 11, 0.1)',
		info: '#06b6d4',
		infoBg: 'rgba(6, 182, 212, 0.1)',
		danger: '#dc2626',
		dangerBg: 'rgba(220, 38, 38, 0.1)',
		success: '#10b981',
		successBg: 'rgba(16, 185, 129, 0.1)',
	},
	border: {
		primary: '#252b3b',
		secondary: '#1e2433',
		focus: '#3b82f6',
		hover: '#2d3548',
	},
	grade: {
		a: '#fbbf24',
		b: '#3b82f6',
		c: '#f97316',
		d: '#ef4444',
	},
	conviction: {
		1: '#6b7280',
		2: '#60a5fa',
		3: '#34d399',
		4: '#fbbf24',
		5: '#f59e0b',
	},
	score: {
		0: '#ef4444',
		25: '#f97316',
		50: '#fbbf24',
		75: '#84cc16',
		100: '#10b981',
	},
} as const;

// ECharts dark theme configuration
export const echartsTheme = {
	color: [
		colors.accent.primary,
		colors.trading.profit,
		colors.trading.loss,
		colors.accent.secondary,
		colors.semantic.warning,
		colors.semantic.info,
	],
	backgroundColor: 'transparent',
	textStyle: {
		color: colors.text.primary,
		fontFamily: 'Inter, system-ui, sans-serif',
	},
	title: {
		textStyle: {
			color: colors.text.primary,
			fontWeight: 600,
		},
		subtextStyle: {
			color: colors.text.secondary,
		},
	},
	line: {
		itemStyle: {
			borderWidth: 2,
		},
		lineStyle: {
			width: 2,
		},
		symbolSize: 6,
		symbol: 'circle',
		smooth: false,
	},
	radar: {
		itemStyle: {
			borderWidth: 2,
		},
		lineStyle: {
			width: 2,
		},
		symbolSize: 6,
		symbol: 'circle',
		smooth: false,
	},
	bar: {
		itemStyle: {
			barBorderWidth: 0,
			barBorderColor: colors.border.primary,
		},
	},
	pie: {
		itemStyle: {
			borderWidth: 0,
			borderColor: colors.border.primary,
		},
	},
	scatter: {
		itemStyle: {
			borderWidth: 0,
			borderColor: colors.border.primary,
		},
	},
	boxplot: {
		itemStyle: {
			borderWidth: 0,
			borderColor: colors.border.primary,
		},
	},
	parallel: {
		itemStyle: {
			borderWidth: 0,
			borderColor: colors.border.primary,
		},
	},
	sankey: {
		itemStyle: {
			borderWidth: 0,
			borderColor: colors.border.primary,
		},
	},
	funnel: {
		itemStyle: {
			borderWidth: 0,
			borderColor: colors.border.primary,
		},
	},
	gauge: {
		itemStyle: {
			borderWidth: 0,
			borderColor: colors.border.primary,
		},
	},
	candlestick: {
		itemStyle: {
			color: colors.trading.profit,
			color0: colors.trading.loss,
			borderColor: colors.trading.profit,
			borderColor0: colors.trading.loss,
			borderWidth: 1,
		},
	},
	graph: {
		itemStyle: {
			borderWidth: 0,
			borderColor: colors.border.primary,
		},
		lineStyle: {
			width: 1,
			color: colors.border.primary,
		},
		symbolSize: 6,
		symbol: 'circle',
		smooth: false,
		color: [
			colors.accent.primary,
			colors.trading.profit,
			colors.trading.loss,
			colors.accent.secondary,
		],
		label: {
			color: colors.text.primary,
		},
	},
	map: {
		itemStyle: {
			areaColor: colors.bg.secondary,
			borderColor: colors.border.primary,
			borderWidth: 0.5,
		},
		label: {
			color: colors.text.primary,
		},
		emphasis: {
			itemStyle: {
				areaColor: colors.bg.hover,
				borderColor: colors.accent.primary,
				borderWidth: 1,
			},
			label: {
				color: colors.text.primary,
			},
		},
	},
	geo: {
		itemStyle: {
			areaColor: colors.bg.secondary,
			borderColor: colors.border.primary,
			borderWidth: 0.5,
		},
		label: {
			color: colors.text.primary,
		},
		emphasis: {
			itemStyle: {
				areaColor: colors.bg.hover,
				borderColor: colors.accent.primary,
				borderWidth: 1,
			},
			label: {
				color: colors.text.primary,
			},
		},
	},
	categoryAxis: {
		axisLine: {
			show: true,
			lineStyle: {
				color: colors.border.primary,
			},
		},
		axisTick: {
			show: true,
			lineStyle: {
				color: colors.border.primary,
			},
		},
		axisLabel: {
			show: true,
			color: colors.text.secondary,
		},
		splitLine: {
			show: false,
			lineStyle: {
				color: [colors.border.secondary],
			},
		},
		splitArea: {
			show: false,
			areaStyle: {
				color: [colors.bg.secondary],
			},
		},
	},
	valueAxis: {
		axisLine: {
			show: false,
			lineStyle: {
				color: colors.border.primary,
			},
		},
		axisTick: {
			show: false,
			lineStyle: {
				color: colors.border.primary,
			},
		},
		axisLabel: {
			show: true,
			color: colors.text.secondary,
		},
		splitLine: {
			show: true,
			lineStyle: {
				color: [colors.border.secondary],
			},
		},
		splitArea: {
			show: false,
			areaStyle: {
				color: [colors.bg.secondary],
			},
		},
	},
	logAxis: {
		axisLine: {
			show: false,
			lineStyle: {
				color: colors.border.primary,
			},
		},
		axisTick: {
			show: false,
			lineStyle: {
				color: colors.border.primary,
			},
		},
		axisLabel: {
			show: true,
			color: colors.text.secondary,
		},
		splitLine: {
			show: true,
			lineStyle: {
				color: [colors.border.secondary],
			},
		},
		splitArea: {
			show: false,
			areaStyle: {
				color: [colors.bg.secondary],
			},
		},
	},
	timeAxis: {
		axisLine: {
			show: true,
			lineStyle: {
				color: colors.border.primary,
			},
		},
		axisTick: {
			show: true,
			lineStyle: {
				color: colors.border.primary,
			},
		},
		axisLabel: {
			show: true,
			color: colors.text.secondary,
		},
		splitLine: {
			show: false,
			lineStyle: {
				color: [colors.border.secondary],
			},
		},
		splitArea: {
			show: false,
			areaStyle: {
				color: [colors.bg.secondary],
			},
		},
	},
	toolbox: {
		iconStyle: {
			borderColor: colors.text.secondary,
		},
		emphasis: {
			iconStyle: {
				borderColor: colors.text.primary,
			},
		},
	},
	legend: {
		textStyle: {
			color: colors.text.secondary,
		},
	},
	tooltip: {
		backgroundColor: colors.bg.tertiary,
		borderColor: colors.border.primary,
		borderWidth: 1,
		textStyle: {
			color: colors.text.primary,
		},
		axisPointer: {
			lineStyle: {
				color: colors.border.hover,
				width: 1,
			},
			crossStyle: {
				color: colors.border.hover,
				width: 1,
			},
		},
	},
	timeline: {
		lineStyle: {
			color: colors.border.primary,
			width: 1,
		},
		itemStyle: {
			color: colors.accent.primary,
			borderWidth: 1,
		},
		controlStyle: {
			color: colors.accent.primary,
			borderColor: colors.accent.primary,
			borderWidth: 0.5,
		},
		checkpointStyle: {
			color: colors.accent.secondary,
			borderColor: colors.accent.secondary,
		},
		label: {
			color: colors.text.secondary,
		},
		emphasis: {
			itemStyle: {
				color: colors.accent.primaryHover,
			},
			controlStyle: {
				color: colors.accent.primary,
				borderColor: colors.accent.primary,
				borderWidth: 0.5,
			},
			label: {
				color: colors.text.primary,
			},
		},
	},
	visualMap: {
		color: [colors.trading.profit, colors.semantic.warning, colors.trading.loss],
		textStyle: {
			color: colors.text.secondary,
		},
	},
	dataZoom: {
		backgroundColor: colors.bg.secondary,
		dataBackgroundColor: colors.bg.hover,
		fillerColor: 'rgba(59, 130, 246, 0.2)',
		handleColor: colors.accent.primary,
		handleSize: '100%',
		textStyle: {
			color: colors.text.secondary,
		},
		borderColor: colors.border.primary,
	},
	markPoint: {
		label: {
			color: colors.text.primary,
		},
		emphasis: {
			label: {
				color: colors.text.primary,
			},
		},
	},
} as const;

// Lightweight Charts theme configuration
export const lightweightChartsTheme = {
	layout: {
		background: {
			color: 'transparent',
		},
		textColor: colors.text.secondary,
		fontSize: 12,
		fontFamily: 'Inter, system-ui, sans-serif',
	},
	grid: {
		vertLines: {
			color: colors.border.secondary,
			style: 0,
			visible: true,
		},
		horzLines: {
			color: colors.border.secondary,
			style: 0,
			visible: true,
		},
	},
	crosshair: {
		vertLine: {
			color: colors.border.hover,
			width: 1,
			style: 3,
			visible: true,
			labelVisible: true,
			labelBackgroundColor: colors.bg.tertiary,
		},
		horzLine: {
			color: colors.border.hover,
			width: 1,
			style: 3,
			visible: true,
			labelVisible: true,
			labelBackgroundColor: colors.bg.tertiary,
		},
		mode: 1,
	},
	priceScale: {
		borderColor: colors.border.primary,
		textColor: colors.text.secondary,
	},
	timeScale: {
		borderColor: colors.border.primary,
		textColor: colors.text.secondary,
		timeVisible: true,
		secondsVisible: false,
	},
	watermark: {
		color: colors.text.tertiary,
		visible: false,
	},
	candlestickSeries: {
		upColor: colors.trading.profit,
		downColor: colors.trading.loss,
		borderUpColor: colors.trading.profit,
		borderDownColor: colors.trading.loss,
		wickUpColor: colors.trading.profit,
		wickDownColor: colors.trading.loss,
	},
	lineSeries: {
		color: colors.accent.primary,
		lineWidth: 2,
		crosshairMarkerVisible: true,
		crosshairMarkerRadius: 4,
		crosshairMarkerBorderColor: colors.accent.primary,
		crosshairMarkerBackgroundColor: colors.bg.primary,
		lastValueVisible: true,
		priceLineVisible: true,
		priceLineWidth: 1,
		priceLineColor: colors.accent.primary,
		priceLineStyle: 2,
	},
	areaSeries: {
		topColor: 'rgba(59, 130, 246, 0.4)',
		bottomColor: 'rgba(59, 130, 246, 0.0)',
		lineColor: colors.accent.primary,
		lineWidth: 2,
		crosshairMarkerVisible: true,
		crosshairMarkerRadius: 4,
		crosshairMarkerBorderColor: colors.accent.primary,
		crosshairMarkerBackgroundColor: colors.bg.primary,
		lastValueVisible: true,
		priceLineVisible: true,
		priceLineWidth: 1,
		priceLineColor: colors.accent.primary,
		priceLineStyle: 2,
	},
	barSeries: {
		upColor: colors.trading.profit,
		downColor: colors.trading.loss,
		openVisible: true,
		thinBars: true,
	},
	histogramSeries: {
		color: colors.accent.primary,
		base: 0,
	},
} as const;

// Helper function to get score color based on value (0-100)
export function getScoreColor(score: number): string {
	if (score >= 90) return colors.score[100];
	if (score >= 70) return colors.score[75];
	if (score >= 50) return colors.score[50];
	if (score >= 30) return colors.score[25];
	return colors.score[0];
}

// Helper function to get conviction color (1-5)
export function getConvictionColor(conviction: 1 | 2 | 3 | 4 | 5): string {
	return colors.conviction[conviction];
}

// Helper function to get grade color
export function getGradeColor(grade: 'A' | 'B' | 'C' | 'D'): string {
	return colors.grade[grade.toLowerCase() as 'a' | 'b' | 'c' | 'd'];
}

// Helper function to get profit/loss color
export function getPnLColor(value: number): string {
	if (value > 0) return colors.trading.profit;
	if (value < 0) return colors.trading.loss;
	return colors.trading.neutral;
}
