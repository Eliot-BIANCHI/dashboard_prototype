<script lang="ts">
	import Form from '$lib/components/utils/form/Form.svelte';
	import Input from '$lib/components/utils/form/Input.svelte';
	import Button from '$lib/components/utils/button/Button.svelte';
	import List from '../List.svelte';

	interface Props {
		list: List;
		onsubmit: (e: SubmitEvent) => void;
	}

	let { list, onsubmit }: Props = $props();
</script>

<Form {onsubmit}>
	<Input text="Label" name="update-list-{list.id}-label" value={list.label} />
	<Button class={['warning']} iconName="update" type="submit">Update</Button>
</Form>
