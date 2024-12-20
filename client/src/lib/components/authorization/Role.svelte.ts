export interface RoleT {
	id: number;
	label: string;
}

export class Role {
	id: number;
	label = $state('');

	constructor({ id, label }: RoleT) {
		this.id = id;
		this.label = label;
	}

	update({ label }: Omit<RoleT, 'id'>) {
		this.label = label;
	}
}

export interface CalendarRoleT {
	id: number;
	label: string;
	isDefault: boolean;
}

export class CalendarRole {
	id: number;
	label = $state('');
	isDefault = $state(false);

	constructor({ id, label, isDefault }: CalendarRoleT) {
		this.id = id;
		this.label = label;
		this.isDefault = isDefault;
	}

	update({ label, isDefault }: Omit<CalendarRoleT, 'id'>) {
		this.label = label;
		this.isDefault = isDefault;
	}
}

export interface KanbanRoleT {
	id: number;
	label: string;
	isDefault: boolean;
}

export class KanbanRole {
	id: number;
	label = $state('');
	isDefault = $state(false);

	constructor({ id, label, isDefault }: KanbanRoleT) {
		this.id = id;
		this.label = label;
		this.isDefault = isDefault;
	}

	update({ label, isDefault }: Omit<KanbanRoleT, 'id'>) {
		this.label = label;
		this.isDefault = isDefault;
	}
}
