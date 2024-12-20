import { apiPreLoadRequest as apiRequest } from '$lib/api-request/ApiRequest';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ url, fetch, parent }) => {
	await parent();

	if (url.searchParams.get('log-out') === 'true') {
		const res = await apiRequest.auth.logOut(fetch);

		if (res.type === 'err') {
			throw new Error('Failed to log out the user');
		}
	}
};
