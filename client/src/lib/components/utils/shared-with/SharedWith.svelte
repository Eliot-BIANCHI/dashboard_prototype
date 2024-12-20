<script lang="ts">
	import Owner from './Owner.svelte';
	import InvitedUser from './InvitedUser.svelte';
	import { apiPostLoadRequest as apiRequest } from '$lib/api-request/ApiRequest';
	import { addToast } from '$lib/components/toaster/Toaster.svelte';
	import Button from '$lib/components/utils/button/Button.svelte';
	import Modal from '$lib/components/utils/Modal.svelte';

	import Form from '$lib/components/utils/form/Form.svelte';
	import Input from '$lib/components/utils/form/Input.svelte';
	import Select from '$lib/components/utils/form/Select.svelte';

	import type { User } from '$lib/Account.svelte';
	import type { KanbanUser } from '$lib/components/kanbans/Kanban.svelte';
	import type { SharedWithContent } from '../types';
	import { debounce } from '$lib/tools';
	import type { KanbanPermission } from '$lib/tools';

	interface Props {
		content: {
			label: SharedWithContent;
			id: number;
		};
		owner: KanbanUser;
		userPermissions: KanbanPermission[];
		users: KanbanUser[];
		userRemoved: (userId: number) => void;
	}

	let openAddUser = $state(false);

	let { content, owner, userPermissions, users, userRemoved }: Props = $props();

	let overviews: User[] = $state([]);

	const getUsers = debounce(async (search: string) => {
		if (search.length === 0) {
			overviews = [];
			return;
		}
		const res = await apiRequest.users.getOverviews({
			query: { search, by: 'username' }
		});

		if (res.type === 'ok') {
			overviews = res.value.filter(
				(overview) => overview.id !== owner.id && !users.some((user) => user.id === overview.id)
			);
		} else {
			overviews = [];
		}
	}, 500);

	async function inviteUser(e: SubmitEvent) {
		const form = e.target as HTMLFormElement;
		const formData = new FormData(form);

		const invitationData = {
			kanbanRoleId: parseInt(formData.get('invitation-user-role') as string),
			inviteeId: parseInt(formData.get('invitation-invitee') as string)
		};

		let res;

		if (content.label === 'kanban') {
			res = await apiRequest.kanbans.inviteUser({
				data: {
					kanban_id: content.id,
					invitee_id: invitationData.inviteeId,
					kanban_role_id: invitationData.kanbanRoleId
				}
			});
		}

		if (res?.type === 'ok') {
			addToast({ message: 'invitation sent', type: 'success' });

			overviews = [];
			form.reset();
		} else {
			addToast({ message: 'Failed to send invitation', type: 'danger' });
		}
	}
</script>

{#if userPermissions.includes('kanban:invite-user')}
	<Modal isOpen={openAddUser} onclose={() => (openAddUser = false)}>
		<Form onsubmit={inviteUser}>
			{#await apiRequest.roles.getKanbans() then roles}
				<Select
					text="User role"
					name="invitation-user-role"
					options={roles.map((role) => ({ text: role.label, value: role.id }))}
				/>
			{:catch error}
				<span>{error}</span>
			{/await}

			<Input
				name="search"
				text="Search by username"
				oninput={(e) => getUsers(e.currentTarget.value)}
			/>

			{#if overviews.length > 0}
				<Select
					text="Users found"
					name="invitation-invitee"
					options={overviews.map((overview) => ({
						text: `${overview.firstName} ${overview.lastName}`,
						value: overview.id
					}))}
				/>
			{:else}
				<p class="no-user-found">No user found...</p>
			{/if}

			<Button type="submit">Invite user</Button>
		</Form>
	</Modal>
{/if}

<div class="shared-with">
	{#if userPermissions.includes('kanban:invite-user')}
		<Button iconName="add" class={['success']} onclick={() => (openAddUser = true)}>Invite</Button>
	{/if}
	<ul class="users">
		<Owner {owner} />
		{#each users as user}
			<InvitedUser
				{user}
				hasPermissionToDelete={userPermissions.includes('kanban:delete-user')}
				{content}
				{userRemoved}
			/>
		{/each}
	</ul>
</div>

<style>
	.shared-with {
		display: flex;
		flex-direction: column;
		gap: 30px;
	}

	.users {
		display: flex;
		flex-direction: column;
		gap: 20px;
		font-size: 22px;
		list-style: none;
		margin-block: 0;
		padding-left: 0;
	}

	.no-user-found {
		font-size: 22px;
		text-align: center;
	}
</style>
