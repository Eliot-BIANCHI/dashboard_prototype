import { postLoadRequest as request, RequestError, type ApiResponse } from '../../tool';
import { type Result, ok, err } from '$lib/Result';

interface LogInBody {
	username: string;
	password: string;
}

export interface Auth {
	logIn: (options: { data: LogInBody }) => Promise<Result<null, string>>;
}

const authEndpoints: Auth = {
	logIn: async (options) => {
		const endpoint = {
			method: 'POST',
			resource: '/auth/log-in',
			params: {},
			body: { username: options.data.username, password: options.data.password }
		};
		const res: Result<ApiResponse<null>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't fetch the user");
			}
		}

		return err('INTERNAL SERVER ERROR');
	}
};

export default authEndpoints;
