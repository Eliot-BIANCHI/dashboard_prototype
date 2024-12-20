<script lang="ts">
	import AppNavbar from '$lib/components/app-navbar/AppNavbar.svelte';

	let { children, data } = $props();
</script>

{@render children()}

<AppNavbar
	userPermissions={data.account.permissions}
	favoriteCalendarId={data.account.favoriteCalendarId}
	favoriteKanbanId={data.account.favoriteKanbanId}
/>
