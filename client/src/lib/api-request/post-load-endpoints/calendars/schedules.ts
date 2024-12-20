import { postLoadRequest as request, RequestError, type ApiResponse } from '../../tool';
import { type Result, ok, err } from '$lib/Result';

interface AddData {
	label: string;
	takes_place: string;
	start_time: string;
	end_time: string;
	all_day: boolean;
	calendar_id: number;
}

interface UpdateData {
	label: string;
	takes_place: string;
	start_time: string;
	end_time: string;
	all_day: boolean;
}

export interface Schedules {
	add: (options: { calendarId: number; data: AddData }) => Promise<Result<number, string>>;
	update: (options: {
		calendarId: number;
		scheduleId: number;
		data: UpdateData;
	}) => Promise<Result<null, string>>;
	delete: (options: { calendarId: number; scheduleId: number }) => Promise<Result<null, string>>;
}

const schedulesEndpoints: Schedules = {
	add: async (options) => {
		const endpoint = {
			method: 'POST',
			resource: `/calendars/${options.calendarId}/schedules`,
			params: {},
			body: { ...options.data }
		};

		const res: Result<ApiResponse<{ scheduleId: number }>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data.scheduleId);
			} else {
				return err("Couldn't add the schedule");
			}
		}

		return err('INTERNAL SERVER ERROR');
	},

	update: async (options) => {
		const endpoint = {
			method: 'PUT',
			resource: `/calendars/${options.calendarId}/schedules/${options.scheduleId}`,
			params: {},
			body: { ...options.data }
		};

		const res: Result<ApiResponse<null>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't update the schedule");
			}
		}

		return err('INTERNAL SERVER ERROR');
	},

	delete: async (options) => {
		const endpoint = {
			method: 'DELETE',
			resource: `/calendars/${options.calendarId}/schedules/${options.scheduleId}`,
			params: {},
			body: null
		};

		const res: Result<ApiResponse<null>, RequestError> = await request(endpoint);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data);
			} else {
				return err("Couldn't delete the schedule");
			}
		}

		return err('INTERNAL SERVER ERROR');
	}
};

export default schedulesEndpoints;
