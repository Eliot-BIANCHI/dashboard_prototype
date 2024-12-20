import { apiPreLoadRequest as apiRequest } from '$lib/api-request/ApiRequest';
import { extractCalendar, daysOfWeek } from '$lib/components/calendars/tools.js';

import { Temporal } from '@js-temporal/polyfill';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, fetch, parent }) => {
	await parent();

	const { id, year, week } = params;

	const calendarId = parseInt(id);

	const viewedYear = parseInt(year);
	const viewedWeek = parseInt(week);

	const res = await apiRequest.calendars.getWeek(calendarId, viewedYear, viewedWeek, fetch);

	if (res.type === 'err') {
		throw new Error('Failed to fetch calendar data');
	}

	const days: Temporal.PlainDate[] = daysOfWeek(viewedWeek, viewedYear);

	const weekDays = extractCalendar(res.value, days);

	return {
		calendarId,
		weekDays,
		year: viewedYear,
		week: viewedWeek
	};
};
