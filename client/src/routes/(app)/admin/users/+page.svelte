<script lang="ts">
	import { apiPostLoadRequest as apiRequest } from '$lib/api-request/ApiRequest';
	import { debounce } from '$lib/tools';

	import Input from '$lib/components/utils/form/Input.svelte';
	import Select from '$lib/components/utils/form/Select.svelte';
	import Button from '$lib/components/utils/button/Button.svelte';
	import type { PaginatedSearchBy as SearchBy } from '$lib/Account.svelte';

	let { data } = $props();

	const LIMIT = 2;

	async function getUsers(page?: number) {
		const res = await apiRequest.users.getPaginatedOverviews({
			query: { by: data.admin.searchBy, page, search: data.admin.search, roleId: data.admin.roleId }
		});

		if (res.type === 'ok') {
			data.admin.overviews = res.value.overviews;

			if (res.value.count === -1) {
				data.admin.pagination!.currentPage = res.value.pageNumber;
			} else {
				data.admin.pagination = {
					pages: Math.ceil(res.value.count / LIMIT),
					currentPage: res.value.pageNumber,
					count: res.value.count
				};
			}
		} else {
			data.admin.overviews = [];
			data.admin.pagination = null;
		}
	}

	const getUsersByName = debounce(async (search: string) => {
		if (search.length === 0) {
			data.admin.overviews = [];
			data.admin.pagination = null;
			return;
		}

		data.admin.search = search;
		getUsers();
	}, 500);

	async function getRoles() {
		if (data.admin.roles === null) {
			data.admin.roles = await apiRequest.roles.get();
		}

		return data.admin.roles;
	}

	const selectOptions: SearchBy[] = ['name', 'role', 'newest', 'oldest'];
</script>

<div class="manage-users">
	<div class="utils">
		<Button iconName="add" class={['success']}>User</Button>
	</div>

	<div class="search">
		<Select
			text="Search by"
			name="search-by"
			options={selectOptions.map((option) => ({
				text: option,
				value: option,
				isSelected: data.admin.searchBy === option
			}))}
			onchange={async (option) => {
				data.admin.searchBy = option as SearchBy;

				if (data.admin.searchBy !== 'name') {
					data.admin.search = '';
				}

				if (data.admin.searchBy !== 'role') {
					data.admin.roleId = -1;
				}

				if (data.admin.searchBy === 'newest' || data.admin.searchBy === 'oldest') {
					await getUsers();
				} else {
					data.admin.overviews = [];
					data.admin.pagination = null;
				}
			}}
		/>

		{#if data.admin.searchBy === 'name'}
			<Input
				text="Search by first name or last name"
				name="search-users"
				value={data.admin.search}
				oninput={(e) => getUsersByName(e.currentTarget.value)}
			/>
		{:else if data.admin.searchBy === 'role'}
			{#await getRoles() then roles}
				<Select
					text="Roles"
					name="roles"
					options={[
						{ text: 'Chose a role', value: -1 },
						...roles.map((role) => ({
							text: role.label,
							value: role.id,
							isSelected: data.admin.roleId === role.id
						}))
					]}
					onchange={(role) => {
						data.admin.roleId = role as number;
						if (data.admin.roleId === -1) {
							data.admin.overviews = [];
							data.admin.pagination = null;
						} else {
							getUsers();
						}
					}}
				/>
			{:catch error}
				<span>{error}</span>
			{/await}
		{/if}
	</div>

	{#if data.admin.pagination !== null}
		<p class="count">Total found : {data.admin.pagination.count}</p>
	{/if}

	<ul class="users">
		{#each data.admin.overviews as { id, firstName, lastName }}
			<li class="user">
				<a href="/admin/users/{id}">{firstName} {lastName}</a>
			</li>
		{/each}
	</ul>

	{#if data.admin.pagination !== null && data.admin.overviews.length > 0}
		<div class="pagination">
			<Button
				isDisabled={data.admin.pagination.currentPage === 1}
				onclick={() => getUsers(data.admin.pagination!.currentPage - 1)}
			>
				Previous
			</Button>
			<ol class="pages">
				{#each { length: data.admin.pagination.pages } as _, pageNumber}
					<li class="page">
						<Button
							isSelected={data.admin.pagination.currentPage === pageNumber + 1}
							onclick={() => getUsers(pageNumber + 1)}
						>
							{pageNumber + 1}
						</Button>
					</li>
				{/each}
			</ol>
			<Button
				isDisabled={data.admin.pagination.currentPage === data.admin.pagination.pages}
				onclick={() => getUsers(data.admin.pagination!.currentPage + 1)}
			>
				Next
			</Button>
		</div>
	{/if}
</div>

<style>
	.manage-users {
		flex-grow: 1;
		display: flex;
		flex-direction: column;
		gap: 20px;
		margin-block: 16px;
		overflow-y: auto;
	}

	.search {
		display: flex;
		align-items: center;
		flex-wrap: wrap;
		gap: 30px;
		justify-content: center;
	}

	.count {
		font-size: 25px;
		margin-block: 0;
		text-align: end;
	}

	.users {
		display: grid;
		gap: 20px;
		grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
		flex-grow: 1;
		list-style: none;
		margin-block: 0;
		padding-left: 0;

		& .user {
			background-color: var(--primary-bg);
			border-radius: 5px;
			font-size: 22px;
			height: 300px;

			& a {
				box-sizing: border-box;
				color: var(--primary-text);
				display: inline-block;
				height: 100%;
				font-size: 25px;
				padding: 20px;
				text-align: center;
				text-decoration: none;
				width: 100%;
			}

			&:hover {
				background-color: var(--tertiary-bg);
			}
		}
	}

	.pagination {
		align-self: center;
		background: var(--primary-bg);
		border-radius: 5px;
		display: flex;
		align-items: center;
		flex-wrap: wrap;
		gap: 20px;
		justify-content: center;
		padding: 10px;

		& .pages {
			display: flex;
			flex-wrap: wrap;
			gap: 20px;
			justify-content: center;
			list-style: none;
			margin-block: 0;
			padding-left: 0;
		}
	}
</style>
