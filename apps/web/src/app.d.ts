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

interface ImportMetaEnv {
	readonly PUBLIC_API_URL: string;
	readonly PUBLIC_APP_URL: string;
}

interface ImportMeta {
	readonly env: ImportMetaEnv;
}

export {};
