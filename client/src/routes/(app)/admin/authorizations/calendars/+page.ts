import { apiPreLoadRequest as apiRequest } from '$lib/api-request/ApiRequest';
import CaleCalendarAuthorizations from '$lib/components/authorization/CalendarAuthorizations.svelte';
import { CalendarRolePermission } from '$lib/components/authorization/RolePermission.svelte';
import { CalendarRole } from '$lib/components/authorization/Role.svelte';
import Permission from '$lib/components/authorization/Permission.svelte';

import type { PageLoad } from './$types';

export const load: PageLoad = async ({ fetch, parent }) => {
	await parent();

	const res = await apiRequest.rolesPermissions.getCalendars(fetch);

	if (res.type === 'err') {
		throw new Error('Failed to fetch roles permissions data');
	}

	const calendarAuthorizations = new CaleCalendarAuthorizations({
		rolesPermissions: res.value.calendarRolesPermissions.map(
			({ calendarRoleId, permissionId }) =>
				new CalendarRolePermission({ calendarRoleId, permissionId })
		),
		roles: res.value.calendarRoles.map(
			({ id, label, isDefault }) => new CalendarRole({ id, label, isDefault })
		),
		permissions: res.value.calendarPermissions.map(({ id, label }) => new Permission({ id, label }))
	});

	return {
		calendarAuthorizations
	};
};
