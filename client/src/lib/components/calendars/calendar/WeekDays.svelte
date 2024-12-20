<script lang="ts">
	import Calendar from '../Calendar.svelte';
	import Toggle from '$lib/components/utils/form/Toggle.svelte';
	import Button from '$lib/components/utils/button/Button.svelte';

	interface Props {
		calendar: Calendar;
	}

	let { calendar }: Props = $props();

	const hiddenDays: string[] = JSON.parse(localStorage.getItem('calendar:hidden-days') || '[]');
	let areDaysHidden = $state(hiddenDays.length === 7);
</script>

<div class="week-days">
	{#each calendar.weekDays as day (day.id)}
		<Button
			onclick={() => calendar.toggleIsDayHidden(day.id)}
			isSelected={!day.isHidden}
			class={['round']}
		>
			{day.firstLetter}
		</Button>
	{/each}
</div>
<Toggle
	name="hide-days"
	text="Hide all days"
	isChecked={areDaysHidden}
	onchange={() => {
		areDaysHidden = !areDaysHidden;
		calendar.toggleAreDaysHidden(areDaysHidden);
	}}
/>

<style>
	.week-days {
		display: flex;
		column-gap: 5px;
		flex-wrap: wrap;
		justify-content: center;
		margin-inline: auto;
		width: 90%;
	}
</style>
