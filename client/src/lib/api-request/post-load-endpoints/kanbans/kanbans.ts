import { postLoadRequest as request, RequestError, type ApiResponse } from '../../tool';
import { type Result, ok, err } from '$lib/Result';

export interface Kanbans {
	add: (options: {
		data: { label: string; is_shared: boolean };
	}) => Promise<Result<number, string>>;
	update: (options: { kanbanId: number; data: { label: string } }) => Promise<Result<null, string>>;
	delete: (options: { kanbanId: number }) => Promise<Result<null, string>>;
	inviteUser: (options: {
		data: { kanban_id: number; invitee_id: number; kanban_role_id: number };
	}) => Promise<Result<null, string>>;
	removeUser: (options: { kanbanId: number; userId: number }) => Promise<Result<null, string>>;
}

const kanbansEndpoints: Kanbans = {
	add: async (options) => {
		const endpoint = {
			method: 'POST',
			resource: '/kanbans',
			params: {},
			body: { ...options.data }
		};

		const res: Result<ApiResponse<{ kanbanId: number }>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data.kanbanId);
			} else {
				return err("Couldn't add the kanban");
			}
		}

		return err('INTERNAL SERVER ERROR');
	},

	update: async (options) => {
		const endpoint = {
			method: 'PUT',
			resource: `/kanbans/${options.kanbanId}`,
			params: {},
			body: { ...options.data }
		};

		const res: Result<ApiResponse<null>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't update the kanban");
			}
		}

		return err('INTERNAL SERVER ERROR');
	},

	delete: async (options) => {
		const endpoint = {
			method: 'DELETE',
			resource: `/kanbans/${options.kanbanId}`,
			params: {},
			body: null
		};

		const res: Result<ApiResponse<null>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't delete the kanban");
			}
		}

		return err('INTERNAL SERVER ERROR');
	},

	inviteUser: async (options) => {
		const endpoint = {
			method: 'POST',
			resource: `/invitations/kanbans`,
			params: {},
			body: { ...options.data }
		};

		const res: Result<ApiResponse<null>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't add the user to the kanban");
			}
		}

		return err('INTERNAL SERVER ERROR');
	},

	removeUser: async (options) => {
		const endpoint = {
			method: 'DELETE',
			resource: `/kanbans/${options.kanbanId}/shared/${options.userId}`,
			params: {},
			body: null
		};

		const res: Result<ApiResponse<null>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't delete the user of the kanban");
			}
		}

		return err('INTERNAL SERVER ERROR');
	}
};

export default kanbansEndpoints;
