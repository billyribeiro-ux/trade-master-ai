<script lang="ts">
	import { goto } from '$app/navigation';
	import { apiClient } from '$lib/api';
	import Button from '$lib/components/ui/button.svelte';
	import Card from '$lib/components/ui/card.svelte';
	import CardHeader from '$lib/components/ui/card-header.svelte';
	import CardTitle from '$lib/components/ui/card-title.svelte';
	import CardDescription from '$lib/components/ui/card-description.svelte';
	import CardContent from '$lib/components/ui/card-content.svelte';
	import CardFooter from '$lib/components/ui/card-footer.svelte';
	import { toasts } from '$lib/stores/toast.svelte';

	let file = $state<File | null>(null);
	let loading = $state(false);
	let csvData = $state<string[][]>([]);
	let headers = $state<string[]>([]);
	let importResult = $state<{ success_count: number; error_count: number; errors: any[] } | null>(null);

	function handleFileChange(e: Event) {
		const input = e.target as HTMLInputElement;
		if (input.files && input.files[0]) {
			file = input.files[0];
			readCsvFile(input.files[0]);
		}
	}

	function readCsvFile(file: File) {
		const reader = new FileReader();
		reader.onload = (e) => {
			const text = e.target?.result as string;
			const lines = text.split('\n').filter(line => line.trim());
			
			if (lines.length === 0) {
				toasts.error('Empty CSV file');
				return;
			}

			headers = lines[0].split(',').map(h => h.trim());
			csvData = lines.slice(1).map(line => {
				return line.split(',').map(cell => cell.trim());
			});

			toasts.success(`Loaded ${csvData.length} trades from CSV`);
		};
		reader.readAsText(file);
	}

	async function handleImport() {
		if (csvData.length === 0) {
			toasts.error('No data to import');
			return;
		}

		loading = true;
		importResult = null;

		try {
			const trades = csvData.map(row => {
				const trade: any = {};
				headers.forEach((header, index) => {
					trade[header] = row[index] || null;
				});
				return trade;
			});

			const response = await apiClient.post<any>('/api/v1/csv/import', { trades });
			importResult = response;

			if (response.error_count === 0) {
				toasts.success(`Successfully imported ${response.success_count} trades!`);
				setTimeout(() => goto('/trades'), 2000);
			} else {
				toasts.error(
					`Imported ${response.success_count} trades`,
					`${response.error_count} trades failed`
				);
			}
		} catch (error) {
			toasts.error('Import failed', error instanceof Error ? error.message : 'Please try again');
		} finally {
			loading = false;
		}
	}

	async function downloadTemplate() {
		try {
			const response = await apiClient.get<{ headers: string[]; example_rows: string[][] }>(
				'/api/v1/csv/template'
			);

			const csvContent = [
				response.headers.join(','),
				...response.example_rows.map(row => row.join(','))
			].join('\n');

			const blob = new Blob([csvContent], { type: 'text/csv' });
			const url = window.URL.createObjectURL(blob);
			const a = document.createElement('a');
			a.href = url;
			a.download = 'trademaster-import-template.csv';
			a.click();
			window.URL.revokeObjectURL(url);

			toasts.success('Template downloaded');
		} catch (error) {
			toasts.error('Failed to download template');
		}
	}
</script>

<svelte:head>
	<title>Import Trades - TradeMaster AI</title>
</svelte:head>

