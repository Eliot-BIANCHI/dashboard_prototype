import { apiPreLoadRequest as apiRequest } from '$lib/api-request/ApiRequest';
// import PendingInvitations, {
// 	KanbanInvitation
// } from '$lib/components/invitations/Invitation.svelte';

import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, parent }) => {
	await parent();

	const res = await apiRequest.invitations.getPending(fetch);

	if (res.type === 'err') {
		throw new Error('Failed to fetch home data');
	}

	// const pendingInvitations = new PendingInvitations({
	// 	kanbansInvitations: res.value.kanbansInvitations.map(
	// 		({ id, inviterFirstName, inviterLastName, kanbanLabel }) =>
	// 			new KanbanInvitation({
	// 				id,
	// 				inviterFirstName,
	// 				inviterLastName,
	// 				kanbanLabel
	// 			})
	// 	)
	// });

	// return {
	// 	pendingInvitations
	// };
};
