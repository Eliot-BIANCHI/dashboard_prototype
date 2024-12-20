import Admin from '$lib/Admin.svelte';
import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ parent }) => {
	await parent();

	const admin = new Admin();

	return {
		admin
	};
};
