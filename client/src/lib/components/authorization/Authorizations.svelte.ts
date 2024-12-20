import { RolePermission, type RolePermissionT } from './RolePermission.svelte';
import { Role, type RoleT } from './Role.svelte';
import Permission, { type PermissionT } from './Permission.svelte';

interface AuthorizationT {
	rolesPermissions: RolePermission[];
	roles: Role[];
	permissions: Permission[];
}

export default class Authorization {
	rolesPermissions: RolePermission[] = $state([]);
	roles: Role[] = $state([]);
	permissions: Permission[] = $state([]);

	constructor({ rolesPermissions, roles, permissions }: AuthorizationT) {
		this.rolesPermissions = rolesPermissions;
		this.roles = roles;
		this.permissions = permissions;
	}

	addRolePermission({ permissionId, roleId }: RolePermissionT) {
		const rolePermission = new RolePermission({ permissionId, roleId });
		this.rolesPermissions.push(rolePermission);
	}

	deleteRolePermission(roleId: number, permissionId: number) {
		const index = this.rolesPermissions.findIndex(
			(rolePermission) =>
				rolePermission.roleId === roleId && rolePermission.permissionId === permissionId
		);
		this.rolesPermissions.splice(index, 1);
	}

	addRole({ id, label }: RoleT) {
		const role = new Role({ id, label });
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