<div class="p-6 max-w-4xl mx-auto space-y-6">
	<div class="flex items-center justify-between">
		<div>
			<h1 class="text-3xl font-bold">Import Trades from CSV</h1>
			<p class="text-muted-foreground mt-1">Upload your trading history in CSV format</p>
		</div>
		<Button variant="outline" onclick={downloadTemplate}>
			{#snippet children()}
				Download Template
			{/snippet}
		</Button>
	</div>

	<Card>
		<CardHeader>
			<CardTitle>Upload CSV File</CardTitle>
			<CardDescription>
				Select a CSV file containing your trade data. Make sure it follows the template format.
			</CardDescription>
		</CardHeader>
		<CardContent class="space-y-4">
			<div class="border-2 border-dashed rounded-lg p-8 text-center">
				<input
					type="file"
					accept=".csv"
					onchange={handleFileChange}
					class="hidden"
					id="csv-upload"
				/>
				<label for="csv-upload" class="cursor-pointer">
					<div class="space-y-2">
						<div class="text-4xl">📄</div>
						<div class="text-sm font-medium">
							{file ? file.name : 'Click to upload CSV file'}
						</div>
						<div class="text-xs text-muted-foreground">
							CSV files only
						</div>
					</div>
				</label>
			</div>

			{#if csvData.length > 0}
				<div class="rounded-lg bg-muted p-4">
					<p class="text-sm font-medium mb-2">Preview:</p>
					<div class="overflow-x-auto">
						<table class="w-full text-xs">
							<thead>
								<tr class="border-b">
									{#each headers.slice(0, 6) as header}
										<th class="px-2 py-1 text-left font-medium">{header}</th>
									{/each}
									{#if headers.length > 6}
										<th class="px-2 py-1 text-left font-medium">...</th>
									{/if}
								</tr>
							</thead>
							<tbody>
								{#each csvData.slice(0, 5) as row}
									<tr class="border-b">
										{#each row.slice(0, 6) as cell}
											<td class="px-2 py-1">{cell}</td>
										{/each}
										{#if row.length > 6}
											<td class="px-2 py-1">...</td>
										{/if}
									</tr>
								{/each}
							</tbody>
						</table>
					</div>
					<p class="text-xs text-muted-foreground mt-2">
						Showing first 5 of {csvData.length} trades
					</p>
				</div>
			{/if}
		</CardContent>
		<CardFooter class="flex justify-between">
			<Button variant="outline" onclick={() => goto('/trades')}>
				{#snippet children()}
					Cancel
				{/snippet}
			</Button>
			<Button onclick={handleImport} disabled={loading || csvData.length === 0}>
				{#snippet children()}
					{loading ? 'Importing...' : `Import ${csvData.length} Trades`}
				{/snippet}
			</Button>
		</CardFooter>
	</Card>

	{#if importResult}
		<Card>
			<CardHeader>
				<CardTitle>Import Results</CardTitle>
			</CardHeader>
			<CardContent class="space-y-4">
				<div class="grid gap-4 md:grid-cols-2">
					<div class="rounded-lg bg-success/10 p-4">
						<div class="text-sm text-muted-foreground">Successfully Imported</div>
						<div class="text-2xl font-bold text-success">{importResult.success_count}</div>
					</div>
					<div class="rounded-lg bg-destructive/10 p-4">
						<div class="text-sm text-muted-foreground">Failed</div>
						<div class="text-2xl font-bold text-destructive">{importResult.error_count}</div>
					</div>
				</div>

				{#if importResult.errors.length > 0}
					<div>
						<p class="text-sm font-medium mb-2">Errors:</p>
						<div class="space-y-2 max-h-60 overflow-y-auto">
							{#each importResult.errors as error}
								<div class="rounded-lg bg-destructive/10 p-3 text-sm">
									<p class="font-medium">Row {error.row}: {error.symbol}</p>
									<p class="text-xs text-muted-foreground">{error.error}</p>
								</div>
							{/each}
						</div>
					</div>
				{/if}
			</CardContent>
		</Card>
	{/if}

	<Card>
		<CardHeader>
			<CardTitle>CSV Format Requirements</CardTitle>
		</CardHeader>
		<CardContent class="space-y-3 text-sm">
			<div>
				<p class="font-medium mb-1">Required Fields:</p>
				<ul class="list-disc list-inside text-muted-foreground space-y-1">
					<li><code class="bg-muted px-1 rounded">symbol</code> - Ticker symbol (e.g., AAPL, TSLA)</li>
					<li><code class="bg-muted px-1 rounded">direction</code> - long or short</li>
					<li><code class="bg-muted px-1 rounded">asset_class</code> - stocks, options, futures, forex, or crypto</li>
					<li><code class="bg-muted px-1 rounded">entry_date</code> - YYYY-MM-DD HH:MM:SS or YYYY-MM-DD</li>
					<li><code class="bg-muted px-1 rounded">entry_price</code> - Numeric value</li>
					<li><code class="bg-muted px-1 rounded">quantity</code> - Numeric value</li>
				</ul>
			</div>
			<div>
				<p class="font-medium mb-1">Optional Fields:</p>
				<ul class="list-disc list-inside text-muted-foreground space-y-1">
					<li><code class="bg-muted px-1 rounded">exit_date</code>, <code class="bg-muted px-1 rounded">exit_price</code> - For closed trades</li>
					<li><code class="bg-muted px-1 rounded">stop_loss</code>, <code class="bg-muted px-1 rounded">take_profit</code> - Risk management</li>
					<li><code class="bg-muted px-1 rounded">setup_name</code>, <code class="bg-muted px-1 rounded">timeframe</code>, <code class="bg-muted px-1 rounded">conviction</code></li>
					<li><code class="bg-muted px-1 rounded">thesis</code>, <code class="bg-muted px-1 rounded">commissions</code></li>
				</ul>
			</div>
		</CardContent>
	</Card>
</div>
