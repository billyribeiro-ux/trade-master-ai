export function calculateRMultiple(
	entryPrice: number,
	exitPrice: number,
	stopLoss: number,
	direction: 'long' | 'short',
): number {
	const risk = direction === 'long' ? entryPrice - stopLoss : stopLoss - entryPrice;
	if (risk <= 0) return 0;

	const pnl = direction === 'long' ? exitPrice - entryPrice : entryPrice - exitPrice;
	return pnl / risk;
}

export function calculatePositionSize(
	accountSize: number,
	riskPercentage: number,
	entryPrice: number,
	stopLoss: number,
): number {
	const riskAmount = accountSize * (riskPercentage / 100);
	const priceRisk = Math.abs(entryPrice - stopLoss);
	if (priceRisk === 0) return 0;

	return riskAmount / priceRisk;
}
