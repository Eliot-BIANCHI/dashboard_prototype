import { postLoadRequest as request, RequestError, type ApiResponse } from '../../tool';
import { type Result, ok, err } from '$lib/Result';

interface AddData {
	label: string;
	completion_id: number;
	priority: boolean;
	list_id: number;
}

interface UpdateData {
	label: string;
	completion_id: number;
	priority: boolean;
}

export interface Tasks {
	add: (options: {
		kanbanId: number;
		listId: number;
		data: AddData;
	}) => Promise<Result<number, string>>;
	update: (options: {
		kanbanId: number;
		listId: number;
		taskId: number;
		data: UpdateData;
	}) => Promise<Result<null, string>>;
	delete: (options: {
		kanbanId: number;
		listId: number;
		taskId: number;
	}) => Promise<Result<null, string>>;
	patchCompletion: (options: {
		kanbanId: number;
		listId: number;
		taskId: number;
		data: { completion_id: number };
	}) => Promise<Result<null, string>>;
	patchList: (options: {
		kanbanId: number;
		listId: number;
		taskId: number;
		data: { list_id: number };
	}) => Promise<Result<null, string>>;
	assignUser: (options: {
		kanbanId: number;
		listId: number;
		taskId: number;
		data: { user_id: number };
	}) => Promise<Result<null, string>>;
	unassignUser: (options: {
		kanbanId: number;
		listId: number;
		taskId: number;
		data: { user_id: number };
	}) => Promise<Result<null, string>>;
}

const tasksEndpoints: Tasks = {
	add: async (options) => {
		const endpoint = {
			method: 'POST',
			resource: `/kanbans/${options.kanbanId}/lists/${options.listId}/tasks`,
			params: {},
			body: { ...options.data }
		};

		const res: Result<ApiResponse<{ taskId: number }>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data.taskId);
			} else {
				return err("Couldn't add the task");
			}
		}

		return err('INTERNAL SERVER ERROR');
	},

	update: async (options) => {
		const endpoint = {
			method: 'PUT',
			resource: `/kanbans/${options.kanbanId}/lists/${options.listId}/tasks/${options.taskId}`,
			params: {},
			body: { ...options.data }
		};

		const res: Result<ApiResponse<null>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't update the task");
			}
		}

		return err('INTERNAL SERVER ERROR');
	},

	delete: async (options) => {
		const endpoint = {
			method: 'DELETE',
			resource: `/kanbans/${options.kanbanId}/lists/${options.listId}/tasks/${options.taskId}`,
			params: {},
			body: null
		};

		const res: Result<ApiResponse<null>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't delete the task");
			}
		}

		return err('INTERNAL SERVER ERROR');
	},

	patchList: async (options) => {
		const endpoint = {
			method: 'PATCH',
			resource: `/kanbans/${options.kanbanId}/lists/${options.listId}/tasks/${options.taskId}/list`,
			params: {},
			body: { list_id: options.data.list_id }
		};

		const res: Result<ApiResponse<null>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't patch the task list");
			}
		}

		return err('INTERNAL SERVER ERROR');
	},

	patchCompletion: async (options) => {
		const endpoint = {
			method: 'PATCH',
			resource: `/kanbans/${options.kanbanId}/lists/${options.listId}/tasks/${options.taskId}/completion`,
			params: {},
			body: { completion_id: options.data.completion_id }
		};

		const res: Result<ApiResponse<null>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't patch the task completion");
			}
		}

		return err('INTERNAL SERVER ERROR');
	},

	assignUser: async (options) => {
		const endpoint = {
			method: 'POST',
			resource: `/kanbans/${options.kanbanId}/lists/${options.listId}/tasks/${options.taskId}/assign-user`,
			params: {},
			body: { user_id: options.data.user_id }
		};

		const res: Result<ApiResponse<null>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't assign the user to the task");
			}
		}

		return err('INTERNAL SERVER ERROR');
	},

	unassignUser: async (options) => {
		const endpoint = {
			method: 'POST',
			resource: `/kanbans/${options.kanbanId}/lists/${options.listId}/tasks/${options.taskId}/unassign-user`,
			params: {},
			body: { user_id: options.data.user_id }
		};

		const res: Result<ApiResponse<null>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't unassign the user to the task");
			}
		}

		return err('INTERNAL SERVER ERROR');
	}
};

export default tasksEndpoints;
