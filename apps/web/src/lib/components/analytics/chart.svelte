<script lang="ts">
	import { onMount } from 'svelte';
	import type { EChartsOption, ECharts } from 'echarts';

	interface Props {
		options: EChartsOption;
		height?: string;
		class?: string;
	}

	let { options, height = '320px', class: className = '' }: Props = $props();

	let container: HTMLDivElement;
	let chart: ECharts | null = null;
	let observer: ResizeObserver | null = null;

	onMount(() => {
		import('echarts').then((echarts) => {
			chart = echarts.init(container, 'dark');
			chart.setOption(options);

			observer = new ResizeObserver(() => chart?.resize());
			observer.observe(container);
		});

		return () => {
			observer?.disconnect();
			chart?.dispose();
		};
	});

	$effect(() => {
		if (chart && options) {
			chart.setOption(options, true);
		}
	});
</script>

<div bind:this={container} style="height: {height}; width: 100%;" class={className}></div>
