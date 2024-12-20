<script lang="ts">
	import Form from '$lib/components/utils/form/Form.svelte';
	import Input from '$lib/components/utils/form/Input.svelte';
	import Button from '$lib/components/utils/button/Button.svelte';

	interface Props {
		onsubmit: (e: SubmitEvent) => void;
	}

	let { onsubmit }: Props = $props();
</script>

<Form {onsubmit}>
	<Input text="Label" name="create-list-label" />
	<Button type="submit">Add</Button>
</Form>
