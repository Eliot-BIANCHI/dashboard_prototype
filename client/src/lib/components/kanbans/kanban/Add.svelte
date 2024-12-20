<script lang="ts">
	import Form from '../../utils/form/Form.svelte';
	import Input from '../../utils/form/Input.svelte';
	import Toggle from '../../utils/form/Toggle.svelte';
	import Button from '../../utils/button/Button.svelte';

	interface Props {
		onsubmit: (e: SubmitEvent) => void;
	}

	let { onsubmit }: Props = $props();
</script>

<Form {onsubmit}>
	<Input text="Label" name="create-kanban-label" />
	<Toggle text="Shared?" name="create-kanban-shared" />
	<Button type="submit">Add</Button>
</Form>
