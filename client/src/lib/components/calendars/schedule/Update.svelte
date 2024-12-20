<script lang="ts">
	import Time from './Time.svelte';
	import Form from '$lib/components/utils/form/Form.svelte';
	import Input from '$lib/components/utils/form/Input.svelte';
	import Toggle from '$lib/components/utils/form/Toggle.svelte';
	import Button from '$lib/components/utils/button/Button.svelte';
	import type Schedule from '../Schedule.svelte';

	interface Props {
		schedule: Schedule;
		onsubmit: (e: SubmitEvent) => void;
	}

	let { schedule, onsubmit }: Props = $props();

	let startHour = $derived(parseInt(schedule.startTime.slice(0, 3)));
	let endHour = $derived(parseInt(schedule.endTime.slice(0, 3)));

	let startMinutes = $derived(parseInt(schedule.startTime.slice(3, 5)));
	let endMinutes = $derived(parseInt(schedule.endTime.slice(3, 5)));

	let allDay = $state(schedule.allDay);
</script>

<Form {onsubmit}>
	<Input
		name="update-schedule-{schedule.id}-takes-place"
		text="The date"
		type="date"
		value={schedule.takesPlace}
	/>

	<Input name="update-schedule-{schedule.id}-label" text="Label" value={schedule.label} />

	<Toggle
		name="update-schedule-{schedule.id}-all-day"
		text="All day?"
		isChecked={schedule.allDay}
		onchange={() => (allDay = !allDay)}
	/>

	{#if allDay === false}
		<Time scheduleId={schedule.id} {startHour} {endHour} {startMinutes} {endMinutes} />
	{/if}

	<Button iconName="update" class={['warning']} type="submit">Update</Button>
</Form>
