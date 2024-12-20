import { apiPreLoadRequest as apiRequest } from '$lib/api-request/ApiRequest';

import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, parent }) => {
	await parent();

	const res = await Promise.all([
		apiRequest.calendars.getOverviews(fetch),
		apiRequest.kanbans.getOverviews(fetch)
	]);

	if (res[0].type === 'err' || res[1].type === 'err') {
		throw new Error('Failed to fetch favorites data');
	}

	return {
		calendars: res[0].value,
		kanbans: res[1].value
	};
};
