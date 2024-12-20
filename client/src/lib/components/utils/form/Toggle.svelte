<script lang="ts">
	interface Props {
		text: string;
		name: string;
		isChecked?: boolean;
		isDisabled?: boolean;
		onchange?: () => void;
	}

	let { text, name, isChecked, isDisabled = false, onchange = () => {} }: Props = $props();
</script>

<div class="form-toggle">
	<input id={name} type="checkbox" {name} {onchange} disabled={isDisabled} checked={isChecked} />
	<label for={name}>{text}</label>
</div>

<style>
	.form-toggle {
		container-type: inline-size;
		min-width: 150px;
		padding-block: 8px;
	}

	label {
		color: var(--primary-text);
		cursor: pointer;
		display: flex;
		align-items: center;
		gap: 10px;
		justify-content: space-between;
		font-size: 1.5rem;
		position: relative;
	}

	label::before {
		aspect-ratio: 1 / 1;
		background-color: var(--field-border-color);
		border-radius: 99px;
		content: '';
		position: absolute;
		top: 5px;
		right: 45px;
		transition:
			left,
			background-color,
			var(--field-transition-duration) ease-in-out;
		width: 30px;
	}

	@container (max-width: 350px) {
		label {
			flex-direction: column;
			align-items: start;
		}

		label::before {
			top: auto;
			right: auto;
			bottom: 5px;
			left: 5px;
		}

		input:checked + label::before {
			left: 45px;
		}
	}

	input:not(:disabled):focus + label::before,
	input:not(:disabled) + label:hover::before {
		background-color: var(--field-border-color-hover);
	}

	input:not(:disabled) + label:hover::after,
	input:not(:disabled):focus + label::after {
		outline-color: var(--field-border-color-hover);
	}

	input:checked + label::before {
		right: 5px;
		background-color: var(--box-checked-color) !important;
	}

	input:checked + label::after {
		outline-color: var(--box-checked-color) !important;
	}

	input:disabled + label {
		cursor: not-allowed;
		opacity: var(--field-disabled-opacity);
	}

	label::after {
		border-radius: 99px;
		content: '';
		height: 40px;
		outline: var(--field-border);
		transition: outline-color var(--field-transition-duration) ease-in-out;
		min-width: 80px;
	}

	input {
		opacity: 0;
		position: absolute;
		top: -999px;
		left: -999px;
	}
</style>
