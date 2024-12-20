<script lang="ts">
	interface Props {
		scheduleId?: number;
		startHour: number;
		endHour: number;
		startMinutes: number;
		endMinutes: number;
	}

	let props: Props = $props();
	let scheduleId = props.scheduleId;

	let startHour = $state(props.startHour);
	let endHour = $state(props.endHour);

	let startMinutes = $state(props.startMinutes);
	let endMinutes = $state(props.endMinutes);

	const MIN_START_HOUR = 7;
	const MIN_END_HOUR = 8;

	const MAX_START_HOUR = 17;
	const MAX_END_HOUR = 18;

	function incrementStartMinutes() {
		if (startMinutes === 45) return;

		if (startHour === endHour && startMinutes + 15 === endMinutes) {
			if (endMinutes === 45) {
				endHour++;
				endMinutes = 0;
			} else {
				endMinutes += 15;
			}
		}

		startMinutes += 15;
	}

	function decrementStartMinutes() {
		if (startMinutes === 0) return;
		startMinutes = startMinutes - 15;
	}

	function incrementEndMinutes() {
		if (endHour === MAX_END_HOUR) return;
		if (endMinutes === 45) return;
		endMinutes += 15;
	}

	function decrementEndMinutes() {
		if (startHour === endHour && endMinutes - 15 === startMinutes) return;
		if (endMinutes === 0) return;
		endMinutes -= 15;
	}

	function handleStartHourChange() {
		if (startHour > endHour) {
			endHour = startHour;
			return;
		}

		if (startHour === endHour && startMinutes >= endMinutes) {
			if (startMinutes === 45) {
				endHour++;
				endMinutes = 0;
			} else {
				endMinutes = startMinutes + 15;
			}
		}
	}

	function handleEndHourChange() {
		if (endHour === MAX_END_HOUR) {
			endMinutes = 0;
			return;
		}

		if (endHour === startHour && endMinutes <= startMinutes) {
			if (startMinutes === 45) {
				startMinutes = startMinutes - 15;
				endMinutes = 45;
			} else {
				endMinutes = startMinutes + 15;
			}
		}
	}

	let inputHourStartName =
		scheduleId === undefined
			? 'create-schedule-hour-start'
			: `update-schedule-${scheduleId}-hour-start`;

	let inputHourEndName =
		scheduleId === undefined
			? 'create-schedule-hour-end'
			: `update-schedule-${scheduleId}-hour-end`;

	let inputMinutesStartName =
		scheduleId === undefined
			? 'create-schedule-minutes-start'
			: `update-schedule-${scheduleId}-minutes-start`;

	let inputMinutesEndName =
		scheduleId === undefined
			? 'create-schedule-minutes-end'
			: `update-schedule-${scheduleId}-minutes-end`;
</script>

<div class="schedule-hour">
	<label for={inputHourStartName}>Start at:</label>
	<input
		defaultValue={MIN_START_HOUR}
		bind:value={startHour}
		type="number"
		class="hour"
		min={MIN_START_HOUR}
		max={MAX_START_HOUR}
		oninput={handleStartHourChange}
		id={inputHourStartName}
		name={inputHourStartName}
	/>
	h
	<label for={inputMinutesStartName}></label>
	<input
		defaultValue="00"
		value={startMinutes === 0 ? '00' : startMinutes.toString()}
		readonly
		type="text"
		class="minutes"
		onkeydown={(e) => {
			e.preventDefault();
			if (e.key === 'ArrowUp') incrementStartMinutes();
			else if (e.key === 'ArrowDown') decrementStartMinutes();
		}}
		id={inputMinutesStartName}
		name={inputMinutesStartName}
	/>
	<span class="increment-decrement">
		<button type="button" onclick={incrementStartMinutes}>&#9650;</button>
		<button type="button" onclick={decrementStartMinutes}>&#9660;</button>
	</span>
</div>

<div class="schedule-hour">
	<label for={inputHourEndName}>End at:</label>
	<input
		defaultValue={MIN_END_HOUR}
		bind:value={endHour}
		type="number"
		class="hour"
		min={startHour}
		max={MAX_END_HOUR}
		oninput={handleEndHourChange}
		id={inputHourEndName}
		name={inputHourEndName}
	/>
	h
	<label for={inputMinutesEndName}></label>
	<input
		defaultValue="00"
		value={endMinutes === 0 ? '00' : endMinutes.toString()}
		readonly
		type="text"
		class="minutes"
		onkeydown={(e) => {
			e.preventDefault();
			if (e.key === 'ArrowUp') incrementEndMinutes();
			else if (e.key === 'ArrowDown') decrementEndMinutes();
		}}
		id={inputMinutesEndName}
		name={inputMinutesEndName}
	/>
	<span class="increment-decrement">
		<button type="button" onclick={incrementEndMinutes}>&#9650;</button>
		<button type="button" onclick={decrementEndMinutes}>&#9660;</button>
	</span>
</div>

<style>
	.schedule-hour {
		display: flex;
		align-items: center;
		gap: 10px;
		justify-content: center;
		font-size: 25px;
		margin-top: 20px;
		position: relative;
		min-width: fit-content;

		& label {
			position: absolute;
			top: -35px;
			left: 0;
		}

		& .hour,
		.minutes {
			border: var(--field-border);
			border-radius: 10px;
			font-size: 22px;
			height: 40px;
			outline: none;
			text-align: center;
			transition: var(--field-transition);
		}

		& .hour {
			width: 65px;

			&:hover,
			&:focus {
				border-color: var(--field-border-color-hover);
			}
		}

		& .minutes {
			outline: none;
			width: 50px;
		}

		& .increment-decrement {
			display: flex;
			flex-direction: column;
			gap: 3px;

			& button {
				background-color: var(--primary-bg);
				border: none;
				border-radius: 3px;
				cursor: pointer;
				padding: 4px 6px;
				transition: background-color 200ms;

				&:hover {
					background-color: var(--secondary-bg);
				}
			}
		}
	}
</style>
