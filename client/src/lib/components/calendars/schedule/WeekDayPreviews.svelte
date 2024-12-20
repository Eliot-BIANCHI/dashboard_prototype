<script lang="ts">
	import AllDaySchedule from '$lib/components/calendars/schedule/PreviewSchedule.svelte';
	import type Day from '../Day.svelte';
	import type Schedule from '../Schedule.svelte';

	import Button from '$lib/components/utils/button/Button.svelte';
	import Modal from '$lib/components/utils/Modal.svelte';

	import { apiPostLoadRequest as apiRequest } from '$lib/api-request/ApiRequest';
	import { addToast } from '$lib/components/toaster/Toaster.svelte';

	interface Props {
		calendarId: number;
		day: Day;
		allDaySchedules: Schedule[];
	}

	let { calendarId, day, allDaySchedules }: Props = $props();

	let openShowSchedules = $state(false);

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
		<h3>All day schedules:</h3>
		{#if allDaySchedules.length > 0}
			{#each allDaySchedules as schedule (schedule.id)}
				<AllDaySchedule
					{calendarId}
					{schedule}
					changeScheduleDay={() => deleteSchedule(schedule.id)}
					deleteSchedule={() => deleteSchedule(schedule.id)}
				/>
			{/each}
		{:else}
			<p>No schedules...</p>
		{/if}
	</div>
</Modal>

<Button class={['primary', 'no-padding']} onclick={() => (openShowSchedules = true)}>
	{#if allDaySchedules.length > 1}
		See all
	{:else}
		{allDaySchedules[0].label}
	{/if}
</Button>

<style>
	.schedules {
		display: flex;
		flex-direction: column;
		gap: 25px;

		& h3 {
			margin-block: 0;
		}
	}
</style>
