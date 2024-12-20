import { postLoadRequest as request, RequestError, type ApiResponse } from '../../tool';
import { type Result, ok, err } from '$lib/Result';
import type { User, SearchBy, PaginatedSearchBy } from '$lib/Account.svelte';

interface PaginatedQuery {
	by: PaginatedSearchBy;
	search?: string;
	roleId?: number;
	page?: number;
}

export interface Users {
	getOverviews: (options: {
		query: { search: string; by: SearchBy };
	}) => Promise<Result<User[], string>>;
	getPaginatedOverviews: (options: {
		query: PaginatedQuery;
	}) => Promise<Result<{ overviews: User[]; count: number; pageNumber: number }, string>>;
}

const usersEndpoints: Users = {
	getOverviews: async (options) => {
		const endpoint = {
			method: 'GET',
			resource: `/users?search=${options.query.search}&by=${options.query.by}`,
			params: {},
			body: null
		};

		const res: Result<ApiResponse<{ overviews: User[] }>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data.overviews);
			} else {
				return err("Couldn't load the users");
			}
		}

		return err('INTERNAL SERVER ERROR');
	},

	getPaginatedOverviews: async (options) => {
		const resource = getResource(options.query);

		const endpoint = {
			method: 'GET',
			resource,
			params: {},
			body: null
		};

		const res: Result<
			ApiResponse<{ overviews: User[]; count: number; pageNumber: number }>,
			RequestError
		> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't load the overviews");
			}
		}

		return err('INTERNAL SERVER ERROR');
	}
};

function getResource(query: PaginatedQuery): string {
	const baseUrl = `/users/paginated?by=${query.by}`;
	let resource = undefined;

	switch (query.by) {
		case 'name':
			resource = baseUrl.concat(`&search=${query.search}`);
			break;
		case 'role':
			resource = baseUrl.concat(`&role_id=${query.roleId}`);
			break;
		default:
			resource = baseUrl;
	}

	return query.page === undefined ? resource : resource.concat(`&page=${query.page}`);
}

export default usersEndpoints;
