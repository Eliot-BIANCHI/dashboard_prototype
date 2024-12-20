<script lang="ts">
	import { goto } from '$app/navigation';
	import Calendar from '../Calendar.svelte';
	import Select from '$lib/components/utils/form/Select.svelte';
	import IconButton from '$lib/components/utils/button/IconButton.svelte';
	import { yearDays } from '../tools';

	import { Temporal } from '@js-temporal/polyfill';

	interface Props {
		calendar: Calendar;
	}

	let { calendar }: Props = $props();

	let days = $derived(yearDays(calendar.year));

	const year = $derived(new Temporal.PlainDate(calendar.year, 1, 1));

	let previousDay = $derived(
		calendar.day === 1 ? year.subtract({ years: 1 }).daysInYear : calendar.day - 1
	);

	let nextDay = $derived(year.daysInYear === calendar.day ? 1 : calendar.day + 1);
</script>

<div class="day-selection">
	<div class="switch-day">
		<a
			href="/calendars/{calendar.id}/{calendar.day === 1
				? calendar.year - 1
				: calendar.year}/days/{previousDay}"
		>
			<IconButton iconName="chevron-left" isFocusable={false} />
		</a>

		<a
			href="/calendars/{calendar.id}/{calendar.day === year.daysInYear
				? calendar.year + 1
				: calendar.year}/days/{nextDay}"
		>
			<IconButton iconName="chevron-right" isFocusable={false} />
		</a>
	</div>
	<Select
		name="year-days"
		text="Day selected"
		options={days.map((day) => ({
			text: day.formatted,
			value: day.raw,
			isSelected: day.raw === calendar.day
		}))}
		onchange={(value) => goto(`/calendars/${calendar.id}/${calendar.year}/days/${value}`)}
	/>
</div>

<style>
	.day-selection {
		display: flex;
		flex-direction: column;
		gap: 10px;

		& .switch-day {
			display: flex;
			justify-content: space-between;
			gap: 20px;
		}
	}
</style>
