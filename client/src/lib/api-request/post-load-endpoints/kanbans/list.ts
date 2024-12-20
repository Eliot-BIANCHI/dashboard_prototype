import { postLoadRequest as request, RequestError, type ApiResponse } from '../../tool';
import { type Result, ok, err } from '$lib/Result';

export interface Lists {
	add: (options: {
		kanbanId: number;
		data: { label: string; kanban_id: number };
	}) => Promise<Result<number, string>>;
	update: (options: {
		kanbanId: number;
		listId: number;
		data: { label: string };
	}) => Promise<Result<null, string>>;
	delete: (options: { kanbanId: number; listId: number }) => Promise<Result<null, string>>;
}

const calendarsEndpoints: Lists = {
	add: async (options) => {
		const endpoint = {
			method: 'POST',
			resource: `/kanbans/${options.kanbanId}/lists`,
			params: {},
			body: { ...options.data }
		};

		const res: Result<ApiResponse<{ listId: number }>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data.listId);
			} else {
				return err("Couldn't add the list");
			}
		}

		return err('INTERNAL SERVER ERROR');
	},

	update: async (options) => {
		const endpoint = {
			method: 'PUT',
			resource: `/kanbans/${options.kanbanId}/lists/${options.listId}`,
			params: {},
			body: { label: options.data.label }
		};

		const res: Result<ApiResponse<null>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't update the list");
			}
		}

		return err('INTERNAL SERVER ERROR');
	},

	delete: async (options) => {
		const endpoint = {
			method: 'DELETE',
			resource: `/kanbans/${options.kanbanId}/lists/${options.listId}`,
			params: {},
			body: null
		};

		const res: Result<ApiResponse<null>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't delete the list");
			}
		}

		return err('INTERNAL SERVER ERROR');
	}
};

export default calendarsEndpoints;
