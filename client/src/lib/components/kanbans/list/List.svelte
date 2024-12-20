<script lang="ts">
	import { apiPostLoadRequest as apiRequest } from '$lib/api-request/ApiRequest';
	import type Account from '$lib/Account.svelte';
	import type List from '../List.svelte';
	import Task from '../task/Task.svelte';
	import UpdateList from './Update.svelte';
	import AddTask from '../task/Add.svelte';
	import Modal from '$lib/components/utils/Modal.svelte';
	import type { User } from '$lib/Account.svelte';
	import type { KanbanPermission } from '$lib/tools';

	import Form from '$lib/components/utils/form/Form.svelte';
	import Button from '$lib/components/utils/button/Button.svelte';
	import IconButton from '$lib/components/utils/button/IconButton.svelte';
	import type { SortTaskOption, TaskDragged } from '../tools';
	import { Temporal } from '@js-temporal/polyfill';

	import { addToast } from '$lib/components/toaster/Toaster.svelte';

	interface Props {
		kanbanId: number;
		list: List;
		account: Account;
		userPermissions: KanbanPermission[];
		setTaskDragged: (task: TaskDragged | undefined) => void;
		changeTaskList: (newListId: number) => void;
		deleteList: () => void;
		sortTasksBy: SortTaskOption;
		isShared: boolean;
	}

	let {
		kanbanId,
		list,
		account,
		userPermissions,
		setTaskDragged,
		changeTaskList,
		deleteList,
		sortTasksBy,
		isShared
	}: Props = $props();

	let isDraggedOver = $state(false);

	let openAddTask = $state(false);
	let openListSettings = $state(false);

	let tasksSorted = $derived.by(() => {
		switch (sortTasksBy.value) {
			case 'date':
				if (sortTasksBy.ascendingOrder) {
					return list.tasks.toSorted((a, b) =>
						Temporal.PlainDateTime.compare(a.createdAt, b.createdAt)
					);
				} else {
					return list.tasks.toSorted((a, b) =>
						Temporal.PlainDateTime.compare(b.createdAt, a.createdAt)
					);
				}
			case 'progress':
				if (sortTasksBy.ascendingOrder) {
					return list.tasks.toSorted((a, b) => a.completionId - b.completionId);
				} else {
					return list.tasks.toSorted((a, b) => b.completionId - a.completionId);
				}
			case 'priority':
				if (sortTasksBy.ascendingOrder) {
					return list.tasks.toSorted((a, b) => Number(b.priority) - Number(a.priority));
				} else {
					return list.tasks.toSorted((a, b) => Number(a.priority) - Number(b.priority));
				}
			default:
				if (sortTasksBy.ascendingOrder) {
					return list.tasks.toSorted((a, b) => a.label.localeCompare(b.label));
				} else {
					return list.tasks.toSorted((a, b) => b.label.localeCompare(a.label));
				}
		}
	});

	let tasksCompletion = $derived(
		(
			(list.tasks.reduce((sum, task) => {
				if (task.completionId === 3) sum++;
				return sum;
			}, 0) /
				list.tasks.length) *
			100
		).toFixed()
	);

	async function addTask(e: SubmitEvent) {
		const form = e.target as HTMLFormElement;
		const formData = new FormData(form);
		const taskData = {
			label: formData.get(`${list.id}-create-task-label`) as string,
			completionId: parseInt(formData.get(`${list.id}-create-task-completion`) as string),
			priority: (formData.get(`${list.id}-create-task-priority`) as null | string) !== null
		};

		const res = await apiRequest.tasks.add({
			kanbanId,
			listId: list.id,
			data: {
				label: taskData.label,
				list_id: list.id,
				completion_id: taskData.completionId,
				priority: taskData.priority
			}
		});

		if (res.type === 'ok') {
			const owner: User = {
				id: account.id,
				firstName: account.firstName,
				lastName: account.lastName,
				imageUrl: account.imageUrl
			};

			list.addTask({
				...taskData,
				id: res.value,
				owner,
				createdAt: Temporal.Now.plainDateISO(),
				assignees: []
			});

			addToast({ message: 'Task added', type: 'success' });
		} else {
			addToast({ message: 'Failed to add task', type: 'danger' });
		}

		form.reset();
		openAddTask = false;
	}

	async function updateList(e: SubmitEvent) {
		const formData = new FormData(e.target as HTMLFormElement);
		const data = {
			label: formData.get(`update-list-${list.id}-label`) as string
		};

		const res = await apiRequest.lists.update({
			kanbanId,
			listId: list.id,
			data: { label: data.label }
		});

		if (res.type === 'ok') {
			list.update(data.label);
			addToast({ message: 'List updated', type: 'success' });
		} else {
			addToast({ message: 'Failed to update the list', type: 'danger' });
		}

		openListSettings = false;
	}

	async function deleteTask(taskId: number) {
		const res = await apiRequest.tasks.delete({ kanbanId, listId: list.id, taskId });

		if (res.type === 'ok') {
			list.deleteTask(taskId);
			addToast({ message: 'Task deleted', type: 'success' });
		} else {
			addToast({ message: 'Failed to delete the task', type: 'danger' });
		}
	}
