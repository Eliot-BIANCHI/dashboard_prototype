<script lang="ts">
	import type Day from '../Day.svelte';
	import MonthSchedule from '$lib/components/calendars/schedule/PreviewSchedule.svelte';
	import type Schedule from '../Schedule.svelte';

	import Button from '$lib/components/utils/button/Button.svelte';
	import Modal from '$lib/components/utils/Modal.svelte';

	import { apiPostLoadRequest as apiRequest } from '$lib/api-request/ApiRequest';
	import { addToast } from '$lib/components/toaster/Toaster.svelte';

	interface Props {
		calendarId: number;
		day: Day;
		changeScheduleDay: (schedule: Schedule) => void;
	}

	let { calendarId, day, changeScheduleDay }: Props = $props();

	let openShowSchedules = $state(false);

	let schedulesSorted = $derived(
		day.schedules
			.filter((schedule) => schedule.allDay === false)
			.toSorted((a, b) => a.startTime.localeCompare(b.startTime))
	);

	let allDaySchedules = $derived(day.schedules.filter((schedule) => schedule.allDay));

	async function deleteSchedule(scheduleId: number) {
		const res = await apiRequest.schedules.delete({ calendarId, scheduleId });
		if (res.type === 'ok') {
			day.deleteSchedule(scheduleId);
			addToast({ message: 'Schedule deleted', type: 'success' });
		} else {
			addToast({ message: 'Failed to delete schedule', type: 'danger' });
		}
	}
</script>

<Modal isOpen={openShowSchedules} class={['right']} onclose={() => (openShowSchedules = false)}>
	<div class="schedules">
		<h3>{day.label}</h3>
		{#if day.schedules.length > 0}
			{#each schedulesSorted as schedule (schedule.id)}
				<MonthSchedule
					{calendarId}
					{schedule}
					{changeScheduleDay}
					deleteSchedule={() => deleteSchedule(schedule.id)}
				/>
			{/each}

			{#if allDaySchedules.length > 0}
				<h4>All days schedules:</h4>
				{#each allDaySchedules as schedule}
					<MonthSchedule
						{calendarId}
						{schedule}
						{changeScheduleDay}
						deleteSchedule={() => deleteSchedule(schedule.id)}
					/>
				{/each}
			{/if}
		{:else}
			<p>No schedules...</p>
		{/if}
	</div>
</Modal>

<Button
	class={['width-100', 'height-100']}
	isSelected={day.schedules.length > 0}
	isDisabled={day.schedules.length === 0}
	onclick={() => (openShowSchedules = true)}
>
	{day.number}
</Button>

<style>
	.schedules {
		display: flex;
		flex-direction: column;
		gap: 25px;

		& h3,
		& h4 {
			margin-block: 0;
		}
	}
</style>
