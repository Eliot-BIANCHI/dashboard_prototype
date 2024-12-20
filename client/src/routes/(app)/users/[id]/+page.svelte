<script lang="ts">
	let { data } = $props();

	const ASSETS_BASE_URL = import.meta.env.VITE_ASSETS_BASE_URL;

	const src = $derived(`${ASSETS_BASE_URL}/images/${data.user.imageUrl || 'default.svg'}`);
</script>

<div class="user">
	<img class="user-picture" {src} alt="User profile" width="150" height="150" />
	<h2>{data.user.firstName} {data.user.lastName}</h2>
</div>

<style>
	.user {
		background-color: var(--quaternary-bg);
		border-radius: 5px;
		display: flex;
		align-items: center;
		gap: 50px;
		flex-grow: 1;
		height: max-content;
		margin-block: 16px;
		padding: 30px;
	}

	.user-picture {
		border-radius: 100px;
		object-fit: cover;
	}
</style>
