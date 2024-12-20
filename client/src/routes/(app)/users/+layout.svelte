<script lang="ts">
	import { apiPostLoadRequest as apiRequest } from '$lib/api-request/ApiRequest';
	import type { Snippet } from 'svelte';
	import Sidebar from '$lib/components/utils/Sidebar.svelte';
	import Input from '$lib/components/utils/form/Input.svelte';
	import Select from '$lib/components/utils/form/Select.svelte';
	import { debounce } from '$lib/tools';
	import type { User, SearchBy } from '$lib/Account.svelte';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();

	let searchBy: SearchBy = $state('username');
	let overviews: User[] = $state([]);

	const getUsers = debounce(async (search: string) => {
		if (search.length === 0) {
			overviews = [];
			return;
		}
		const res = await apiRequest.users.getOverviews({
			query: { search, by: searchBy }
		});

		if (res.type === 'ok') {
			overviews = res.value;
		} else {
			overviews = [];
		}
	}, 500);
</script>

<svelte:head>
	<title>Users</title>
</svelte:head>

<section class="users-layout">
	<Sidebar>
		<div class="users-navbar">
			<Select
				name="search-by"
				text="Search by"
				options={[
					{ text: 'Username', value: 'username' },
					{ text: 'First name', value: 'first_name' },
					{ text: 'Last name', value: 'last_name' }
				]}
				onchange={(by) => (searchBy = by as SearchBy)}
			/>
			<Input name="search" text="Search user" oninput={(e) => getUsers(e.currentTarget.value)} />
			{#if overviews.length > 0}
				<ul class="overviews">
					{#each overviews as { id, firstName, lastName }}
						<li class="overview">
							<a href="/users/{id}">{firstName} {lastName}</a>
						</li>
					{/each}
				</ul>
			{:else}
				<p class="empty-overviews">No user found...</p>
			{/if}
		</div>
	</Sidebar>

	{@render children()}
</section>

<style>
	.users-layout {
		flex-grow: 1;
		display: flex;
		gap: 20px;
	}

	.users-navbar {
		display: flex;
		flex-direction: column;
		gap: 20px;
	}

	.overviews {
		display: flex;
		flex-direction: column;
		gap: 15px;
		list-style: none;
		margin-block: 0;
		padding-left: 0;
	}

	.overview a {
		background-color: var(--primary-bg);
		border-radius: 5px;
		box-sizing: border-box;
		display: inline-flex;
		font-size: 22px;
		padding: 10px;
		text-decoration: none;
		width: 100%;

		&:hover {
			background-color: var(--secondary-bg);
		}
	}

	.empty-overviews {
		font-size: 22px;
		margin-block: 0;
		text-align: center;
	}
</style>
