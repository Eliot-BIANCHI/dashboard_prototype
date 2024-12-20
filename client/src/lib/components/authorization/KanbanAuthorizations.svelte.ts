import { KanbanRolePermission, type KanbanRolePermissionT } from './RolePermission.svelte';
import { KanbanRole, type KanbanRoleT } from './Role.svelte';
import Permission, { type PermissionT } from './Permission.svelte';

interface KanbanAuthorizationT {
	rolesPermissions: KanbanRolePermission[];
	roles: KanbanRole[];
	permissions: Permission[];
}

export default class KanbanAuthorization {
	rolesPermissions: KanbanRolePermission[] = $state([]);
	roles: KanbanRole[] = $state([]);
	permissions: Permission[] = $state([]);

	constructor({ rolesPermissions, roles, permissions }: KanbanAuthorizationT) {
		this.rolesPermissions = rolesPermissions;
		this.roles = roles;
		this.permissions = permissions;
	}

	addRolePermission({ kanbanRoleId, permissionId }: KanbanRolePermissionT) {
		const rolePermission = new KanbanRolePermission({ kanbanRoleId, permissionId });
		this.rolesPermissions.push(rolePermission);
	}

	deleteRolePermission(kanbanRoleId: number, permissionId: number) {
		const index = this.rolesPermissions.findIndex(
			(rolePermission) =>
				rolePermission.kanbanRoleId === kanbanRoleId && rolePermission.permissionId === permissionId
		);
		this.rolesPermissions.splice(index, 1);
	}

	addRole({ id, label, isDefault }: KanbanRoleT) {
		const role = new KanbanRole({ id, label, isDefault });
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
