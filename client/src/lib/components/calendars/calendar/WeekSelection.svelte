<script lang="ts">
	import { goto } from '$app/navigation';
	import Calendar from '../Calendar.svelte';
	import Select from '$lib/components/utils/form/Select.svelte';
	import IconButton from '$lib/components/utils/button/IconButton.svelte';
	import { numberOfWeeksInYear, yearWeeks } from '../tools';

	interface Props {
		calendar: Calendar;
	}

	let { calendar }: Props = $props();

	let weeks = $derived(yearWeeks(calendar.year));

	let weeksInYear = $derived(numberOfWeeksInYear(calendar.year));

	let previousWeek = $derived(
		calendar.week === 1 ? numberOfWeeksInYear(calendar.year - 1) : calendar.week - 1
	);

	let nextWeek = $derived(calendar.week === weeksInYear ? 1 : calendar.week + 1);
</script>

<div class="week-selection">
	<div class="switch-week">
		<a
			href="/calendars/{calendar.id}/{calendar.week === 1
				? calendar.year - 1
				: calendar.year}/weeks/{previousWeek}"
		>
			<IconButton iconName="chevron-left" isFocusable={false} />
		</a>

		<a
			href="/calendars/{calendar.id}/{calendar.week === weeksInYear
				? calendar.year + 1
				: calendar.year}/weeks/{nextWeek}"
		>
			<IconButton iconName="chevron-right" isFocusable={false} />
		</a>
	</div>
	<Select
		name="year-weeks"
		text="Week selected"
		options={weeks.map((week) => ({
			text: week.formatted,
			value: week.raw,
			isSelected: week.raw === calendar.week
		}))}
		onchange={(value) => goto(`/calendars/${calendar.id}/${calendar.year}/weeks/${value}`)}
	/>
</div>

<style>
	.week-selection {
		display: flex;
		flex-direction: column;
		gap: 10px;

		& .switch-week {
			display: flex;
			justify-content: space-between;
			gap: 20px;
		}
	}
</style>
