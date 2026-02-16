export function calculateWinRate(wins: number, totalTrades: number): number {
	if (totalTrades === 0) return 0;
	return (wins / totalTrades) * 100;
}

export function calculateProfitFactor(grossProfit: number, grossLoss: number): number {
	if (grossLoss === 0) return grossProfit > 0 ? Infinity : 0;
	return grossProfit / Math.abs(grossLoss);
}

export function calculateExpectancy(
	avgWin: number,
	avgLoss: number,
	winRate: number,
): number {
	const lossRate = 1 - winRate / 100;
	return avgWin * (winRate / 100) - Math.abs(avgLoss) * lossRate;
}
