<script lang="ts">
	import { page } from '$app/state';
	import Sidebar from '$lib/components/utils/Sidebar.svelte';
	import Icon from '$lib/components/utils/Icon.svelte';
	import type { Link } from '$lib/components/app-navbar/Navbar.svelte';

	let { children, data } = $props();

	const links: Link[] = [
		{
			href: '/users',
			routeName: 'Users',
			iconName: 'manage-users',
			permissionRequired: 'admin-section:users'
		}
	];

	const authorizationsLinks: Link[] = [
		{
			href: '/authorizations/global',
			routeName: 'Global',
			iconName: 'authorizations'
		},
		{
			href: '/authorizations/calendars',
			routeName: 'Calendars',
			iconName: 'calendar-authorizations'
		},
		{
			href: '/authorizations/kanbans',
			routeName: 'Kanbans',
			iconName: 'kanban-authorizations'
		}
	];
</script>

<svelte:head>
	<title>Admin</title>
</svelte:head>

<div class="admin-layout">
	<Sidebar>
		<nav class="admin-navbar">
			<ul class="navbar-links">
				{#each links as { href, routeName, iconName, permissionRequired }}
					{#if permissionRequired === undefined || data.account.permissions.includes(permissionRequired)}
						<li class="navbar-link">
							<a class:active={page.url.pathname.startsWith(`/admin${href}`)} href="/admin{href}">
								<Icon name={iconName} />
								{routeName}
							</a>
						</li>
					{/if}
				{/each}
			</ul>
		</nav>

		{#if data.account.permissions.includes('admin-section:authorizations')}
			<h2 class="navbar-sub-section-title">Authorizations</h2>

			<nav class="admin-navbar">
				<ul class="navbar-links">
					{#each authorizationsLinks as { href, routeName, iconName }}
						<li class="navbar-link">
							<a class:active={page.url.pathname === `/admin${href}`} href="/admin{href}">
								<Icon name={iconName} />
								{routeName}
							</a>
						</li>
					{/each}
				</ul>
			</nav>
		{/if}
	</Sidebar>

	{@render children()}
</div>

<style>
	.admin-layout {
		flex-grow: 1;
		display: flex;
		gap: 20px;
	}

	.admin-navbar {
		background-color: var(--tertiary-bg);
	}

	.navbar-links {
		display: flex;
		flex-direction: column;
		list-style: none;
		padding-left: 0;
		margin-block: 10px;
	}

	.navbar-link {
		&:first-child > a {
			border-top-left-radius: 20px;
			border-top-right-radius: 20px;
		}

		&:last-child > a {
			border-bottom-left-radius: 20px;
			border-bottom-right-radius: 20px;
		}

		& a {
			background-color: var(--primary-bg);
			color: var(--primary-text);
			display: flex;
			align-items: center;
			gap: 15px;
			font-size: 1.5em;
			padding: 15px;
			text-align: center;
			text-decoration: none;

			&.active,
			&:hover {
				background-color: var(--quaternary-bg);
			}
		}
	}

	.navbar-sub-section-title {
		text-align: center;
	}
</style>
