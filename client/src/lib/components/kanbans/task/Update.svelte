<script lang="ts">
	import Form from '../../utils/form/Form.svelte';
	import Input from '../../utils/form/Input.svelte';
	import Select from '../../utils/form/Select.svelte';
	import Toggle from '$lib/components/utils/form/Toggle.svelte';
	import Task, { completions } from '../Task.svelte';
	import Button from '$lib/components/utils/button/Button.svelte';

	interface Props {
		task: Task;
		onsubmit: (e: SubmitEvent) => void;
	}

	let { task, onsubmit }: Props = $props();
</script>

<Form {onsubmit}>
	<Input text="Label" name="update-task-{task.id}-label" value={task.label} />
	<Select
		options={completions.map((completion) => ({
			text: completion.label,
			value: completion.id,
			isSelected: completion.id === task.completionId
		}))}
		text="Completion"
		name="update-task-{task.id}-completion"
	/>
	<Toggle name="update-task-{task.id}-priority" text="Priority?" isChecked={task.priority} />
	<Button iconName="update" class={['warning']} type="submit">Update</Button>
</Form>
