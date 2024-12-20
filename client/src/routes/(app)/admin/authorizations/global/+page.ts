import { apiPreLoadRequest as apiRequest } from '$lib/api-request/ApiRequest';
import Authorizations from '$lib/components/authorization/Authorizations.svelte';
import { RolePermission } from '$lib/components/authorization/RolePermission.svelte';
import { Role } from '$lib/components/authorization/Role.svelte';
import Permission from '$lib/components/authorization/Permission.svelte';

import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, parent }) => {
	await parent();

	const res = await apiRequest.rolesPermissions.get(fetch);

	if (res.type === 'err') {
		throw new Error('Failed to fetch roles permissions data');
	}

	const authorizations = new Authorizations({
		rolesPermissions: res.value.rolesPermissions.map(
			({ roleId, permissionId }) => new RolePermission({ roleId, permissionId })
		),
		roles: res.value.roles.map(({ id, label }) => new Role({ id, label })),
		permissions: res.value.permissions.map(({ id, label }) => new Permission({ id, label }))
	});

	return {
		authorizations
	};
};
