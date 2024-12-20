<script lang="ts">
	import Schedule from '../Schedule.svelte';
	import UpdateSchedule from '../schedule/Update.svelte';
	import Form from '$lib/components/utils/form/Form.svelte';
	import Button from '$lib/components/utils/button/Button.svelte';
	import IconButton from '$lib/components/utils/button/IconButton.svelte';
	import Modal from '$lib/components/utils/Modal.svelte';

	import { Temporal, Intl } from '@js-temporal/polyfill';
	import { scheduleFormData } from '$lib/components/calendars/tools.js';

	import { apiPostLoadRequest as apiRequest } from '$lib/api-request/ApiRequest';
	import { addToast } from '$lib/components/toaster/Toaster.svelte';

	interface Props {
		calendarId: number;
		schedule: Schedule;
		changeScheduleDay: (schedule: Schedule) => void;
		deleteSchedule: () => void;
	}

	let { calendarId, schedule, changeScheduleDay, deleteSchedule }: Props = $props();

	let timeStart = $derived(Temporal.PlainTime.from(schedule.startTime));

	let timeEnd = $derived(Temporal.PlainTime.from(schedule.endTime));

	const timeFormatter = new Intl.DateTimeFormat(undefined, { hour: '2-digit', minute: '2-digit' });

	async function updateSchedule(e: SubmitEvent) {
		const form = e.target as HTMLFormElement;

		const scheduleData = scheduleFormData(form, schedule.id);

		const res = await apiRequest.schedules.update({
			calendarId,
			scheduleId: schedule.id,
			data: {
				label: scheduleData.label,
				takes_place: scheduleData.takesPlace,
				start_time: scheduleData.startTime,
				end_time: scheduleData.endTime,
				all_day: scheduleData.allDay
			}
		});

		if (res.type === 'ok') {
			if (schedule.takesPlace !== scheduleData.takesPlace) {
				const newSchedule = new Schedule({ id: schedule.id, ...scheduleData });
				changeScheduleDay(newSchedule);
			} else {
				schedule.update(scheduleData);
			}

			addToast({ message: 'Schedule updated', type: 'success' });
		} else {
			addToast({ message: 'Failed to update schedule', type: 'danger' });
		}

		openScheduleSettings = false;
	}

	let openScheduleSettings = $state(false);
</script>

<Modal isOpen={openScheduleSettings} onclose={() => (openScheduleSettings = false)}>
	<UpdateSchedule onsubmit={updateSchedule} {schedule} />
	<Form class={['no-border', 'no-padding-bottom']} onsubmit={deleteSchedule}>
		<Button class={['danger']} iconName="delete" type="submit">Delete</Button>
	</Form>
</Modal>

<div class="schedule">
	<span class="header">
		<IconButton
			iconName="vertical-dots"
			class={['round']}
			onclick={() => (openScheduleSettings = true)}
		/>
	</span>
	{#if !schedule.allDay}
		<span class="time-slot">
			<span class="time-start">{timeFormatter.format(timeStart)}</span>
			<span class="time-end">{timeFormatter.format(timeEnd)}</span>
		</span>
	{/if}
	<span>{schedule.label}</span>
</div>

<style>
	.schedule {
		border: 3px solid var(--app-color);
		border-radius: 15px;
		display: flex;
		flex-direction: column;
		gap: 20px;
		padding: 20px;
		position: relative;

		& .header {
			display: flex;
			justify-content: flex-end;
		}

		& .time-slot {
			display: flex;
			justify-content: space-between;
		}
	}
</style>
