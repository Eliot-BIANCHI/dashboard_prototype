import { preLoadRequest as request, RequestError, type LoadFetch, type ApiResponse } from '../tool';
import { type Result, ok, err } from '$lib/Result';
import type {
	RolePermissionT,
	KanbanRolePermissionT,
	CalendarRolePermissionT
} from '$lib/components/authorization/RolePermission.svelte';
import type { PermissionT } from '$lib/components/authorization/Permission.svelte';
import type { RoleT, KanbanRoleT, CalendarRoleT } from '$lib/components/authorization/Role.svelte';

export interface RolesPermissions {
	get: (loadFetch: LoadFetch) => Promise<
		Result<
			{
				roles: RoleT[];
				permissions: PermissionT[];
				rolesPermissions: RolePermissionT[];
			},
			string
		>
	>;
	getCalendars: (loadFetch: LoadFetch) => Promise<
		Result<
			{
				calendarRoles: CalendarRoleT[];
				calendarPermissions: PermissionT[];
				calendarRolesPermissions: CalendarRolePermissionT[];
			},
			string
		>
	>;
	getKanbans: (loadFetch: LoadFetch) => Promise<
		Result<
			{
				kanbanRoles: KanbanRoleT[];
				kanbanPermissions: PermissionT[];
				kanbanRolesPermissions: KanbanRolePermissionT[];
			},
			string
		>
	>;
}

const rolesPermissionsEndpoints: RolesPermissions = {
	get: async (loadFetch) => {
		const resource = '/roles-permissions';
		const res: Result<
			ApiResponse<{
				roles: RoleT[];
				permissions: PermissionT[];
				rolesPermissions: RolePermissionT[];
			}>,
			RequestError
		> = await request(resource, loadFetch);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't load the roles permissions");
			}
		}

		return err('INTERNAL SERVER ERROR');
	},

	getCalendars: async (loadFetch) => {
		const resource = '/roles-permissions/calendars';
		const res: Result<
			ApiResponse<{
				calendarRoles: CalendarRoleT[];
				calendarPermissions: PermissionT[];
				calendarRolesPermissions: CalendarRolePermissionT[];
			}>,
			RequestError
		> = await request(resource, loadFetch);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't load the calendar roles permissions");
			}
		}

		return err('INTERNAL SERVER ERROR');
	},

	getKanbans: async (loadFetch) => {
		const resource = '/roles-permissions/kanbans';
		const res: Result<
			ApiResponse<{
				kanbanRoles: KanbanRoleT[];
				kanbanPermissions: PermissionT[];
				kanbanRolesPermissions: KanbanRolePermissionT[];
			}>,
			RequestError
		> = await request(resource, loadFetch);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't load the kanban roles permissions");
			}
		}

		return err('INTERNAL SERVER ERROR');
	}
};

export default rolesPermissionsEndpoints;
