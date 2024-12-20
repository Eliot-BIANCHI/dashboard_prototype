import { postLoadRequest as request, RequestError, type ApiResponse } from '../../tool';
import type { RoleT } from '$lib/components/authorization/Role.svelte';
import { type Result, ok, err } from '$lib/Result';

export interface Roles {
	get: () => Promise<RoleT[]>;
	getKanbans: () => Promise<RoleT[]>;
	add: (options: { data: { label: string } }) => Promise<Result<number, string>>;
}

const rolesEndpoints: Roles = {
	get: async () => {
		const endpoint = {
			method: 'GET',
			resource: '/roles',
			params: {},
			body: null
		};

		const res: Result<ApiResponse<{ roles: RoleT[] }>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return res.value.data.roles;
			} else {
				throw new Error("Couldn't get the roles");
			}
		}

		throw new Error("Couldn't get the roles");
	},

	getKanbans: async () => {
		const endpoint = {
			method: 'GET',
			resource: '/roles/kanbans',
			params: {},
			body: null
		};

		const res: Result<ApiResponse<{ kanbanRoles: RoleT[] }>, RequestError> = await request(
			endpoint
		);

		if (res.type === 'ok') {
			if (res.value.success) {
				return res.value.data.kanbanRoles;
			} else {
				throw new Error("Couldn't get the roles");
			}
		}

		throw new Error("Couldn't get the roles");
	},

	add: async (options) => {
		const endpoint = {
			method: 'POST',
			resource: '/roles',
			params: {},
			body: { label: options.data.label }
		};

		const res: Result<ApiResponse<{ roleId: number }>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data.roleId);
			} else {
				return err("Couldn't add the role");
			}
		}

		return err('INTERNAL SERVER ERROR');
	}
};

export default rolesEndpoints;
