<script lang="ts">
	import type { User } from '$lib/Account.svelte';

	const ASSETS_BASE_URL = import.meta.env.VITE_ASSETS_BASE_URL;

	interface Props {
		user: User;
	}

	let { user }: Props = $props();

	let initials = $derived(user.firstName.charAt(0) + user.lastName.charAt(0));

	let name: HTMLDialogElement;
</script>

<span class="user" onpointerenter={() => name.show()} onpointerleave={() => name.close()}>
	<dialog class="user-full-name" bind:this={name}>
		{user.firstName}
		{user.lastName}
	</dialog>
	{#if user.imageUrl === null}
		<span class="initials">{initials}</span>
	{:else}
		<img
			class="user-picture"
			src="{ASSETS_BASE_URL}/images/{user.imageUrl}"
			alt="Avatar"
			width="50"
			height="50"
			draggable="false"
		/>
	{/if}
</span>

<style>
	.user {
		padding-block: 5px;
		position: relative;

		& .user-full-name {
			background-color: var(--primary-bg);
			border: 1px solid darkgray;
			border-radius: 5px;
			outline: none;
			padding-block: 5px;
			padding-inline: 7px;
			top: -45px;
			left: 50%;
			translate: -50%;
			white-space: nowrap;

			&::before {
				border-top: 11px solid darkgrey;
				border-right: 8px solid transparent;
				border-left: 8px solid transparent;
				content: '';
				position: absolute;
				bottom: -12px;
				left: 50%;
				translate: -50%;
				z-index: -1;
			}

			&::after {
				border-top: 11px solid var(--primary-bg);
				border-right: 8px solid transparent;
				border-left: 8px solid transparent;
				content: '';
				position: absolute;
				bottom: -11px;
				left: 50%;
				translate: -50%;
			}
		}

		& .user-picture {
			border-radius: 100px;
			display: block;
			object-fit: cover;
		}

		& .initials {
			background-color: var(--secondary-bg);
			border-radius: 100px;
			display: inline-flex;
			align-items: center;
			justify-content: center;
			font-size: 20px;
			height: 50px;
			width: 50px;
		}
	}

	@media all and (width < 1300px) {
		.user-full-name {
			display: none;
		}
	}
</style>
