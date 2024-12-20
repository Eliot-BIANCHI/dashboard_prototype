import { apiPreLoadRequest as apiRequest } from '$lib/api-request/ApiRequest';
import Calendar from '$lib/components/calendars/Calendar.svelte';
import Calendars, { CalendarOverview } from '$lib/components/calendars/Calendars.svelte';
import type { LayoutLoad } from './$types';

import { Temporal } from '@js-temporal/polyfill';

export const load: LayoutLoad = async ({ fetch, parent }) => {
	await parent();

	const res = await apiRequest.calendars.getOverviews(fetch);

	if (res.type === 'err') {
		throw new Error('Failed to fetch calendar data');
	}

	const calendars = new Calendars(
		res.value.map(({ id, label }) => new CalendarOverview({ id, label }))
	);

	const date = Temporal.Now.plainDateTimeISO();

	const calendar = new Calendar({
		day: date.dayOfYear,
		week: date.weekOfYear,
		month: date.month,
		year: date.year,
		dayOfWeek: date.dayOfWeek,
		dayOfMonth: date.day
	});

	return {
		calendars,
		calendar
	};
};
