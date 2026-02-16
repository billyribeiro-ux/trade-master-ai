declare global {
	namespace App {
		interface Locals {
			user: {
				id: string;
				email: string;
				displayName: string | null;
			} | null;
		}
		interface PageData {
			user: {
				id: string;
				email: string;
				displayName: string | null;
			} | null;
		}
	}
}

export {};
