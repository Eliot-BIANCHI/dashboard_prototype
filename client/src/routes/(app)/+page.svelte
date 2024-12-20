<script lang="ts">
	let { data } = $props();
</script>

<svelte:head>
	<title>Home</title>
</svelte:head>

<section class="home">
	<h1>Bienvenue {data.account.firstName}!</h1>

	<div class="header">
		<section class="invitations-container">
			<h3>Invitations</h3>
			<!-- {#if data.pendingInvitations.kanbansInvitations.length > 0}
				<ul class="invitations">
					{#each data.pendingInvitations.kanbansInvitations as invitation}

					{/each}
				</ul>
			{:else}
				<p>No invitations received</p>
			{/if} -->
		</section>
	</div>
</section>

<style>
	.home {
		flex-grow: 1;
		padding-left: 20px;
	}

	.header {
		display: flex;
	}

	.invitations-container {
		background-color: var(--tertiary-bg);
		border-radius: 5px;
		font-size: 25px;
		max-height: 400px;
		overflow-y: auto;
		padding: 30px;

		& h3 {
			margin-top: 0;
			text-align: center;
		}

		.invitations {
			display: flex;
			flex-direction: column;
			gap: 15px;
			list-style: none;
			margin-block: 0;
			padding-left: 0;
		}
	}
</style>
