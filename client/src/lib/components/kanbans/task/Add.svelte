<script lang="ts">
	import Form from '../../utils/form/Form.svelte';
	import Input from '$lib/components/utils/form/Input.svelte';
	import Select from '$lib/components/utils/form/Select.svelte';
	import Button from '$lib/components/utils/button/Button.svelte';
	import Toggle from '$lib/components/utils/form/Toggle.svelte';
	import { completions } from '../Task.svelte';

	interface Props {
		listId: number;
		onsubmit: (e: SubmitEvent) => void;
	}

	let { listId, onsubmit }: Props = $props();
</script>

<Form {onsubmit}>
	<Input text="Label" name="{listId}-create-task-label" />
	<Select
		options={completions.map((completion) => ({
			text: completion.label,
			value: completion.id
		}))}
		text="Completion"
		name="{listId}-create-task-completion"
	/>
	<Toggle name="{listId}-create-task-priority" text="Priority?" />
	<Button type="submit" class={['success']}>Add task</Button>
</Form>
