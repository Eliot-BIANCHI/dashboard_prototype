<script lang="ts">
	import WeekDayPreviews from '$lib/components/calendars/schedule/WeekDayPreviews.svelte';
	import DaySchedule from '$lib/components/calendars/schedule/Schedule.svelte';
	import { CalendarMode } from '$lib/components/calendars/Calendar.svelte';
	import { hours, sortSchedules } from '$lib/components/calendars/tools';

	import { apiPostLoadRequest as apiRequest } from '$lib/api-request/ApiRequest';
	import { addToast } from '$lib/components/toaster/Toaster.svelte';

	import { Temporal } from '@js-temporal/polyfill';

	let { data } = $props();

	let schedulesSorted = $derived(
		sortSchedules(data.weekDay.schedules.filter((schedule) => !schedule.allDay))
	);

	let allDaySchedules = $derived(data.weekDay.schedules.filter((schedule) => schedule.allDay));

	$effect(() => {
		data.calendar.setCalendarId(data.calendarId);
		data.calendar.setCalendarMode(CalendarMode.Day);

		data.calendar.setDay(data.day);
		data.calendar.setYear(data.year);

		const currentDay = new Temporal.PlainDate(data.year, 1, 1).add({
			days: data.day - 1
		});

		data.calendar.setWeek(currentDay.weekOfYear);
		data.calendar.setMonth(currentDay.month);

		data.calendar.setDayOfWeek(currentDay.dayOfWeek);
		data.calendar.setDayOfMonth(currentDay.day);

		if (data.calendar.newSchedule !== null) {
			const date = Temporal.PlainDate.from(data.calendar.newSchedule.takesPlace);

			if (data.weekDay.number === date.day) {
				data.weekDay.addSchedule(data.calendar.newSchedule);
			}

			data.calendar.setNewSchedule(null);
		}
	});

	async function deleteSchedule(scheduleId: number) {
		const res = await apiRequest.schedules.delete({
			calendarId: data.calendar.id,
			scheduleId
		});
		if (res.type === 'ok') {
			data.weekDay.deleteSchedule(scheduleId);
			addToast({ message: 'Schedule deleted', type: 'success' });
		} else {
			addToast({ message: 'Failed to delete schedule', type: 'danger' });
		}
	}
</script>

<div class="day-calendar">
	<div class="header">
		<span class="label">{data.weekDay.label}</span>
		{#if allDaySchedules.length > 0}
			<WeekDayPreviews calendarId={data.calendar.id} {allDaySchedules} day={data.weekDay} />
		{/if}
	</div>

	<div class="content">
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
						<DaySchedule
							calendarId={data.calendar.id}
							{schedule}
							changeScheduleDay={() => deleteSchedule(schedule.id)}
							deleteSchedule={() => deleteSchedule(schedule.id)}
						/>
					</li>
				{:else}
					<li class="empty"></li>
				{/if}
			{/each}
		</ol>
	</div>
</div>

<style>
	.day-calendar {
		--fifteen-minutes-height: 50px;
		--row-gap: 5px;
		flex-grow: 1;
		display: flex;
		flex-direction: column;
		gap: 10px;
		margin-block: 16px;
		overflow-y: auto;
	}

	.header {
		background: var(--primary-bg);
		border-radius: 5px;
		display: flex;
		align-items: center;
		gap: 20px;
		font-size: 30px;
		margin-block: 0;
		padding: 20px;

		& .label {
			text-wrap: nowrap;
		}
	}

	.content {
		display: flex;
		gap: 10px;
	}

	.hours {
		display: flex;
		flex-direction: column;
		gap: var(--row-gap);
		font-size: 30px;
		list-style: none;
		margin-block: 0;
		padding-left: 0;

		& .hour {
			background-color: var(--primary-bg);
			border-radius: 5px;
			height: calc(var(--fifteen-minutes-height) * 4 + var(--row-gap) * 3);
			padding-inline: 35px;

			&:last-child {
				height: var(--fifteen-minutes-height);
			}
		}
	}

	.schedules {
		flex-grow: 1;
		display: grid;
		gap: var(--row-gap);
		grid-auto-rows: var(--fifteen-minutes-height);
		list-style: none;
		margin-block: 0;
		padding-left: 0;

		& .empty {
			background-color: var(--quaternary-bg);
			border-radius: 3px;
		}
	}
</style>
