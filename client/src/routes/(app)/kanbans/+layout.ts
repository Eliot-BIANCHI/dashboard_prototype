import { apiPreLoadRequest as apiRequest } from '$lib/api-request/ApiRequest';
import Kanbans, { KanbanOverview } from '$lib/components/kanbans/Kanbans.svelte';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ fetch, parent }) => {
	await parent();

	const res = await apiRequest.kanbans.getOverviews(fetch);

	if (res.type === 'err') {
		throw new Error('Failed to fetch kanbans overviews');
	}

	const kanbans = new Kanbans(
		res.value.map(({ id, label, isShared }) => new KanbanOverview({ id, label, isShared }))
	);

	return {
		kanbans
	};
};
