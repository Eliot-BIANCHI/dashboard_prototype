<script lang="ts">
	import { KanbanOverview } from '$lib/components/kanbans/Kanbans.svelte';

	import Form from '$lib/components/utils/form/Form.svelte';
	import Input from '$lib/components/utils/form/Input.svelte';
	import Button from '$lib/components/utils/button/Button.svelte';

	interface Props {
		overview: KanbanOverview;
		onsubmit: (e: SubmitEvent) => void;
	}

	let { overview, onsubmit }: Props = $props();
</script>

<Form {onsubmit}>
	<Input text="Label" name="update-kanban-{overview.id}-label" value={overview.label} />
	<Button iconName="update" class={['warning']} type="submit">Update</Button>
</Form>
