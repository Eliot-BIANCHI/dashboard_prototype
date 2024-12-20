<script lang="ts">
	import { page } from '$app/state';
	import { KanbanOverview } from '../Kanbans.svelte';
	import UpdateKanban from './Update.svelte';
	import { apiPostLoadRequest as apiRequest } from '$lib/api-request/ApiRequest';
	import { addToast } from '$lib/components/toaster/Toaster.svelte';

	import Button from '$lib/components/utils/button/Button.svelte';
	import IconButton from '$lib/components/utils/button/IconButton.svelte';
	import Modal from '$lib/components/utils/Modal.svelte';
	import Form from '$lib/components/utils/form/Form.svelte';

	interface Props {
		overview: KanbanOverview;
		deleteKanban: () => void;
	}

	let { overview, deleteKanban }: Props = $props();

	async function updateKanban(e: SubmitEvent) {
		const formData = new FormData(e.target as HTMLFormElement);
		const kanbanData = {
			label: formData.get(`update-kanban-${overview.id}-label`) as string
		};

		const res = await apiRequest.kanbans.update({
			kanbanId: overview.id,
			data: { label: kanbanData.label }
		});

		if (res.type === 'ok') {
			overview.update(kanbanData.label);

			addToast({ message: 'Kanban updated', type: 'success' });
		} else {
			addToast({ message: 'Failed to update kanban', type: 'danger' });
		}

		openKanbanSettings = false;
	}

	let openKanbanSettings = $state(false);

	const href = `/kanbans/${overview.id}`;
</script>

<Modal isOpen={openKanbanSettings} onclose={() => (openKanbanSettings = false)}>
	<UpdateKanban {overview} onsubmit={updateKanban} />
	<Form class={['no-border', 'no-padding-bottom']} onsubmit={deleteKanban}>
		<Button class={['danger']} iconName="delete" type="submit">Delete</Button>
	</Form>
</Modal>

<li class="overview">
	<a {href} class:active={page.url.pathname === href}>
		{overview.label}
	</a>
	<IconButton
		iconName="vertical-dots"
		class={['right-border-radius']}
		onclick={() => (openKanbanSettings = true)}
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
			gap: 20px;
			flex-grow: 1;
			font-size: 23px;
			padding: 15px;
			text-decoration: none;

			&:hover,
			&:focus,
			&.active {
				background-color: var(--secondary-bg);
			}
		}
	}
</style>
