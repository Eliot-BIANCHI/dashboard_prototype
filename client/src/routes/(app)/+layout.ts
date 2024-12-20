import { apiPreLoadRequest as apiRequest } from '$lib/api-request/ApiRequest';
import { redirect } from '@sveltejs/kit';

import type { LayoutLoad } from './$types';
import Account from '$lib/Account.svelte';

export const load: LayoutLoad = async ({ url, fetch, parent }) => {
	await parent();

	const res = await apiRequest.auth.autoLogIn(fetch);

	if (res.type === 'ok') {
		return {
			account: new Account(res.value)
		};
	} else {
		const fromUrl = url.pathname + url.search;
		redirect(302, `/log-in?redirectTo=${fromUrl}`);
	}
};
