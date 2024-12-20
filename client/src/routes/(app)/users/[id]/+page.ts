import { apiPreLoadRequest as apiRequest } from '$lib/api-request/ApiRequest';

import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, parent, params }) => {
	await parent();

	const { id } = params;

	const userId = parseInt(id);

	const res = await apiRequest.users.get(userId, fetch);

	if (res.type === 'err') {
		throw new Error('Failed to fetch user data');
	}

	return {
		user: res.value
	};
};
