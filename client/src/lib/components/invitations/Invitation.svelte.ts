type InvitationType = 'calendar' | 'kanban';
type InvitationStatus = 'pending' | 'accepted' | 'rejected';

export interface InvitationT {
	id: number;
	type: InvitationType;
	status: InvitationStatus;
	label: string;
	inviterFirstName: string;
	inviterLastName: string;
}

export default class Invitation {
	id: number;
	type: InvitationType;
	status: InvitationStatus;
	label: string;
	inviterFirstName: string;
	inviterLastName: string;

	constructor({ id, type, status, label, inviterFirstName, inviterLastName }: InvitationT) {
		this.id = id;
		this.type = type;
		this.status = status;
		this.label = label;
		this.inviterFirstName = inviterFirstName;
		this.inviterLastName = inviterLastName;
	}
}
