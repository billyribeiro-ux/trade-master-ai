<script lang="ts">
	import Chart from '$lib/components/analytics/chart.svelte';
	import type { EChartsOption } from 'echarts';

	interface MoodLog {
		log_date: string;
		pre_market_mood: number | null;
		post_market_mood: number | null;
		stress_level: number | null;
		confidence_level: number | null;
	}

	interface Props {
		logs: MoodLog[];
	}

	let { logs }: Props = $props();

	function buildOptions(): EChartsOption {
		const sorted = [...logs].reverse();
		const dates = sorted.map(l => l.log_date);
		const preMood = sorted.map(l => l.pre_market_mood);
		const postMood = sorted.map(l => l.post_market_mood);
		const stress = sorted.map(l => l.stress_level);
		const confidence = sorted.map(l => l.confidence_level);

		return {
			backgroundColor: 'transparent',
			tooltip: {
				trigger: 'axis',
				backgroundColor: 'rgba(30,30,30,0.95)',
				borderColor: '#444',
				textStyle: { color: '#eee', fontSize: 12 },
			},
			legend: {
				data: ['Pre-Market Mood', 'Post-Market Mood', 'Stress', 'Confidence'],
				textStyle: { color: '#999' },
				top: 0
			},
			grid: { left: 40, right: 20, top: 40, bottom: 30 },
			xAxis: {
				type: 'category',
				data: dates,
				axisLabel: { color: '#888', fontSize: 10 },
				axisLine: { lineStyle: { color: '#333' } },
			},
			yAxis: {
				type: 'value',
				min: 0,
				max: 10,
				axisLabel: { color: '#888' },
				splitLine: { lineStyle: { color: '#222' } }
			},
			series: [
				{
					name: 'Pre-Market Mood',
					type: 'line',
					data: preMood,
					smooth: true,
					symbol: 'circle',
					symbolSize: 6,
					lineStyle: { width: 2, color: '#3b82f6' },
					itemStyle: { color: '#3b82f6' }
				},
				{
					name: 'Post-Market Mood',
					type: 'line',
					data: postMood,
					smooth: true,
					symbol: 'circle',
					symbolSize: 6,
					lineStyle: { width: 2, color: '#8b5cf6' },
					itemStyle: { color: '#8b5cf6' }
				},
				{
					name: 'Stress',
					type: 'line',
					data: stress,
					smooth: true,
					symbol: 'diamond',
					symbolSize: 6,
					lineStyle: { width: 2, color: '#ef4444', type: 'dashed' },
					itemStyle: { color: '#ef4444' }
				},
				{
					name: 'Confidence',
					type: 'line',
					data: confidence,
					smooth: true,
					symbol: 'triangle',
					symbolSize: 6,
					lineStyle: { width: 2, color: '#22c55e' },
					itemStyle: { color: '#22c55e' }
				}
			]
		};
	}

	const options = $derived(buildOptions());
</script>

<Chart {options} height="300px" />
