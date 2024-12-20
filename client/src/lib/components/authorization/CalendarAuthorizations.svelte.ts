import { CalendarRolePermission, type CalendarRolePermissionT } from './RolePermission.svelte';
import { CalendarRole, type CalendarRoleT } from './Role.svelte';
import Permission, { type PermissionT } from './Permission.svelte';

interface CalendarAuthorizationT {
	rolesPermissions: CalendarRolePermission[];
	roles: CalendarRole[];
	permissions: Permission[];
}

export default class CalendarAuthorization {
	rolesPermissions: CalendarRolePermission[] = $state([]);
	roles: CalendarRole[] = $state([]);
	permissions: Permission[] = $state([]);

	constructor({ rolesPermissions, roles, permissions }: CalendarAuthorizationT) {
		this.rolesPermissions = rolesPermissions;
		this.roles = roles;
		this.permissions = permissions;
	}

	addRolePermission({ calendarRoleId, permissionId }: CalendarRolePermissionT) {
		const rolePermission = new CalendarRolePermission({ calendarRoleId, permissionId });
		this.rolesPermissions.push(rolePermission);
	}

	deleteRolePermission(calendarRoleId: number, permissionId: number) {
		const index = this.rolesPermissions.findIndex(
			(rolePermission) =>
				rolePermission.calendarRoleId === calendarRoleId &&
				rolePermission.permissionId === permissionId
		);
		this.rolesPermissions.splice(index, 1);
	}

	addRole({ id, label, isDefault }: CalendarRoleT) {
		const role = new CalendarRole({ id, label, isDefault });
		this.roles.push(role);
	}

	deleteRole(roleId: number) {
		const index = this.roles.findIndex((role) => role.id === roleId);
		this.roles.splice(index, 1);
	}

	addPermission({ id, label }: PermissionT) {
		const permission = new Permission({ id, label });
		this.permissions.push(permission);
	}

	deletePermission(permissionId: number) {
		const index = this.permissions.findIndex((permission) => permission.id === permissionId);
		this.permissions.splice(index, 1);
	}
}
