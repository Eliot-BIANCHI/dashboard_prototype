import Invitation from './Invitation.svelte';

interface InvitationsT {
	kanbans: Invitation[];
}

export default class Invitations {
	kanbans: Invitation[] = $state([]);

	constructor({ kanbans }: InvitationsT) {
		this.kanbans = kanbans;
	}

	deletePendingKanbanInvitation(invitationId: number) {
		const index = this.kanbans.findLastIndex((invitation) => invitation.id === invitationId);
		this.kanbans.splice(index, 1);
	}
}
