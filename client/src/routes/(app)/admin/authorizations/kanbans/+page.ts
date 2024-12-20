import { apiPreLoadRequest as apiRequest } from '$lib/api-request/ApiRequest';
import KanbanAuthorizations from '$lib/components/authorization/KanbanAuthorizations.svelte';
import { KanbanRolePermission } from '$lib/components/authorization/RolePermission.svelte';
import { KanbanRole } from '$lib/components/authorization/Role.svelte';
import Permission from '$lib/components/authorization/Permission.svelte';

import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, parent }) => {
	await parent();

	const res = await apiRequest.rolesPermissions.getKanbans(fetch);

	if (res.type === 'err') {
		throw new Error('Failed to fetch roles permissions data');
	}

	const kanbanAuthorizations = new KanbanAuthorizations({
		rolesPermissions: res.value.kanbanRolesPermissions.map(
			({ kanbanRoleId, permissionId }) => new KanbanRolePermission({ kanbanRoleId, permissionId })
		),
		roles: res.value.kanbanRoles.map(
			({ id, label, isDefault }) => new KanbanRole({ id, label, isDefault })
		),
		permissions: res.value.kanbanPermissions.map(({ id, label }) => new Permission({ id, label }))
	});

	return {
		kanbanAuthorizations
	};
};
