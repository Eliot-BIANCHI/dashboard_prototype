import { apiPreLoadRequest as apiRequest } from '$lib/api-request/ApiRequest';
import { Temporal, Intl } from '@js-temporal/polyfill';
import Day from '$lib/components/calendars/Day.svelte';
import { calculateQuartersBetween } from '$lib/components/calendars/tools.js';

import type { PageLoad } from './$types';
import Schedule from '$lib/components/calendars/Schedule.svelte';

export const load: PageLoad = async ({ params, fetch, parent }) => {
	await parent();

	const { id, year, day } = params;

	const calendarId = parseInt(id);

	const viewedYear = parseInt(year);
	const viewedDay = parseInt(day);

	const res = await apiRequest.calendars.getDay(calendarId, viewedYear, viewedDay, fetch);

	if (res.type === 'err') {
		throw new Error('Failed to fetch calendar data');
	}

	const yearDay = new Temporal.PlainDate(viewedYear, 1, 1).add({ days: viewedDay - 1 });

	const dateFormatter = new Intl.DateTimeFormat(undefined, {
		weekday: 'long',
		day: 'numeric',
		month: 'long'
	});

	const schedules = res.value.map(({ id, takesPlace, label, startTime, endTime, allDay }) => {
		const duration = calculateQuartersBetween(startTime, endTime);
		return new Schedule({ id, takesPlace, label, startTime, endTime, allDay, duration });
	});

	const weekDay = new Day({
		label: dateFormatter.format(yearDay),
		number: yearDay.day,
		schedules
	});

	return {
		calendarId: parseInt(id),
		weekDay,
		year: viewedYear,
		day: viewedDay
	};
};
