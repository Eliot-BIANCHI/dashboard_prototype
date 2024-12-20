<script lang="ts">
	import { goto } from '$app/navigation';
	import IconButton from '$lib/components/utils/button/IconButton.svelte';

	let { data } = $props();

	const ASSETS_BASE_URL = import.meta.env.VITE_ASSETS_BASE_URL;

	const src = $derived(`${ASSETS_BASE_URL}/images/${data.user.imageUrl || 'default.svg'}`);
</script>

<div class="user-container">
	<div class="utils">
		<IconButton iconName="left-arrow" onclick={() => goto('/admin/users')} />
	</div>

	<div class="user">
		<img class="user-picture" {src} alt="User profile" width="150" height="150" />
		<h2>{data.user.firstName} {data.user.lastName}</h2>
	</div>
</div>

<style>
	.user-container {
		display: flex;
		flex-direction: column;
		gap: 25px;
		flex-grow: 1;
		margin-block: 16px;
		width: 300px;
	}

	.user {
		background-color: var(--quaternary-bg);
		border-radius: 5px;
		display: flex;
		align-items: center;
		gap: 50px;
		margin-block: 16px;
		padding: 15px 25px;
	}

	.user-picture {
		border-radius: 100px;
		object-fit: cover;
	}
</style>
