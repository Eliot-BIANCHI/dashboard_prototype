<script lang="ts">
	import { page } from '$app/state';
	import Theme from './Theme.svelte';
	import IconButton from '$lib/components/utils/button/IconButton.svelte';
	import Icon from '$lib/components/utils/Icon.svelte';
	import navbar from './Navbar.svelte';
	import type { Permission } from '$lib/tools';

	interface Props {
		userPermissions: Permission[];
		favoriteCalendarId: string | null;
		favoriteKanbanId: string | null;
	}

	let { userPermissions, favoriteCalendarId, favoriteKanbanId }: Props = $props();

	$effect(() => navbar.setCalendarRoute(favoriteCalendarId));
	$effect(() => navbar.setKanbanRoute(favoriteKanbanId));

	let isOpen = $state(false);
</script>

<nav class="navbar-preview">
	<IconButton iconName="hamburger-menu" onclick={() => (isOpen = true)} />

	<ul class="navbar-links">
		{#each navbar.links as { href, iconName, isHidden, permissionRequired }}
			{#if isHidden === false && (permissionRequired === undefined || userPermissions.includes(permissionRequired))}
				{@const match = href.match(/^\/[^/]+/)}
				<li class="link-preview">
					<a
						{href}
						class:active={match === null
							? page.url.pathname === '/'
							: page.url.pathname.startsWith(match[0])}
					>
						<Icon name={iconName} />
					</a>
				</li>
			{/if}
		{/each}
	</ul>

	<Theme />
</nav>

<nav class="app-navbar" class:active={isOpen}>
	<IconButton iconName="navbar-cross" onclick={() => (isOpen = false)} />

	<ul class="navbar-links">
		{#each navbar.links as { href, iconName, routeName, isHidden, permissionRequired }}
			{#if isHidden === false && (permissionRequired === undefined || userPermissions.includes(permissionRequired))}
				{@const match = href.match(/^\/[^/]+/)}
				<li class="navbar-link">
					<a
						{href}
						class:active={match === null
							? page.url.pathname === '/'
							: page.url.pathname.startsWith(match[0])}
					>
						<Icon name={iconName} />
						{routeName}
					</a>
				</li>
			{/if}
		{/each}
	</ul>
</nav>

<style>
	.app-navbar,
	.navbar-preview {
		background-color: var(--primary-bg);
		box-sizing: border-box;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 20px;
		height: 100dvh;
		margin-left: auto;
		min-width: 95px;
		overflow-y: auto;
		padding: 10px;
	}

	.app-navbar {
		position: fixed;
		right: 0;
		transition: translate 250ms;
		translate: 100%;

		&.active {
			translate: 0%;
		}
	}

	.navbar-links {
		display: flex;
		flex-direction: column;
		list-style: none;
		margin-block: 0;
		padding-left: 0;
	}

	.link-preview {
		& a {
			display: inline-block;
			height: 35px;
			outline-offset: 2px;
			padding: 10px;
		}
	}

	.navbar-link {
		& a {
			color: var(--primary-text);
			display: flex;
			align-items: center;
			gap: 15px;
			font-size: 22px;

			padding: 10px;
			text-decoration: none;
		}
	}

	a:hover,
	a.active {
		background-color: var(--quaternary-bg);
	}
</style>
