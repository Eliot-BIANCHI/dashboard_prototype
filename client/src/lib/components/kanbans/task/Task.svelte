<script lang="ts">
	import type Task from '../Task.svelte';
	import type Account from '$lib/Account.svelte';
	import { apiPostLoadRequest as apiRequest } from '$lib/api-request/ApiRequest';
	import { addToast } from '$lib/components/toaster/Toaster.svelte';
	import { completions } from '../Task.svelte';
	import Avatar from '$lib/components/utils/Avatar.svelte';
	import Progress from './Progress.svelte';
	import UpdateCard from './Update.svelte';
	import type { KanbanPermission } from '$lib/tools';

	import Form from '$lib/components/utils/form/Form.svelte';
	import Button from '$lib/components/utils/button/Button.svelte';
	import Icon from '$lib/components/utils/Icon.svelte';
	import IconButton from '$lib/components/utils/button/IconButton.svelte';
	import Modal from '$lib/components/utils/Modal.svelte';
	import { Intl } from '@js-temporal/polyfill';

	interface Props {
		kanbanId: number;
		listId: number;
		task: Task;
		account: Account;
		userPermissions: KanbanPermission[];
		onDrag: (taskId: number) => void;
		onDrop: () => void;
		deleteTask: () => void;
		isShared: boolean;
	}

	let {
		kanbanId,
		listId,
		task,
		account,
		userPermissions,
		onDrag,
		onDrop,
		deleteTask,
		isShared
	}: Props = $props();

	let openTaskSettings = $state(false);

	let isDragged = $state(false);

	async function updateTask(e: SubmitEvent) {
		const formData = new FormData(e.target as HTMLFormElement);
		const taskData = {
			label: formData.get(`update-task-${task.id}-label`) as string,
			completionId: parseInt(formData.get(`update-task-${task.id}-completion`) as string),
			priority: (formData.get(`update-task-${task.id}-priority`) as null | string) !== null
		};

		const res = await apiRequest.tasks.update({
			kanbanId,
			listId,
			taskId: task.id,
			data: {
				label: taskData.label,
				completion_id: taskData.completionId,
				priority: taskData.priority
			}
		});

		if (res.type === 'ok') {
			task.update(taskData);
			addToast({ message: 'Task updated', type: 'success' });

			openTaskSettings = false;
		} else {
			addToast({ message: 'Failed to update the task', type: 'danger' });
		}
	}

	async function increaseCompletion() {
		const newIndex = completions.findIndex((completion) => completion.id === task.completionId) + 1;
		if (newIndex === completions.length) return;

		const completionId = completions[newIndex].id;
		const res = await apiRequest.tasks.patchCompletion({
			kanbanId,
			listId,
			taskId: task.id,
			data: { completion_id: completionId }
		});

		if (res.type === 'ok') {
			task.increaseCompletion(newIndex);
		} else {
			addToast({ message: 'Failed to change completion', type: 'danger' });
		}
	}

	async function assignUser() {
		const res = await apiRequest.tasks.assignUser({
			kanbanId,
			listId,
			taskId: task.id,
			data: { user_id: account.id }
		});

		if (res.type === 'ok') {
			task.assignUser({
				id: account.id,
				firstName: account.firstName,
				lastName: account.lastName,
				imageUrl: account.imageUrl
			});

			addToast({ message: 'Assigned to task', type: 'success' });

			openTaskSettings = false;
		} else {
			addToast({ message: 'Failed to assign', type: 'danger' });
		}
	}

	async function unassignUser() {
		const res = await apiRequest.tasks.unassignUser({
			kanbanId,
			listId,
			taskId: task.id,
			data: { user_id: account.id }
		});

		if (res.type === 'ok') {
			task.unassignUser(account.id);

			addToast({ message: 'Unassigned', type: 'success' });

			openTaskSettings = false;
		} else {
			addToast({ message: 'Failed to unassign', type: 'danger' });
		}
	}

	const dateFormatter = new Intl.DateTimeFormat('en-US', {
		year: '2-digit',
		month: '2-digit',
		day: '2-digit'
	});
</script>

