<script lang="ts">
	interface Props {
		title: string;
		description?: string;
		keywords?: string[];
		ogImage?: string;
		ogType?: 'website' | 'article';
		canonical?: string;
		noindex?: boolean;
	}

	let {
		title,
		description = 'The ultimate AI-powered trading journal for serious traders. Track trades, analyze performance, and improve with AI-powered insights.',
		keywords = ['trading journal', 'trade tracker', 'trading analytics', 'AI trading coach', 'trade analysis'],
		ogImage = '/og-image.png',
		ogType = 'website',
		canonical,
		noindex = false
	}: Props = $props();

	const fullTitle = title.includes('TradeMaster') ? title : `${title} | TradeMaster AI`;
	const keywordsString = keywords.join(', ');
</script>

<svelte:head>
	<title>{fullTitle}</title>
	<meta name="description" content={description} />
	<meta name="keywords" content={keywordsString} />
	
	{#if noindex}
		<meta name="robots" content="noindex, nofollow" />
	{:else}
		<meta name="robots" content="index, follow" />
	{/if}
	
	{#if canonical}
		<link rel="canonical" href={canonical} />
	{/if}
	
	<!-- Open Graph / Facebook -->
	<meta property="og:type" content={ogType} />
	<meta property="og:title" content={fullTitle} />
	<meta property="og:description" content={description} />
	<meta property="og:image" content={ogImage} />
	
	<!-- Twitter -->
	<meta name="twitter:card" content="summary_large_image" />
	<meta name="twitter:title" content={fullTitle} />
	<meta name="twitter:description" content={description} />
	<meta name="twitter:image" content={ogImage} />
	
	<!-- Theme Color -->
	<meta name="theme-color" content="#0f172a" />
	<meta name="color-scheme" content="dark light" />
</svelte:head>

