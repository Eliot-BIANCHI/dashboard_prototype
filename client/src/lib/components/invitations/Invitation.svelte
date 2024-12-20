<script lang="ts">
	import Button from '$lib/components/utils/button/Button.svelte';
	import Invitation from '$lib/components/invitations/Invitation.svelte.js';
	import { apiPostLoadRequest as apiRequest } from '$lib/api-request/ApiRequest';
	import { addToast } from '$lib/components/toaster/Toaster.svelte';
	import { goto } from '$app/navigation';

	interface Props {
		invitation: Invitation;
	}

	let { invitation }: Props = $props();

	async function acceptInvitation(invitationId: number, join: boolean) {
		const res = await apiRequest.invitations.acceptKanban({ invitationId });

		if (res.type === 'ok') {
			if (join) {
				goto(`/kanbans/${invitationId}`);
			} else {
				addToast({ message: 'Invitation accepted', type: 'success' });
				// data.pendingInvitations.deletePendingKanbanInvitation(invitationId);
			}
		} else {
			addToast({ message: 'Failed to accept the invitation', type: 'danger' });
		}
	}

	async function rejectInvitation(invitationId: number) {
		const res = await apiRequest.invitations.rejectKanban({ invitationId });

		if (res.type === 'ok') {
			addToast({ message: 'Invitation rejected' });
			// data.pendingInvitations.deletePendingKanbanInvitation(invitationId);
		} else {
			addToast({ message: 'Failed to reject the invitation', type: 'danger' });
		}
	}
</script>

<li class="invitation">
	<p>
		{invitation.inviterFirstName}
		{invitation.inviterLastName}
		invited you to the {invitation.type} "{invitation.label}"
	</p>
	<div class="actions">
		<Button class={['success']} onclick={() => acceptInvitation(invitation.id, false)}
			>Accept</Button
		>
		<Button class={['success']} onclick={() => acceptInvitation(invitation.id, true)}>
			Accept and join
		</Button>
		<Button class={['danger']} onclick={() => rejectInvitation(invitation.id)}>Reject</Button>
	</div>
</li>

<style>
	.invitation {
		background-color: var(--primary-bg);
		border-radius: 5px;
		line-height: 1.5;
		max-width: 300px;
		padding: 0 15px 10px 15px;
		text-align: center;

		& .actions {
			display: flex;
			flex-direction: column;
			gap: 10px;
		}
	}
</style>
