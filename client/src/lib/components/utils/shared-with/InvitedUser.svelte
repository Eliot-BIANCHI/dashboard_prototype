<script lang="ts">
	import { apiPostLoadRequest as apiRequest } from '$lib/api-request/ApiRequest';
	import { addToast } from '$lib/components/toaster/Toaster.svelte';
	import IconButton from '$lib/components/utils/button/IconButton.svelte';
	import Modal from '$lib/components/utils/Modal.svelte';
	import Button from '$lib/components/utils/button/Button.svelte';
	import Form from '$lib/components/utils/form/Form.svelte';
	import type { KanbanUser } from '$lib/components/kanbans/Kanban.svelte';
	import type { SharedWithContent } from '../types';

	const ASSETS_BASE_URL = import.meta.env.VITE_ASSETS_BASE_URL;

	interface Props {
		content?: {
			label: SharedWithContent;
			id: number;
		};
		hasPermissionToDelete: boolean;
		user: KanbanUser;
		userRemoved: (userId: number) => void;
	}

	let { user, content, hasPermissionToDelete, userRemoved }: Props = $props();

	let openUserSettings = $state(false);

	async function removeUser() {
		let res;

		if (content!.label === 'kanban') {
			res = await apiRequest.kanbans.removeUser({
				kanbanId: content!.id,
				userId: user.id
			});
		}

		if (res?.type === 'ok') {
			userRemoved(user.id);
			addToast({ message: 'User removed', type: 'success' });
		} else {
			addToast({ message: 'Failed to remove user', type: 'danger' });
		}
	}
</script>

<Modal isOpen={openUserSettings} onclose={() => (openUserSettings = false)}>
	<Form class={['no-border', 'no-padding-bottom']} onsubmit={removeUser}>
		<Button class={['danger']} iconName="delete" type="submit">Delete</Button>
	</Form>
</Modal>

<li class="user" data-role={user.kanbanRoleLabel}>
	{#if user.imageUrl === null}
		{@const initials = user.firstName.charAt(0) + user.lastName.charAt(0)}
		<span class="initials">{initials}</span>
	{:else}
		<img
			class="user-picture"
			src="{ASSETS_BASE_URL}/images/{user.imageUrl}"
			alt="User"
			width="50"
			height="50"
			draggable="false"
		/>
	{/if}
	<span class="user-full-name">
		{user.firstName}
		{user.lastName}
	</span>
	{#if hasPermissionToDelete}
		<span>
			<IconButton
				iconName="vertical-dots"
				class={['round']}
				onclick={() => (openUserSettings = true)}
			/>
		</span>
	{/if}
</li>

<style>
	.user {
		border: 2px solid var(--secondary-border);
		border-radius: 5px;
		display: flex;
		align-items: center;
		gap: 20px;
		padding: 10px 15px;
		position: relative;

		&::before {
			background-color: var(--quaternary-bg);
			content: attr(data-role);
			padding-inline: 5px;
			position: absolute;
			top: 0;
			right: 25px;
			transform: translateY(-50%);
		}

		& .user-picture {
			border-radius: 100px;
			display: block;
			min-height: 50px;
			min-width: 50px;
			object-fit: cover;
		}

		& .initials {
			background-color: var(--quaternary-bg);
			border-radius: 100px;
			display: inline-flex;
			align-items: center;
			justify-content: center;
			min-height: 50px;
			min-width: 50px;
		}

		& .user-full-name {
			flex-grow: 1;
		}
	}
</style>
