<script lang="ts">
	import Overview from '$lib/components/kanbans/kanban/Overview.svelte';
	import AddKanban from '$lib/components/kanbans/kanban/Add.svelte';

	import Button from '$lib/components/utils/button/Button.svelte';
	import Modal from '$lib/components/utils/Modal.svelte';
	import Sidebar from '$lib/components/utils/Sidebar.svelte';

	import { goto } from '$app/navigation';
	import { addToast } from '$lib/components/toaster/Toaster.svelte.js';

	import { apiPostLoadRequest as apiRequest } from '$lib/api-request/ApiRequest.js';

	let { children, data } = $props();

	async function addKanban(e: SubmitEvent) {
		const form = e.target as HTMLFormElement;
		const formData = new FormData(form);
		const kanbanData = {
			label: formData.get('create-kanban-label') as string,
			isShared: (formData.get('create-kanban-shared') as null | string) !== null
		};

		const res = await apiRequest.kanbans.add({
			data: { label: kanbanData.label, is_shared: kanbanData.isShared }
		});

		if (res.type === 'ok') {
			data.kanbans.addKanban({ ...kanbanData, id: res.value });

			addToast({ message: 'Kanban added', type: 'success' });
		} else {
			addToast({ message: 'Failed to add kanban', type: 'danger' });
		}

		form.reset();
		openAddKanban = false;
	}

	async function deleteKanban(kanbanId: number) {
		const res = await apiRequest.kanbans.delete({ kanbanId });

		if (res.type === 'ok') {
			data.kanbans.deleteKanban(kanbanId);

			if (localStorage.getItem(`${data.account.id}-favorites:kanban-id`) !== null) {
				data.account.setFavoriteKanbanId(null);
			}

			addToast({ message: 'Kanban deleted', type: 'success' });
			goto('/kanbans');
		} else {
			addToast({ message: 'Failed to delete kanban', type: 'danger' });
		}
	}

	let openAddKanban = $state(false);
</script>

<svelte:head>
	<title>Kanbans</title>
</svelte:head>

<Modal isOpen={openAddKanban} onclose={() => (openAddKanban = false)}>
	<AddKanban onsubmit={addKanban} />
</Modal>

<section class="kanbans-layout">
	<Sidebar>
		<nav class="kanbans-navbar">
			<Button iconName="add" class={['success']} onclick={() => (openAddKanban = true)}>
				Kanban
			</Button>
			<ul class="overviews">
				{#each data.kanbans.overviews as overview (overview.id)}
					<Overview {overview} deleteKanban={() => deleteKanban(overview.id)} />
				{/each}
			</ul>
		</nav>
	</Sidebar>

	{@render children()}
</section>

<style>
	.kanbans-layout {
		flex-grow: 1;
		display: flex;
		gap: 20px;
	}

	.kanbans-navbar {
		display: flex;
		flex-direction: column;
		gap: 30px;
	}

	.overviews {
		display: flex;
		flex-direction: column;
		gap: 15px;
		list-style: none;
		margin-block: 0;
		padding-left: 0;
	}
</style>
