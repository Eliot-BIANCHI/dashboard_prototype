<script lang="ts">
	import { CalendarMode } from '$lib/components/calendars/Calendar.svelte';
	import MonthDayPreviews from '$lib/components/calendars/schedule/MonthDayPreviews.svelte';
	import Schedule from '$lib/components/calendars/Schedule.svelte';

	import { Temporal } from '@js-temporal/polyfill';

	let { data } = $props();

	let length = $derived(data.previousDays.length);

	$effect(() => {
		data.calendar.setCalendarId(data.calendarId);
		data.calendar.setCalendarMode(CalendarMode.Month);

		data.calendar.setMonth(data.month);
		data.calendar.setYear(data.year);

		const currentMonth = new Temporal.PlainYearMonth(data.year, data.month);
		const currentDay = new Temporal.PlainDate(
			data.year,
			data.month,
			currentMonth.daysInMonth < data.calendar.dayOfMonth
				? currentMonth.daysInMonth
				: data.calendar.dayOfMonth
		);

		data.calendar.setDay(currentDay.dayOfYear);
		data.calendar.setWeek(currentDay.weekOfYear);

		if (data.calendar.newSchedule !== null) {
			const date = Temporal.PlainDate.from(data.calendar.newSchedule.takesPlace);
			const day = data.monthDays.find((day) => day.number === date.day);

			if (day !== undefined) {
				day.addSchedule(data.calendar.newSchedule);
			}

			data.calendar.setNewSchedule(null);
		}
	});

	function changeScheduleDay(dayNumber: number, schedule: Schedule) {
		const formerDay = data.monthDays.find((day) => day.number === dayNumber)!;
		formerDay.deleteSchedule(schedule.id);

		const date = Temporal.PlainDate.from(schedule.takesPlace);
		const newDay = data.monthDays.find((day) => day.number === date.day);
		if (newDay === undefined) return;

		newDay.addSchedule(schedule);
	}

	const days = ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday', 'Saturday', 'Sunday'];
</script>

<ol class="month-calendar">
	{#each data.previousDays as day, i}
		<li
			class="day out-of-month"
			data-day={i < 7 ? days[i] : undefined}
			data-day-first-letter={i < 7 ? days[i][0] : undefined}
		>
			{day}
		</li>
	{/each}

	{#each data.monthDays as day, i}
		<li
			class="day"
			data-day={i + length < 7 ? days[i + length] : undefined}
			data-day-first-letter={i + length < 7 ? days[i + length][0] : undefined}
		>
			<MonthDayPreviews
				calendarId={data.calendar.id}
				{day}
				changeScheduleDay={changeScheduleDay.bind(null, day.number)}
			/>
		</li>
	{/each}

	{#each data.nextDays as day}
		<li class="day out-of-month">{day}</li>
	{/each}
</ol>

<style>
	.month-calendar {
		flex-grow: 1;
		background-color: var(--primary-bg);
		border-radius: 5px;
		display: grid;
		grid-template-columns: repeat(7, minmax(50px, 1fr));
		gap: 10px;
		list-style: none;
		overflow-y: auto;
		padding: 65px 15px 15px 15px;

		& .day {
			aspect-ratio: 1 / 1;
			border-radius: 5px;
			font-size: 30px;

			&.out-of-month {
				background-color: var(--tertiary-bg);
				display: flex;
				padding: 10px;
			}

			&[data-day],
			&[data-day-first-letter] {
				position: relative;
			}

			&[data-day]::before,
			&[data-day-first-letter]::after {
				padding-block: 15px;
				position: absolute;
				top: 0;
				left: 0;
				text-align: center;
				transform: translateY(-100%);
				width: 100%;
			}

			&[data-day-first-letter]::before {
				content: attr(data-day);
			}

			&[data-day-first-letter]::after {
				content: attr(data-day-first-letter);
			}
		}
	}

	@media all and (width < 1400px) {
		.month-calendar {
			height: max-content;
		}

		.day[data-day-first-letter]::before {
			display: none;
		}

		.out-of-month {
			align-items: center;
			justify-content: center;
		}
	}

	@media all and (width >= 1400px) {
		.day[data-day-first-letter]::after {
			display: none;
		}
	}
</style>
