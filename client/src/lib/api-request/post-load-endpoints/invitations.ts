import { postLoadRequest as request, RequestError, type ApiResponse } from '../tool';
import { type Result, ok, err } from '$lib/Result';

export interface Invitations {
	acceptKanban: (options: { invitationId: number }) => Promise<Result<null, string>>;
	rejectKanban: (options: { invitationId: number }) => Promise<Result<null, string>>;
}

const invitationsEndpoints: Invitations = {
	acceptKanban: async (options) => {
		const endpoint = {
			method: 'POST',
			resource: `/invitations/kanbans/${options.invitationId}/accept`,
			params: {},
			body: null
		};

		const res: Result<ApiResponse<null>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't accept the invitation");
			}
		}

		return err('INTERNAL SERVER ERROR');
	},

	rejectKanban: async (options) => {
		const endpoint = {
			method: 'POST',
			resource: `/invitations/kanbans/${options.invitationId}/reject`,
			params: {},
			body: null
		};

		const res: Result<ApiResponse<null>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't reject the invitation");
			}
		}

		return err('INTERNAL SERVER ERROR');
	}
};

export default invitationsEndpoints;
