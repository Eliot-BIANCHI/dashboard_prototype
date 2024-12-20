import { apiPreLoadRequest as apiRequest } from '$lib/api-request/ApiRequest';
import Kanban from '$lib/components/kanbans/Kanban.svelte.js';
import { extractLists } from '$lib/components/kanbans/tools.js';

import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, fetch, parent }) => {
	await parent();

	const { id } = params;

	const kanbanId = parseInt(id);

	const res = await apiRequest.kanbans.get(kanbanId, fetch);

	if (res.type === 'err') {
		throw new Error('Failed to fetch calendar data');
	}

	const lists = extractLists(
		res.value.lists,
		res.value.tasks,
		res.value.owner,
		res.value.sharedWith,
		res.value.tasksAssignees
	);

	const kanban = new Kanban({
		id: parseInt(id),
		owner: res.value.owner,
		lists,
		isShared: res.value.isShared,
		sharedWith: res.value.sharedWith
	});

	return {
		kanban,
		userPermissions: res.value.userPermissions
	};
};
