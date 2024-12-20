import { apiPreLoadRequest as apiRequest } from '$lib/api-request/ApiRequest';
import {
	extractCalendar,
	daysOfMonth,
	previousMonthDays,
	nextMonthDays
} from '$lib/components/calendars/tools.js';
import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, fetch, parent }) => {
	await parent();

	const { id, year, month } = params;

	const calendarId = parseInt(id);

	const viewedYear = parseInt(year);
	const viewedMonth = parseInt(month);

	const res = await apiRequest.calendars.getMonth(calendarId, viewedYear, viewedMonth, fetch);

	if (res.type === 'err') {
		throw new Error('Failed to fetch calendar data');
	}

	const days = daysOfMonth(viewedMonth, viewedYear);

	const previousDays = previousMonthDays(viewedMonth, viewedYear);
	const nextDays = nextMonthDays(viewedMonth, viewedYear);

	const monthDays = extractCalendar(res.value, days);

	return {
		calendarId: parseInt(id),
		monthDays,
		previousDays,
		nextDays,
		year: viewedYear,
		month: viewedMonth
	};
};
