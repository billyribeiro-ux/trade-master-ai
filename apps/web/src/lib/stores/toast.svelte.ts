interface Toast {
	id: string;
	title?: string;
	description?: string;
	variant?: 'default' | 'destructive' | 'success';
	duration?: number;
}

class ToastStore {
	items = $state<Toast[]>([]);

	add(toast: Omit<Toast, 'id'>) {
		const id = Math.random().toString(36).substring(7);
		this.items = [...this.items, { ...toast, id }];
		return id;
	}

	remove(id: string) {
		this.items = this.items.filter((t) => t.id !== id);
	}

	success(title: string, description?: string) {
		return this.add({ title, description, variant: 'success' });
	}

	error(title: string, description?: string) {
		return this.add({ title, description, variant: 'destructive' });
	}

	info(title: string, description?: string) {
		return this.add({ title, description, variant: 'default' });
	}
}

export const toasts = new ToastStore();
