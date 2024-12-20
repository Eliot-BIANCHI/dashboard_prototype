<script lang="ts">
	import type { KanbanUser } from '$lib/components/kanbans/Kanban.svelte';

	const ASSETS_BASE_URL = import.meta.env.VITE_ASSETS_BASE_URL;

	interface Props {
		owner: KanbanUser;
	}

	let { owner }: Props = $props();
</script>

<li class="owner" data-role={owner.kanbanRoleLabel}>
	{#if owner.imageUrl === null}
		{@const initials = owner.firstName.charAt(0) + owner.lastName.charAt(0)}
		<span class="initials">{initials}</span>
	{:else}
		<img
			class="owner-picture"
			src="{ASSETS_BASE_URL}/images/{owner.imageUrl}"
			alt="Owner"
			width="50"
			height="50"
			draggable="false"
		/>
	{/if}
	<span class="owner-full-name">
		{owner.firstName}
		{owner.lastName}
	</span>
</li>

<style>
	.owner {
		border: 2px solid var(--app-color);
		border-radius: 5px;
		display: flex;
		align-items: center;
		gap: 20px;
		padding: 10px 15px;
		position: relative;

		&::before {
			background-color: var(--app-color);
			content: attr(data-role);
			padding-inline: 5px;
			position: absolute;
			top: 0;
			right: 25px;
			transform: translateY(-50%);
		}

		& .owner-picture {
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

		& .owner-full-name {
			flex-grow: 1;
		}
	}
</style>