<Modal isOpen={openTaskSettings} onclose={() => (openTaskSettings = false)}>
	{#if isShared}
		<span class="created-by">
			{#if task.owner.id === account.id}
				Created by you
			{:else}
				Created by {task.owner.firstName} {task.owner.lastName}
			{/if}
		</span>
	{/if}

	<UpdateCard {task} onsubmit={updateTask} />

	{#if isShared && userPermissions.includes('task:unassign-self') && task.assignees.some((assignee) => assignee.id === account.id)}
		<Form class={['no-border', 'no-padding-bottom']} onsubmit={unassignUser}>
			<Button type="submit">Unassign</Button>
		</Form>
	{:else if isShared && userPermissions.includes('task:assign-self')}
		<Form class={['no-border', 'no-padding-bottom']} onsubmit={assignUser}>
			<Button type="submit">Assign</Button>
		</Form>
	{/if}

	{#if userPermissions.includes('task:delete-all') || (userPermissions.includes('task:delete-own') && task.owner.id === account.id)}
		<Form class={['no-border', 'no-padding-bottom']} onsubmit={deleteTask}>
			<Button class={['danger']} iconName="delete" type="submit">Delete</Button>
		</Form>
	{/if}
</Modal>

<li
	class="task"
	class:priority={task.priority}
	class:dragged={isDragged}
	draggable="true"
	ondragstart={() => {
		onDrag(task.id);
		isDragged = true;
	}}
	ondragend={() => {
		onDrop();
		isDragged = false;
	}}
>
	{#if task.priority}
		<span class="priority">
			<Icon name="exclamation-mark" />
		</span>
	{/if}

	<span class="created-at">{dateFormatter.format(task.createdAt)}</span>

	{#if isShared}
		{#if task.assignees.length > 0}
			<div class="avatars">
				{#each task.assignees as assignee}
					<Avatar user={assignee} />
				{/each}
			</div>
		{:else}
			<Button
				class={['align-self-center', 'round', 'grid-area-no-assignee']}
				isDisabled={!userPermissions.includes('task:assign-self')}
				onclick={assignUser}>?</Button
			>
		{/if}
	{/if}

	<p class="label">{task.label}</p>

	<Progress
		hasPermissionToIncrease={userPermissions.includes('task:update-all') ||
			(userPermissions.includes('task:update-own') && task.owner.id === account.id)}
		taskCompletionId={task.completionId}
		{increaseCompletion}
	/>

	{#if userPermissions.includes('task:update-all') || (userPermissions.includes('task:update-own') && task.owner.id === account.id)}
		<IconButton
			iconName="vertical-dots"
			class={['right-border-radius', 'grid-area-settings']}
			onclick={() => (openTaskSettings = true)}
		/>
	{/if}
</li>

<style>
	.created-by {
		display: inline-block;
		font-size: 22px;
		text-align: center;
		padding-block: 10px;
		width: 100%;
	}

	.task {
		background-color: var(--primary-bg);
		outline: 2px solid transparent;
		border-radius: 10px;
		display: flex;
		gap: 20px;
		min-height: 60px;
		padding-left: 10px;
		position: relative;

		&.priority {
			outline-color: var(--danger-color);
		}

		&.dragged {
			opacity: 0.5;
		}

		& .priority {
			align-self: center;
			grid-area: status;
		}

		& .created-at {
			align-self: center;
			font-size: 22px;
			grid-area: date;
		}

		& .label {
			align-self: center;
			font-size: 25px;
			flex-grow: 1;
			grid-area: label;
			margin-block: 0;
		}

		& .avatars {
			display: flex;
			gap: 5px;
			grid-area: users;
		}
	}

	@media all and (width < 1300px) {
		.task {
			align-self: center;
			display: grid;
			grid-template-columns: repeat(5, 60px);
			grid-template-rows: repeat(4, 40px);
			grid-template-areas:
				'status date date date settings'
				'label label label label users'
				'label label label label users'
				'progress progress progress progress users';
			padding: 10px;
			width: max-content;

			& .priority {
				justify-self: center;
			}

			& .label {
				line-height: 1.35;
				padding-left: 20px;
			}

			& .avatars {
				flex-direction: column;
				gap: 0px;
				overflow-y: scroll;
			}
		}
	}
</style>
