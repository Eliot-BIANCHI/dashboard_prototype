<script lang="ts">
	import { CalendarOverview } from '$lib/components/calendars/Calendars.svelte';

	import Form from '../../utils/form/Form.svelte';
	import Input from '../../utils/form/Input.svelte';
	import Button from '../../utils/button/Button.svelte';

	interface Props {
		overview: CalendarOverview;
		onsubmit: (e: SubmitEvent) => void;
	}

	let { overview, onsubmit }: Props = $props();
</script>

<Form {onsubmit}>
	<Input text="Label" name="update-calendar-{overview.id}-label" value={overview.label} />
	<Button iconName="update" class={['warning']} type="submit">Update</Button>
</Form>
