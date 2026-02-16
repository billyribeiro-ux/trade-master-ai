export interface Tag {
	id: string;
	user_id?: string;
	name: string;
	color: string;
	category?: string;
	is_system: boolean;
	created_at: string;
	updated_at: string;
}

export interface CreateTagRequest {
	name: string;
	color: string;
	category?: string;
}

export interface UpdateTagRequest {
	name?: string;
	color?: string;
	category?: string;
}

export interface TagWithCount extends Tag {
	trade_count: number;
}
