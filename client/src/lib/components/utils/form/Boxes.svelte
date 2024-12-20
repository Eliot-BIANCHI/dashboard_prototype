<script lang="ts">
	import type { Box, BoxesType } from './types';

	interface Props {
		name: string;
		text: string;
		boxes: Box[];
		type?: BoxesType;
		areDisabled?: boolean;
		onchange?: (event: Event & { currentTarget: HTMLInputElement }) => void;
	}

	let {
		name,
		text,
		boxes,
		type = 'checkbox',
		areDisabled = false,
		onchange = () => {}
	}: Props = $props();
</script>

<div class="form-boxes" class:disabled={areDisabled}>
	<span>{text}:</span>
	<div class="boxes-container">
		{#each boxes as { text, value, isChecked, isDisabled }}
			<div class="form-box">
				<input
					id={value.toString()}
					{type}
					{name}
					{value}
					checked={isChecked}
					disabled={isDisabled || areDisabled}
					{onchange}
				/>
				<label for={value.toString()}>{text}</label>
			</div>
		{/each}
	</div>
</div>

<style>
	.form-boxes.disabled {
		cursor: not-allowed;

		& > span {
			opacity: var(--field-disabled-opacity);
		}
	}

	.boxes-container {
		display: flex;
		flex-direction: column;
		flex-wrap: wrap;
		gap: 20px;
		max-height: 200px;
		padding: 20px 0;
		overflow-x: auto;
	}

	span {
		color: var(--primary-text);
		font-size: 1.5rem;
	}

	.form-box {
		cursor: pointer;
	}

	.form-box:has(input:disabled) {
		cursor: not-allowed;
		opacity: var(--field-disabled-opacity);
	}

	input {
		opacity: 0;
		position: absolute;
		top: -999px;
		left: -999px;
	}

	label {
		color: var(--primary-text);
		cursor: inherit;
		display: flex;
		align-items: center;
		gap: 20px;
		font-size: 1.5rem;
		padding: 0 10px;
		position: relative;
		transition: color var(--field-transition-duration);
	}

	label::before {
		--alpha: 0.5;
		border-radius: 3px;
		color: #fff;
		content: '';
		display: flex;
		justify-content: center;
		align-items: center;
		flex-shrink: 0;
		font-size: 25px;
		height: 25px;
		outline: var(--field-border);
		outline-offset: 5px;
		transition:
			background-color,
			outline-color,
			var(--field-transition-duration) ease-in-out;
		width: 25px;
	}

	input:not(:disabled) + label:hover::before,
	input:focus:not(:disabled) + label::before {
		outline-color: var(--field-border-color-hover);
	}

	input[type='radio'] + label::before {
		border-radius: 100px;
	}

	input:checked + label {
		color: var(--box-checked-color) !important;
	}

	input:checked + label::before {
		background-color: var(--box-checked-color) !important;
		outline-color: var(--box-checked-color) !important;
	}

	input[type='checkbox']:checked + label::before {
		content: '\2713';
	}
</style>
