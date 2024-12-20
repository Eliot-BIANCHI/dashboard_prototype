<script lang="ts">
	import { goto } from '$app/navigation';
	import Calendar from '../Calendar.svelte';
	import Select from '$lib/components/utils/form/Select.svelte';
	import IconButton from '$lib/components/utils/button/IconButton.svelte';
	import { yearMonths } from '../tools';

	interface Props {
		calendar: Calendar;
	}

	let { calendar }: Props = $props();

	let months = $derived(yearMonths(calendar.year));

	let previousMonth = $derived(calendar.month === 1 ? 12 : calendar.month - 1);

	let nextMonth = $derived(calendar.month === 12 ? 1 : calendar.month + 1);
</script>

<div class="month-selection">
	<div class="switch-month">
		<a
			href="/calendars/{calendar.id}/{calendar.month === 1
				? calendar.year - 1
				: calendar.year}/months/{previousMonth}"
		>
			<IconButton iconName="chevron-left" isFocusable={false} />
		</a>

		<a
			href="/calendars/{calendar.id}/{calendar.month === 12
				? calendar.year + 1
				: calendar.year}/months/{nextMonth}"
		>
			<IconButton iconName="chevron-right" isFocusable={false} />
		</a>
	</div>
	<Select
		name="year-months"
		text="Month selected"
		options={months.map((month) => ({
			text: month.formatted,
			value: month.raw,
			isSelected: month.raw === calendar.month
		}))}
		onchange={(value) => goto(`/calendars/${calendar.id}/${calendar.year}/months/${value}`)}
	/>
</div>

<style>
	.month-selection {
		display: flex;
		flex-direction: column;
		gap: 10px;

		& .switch-month {
			display: flex;
			justify-content: space-between;
			gap: 20px;
		}
	}
</style>
