import { preLoadRequest as request, RequestError, type LoadFetch, type ApiResponse } from '../tool';
import { type Result, ok, err } from '$lib/Result';

import type { KanbanUser } from '$lib/components/kanbans/Kanban.svelte';
import type { TaskLoaded, ListLoaded, TaskAssignee } from '$lib/components/kanbans/tools';
import type { KanbanOverviewT } from '$lib/components/kanbans/Kanbans.svelte';
import type { KanbanPermission } from '$lib/tools';

export interface Kanbans {
	getOverviews: (loadFetch: LoadFetch) => Promise<Result<KanbanOverviewT[], string>>;
	get: (
		kanbanId: number,
		loadFetch: LoadFetch
	) => Promise<
		Result<
			{
				owner: KanbanUser;
				lists: ListLoaded[];
				tasks: TaskLoaded[];
				isShared: boolean;
				sharedWith: KanbanUser[];
				tasksAssignees: TaskAssignee[];
				userPermissions: KanbanPermission[];
			},
			string
		>
	>;
}

const kanbansEndpoints: Kanbans = {
	getOverviews: async (loadFetch) => {
		const resource = '/kanbans';
		const res: Result<ApiResponse<{ overviews: KanbanOverviewT[] }>, RequestError> = await request(
			resource,
			loadFetch
		);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data.overviews);
			} else {
				return err("Couldn't load the overviews");
			}
		}

		return err('INTERNAL SERVER ERROR');
	},
	get: async (kanbanId: number, loadFetch: LoadFetch) => {
		const resource = `/kanbans/${kanbanId}`;
		const res: Result<
			ApiResponse<{
				owner: KanbanUser;
				lists: ListLoaded[];
				tasks: TaskLoaded[];
				isShared: boolean;
				sharedWith: KanbanUser[];
				tasksAssignees: TaskAssignee[];
				userPermissions: KanbanPermission[];
			}>,
			RequestError
		> = await request(resource, loadFetch);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't load the kanban");
			}
		}

		return err('INTERNAL SERVER ERROR');
	}
};

export default kanbansEndpoints;
