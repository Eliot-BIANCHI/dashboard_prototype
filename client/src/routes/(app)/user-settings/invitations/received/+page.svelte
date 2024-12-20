<script lang="ts">
	let { data } = $props();
</script>

<section class="invitations">
	<!-- {#if data.invitations.kanbansInvitations.length > 0}
		<ul>
			{#each data.invitations.kanbansInvitations as invitation}
				<li>
					{invitation.inviterFirstName}
					{invitation.inviterLastName}
					invited you to the kanban "{invitation.kanbanLabel}"
				</li>
			{/each}
		</ul>
	{:else}
		<p>No invitation received</p>
	{/if} -->
</section>

<style>
	.invitations {
		align-self: center;
		font-size: 25px;
	}
</style>
