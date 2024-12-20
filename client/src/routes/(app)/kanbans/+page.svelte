<script>
	const { data } = $props();
</script>

{#if data.kanbans.overviews.length === 0}
	<p>Start by creating or joining a kanban</p>
{:else}
	<p>Select one of your kanban</p>
{/if}

<style>
	p {
		display: flex;
		align-items: center;
		justify-content: center;
		flex-grow: 1;
		font-size: 32px;
		margin-block: 0;
	}
</style>