</script>

<Modal isOpen={openAddTask} onclose={() => (openAddTask = false)}>
	<AddTask listId={list.id} onsubmit={addTask} />
</Modal>

<Modal isOpen={openListSettings} onclose={() => (openListSettings = false)}>
	<UpdateList {list} onsubmit={updateList} />
	{#if userPermissions.includes('list:delete-all') || (userPermissions.includes('list:delete-own') && list.ownerId === account.id)}
		<Form class={['no-border', 'no-padding-bottom']} onsubmit={deleteList}>
			<Button class={['danger']} iconName="delete" type="submit">Delete</Button>
		</Form>
	{/if}
</Modal>

<li
	class="list"
	class:dragged-over={isDraggedOver}
	ondrop={() => {
		isDraggedOver = false;
		changeTaskList(list.id);
	}}
	ondragleave={() => (isDraggedOver = false)}
	ondragover={(e) => {
		isDraggedOver = true;
		e.preventDefault();
	}}
>
	<div class="header">
		<span class="label">{list.label}</span>
		{#if userPermissions.includes('task:create')}
			<IconButton onclick={() => (openAddTask = true)} class={['round']} iconName="add" />
		{/if}
		{#if userPermissions.includes('list:update-all') || (userPermissions.includes('list:update-own') && list.ownerId === account.id)}
			<IconButton
				onclick={() => (openListSettings = true)}
				iconName="vertical-dots"
				class={['round']}
			/>
		{/if}

		<!-- <IconButton iconName="chevron-up" />

		<IconButton iconName="chevron-down" /> -->

		{#if tasksCompletion !== 'NaN'}
			<span class="tasks-completion">{tasksCompletion} %</span>
		{/if}
	</div>
	<ul class="tasks">
		{#each tasksSorted as task (task.id)}
			<Task
				{account}
				{kanbanId}
				{userPermissions}
				listId={list.id}
				{task}
				onDrag={(taskId) => setTaskDragged({ id: taskId, listId: list.id })}
				onDrop={() => setTaskDragged(undefined)}
				deleteTask={() => deleteTask(task.id)}
				{isShared}
			/>
		{/each}
	</ul>
</li>

<style>
	.list {
		border-radius: 10px;
		padding: 15px;
		transition: background-color 200ms;

		&.dragged-over {
			background-color: var(--quaternary-bg);
		}
	}

	.header {
		display: flex;
		align-items: center;
		flex-wrap: wrap;
		gap: 10px;
		padding-block: 10px;

		& .label {
			font-size: 22px;
		}

		& .tasks-completion {
			color: var(--success-color);
			font-size: 25px;
			margin-left: auto;
		}
	}

	.tasks {
		display: flex;
		flex-direction: column;
		gap: 10px;
		padding-left: 0;
	}
</style>
