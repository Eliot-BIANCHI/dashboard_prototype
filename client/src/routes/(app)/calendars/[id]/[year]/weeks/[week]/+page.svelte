<script lang="ts">
	import WeekDayPreviews from '$lib/components/calendars/schedule/WeekDayPreviews.svelte';
	import WeekSchedule from '$lib/components/calendars/schedule/Schedule.svelte';
	import { CalendarMode } from '$lib/components/calendars/Calendar.svelte';
	import Schedule from '$lib/components/calendars/Schedule.svelte.js';
	import { hours, sortSchedules, firstMondayOfYear } from '$lib/components/calendars/tools';

	import { apiPostLoadRequest as apiRequest } from '$lib/api-request/ApiRequest';
	import { addToast } from '$lib/components/toaster/Toaster.svelte';

	import { Temporal } from '@js-temporal/polyfill';

	let { data } = $props();

	$effect(() => {
		data.calendar.setCalendarId(data.calendarId);
		data.calendar.setCalendarMode(CalendarMode.Week);

		data.calendar.setWeek(data.week);
		data.calendar.setYear(data.year);

		const firstMonday = firstMondayOfYear(data.year);
		const currentWeek = firstMonday.add({ weeks: data.week - 1 });

		const steps = (data.calendar.dayOfWeek - currentWeek.dayOfWeek + 7) % 7;
		const dayOfYear = currentWeek.add({ days: steps }).dayOfYear;

		data.calendar.setDay(dayOfYear);
		data.calendar.setMonth(currentWeek.month);

		if (data.calendar.newSchedule !== null) {
			const date = Temporal.PlainDate.from(data.calendar.newSchedule.takesPlace);

			const day = data.weekDays.find((day) => day.number === date.day);

			if (day !== undefined) {
				day.addSchedule(data.calendar.newSchedule);
			}

			data.calendar.setNewSchedule(null);
		}
	});

	function changeScheduleDay(dayNumber: number, schedule: Schedule) {
		const formerDay = data.weekDays.find((day) => day.number === dayNumber)!;
		formerDay.deleteSchedule(schedule.id);

		const date = Temporal.PlainDate.from(schedule.takesPlace);
		const newDay = data.weekDays.find((day) => day.number === date.day);
		if (newDay === undefined) return;

		newDay.addSchedule(schedule);
	}

	async function deleteSchedule(dayNumber: number, scheduleId: number) {
		const res = await apiRequest.schedules.delete({
			calendarId: data.calendar.id,
			scheduleId
		});
		if (res.type === 'ok') {
			const day = data.weekDays.find((day) => day.number === dayNumber)!;
			day.deleteSchedule(scheduleId);
			addToast({ message: 'Schedule deleted', type: 'success' });
		} else {
			addToast({ message: 'Failed to delete schedule', type: 'danger' });
		}
	}
</script>

<ol class="week-calendar">
	{#each data.weekDays as day, i}
		{@const schedulesSorted = sortSchedules(
			day.schedules.filter((schedule) => schedule.allDay === false)
		)}
		{@const allDaySchedules = day.schedules.filter((schedule) => schedule.allDay)}
		{#if data.calendar.weekDays[i].isHidden === false}
			<li class="day">
				<div class="header">
					<span class="label">{day.label}</span>
					{#if allDaySchedules.length > 0}
						<WeekDayPreviews calendarId={data.calendar.id} {day} {allDaySchedules} />
					{/if}
				</div>

				<ol class="hours">
					{#each hours as hour, i}
						{#if i % 4 === 0}
							<li class="hour">{hour.formatted}</li>
						{/if}
					{/each}
				</ol>

				<ol class="schedules">
					{#each schedulesSorted as schedule}
						{#if schedule !== undefined}
							<li style:grid-row-start="span {schedule.duration}">
								<WeekSchedule
									calendarId={data.calendar.id}
									{schedule}
									changeScheduleDay={changeScheduleDay.bind(null, day.number)}
									deleteSchedule={() => deleteSchedule(day.number, schedule.id)}
								/>
							</li>
						{:else}
							<li class="empty"></li>
						{/if}
					{/each}
				</ol>
			</li>
		{/if}
	{/each}
</ol>

<style>
	.week-calendar {
		--fifteen-minutes-height: 30px;
		--row-gap: 5px;
		flex-grow: 1;
		display: flex;
		flex-wrap: wrap;
		column-gap: 5px;
		row-gap: 30px;
		list-style: none;
		overflow-y: auto;
		padding-left: 100px;
	}

	.day {
		flex-grow: 1;
		background-color: var(--secondary-bg);
		position: relative;
		min-width: 200px;
		max-width: 400px;
	}

	.header {
		background: var(--primary-bg);
		border-radius: 5px;
		display: flex;
		align-items: center;
		gap: 10px;
		font-size: 30px;
		margin-block: 0;
		padding: 20px;

		& .label {
			text-wrap: nowrap;
		}
	}

	.hours {
		display: flex;
		flex-direction: column;
		gap: var(--row-gap);
		font-size: 27px;
		list-style: none;
		padding-left: 0;
		position: absolute;
		translate: calc(-100% - 5px);
		width: 95px;
		z-index: -1;

		& .hour {
			background-color: var(--primary-bg);
			border-radius: 5px;
			height: calc(var(--fifteen-minutes-height) * 4 + var(--row-gap) * 3);
			padding-inline: 15px;

			&:last-child {
				height: 30px;
			}
		}
	}

	.schedules {
		display: grid;
		gap: var(--row-gap);
		grid-auto-rows: var(--fifteen-minutes-height);
		list-style: none;
		padding-left: 0;

		& .empty {
			background-color: var(--quaternary-bg);
			border-radius: 3px;
		}
	}
</style>
