import { postLoadRequest as request, RequestError, type ApiResponse } from '../../tool';
import { type Result, ok, err } from '$lib/Result';

export interface Calendars {
	add: (options: { data: { label: string } }) => Promise<Result<number, string>>;
	update: (options: {
		calendarId: number;
		data: { label: string };
	}) => Promise<Result<null, string>>;
	delete: (options: { calendarId: number }) => Promise<Result<null, string>>;
}

const calendarsEndpoints: Calendars = {
	add: async (options) => {
		const endpoint = {
			method: 'POST',
			resource: '/calendars',
			params: {},
			body: { label: options.data.label, user_id: 1 }
		};

		const res: Result<ApiResponse<{ calendarId: number }>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data.calendarId);
			} else {
				return err("Couldn't add the calendar");
			}
		}

		return err('INTERNAL SERVER ERROR');
	},

	update: async (options) => {
		const endpoint = {
			method: 'PUT',
			resource: `/calendars/${options.calendarId}`,
			params: {},
			body: { label: options.data.label }
		};

		const res: Result<ApiResponse<null>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't update the calendar");
			}
		}

		return err('INTERNAL SERVER ERROR');
	},

	delete: async (options) => {
		const endpoint = {
			method: 'DELETE',
			resource: `/calendars/${options.calendarId}`,
			params: {},
			body: null
		};

		const res: Result<ApiResponse<null>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't delete the calendar");
			}
		}

		return err('INTERNAL SERVER ERROR');
	}
};

export default calendarsEndpoints;
