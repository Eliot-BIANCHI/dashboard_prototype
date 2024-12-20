import { postLoadRequest as request, RequestError, type ApiResponse } from '../../tool';
import { type Result, ok, err } from '$lib/Result';

export interface RolesPermissions {
	add: (options: { roleId: number; permissionId: number }) => Promise<Result<null, string>>;
	delete: (options: { roleId: number; permissionId: number }) => Promise<Result<null, string>>;
	addKanban: (options: {
		kanbanRoleId: number;
		permissionId: number;
	}) => Promise<Result<null, string>>;
	deleteKanban: (options: {
		kanbanRoleId: number;
		permissionId: number;
	}) => Promise<Result<null, string>>;
	addCalendar: (options: {
		calendarRoleId: number;
		permissionId: number;
	}) => Promise<Result<null, string>>;
	deleteCalendar: (options: {
		calendarRoleId: number;
		permissionId: number;
	}) => Promise<Result<null, string>>;
}

const rolesPermissionsEndpoints: RolesPermissions = {
	add: async (options) => {
		const endpoint = {
			method: 'POST',
			resource: `/roles-permissions/${options.roleId}/${options.permissionId}`,
			params: {},
			body: null
		};

		const res: Result<ApiResponse<null>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't add the role permission");
			}
		}

		return err('INTERNAL SERVER ERROR');
	},
	delete: async (options) => {
		const endpoint = {
			method: 'DELETE',
			resource: `/roles-permissions/${options.roleId}/${options.permissionId}`,
			params: {},
			body: null
		};

		const res: Result<ApiResponse<null>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't delete the role permission");
			}
		}

		return err('INTERNAL SERVER ERROR');
	},
	addKanban: async (options) => {
		const endpoint = {
			method: 'POST',
			resource: `/roles-permissions/kanbans/${options.kanbanRoleId}/${options.permissionId}`,
			params: {},
			body: null
		};

		const res: Result<ApiResponse<null>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't add the role permission");
			}
		}

		return err('INTERNAL SERVER ERROR');
	},
	deleteKanban: async (options) => {
		const endpoint = {
			method: 'DELETE',
			resource: `/roles-permissions/kanbans/${options.kanbanRoleId}/${options.permissionId}`,
			params: {},
			body: null
		};

		const res: Result<ApiResponse<null>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't delete the role permission");
			}
		}

		return err('INTERNAL SERVER ERROR');
	},
	addCalendar: async (options) => {
		const endpoint = {
			method: 'POST',
			resource: `/roles-permissions/calendars/${options.calendarRoleId}/${options.permissionId}`,
			params: {},
			body: null
		};

		const res: Result<ApiResponse<null>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't add the role permission");
			}
		}

		return err('INTERNAL SERVER ERROR');
	},
	deleteCalendar: async (options) => {
		const endpoint = {
			method: 'DELETE',
			resource: `/roles-permissions/calendars/${options.calendarRoleId}/${options.permissionId}`,
			params: {},
			body: null
		};

		const res: Result<ApiResponse<null>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't delete the role permission");
			}
		}

		return err('INTERNAL SERVER ERROR');
	}
};

export default rolesPermissionsEndpoints;
