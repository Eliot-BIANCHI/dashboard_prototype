<script lang="ts">
	import Calendar, { CalendarMode } from '../Calendar.svelte';
	import Button from '$lib/components/utils/button/Button.svelte';

	interface Props {
		calendar: Calendar;
	}

	let { calendar }: Props = $props();

	let mode = $derived(getMode());

	let today = $derived(getCurrent());

	let isToday = $derived(getIsToday());

	function getMode() {
		switch (calendar.calendarMode) {
			case CalendarMode.Day: {
				return 'days';
			}
			case CalendarMode.Week: {
				return 'weeks';
			}
			case CalendarMode.Month: {
				return 'months';
			}
		}
	}

	function getCurrent() {
		switch (calendar.calendarMode) {
			case CalendarMode.Day: {
				return calendar.today;
			}
			case CalendarMode.Week: {
				return calendar.todayWeek;
			}
			case CalendarMode.Month: {
				return calendar.todayMonth;
			}
		}
	}

	function getIsToday() {
		switch (calendar.calendarMode) {
			case CalendarMode.Day: {
				return calendar.today === calendar.day && calendar.todayYear === calendar.year;
			}
			case CalendarMode.Week: {
				return calendar.todayWeek === calendar.week && calendar.todayYear === calendar.year;
			}
			case CalendarMode.Month: {
				return calendar.todayMonth === calendar.month && calendar.todayYear === calendar.year;
			}
		}
	}
</script>

<div class="mode-selection">
	<a href="/calendars/{calendar.id}/{calendar.todayYear}/{mode}/{today}">
		<Button class={['width-100', 'height-100']} isSelected={isToday} isFocusable={false}>
			Today
		</Button>
	</a>

	<div class="modes">
		<a href="/calendars/{calendar.id}/{calendar.year}/days/{calendar.day}">
			<Button
				class={['width-100']}
				isSelected={calendar.calendarMode === CalendarMode.Day}
				isFocusable={false}
			>
				{CalendarMode.Day}
			</Button>
		</a>

		<a href="/calendars/{calendar.id}/{calendar.year}/weeks/{calendar.week}">
			<Button
				class={['width-100']}
				isSelected={calendar.calendarMode === CalendarMode.Week}
				isFocusable={false}
			>
				{CalendarMode.Week}
			</Button>
		</a>

		<a href="/calendars/{calendar.id}/{calendar.year}/months/{calendar.month}">
			<Button
				class={['width-100']}
				isSelected={calendar.calendarMode === CalendarMode.Month}
				isFocusable={false}
			>
				{CalendarMode.Month}
			</Button>
		</a>
	</div>
</div>

<style>
	.mode-selection {
		display: flex;
		gap: 10px;
	}

	.modes {
		display: flex;
		flex-direction: column;
		gap: 5px;
	}

	a {
		width: 100%;
		text-decoration: none;
	}
</style>
