import { Temporal, Intl } from '@js-temporal/polyfill';

import Day from './Day.svelte';
import Schedule from './Schedule.svelte';

export interface ScheduleLoaded {
	id: number;
	label: string;
	takesPlace: string;
	startTime: string;
	endTime: string;
	allDay: boolean;
}

export interface OverviewLoaded {
	id: number;
	label: string;
}

interface Hour {
	formatted: string;
	raw: string;
}

interface Date {
	formatted: string;
	raw: number;
}

const START_HOUR = 7;
const END_HOUR = 18;
const INTERVAL_MINUTES = 15;

export const hours: Hour[] = generateHours(START_HOUR, END_HOUR, INTERVAL_MINUTES);

export function extractCalendar(data: ScheduleLoaded[], days: Temporal.PlainDate[]): Day[] {
	const calendar: Day[] = [];

	const dateFormatter = new Intl.DateTimeFormat(undefined, {
		weekday: 'long',
		day: 'numeric'
	});

	days.forEach((day) => {
		const schedulesLoaded = data.filter((schedule) => {
			const takesPlace = Temporal.PlainDate.from(schedule.takesPlace);
			return day.day === takesPlace.day;
		});

		const schedules = schedulesLoaded.map(
			({ id, takesPlace, label, startTime, endTime, allDay }) => {
				const duration = calculateQuartersBetween(startTime, endTime);
				return new Schedule({ id, takesPlace, label, startTime, endTime, duration, allDay });
			}
		);

		calendar.push(
			new Day({
				label: dateFormatter.format(day),
				number: day.day,
				schedules
			})
		);
	});

	return calendar;
}

export function sortSchedules(daySchedules: Schedule[]): (Schedule | undefined)[] {
	const sortedSchedules: (Schedule | undefined)[] = [];

	let i = 0;
	let hour: Hour;

	while ((hour = hours[i]) !== undefined) {
		const schedule = daySchedules.find((schedule) => schedule.startTime === hour.raw);

		if (schedule === undefined) {
			sortedSchedules.push(undefined);
			i++;
		} else {
			sortedSchedules.push(schedule);
			i += schedule.duration;
		}
	}

	return sortedSchedules;
}

export function daysOfWeek(week: number, year: number): Temporal.PlainDate[] {
	const days: Temporal.PlainDate[] = [];

	let firstMonday = firstMondayOfYear(year);

	firstMonday = firstMonday.add({ weeks: week - 1 });

	for (let day = 1; day <= 7; day++) {
		days.push(firstMonday);
		firstMonday = firstMonday.add({ days: 1 });
	}

	return days;
}

export function daysOfMonth(month: number, year: number): Temporal.PlainDate[] {
	const days: Temporal.PlainDate[] = [];
	const yearMonth = Temporal.PlainDate.from({ year, month, day: 1 });

	const daysInMonth = yearMonth.daysInMonth;
	for (let day = 1; day <= daysInMonth; day++) {
		const monthDay = Temporal.PlainDate.from({ year, month, day });
		days.push(monthDay);
	}

	return days;
}

export function previousMonthDays(month: number, year: number): number[] {
	const days: number[] = [];
	const firstDayOfMonth = new Temporal.PlainDate(year, month, 1);
	const dayOfWeek = firstDayOfMonth.dayOfWeek;

	if (dayOfWeek > 1) {
		const previousMonth =
			month === 1
				? Temporal.PlainYearMonth.from({ year: year - 1, month: 12 })
				: Temporal.PlainYearMonth.from({ year, month: month - 1 });
		const daysInMonth = previousMonth.daysInMonth;

		for (let day = daysInMonth - dayOfWeek + 2; day <= daysInMonth; day++) {
			const monthDay = Temporal.PlainMonthDay.from({ month: month === 1 ? 12 : month - 1, day });
			days.push(monthDay.day);
		}
	}

	return days;
}

export function nextMonthDays(month: number, year: number): number[] {
	const days: number[] = [];

	const currentMonth = Temporal.PlainYearMonth.from({ year, month });
	const lastDayOfMonth = new Temporal.PlainDate(year, month, currentMonth.daysInMonth);
	const dayOfWeek = lastDayOfMonth.dayOfWeek;

	if (dayOfWeek < 7) {
		for (let day = 1; day <= 7 - dayOfWeek; day++) {
			const monthDay = Temporal.PlainMonthDay.from({ month: month + 1, day });
			days.push(monthDay.day);
		}
	}

	return days;
}

export function firstMondayOfYear(year: number): Temporal.PlainDate {
	const yearStart = new Temporal.PlainDate(year, 1, 1);
	const daysUntilThursday = (4 - yearStart.dayOfWeek + 7) % 7;

	const firstThursday = yearStart.add({ days: daysUntilThursday });
	const firstMonday = firstThursday.subtract({ days: firstThursday.dayOfWeek - 1 });

	return firstMonday;
}

