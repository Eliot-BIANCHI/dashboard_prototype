<script lang="ts">
	import { apiPostLoadRequest as apiRequest } from '$lib/api-request/ApiRequest.js';
	import { addToast } from '$lib/components/toaster/Toaster.svelte.js';
	import List from '$lib/components/kanbans/list/List.svelte';
	import AddList from '$lib/components/kanbans/list/Add.svelte';

	import SharedWith from '$lib/components/utils/shared-with/SharedWith.svelte';
	import Button from '$lib/components/utils/button/Button.svelte';
	import Modal from '$lib/components/utils/Modal.svelte';
	import SortTasks from '$lib/components/kanbans/kanban/SortTasks.svelte';
	import type {
		SortTaskOption,
		SortTaskOptionValue,
		TaskDragged
	} from '$lib/components/kanbans/tools.js';
	import { onDestroy } from 'svelte';

	let { data } = $props();

	const WS_BASE_URL = import.meta.env.VITE_WS_BASE_URL;

	let ws: WebSocket | null = null;

	function connectWebSocket() {
		// Problème quand on passe d'un kanban à l'autre
		if (data.kanban.isShared) {
			ws = new WebSocket(`${WS_BASE_URL}/kanban`);

			ws.onopen = () => console.log('Kanban WebSocket connected');

			ws.onmessage = (event) => console.log('Kanban update:', event.data);

			ws.onclose = () => console.log('Kanban WebSocket disconnected');
		}
	}

	function sendMessageToServer(message: string) {
		if (ws !== null && ws.readyState === WebSocket.OPEN) {
			ws.send(JSON.stringify({ action: 'update', payload: message }));
			console.log('Message sent to server:', message);
		} else {
			console.error('WebSocket is not open.');
		}
	}

	onDestroy(() => {
		if (ws !== null) {
			ws.close();
			console.log('WebSocket connection closed');
		}
	});

	connectWebSocket();

	let openAddList = $state(false);

	const ascendingOrder = localStorage.getItem('kanban:sort-tasks-ascending-order');
	const value = localStorage.getItem('kanban:sort-tasks-value');

	let sortTasksBy: SortTaskOption = $state({
		ascendingOrder: ascendingOrder === null ? true : ascendingOrder === 'true',
		value: value === null ? 'alpha' : (value as SortTaskOptionValue)
	});

	let taskDragged: TaskDragged | undefined = $state();

	let openShowSharedUsers = $state(false);

	async function addList(e: SubmitEvent) {
		const form = e.target as HTMLFormElement;
		const formData = new FormData(form);

		const listData = {
			label: formData.get('create-list-label') as string
		};

		const res = await apiRequest.lists.add({
			kanbanId: data.kanban.id,
			data: { label: listData.label, kanban_id: data.kanban.id }
		});

		if (res.type === 'ok') {
			data.kanban.addList({
				id: res.value,
				label: listData.label,
				ownerId: data.account.id,
				tasks: []
			});
			addToast({ message: 'List added', type: 'success' });

			form.reset();
			openAddList = false;
		} else {
			addToast({ message: 'Failed to add the list', type: 'danger' });
		}
	}

	async function changeTaskList(newListId: number) {
		const dragged = taskDragged!;

		const res = await apiRequest.tasks.patchList({
			kanbanId: data.kanban.id,
			listId: dragged.listId,
			taskId: dragged.id,
			data: { list_id: newListId }
		});

		if (res.type === 'ok') {
			const currentList = data.kanban.lists.find((list) => list.id === dragged.listId)!;
			const index = currentList.tasks.findIndex((task) => task.id === dragged.id);
			const task = currentList.tasks.splice(index, 1)[0];

			const newList = data.kanban.lists.find((list) => list.id === newListId)!;
			newList.tasks.push(task);
		} else {
			addToast({ message: 'Failed to move the task', type: 'danger' });
		}
	}

	async function deleteList(listId: number) {
		const res = await apiRequest.lists.delete({ kanbanId: data.kanban.id, listId });

		if (res.type === 'ok') {
			data.kanban.deleteList(listId);
			addToast({ message: 'List deleted', type: 'success' });
		} else {
			addToast({ message: 'Failed to delete the list', type: 'danger' });
		}
	}
</script>

<Modal isOpen={openAddList} onclose={() => (openAddList = false)}>
	<AddList onsubmit={addList} />
</Modal>

<Modal isOpen={openShowSharedUsers} onclose={() => (openShowSharedUsers = false)} class={['right']}>
	<SharedWith
		content={{ id: data.kanban.id, label: 'kanban' }}
		owner={data.kanban.owner}
		userPermissions={data.userPermissions}
		users={data.kanban.sharedWith}
		userRemoved={(userId) => data.kanban.removeUser(userId)}
	/>
</Modal>

<div class="kanban">
	<div class="utils">
		{#if data.userPermissions.includes('list:create')}
			<Button onclick={() => (openAddList = true)} iconName="add" class={['success']}>List</Button>
		{/if}
		<SortTasks by={sortTasksBy} setBy={(option) => (sortTasksBy = option)} />
		{#if data.kanban.isShared}
			<Button iconName="eye-opened" onclick={() => (openShowSharedUsers = true)}>Users</Button>
		{/if}
	</div>

	<ul class="columns">
		{#each data.kanban.lists as list (list.id)}
			<List
				kanbanId={data.kanban.id}
				{list}
				setTaskDragged={(task) => (taskDragged = task)}
				{changeTaskList}
				userPermissions={data.userPermissions}
				account={data.account}
				deleteList={() => deleteList(list.id)}
				{sortTasksBy}
				isShared={data.kanban.isShared}
			/>
		{/each}
	</ul>
</div>

<style>
	.kanban {
		flex-grow: 1;
		margin-block: 16px;
		margin-right: 30px;
		overflow-y: auto;

		& > .utils {
			display: flex;
			align-items: center;
			flex-wrap: wrap;
			gap: 30px;
		}
	}

	.columns {
		display: flex;
		flex-direction: column;
		gap: 20px;
		list-style: none;
		padding-left: 0;
	}
</style>
