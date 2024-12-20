<script lang="ts">
	import { page } from '$app/state';
	import { goto } from '$app/navigation';
	import ModeSelection from '$lib/components/calendars/calendar/ModeSelection.svelte';

	import DaySelection from '$lib/components/calendars/calendar/DaySelection.svelte';
	import WeekSelection from '$lib/components/calendars/calendar/WeekSelection.svelte';
	import MonthSelection from '$lib/components/calendars/calendar/MonthSelection.svelte';
	import WeekDays from '$lib/components/calendars/calendar/WeekDays.svelte';

	import Overview from '$lib/components/calendars/calendar/Overview.svelte';

	import AddCalendar from '$lib/components/calendars/calendar/Add.svelte';
	import AddSchedule from '$lib/components/calendars/schedule/Add.svelte';
	import Schedule from '$lib/components/calendars/Schedule.svelte';
	import { CalendarMode } from '$lib/components/calendars/Calendar.svelte.js';

	import Button from '$lib/components/utils/button/Button.svelte';
	import Modal from '$lib/components/utils/Modal.svelte';
	import Sidebar from '$lib/components/utils/Sidebar.svelte';

	import { scheduleFormData } from '$lib/components/calendars/tools.js';
	import { apiPostLoadRequest as apiRequest } from '$lib/api-request/ApiRequest';
	import { addToast } from '$lib/components/toaster/Toaster.svelte';

	let { children, data } = $props();

	let openAddSchedule = $state(false);

	let openAddKanban = $state(false);

	async function addSchedule(e: SubmitEvent) {
		const form = e.target as HTMLFormElement;

		const scheduleData = scheduleFormData(form);

		const res = await apiRequest.schedules.add({
			calendarId: data.calendar.id,
			data: {
				label: scheduleData.label,
				takes_place: scheduleData.takesPlace,
				start_time: scheduleData.startTime,
				end_time: scheduleData.endTime,
				all_day: scheduleData.allDay,
				calendar_id: data.calendar.id
			}
		});

		if (res.type === 'ok') {
			const schedule = new Schedule({
				...scheduleData,
				id: res.value
			});

			data.calendar.setNewSchedule(schedule);

			addToast({ message: 'Schedule added', type: 'success' });

			form.reset();
			openAddSchedule = false;
		} else {
			addToast({ message: 'Failed to add schedule', type: 'danger' });
		}
	}

	async function addCalendar(e: SubmitEvent) {
		const form = e.target as HTMLFormElement;

		const formData = new FormData(form);

		const calendarData = {
			label: formData.get('create-calendar-label') as string
		};

		const res = await apiRequest.calendars.add({ data: calendarData });

		if (res.type === 'ok') {
			data.calendars.addCalendar({ ...calendarData, id: res.value });

			addToast({ message: 'Calendar added', type: 'success' });
		} else {
			addToast({ message: 'Failed to add calendar', type: 'danger' });
		}

		form.reset();
		openAddSchedule = false;
	}

	async function deleteCalendar(calendarId: number) {
		const res = await apiRequest.calendars.delete({ calendarId });
		if (res.type === 'ok') {
			data.calendars.deleteCalendar(calendarId);

			if (localStorage.getItem(`${data.account.id}-favorites:calendar-id`) !== null) {
				data.account.setFavoriteCalendarId(null);
			}

			addToast({ message: 'Calendar deleted', type: 'success' });
			goto('/calendars');
		} else {
			addToast({ message: 'Failed to delete calendar', type: 'danger' });
		}
	}
</script>

<svelte:head>
	<title>Calendars</title>
</svelte:head>

<Modal isOpen={openAddSchedule} onclose={() => (openAddSchedule = false)}>
	<AddSchedule onsubmit={addSchedule} />
</Modal>

<Modal isOpen={openAddKanban} onclose={() => (openAddKanban = false)}>
	<AddCalendar onsubmit={addCalendar} />
</Modal>

<div class="calendars-layout">
	<Sidebar>
		<div class="controls-navbar">
			{#if page.url.pathname !== '/calendars'}
				<div class="controls">
					{#if data.calendar.calendarMode === CalendarMode.Day}
						<DaySelection calendar={data.calendar} />
					{:else if data.calendar.calendarMode === CalendarMode.Week}
						<WeekSelection calendar={data.calendar} />
					{:else if data.calendar.calendarMode === CalendarMode.Month}
						<MonthSelection calendar={data.calendar} />
					{/if}

					<Button onclick={() => (openAddSchedule = true)} iconName="add" class={['success']}>
						Schedule
					</Button>

					<ModeSelection calendar={data.calendar} />

					{#if data.calendar.calendarMode === CalendarMode.Week}
						<WeekDays calendar={data.calendar} />
					{/if}
				</div>
			{/if}

			<div class="calendars-navbar">
				<Button iconName="add" class={['success']} onclick={() => (openAddKanban = true)}>
					Calendar
				</Button>
				<ul class="overviews">
					{#each data.calendars.overviews as overview (overview.id)}
						<Overview {overview} deleteCalendar={() => deleteCalendar(overview.id)} />
					{/each}
				</ul>
			</div>
		</div>
	</Sidebar>

	{@render children()}
</div>

<style>
	.calendars-layout {
		flex-grow: 1;
		display: flex;
		gap: 20px;
	}

	.controls-navbar {
		display: flex;
		flex-direction: column;
		gap: 40px;
	}

	.controls {
		display: flex;
		flex-direction: column;
		gap: 20px;
	}

	.calendars-navbar {
		background-color: var(--tertiary-bg);
		display: flex;
		flex-direction: column;
		gap: 40px;
		list-style: none;
	}

	.overviews {
		display: flex;
		flex-direction: column;
		gap: 15px;
		list-style: none;
		margin-block: 0;
		padding-left: 0;
	}
</style>
