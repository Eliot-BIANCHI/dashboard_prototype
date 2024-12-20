export interface PermissionT {
	id: number;
	label: string;
}

export default class Permission {
	id: number;
	label = $state('');

	constructor({ id, label }: PermissionT) {
		this.id = id;
		this.label = label;
	}

	update({ label }: Omit<PermissionT, 'id'>) {
		this.label = label;
	}
}
