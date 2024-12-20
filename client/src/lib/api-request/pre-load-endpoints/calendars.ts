import { preLoadRequest as request, RequestError, type LoadFetch, type ApiResponse } from '../tool';
import { type Result, ok, err } from '$lib/Result';

import type { CalendarOverviewT } from '$lib/components/calendars/Calendars.svelte';
import type { ScheduleLoaded } from '$lib/components/calendars/tools';

export interface Calendars {
	getOverviews: (loadFetch: LoadFetch) => Promise<Result<CalendarOverviewT[], string>>;
	getDay: (
		calendarId: number,
		year: number,
		dayOfYear: number,
		loadFetch: LoadFetch
	) => Promise<Result<ScheduleLoaded[], string>>;
	getWeek: (
		calendarId: number,
		year: number,
		weekOfYear: number,
		loadFetch: LoadFetch
	) => Promise<Result<ScheduleLoaded[], string>>;
	getMonth: (
		calendarId: number,
		year: number,
		month: number,
		loadFetch: LoadFetch
	) => Promise<Result<ScheduleLoaded[], string>>;
}

const calendarsEndpoints: Calendars = {
	getOverviews: async (loadFetch) => {
		const resource = '/calendars';
		const res: Result<
			ApiResponse<{ overviews: CalendarOverviewT[] }>,
			RequestError
		> = await request(resource, loadFetch);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data.overviews);
			} else {
				return err("Couldn't load the overviews");
			}
		}

		return err('INTERNAL SERVER ERROR');
	},
	getDay: async (calendarId: number, year: number, weekOfYear: number, loadFetch: LoadFetch) => {
		const resource = `/calendars/${calendarId}/days?year=${year}&day=${weekOfYear}`;
		const res: Result<
			ApiResponse<{ calendarSchedules: ScheduleLoaded[] }>,
			RequestError
		> = await request(resource, loadFetch);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data.calendarSchedules);
			} else {
				return err("Couldn't load the calendar");
			}
		}

		return err('INTERNAL SERVER ERROR');
	},
	getWeek: async (calendarId: number, year: number, week: number, loadFetch: LoadFetch) => {
		const resource = `/calendars/${calendarId}/weeks?year=${year}&week=${week}`;
		const res: Result<
			ApiResponse<{ calendarSchedules: ScheduleLoaded[] }>,
			RequestError
		> = await request(resource, loadFetch);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data.calendarSchedules);
			} else {
				return err("Couldn't load the calendar");
			}
		}

		return err('INTERNAL SERVER ERROR');
	},
	getMonth: async (calendarId: number, year: number, month: number, loadFetch: LoadFetch) => {
		const resource = `/calendars/${calendarId}/months?year=${year}&month=${month}`;
		const res: Result<
			ApiResponse<{ calendarSchedules: ScheduleLoaded[] }>,
			RequestError
		> = await request(resource, loadFetch);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data.calendarSchedules);
			} else {
				return err("Couldn't load the calendar");
			}
		}

		return err('INTERNAL SERVER ERROR');
	}
};

export default calendarsEndpoints;
