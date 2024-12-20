import { preLoadRequest as request, RequestError, type LoadFetch, type ApiResponse } from '../tool';
import { type Result, ok, err } from '$lib/Result';
import type { KanbanInvitationT } from '$lib/components/invitations/Invitation.svelte';

export interface Invitations {
	getPending: (loadFetch: LoadFetch) => Promise<
		Result<
			{
				kanbansInvitations: KanbanInvitationT[];
			},
			string
		>
	>;
	getReceived: (loadFetch: LoadFetch) => Promise<
		Result<
			{
				kanbansInvitations: KanbanInvitationT[];
			},
			string
		>
	>;
}

const invitationsEndpoints: Invitations = {
	getPending: async (loadFetch) => {
		const resource = '/invitations';
		const res: Result<
			ApiResponse<{ kanbansInvitations: KanbanInvitationT[] }>,
			RequestError
		> = await request(resource, loadFetch);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't load the invitations");
			}
		}

		return err('INTERNAL SERVER ERROR');
	},

	getReceived: async (loadFetch) => {
		const resource = '/invitations/received';
		const res: Result<
			ApiResponse<{ kanbansInvitations: KanbanInvitationT[] }>,
			RequestError
		> = await request(resource, loadFetch);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't load the invitations");
			}
		}

		return err('INTERNAL SERVER ERROR');
	}
};

export default invitationsEndpoints;
