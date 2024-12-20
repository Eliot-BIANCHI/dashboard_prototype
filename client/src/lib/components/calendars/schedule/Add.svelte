<script lang="ts">
	import Form from '$lib/components/utils/form/Form.svelte';
	import Input from '$lib/components/utils/form/Input.svelte';
	import Toggle from '$lib/components/utils/form/Toggle.svelte';
	import Button from '$lib/components/utils/button/Button.svelte';
	import Time from './Time.svelte';

	interface Props {
		onsubmit: (e: SubmitEvent) => void;
	}

	let { onsubmit: handler }: Props = $props();

	let allDay = $state(false);
</script>

<Form
	onsubmit={(e) => {
		allDay = false;
		handler(e);
	}}
>
	<Input name="create-schedule-takes-place" text="The date" type="date" />

	<Input name="create-schedule-label" text="Label" />

	<Toggle name="create-schedule-all-day" text="All day?" onchange={() => (allDay = !allDay)} />

	{#if allDay === false}
		<Time startHour={7} endHour={8} startMinutes={0} endMinutes={0} />
	{/if}

	<Button type="submit">Add</Button>
</Form>
