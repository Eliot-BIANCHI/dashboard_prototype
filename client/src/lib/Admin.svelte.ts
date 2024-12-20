import type { User } from '$lib/Account.svelte';
import type { PaginatedSearchBy as SearchBy } from '$lib/Account.svelte';
import type { RoleT } from '$lib/components/authorization/Role.svelte';

interface Pagination {
	pages: number;
	currentPage: number;
	count: number;
}

export default class Admin {
	overviews: User[] = $state([]);
	search = $state('');
	searchBy: SearchBy = $state('name');
	roles: RoleT[] | null = $state(null);
	roleId: number = $state(-1);
	pagination: Pagination | null = $state(null);
}
