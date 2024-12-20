export interface RolePermissionT {
	roleId: number;
	permissionId: number;
}

export class RolePermission {
	roleId: number;
	permissionId: number;

	constructor({ roleId, permissionId }: RolePermissionT) {
		this.roleId = roleId;
		this.permissionId = permissionId;
	}
}

export interface CalendarRolePermissionT {
	calendarRoleId: number;
	permissionId: number;
}

export class CalendarRolePermission {
	calendarRoleId: number;
	permissionId: number;

	constructor({ calendarRoleId, permissionId }: CalendarRolePermissionT) {
		this.calendarRoleId = calendarRoleId;
		this.permissionId = permissionId;
	}
}

export interface KanbanRolePermissionT {
	kanbanRoleId: number;
	permissionId: number;
}

export class KanbanRolePermission {
	kanbanRoleId: number;
	permissionId: number;

	constructor({ kanbanRoleId, permissionId }: KanbanRolePermissionT) {
		this.kanbanRoleId = kanbanRoleId;
		this.permissionId = permissionId;
	}
}
