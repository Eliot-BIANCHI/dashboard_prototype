<script lang="ts">
	import { page } from '$app/state';
	import { CalendarOverview } from '../Calendars.svelte';

	import Button from '$lib/components/utils/button/Button.svelte';
	import IconButton from '$lib/components/utils/button/IconButton.svelte';
	import Modal from '$lib/components/utils/Modal.svelte';

	import UpdateCalendar from './Update.svelte';
	import Form from '$lib/components/utils/form/Form.svelte';
	import { apiPostLoadRequest as apiRequest } from '$lib/api-request/ApiRequest';
	import { addToast } from '$lib/components/toaster/Toaster.svelte';

	import { Temporal } from '@js-temporal/polyfill';

	interface Props {
		overview: CalendarOverview;
		deleteCalendar: () => void;
	}

	let { overview, deleteCalendar }: Props = $props();

	async function updateCalendar(e: SubmitEvent) {
		const formData = new FormData(e.target as HTMLFormElement);
		const calendarData = {
			label: formData.get(`update-calendar-${overview.id}-label`) as string
		};

		const res = await apiRequest.calendars.update({
			calendarId: overview.id,
			data: calendarData
		});

		if (res.type === 'ok') {
			overview.update(calendarData.label);

			addToast({ message: 'Calendar updated', type: 'success' });
		} else {
			addToast({ message: 'Failed to update calendar', type: 'danger' });
		}

		openCalendarSettings = false;
	}

	let openCalendarSettings = $state(false);

	const date = Temporal.Now.plainDateTimeISO();

	const year = date.year;
	const week = date.weekOfYear;

	const href = `/calendars/${overview.id}/${year}/weeks/${week}`;

	const match = href.match(/^\/calendars\/\d+/)![0];
</script>

<Modal isOpen={openCalendarSettings} onclose={() => (openCalendarSettings = false)}>
	<UpdateCalendar {overview} onsubmit={updateCalendar} />
	<Form class={['no-border', 'no-padding-bottom']} onsubmit={deleteCalendar}>
		<Button class={['danger']} iconName="delete" type="submit">Delete</Button>
	</Form>
</Modal>

<li class="overview">
	<a {href} class:active={page.url.pathname.startsWith(match)}>{overview.label}</a>
	<IconButton
		iconName="vertical-dots"
		class={['right-border-radius']}
		onclick={() => (openCalendarSettings = true)}
	/>
</li>

<style>
	.overview {
		display: flex;

		& a {
			background-color: var(--primary-bg);
			border-top-left-radius: 5px;
			border-bottom-left-radius: 5px;
			box-sizing: border-box;
			color: var(--primary-text);
			display: inline-block;
			flex-grow: 1;
			font-size: 23px;
			outline-color: orange;
			outline-offset: 3px;
			padding: 15px;
			text-decoration: none;

			&:hover,
			&.active {
				background-color: var(--secondary-bg);
			}
		}
	}
</style>
