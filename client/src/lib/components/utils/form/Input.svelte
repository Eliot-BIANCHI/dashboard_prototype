<script lang="ts">
	import type { InputType, InputAutocomplete } from './types';

	interface Props {
		text: string;
		name: string;
		value?: string | number;
		type?: InputType;
		placeholder?: string;
		isDisabled?: boolean;
		autocomplete?: InputAutocomplete;
		onchange?: (event: Event & { currentTarget: HTMLInputElement }) => void;
		oninput?: (event: Event & { currentTarget: HTMLInputElement }) => void;
	}

	let {
		text,
		name,
		value = '',
		type = 'text',
		placeholder = '',
		isDisabled = false,
		autocomplete = 'off',
		onchange = () => {},
		oninput = () => {}
	}: Props = $props();
</script>

<div class="form-input">
	<label for={name}>{text}:</label>
	<input
		id={name}
		{type}
		{name}
		{value}
		{placeholder}
		disabled={isDisabled}
		{autocomplete}
		{onchange}
		{oninput}
	/>
</div>

<style>
	.form-input {
		display: flex;
		flex-direction: column;
		gap: 5px;

		&:has(input:disabled) {
			cursor: not-allowed;
			opacity: var(--field-disabled-opacity);
		}
	}

	label {
		color: var(--primary-text);
		cursor: inherit;
		font-size: 1.5rem;
	}

	input {
		background-color: var(--primary-bg);
		border: var(--field-border);
		border-radius: 10px;
		box-sizing: border-box;
		color: var(--primary-text);
		cursor: inherit;
		font-size: 1.75rem;
		outline: none;
		padding: 10px 20px;
		transition: var(--field-transition);

		min-width: 100px;

		&:hover:not(:disabled),
		&:focus {
			border-color: var(--field-border-color-hover);
		}

		&[type='color'] {
			background-color: transparent;
			border-radius: 5px;
			height: 50px;
			padding: 10px;
		}
	}
</style>
