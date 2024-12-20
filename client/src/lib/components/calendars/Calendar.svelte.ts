import type Schedule from './Schedule.svelte';
import { Temporal } from '@js-temporal/polyfill';

interface WeekDay {
	firstLetter: string;
	id: string;
	isHidden: boolean;
}

interface CalendarT {
	day: number;
	week: number;
	month: number;
	year: number;
	dayOfWeek: number;
	dayOfMonth: number;
}

export enum CalendarMode {
	Day = 'Day',
	Week = 'Week',
	Month = 'Month'
}

const TODAY_DATE = Temporal.Now.plainDateTimeISO();

const WEEK_DAYS = [
	{ firstLetter: 'M', id: 'monday' },
	{ firstLetter: 'T', id: 'tuesday' },
	{ firstLetter: 'W', id: 'wednesday' },
	{ firstLetter: 'T', id: 'thursday' },
	{ firstLetter: 'F', id: 'friday' },
	{ firstLetter: 'S', id: 'saturday' },
	{ firstLetter: 'S', id: 'sunday' }
];

export default class Calendar {
	id: number = $state(-1);

	day: number = $state(0);
	today: number = TODAY_DATE.dayOfYear;

	week: number = $state(0);
	todayWeek: number = TODAY_DATE.weekOfYear;

	month: number = $state(0);
	todayMonth: number = TODAY_DATE.month;

	year: number = $state(0);
	todayYear: number = TODAY_DATE.year;

	dayOfWeek: number;
	dayOfMonth: number;

	weekDays: WeekDay[] = $state([]);

	calendarMode: CalendarMode = $state(CalendarMode.Week);

	newSchedule: Schedule | null = $state(null);

	constructor({ day, week, month, year, dayOfWeek, dayOfMonth }: CalendarT) {
		this.day = day;
		this.week = week;
		this.month = month;
		this.year = year;
		this.dayOfWeek = dayOfWeek;
		this.dayOfMonth = dayOfMonth;

		const hiddenDays: string[] = JSON.parse(localStorage.getItem('calendar:hidden-days') || '[]');

		this.weekDays = WEEK_DAYS.map((day) => ({
			firstLetter: day.firstLetter,
			isHidden: hiddenDays.includes(day.id),
			id: day.id
		}));
	}

	setCalendarId(calendarId: number) {
		this.id = calendarId;
	}

	setDay(day: number) {
		this.day = day;
	}

	setWeek(week: number) {
		this.week = week;
	}

	setMonth(month: number) {
		this.month = month;
	}

	setYear(year: number) {
		this.year = year;
	}

	setDayOfWeek(dayOfWeek: number) {
		this.dayOfWeek = dayOfWeek;
	}

	setDayOfMonth(dayOfMonth: number) {
		this.dayOfMonth = dayOfMonth;
	}

	setCalendarMode(calendarMode: CalendarMode) {
		this.calendarMode = calendarMode;
	}

	toggleAreDaysHidden(areHidden: boolean) {
		this.weekDays.forEach((day) => (day.isHidden = areHidden));

		const hiddenDays: string[] = JSON.parse(localStorage.getItem('calendar:hidden-days') || '[]');

		if (areHidden) {
			this.weekDays.forEach((day) => {
				if (!hiddenDays.includes(day.id)) {
					hiddenDays.push(day.id);
				}
			});
		} else {
			hiddenDays.length = 0;
		}

		localStorage.setItem('calendar:hidden-days', JSON.stringify(hiddenDays));
	}

	toggleIsDayHidden(dayId: string) {
		const day = this.weekDays.find((day) => day.id === dayId)!;
		day.isHidden = !day.isHidden;

		const hiddenDays: string[] = JSON.parse(localStorage.getItem('calendar:hidden-days') || '[]');

		if (day.isHidden) {
			hiddenDays.push(day.id);
		} else {
			const index = hiddenDays.findIndex((hiddenDay) => hiddenDay === day.id);
			hiddenDays.splice(index, 1);
		}

		localStorage.setItem('calendar:hidden-days', JSON.stringify(hiddenDays));
	}

	setNewSchedule(schedule: Schedule | null) {
		this.newSchedule = schedule;
	}
}