export function numberOfWeeksInYear(year: number): number {
	const firstMonday = firstMondayOfYear(year);

	const endOfYear = firstMonday.add({ weeks: 52 });

	return endOfYear.weekOfYear === 1 ? 52 : 53;
}

export function yearDays(year: number): Date[] {
	const days: Date[] = [];

	const dateFormatter = new Intl.DateTimeFormat(undefined, { dateStyle: 'long' });

	let yearDay = new Temporal.PlainDate(year, 1, 1);
	const daysInYear = yearDay.daysInYear;

	for (let day = 1; day <= daysInYear; day++) {
		days.push({ formatted: dateFormatter.format(yearDay), raw: day });

		yearDay = yearDay.add({ days: 1 });
	}

	return days;
}

export function yearWeeks(year: number): Date[] {
	const weeks: Date[] = [];
	const dateFormatter = new Intl.DateTimeFormat(undefined, { dateStyle: 'long' });

	let firstMonday = firstMondayOfYear(year);

	const weeksInYear = numberOfWeeksInYear(year);

	for (let week = 1; week <= weeksInYear; week++) {
		const lastDayOfWeek = firstMonday.add({ days: 6 });

		weeks.push({
			formatted: dateFormatter.formatRange(firstMonday, lastDayOfWeek),
			raw: firstMonday.weekOfYear
		});

		firstMonday = firstMonday.add({ weeks: 1 });
	}

	return weeks;
}

export function yearMonths(year: number): Date[] {
	const months: Date[] = [];

	const dateFormatter = new Intl.DateTimeFormat(undefined, { dateStyle: 'long' });

	for (let month = 1; month <= 12; month++) {
		const yearMonth = Temporal.PlainYearMonth.from({ year, month });

		const firstDayOfMonth = new Temporal.PlainDate(year, month, 1);
		const lastDayOfMonth = new Temporal.PlainDate(year, month, yearMonth.daysInMonth);

		months.push({
			formatted: dateFormatter.formatRange(firstDayOfMonth, lastDayOfMonth),
			raw: month
		});
	}

	return months;
}

function generateHours(startHour: number, endHour: number, intervalMinutes: number): Hour[] {
	const hours: Hour[] = [];

	let time = new Temporal.PlainTime(startHour);
	const endTime = new Temporal.PlainTime(endHour).add({ minutes: intervalMinutes });

	while (!time.equals(endTime)) {
		hours.push({
			formatted: time.toString().slice(0, 5).replace(':', 'h'),
			raw: time.toString()
		});
		time = time.add({ minutes: intervalMinutes });
	}

	return hours;
}

export function calculateQuartersBetween(start: string, end: string): number {
	const startTime = Temporal.PlainTime.from(start);
	const endTime = Temporal.PlainTime.from(end);

	const difference = startTime.until(endTime);

	return difference.total({ unit: 'minutes' }) / 15;
}

export function scheduleFormData(form: HTMLFormElement, scheduleId?: number) {
	const formData = new FormData(form);

	const label = formData.get(
		scheduleId === undefined ? 'create-schedule-label' : `update-schedule-${scheduleId}-label`
	) as string;

	const takesPlace = formData.get(
		scheduleId === undefined
			? 'create-schedule-takes-place'
			: `update-schedule-${scheduleId}-takes-place`
	) as string;

	const allDay =
		(formData.get(
			scheduleId === undefined ? 'create-schedule-all-day' : `update-schedule-${scheduleId}-all-day`
		) as null | string) !== null;

	const startHour = allDay
		? 7
		: parseInt(
				formData.get(
					scheduleId === undefined
						? 'create-schedule-hour-start'
						: `update-schedule-${scheduleId}-hour-start`
				) as string
			);

	const startMinutes = allDay
		? '00'
		: (formData.get(
				scheduleId === undefined
					? 'create-schedule-minutes-start'
					: `update-schedule-${scheduleId}-minutes-start`
			) as string);

	const endHour = allDay
		? 18
		: parseInt(
				formData.get(
					scheduleId === undefined
						? 'create-schedule-hour-end'
						: `update-schedule-${scheduleId}-hour-end`
				) as string
			);

	const endMinutes = allDay
		? '00'
		: (formData.get(
				scheduleId === undefined
					? 'create-schedule-minutes-end'
					: `update-schedule-${scheduleId}-minutes-end`
			) as string);

	const startTime = `${startHour < 10 ? '0' + startHour : startHour}:${startMinutes}:00`;
	const endTime = `${endHour < 10 ? '0' + endHour : endHour}:${endMinutes}:00`;

	const scheduleData = {
		label,
		takesPlace,
		startTime,
		endTime,
		allDay,
		duration: calculateQuartersBetween(startTime, endTime)
	};

	return scheduleData;
}
