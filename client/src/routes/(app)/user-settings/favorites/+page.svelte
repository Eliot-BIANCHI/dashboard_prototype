<script lang="ts">
	import Toggle from '$lib/components/utils/form/Toggle.svelte';
	import Boxes from '$lib/components/utils/form/Boxes.svelte';
	import navbar from '$lib/components/app-navbar/Navbar.svelte';
	import { toastsTypes } from '$lib/components/toaster/Toast.svelte';

	let { data } = $props();
</script>

<div class="favorites">
	<Boxes
		name="favorite-calendar"
		text="Favorite calendar"
		boxes={[
			{ text: 'None', value: 'calendar-null', isChecked: data.account.favoriteCalendarId === null },
			...data.calendars.map((calendar) => ({
				text: calendar.label,
				value: `calendar-${calendar.id}`,
				isChecked:
					data.account.favoriteCalendarId !== null &&
					parseInt(data.account.favoriteCalendarId) === calendar.id
			}))
		]}
		type="radio"
		onchange={(e) => {
			const match = e.currentTarget.value.match(/-(\d+)$/);
			const favoriteCalendarId = match !== null ? match[1] : null;

			data.account.setFavoriteCalendarId(favoriteCalendarId);
		}}
	/>

	<Boxes
		name="favorite-kanban"
		text="Favorite kanban"
		boxes={[
			{ text: 'None', value: 'kanban-null', isChecked: data.account.favoriteKanbanId === null },
			...data.kanbans.map((kanban) => ({
				text: kanban.label,
				value: `kanban-${kanban.id}`,
				isChecked:
					data.account.favoriteKanbanId !== null &&
					parseInt(data.account.favoriteKanbanId) === kanban.id
			}))
		]}
		type="radio"
		onchange={async (e) => {
			const match = e.currentTarget.value.match(/-(\d+)$/);
			const kanbanId = match !== null ? match[1] : null;

			data.account.setFavoriteKanbanId(kanbanId);
		}}
	/>

	<div class="navbar-links">
		<h2>Hide/Expose navbar links</h2>

		{#each navbar.links.slice(0, -2) as { routeName, isHidden, permissionRequired }}
			{#if permissionRequired === undefined || data.account.permissions.includes(permissionRequired)}
				<Toggle
					name="toggle-{routeName.toLowerCase()}-visibility"
					text="Hide {routeName} ?"
					isChecked={isHidden}
					onchange={() => navbar.toggleLinkVisibility(routeName)}
				/>
			{/if}
		{/each}
	</div>

	<section class="notification-types">
		<h2>Hide/Expose notifications</h2>

		{#each toastsTypes as toastType}
			<Toggle
				name="toggle-{toastType.value}-visibility"
				text="Hide {toastType.value} notifications ?"
				isChecked={toastType.isHidden}
				onchange={() => {
					toastType.isHidden = !toastType.isHidden;

					const hiddenTypes: string[] = JSON.parse(
						localStorage.getItem('favorites:hidden-notification-types') || '[]'
					);

					if (hiddenTypes.includes(toastType.value)) {
						const index = hiddenTypes.findIndex((type) => type === toastType.value);
						hiddenTypes.splice(index, 1);
					} else {
						hiddenTypes.push(toastType.value);
					}

					localStorage.setItem('favorites:hidden-notification-types', JSON.stringify(hiddenTypes));
				}}
			/>
		{/each}
	</section>
</div>

<style>
	.favorites {
		display: flex;
		flex-direction: column;
		gap: 25px;
		flex-grow: 1;
		margin-block: 16px;
		overflow-y: auto;
		padding-inline: 25px;
	}
</style>
