import { postLoadRequest as request, RequestError, type ApiResponse } from '../../tool';
import { type Result, ok, err } from '$lib/Result';

export interface Permissions {
	add: (options: { data: { label: string } }) => Promise<Result<number, string>>;
}

const permissionsEndpoints: Permissions = {
	add: async (options) => {
		const endpoint = {
			method: 'POST',
			resource: '/permissions',
			params: {},
			body: { label: options.data.label }
		};

		const res: Result<ApiResponse<{ permissionId: number }>, RequestError> = await request(
			endpoint
		);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data.permissionId);
			} else {
				return err("Couldn't add the permission");
			}
		}

		return err('INTERNAL SERVER ERROR');
	}
};

export default permissionsEndpoints;
