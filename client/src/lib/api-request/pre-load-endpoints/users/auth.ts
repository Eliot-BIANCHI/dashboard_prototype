import {
	preLoadRequest as request,
	RequestError,
	type LoadFetch,
	type ApiResponse
} from '../../tool';
import { type Result, ok, err } from '$lib/Result';
import type { Permission } from '$lib/tools';

export interface Auth {
	autoLogIn: (loadFetch: LoadFetch) => Promise<Result<AutoLoggedUser, string>>;
	logOut: (loadFetch: LoadFetch) => Promise<Result<null, string>>;
}

interface AutoLoggedUser {
	id: number;
	firstName: string;
	lastName: string;
	imageUrl: string | null;
	roleId: number;
	roleLabel: string;
	permissions: Permission[];
}

const authEndpoints: Auth = {
	autoLogIn: async (loadFetch) => {
		const resource = '/auth/auto-log-in';
		const res: Result<
			ApiResponse<{ autoLoggedUser: AutoLoggedUser }>,
			RequestError
		> = await request(resource, loadFetch);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data.autoLoggedUser);
			} else {
				return err("Couldn't fetch the user");
			}
		}

		return err('INTERNAL SERVER ERROR');
	},
	logOut: async (loadFetch) => {
		const resource = '/auth/log-out';
		const res: Result<ApiResponse<null>, RequestError> = await request(resource, loadFetch);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't log-out the user");
			}
		}

		return err('INTERNAL SERVER ERROR');
	}
};

export default authEndpoints;
