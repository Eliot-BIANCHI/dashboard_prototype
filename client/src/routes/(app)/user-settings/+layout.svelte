<script lang="ts">
	import { page } from '$app/state';
	import type { Snippet } from 'svelte';
	import Sidebar from '$lib/components/utils/Sidebar.svelte';
	import Icon from '$lib/components/utils/Icon.svelte';
	import type { Link } from '$lib/components/app-navbar/Navbar.svelte';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();

	const links: Link[] = [
		{ href: '/profile', routeName: 'Profile', iconName: 'profile' },
		{ href: '/favorites', routeName: 'Favorites', iconName: 'favorites' },
		{ href: '/invitations', routeName: 'Invitations', iconName: 'invitation' },
		{ href: '/language', routeName: 'Language', iconName: 'language' }
	];
</script>

<svelte:head>
	<title>Settings</title>
</svelte:head>

<div class="user-settings-layout">
	<Sidebar>
		<nav class="user-settings-navbar">
			<ul class="navbar-links">
				{#each links as { href, routeName, iconName }}
					<li class="navbar-link">
						<a
							class:active={page.url.pathname.startsWith(`/user-settings${href}`)}
							href="/user-settings{href}"
						>
							<Icon name={iconName} />
							{routeName}
						</a>
					</li>
				{/each}
			</ul>
		</nav>
	</Sidebar>

	{@render children()}
</div>

<style>
	.user-settings-layout {
		flex-grow: 1;
		display: flex;
		gap: 20px;
	}

	.user-settings-navbar {
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
</style>
