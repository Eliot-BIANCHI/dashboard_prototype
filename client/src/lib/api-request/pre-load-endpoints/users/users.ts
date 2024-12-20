import {
	preLoadRequest as request,
	RequestError,
	type LoadFetch,
	type ApiResponse
} from '../../tool';
import { type Result, ok, err } from '$lib/Result';
import type { User } from '$lib/Account.svelte';

export interface Users {
	get: (userId: number, loadFetch: LoadFetch) => Promise<Result<User, string>>;
}

const usersEndpoints: Users = {
	get: async (userId, loadFetch) => {
		const resource = `/users/${userId}`;
		const res: Result<ApiResponse<{ user: User }>, RequestError> = await request(
			resource,
			loadFetch
		);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data.user);
			} else {
				return err("Couldn't fetch the user");
			}
		}

		return err('INTERNAL SERVER ERROR');
	}
};

export default usersEndpoints;
