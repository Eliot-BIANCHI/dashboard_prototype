<script lang="ts">
	interface Props {
		name: string;
		text: string;
		value?: string;
		placeholder?: string;
		isDisabled?: boolean;
	}

	let { name, text, value = '', placeholder = '', isDisabled = false }: Props = $props();
</script>

<div class="form-area">
	<label for={name}>{text}:</label>
	<textarea id={name} {name} {placeholder} disabled={isDisabled}>{value}</textarea>
</div>

<style>
	.form-area {
		display: flex;
		flex-direction: column;
		gap: 10px;

		&:has(textarea:disabled) {
			cursor: not-allowed;
			opacity: var(--field-disabled-opacity);
		}
	}

	label {
		color: var(--primary-text);
		cursor: inherit;
		font-size: 1.5rem;
	}

	textarea {
		background-color: var(--primary-bg);
		border: var(--field-border);
		border-radius: 10px;
		cursor: inherit;
		font-size: 2rem;
		height: 100px;
		line-height: 1.4;
		outline: none;
		padding: 15px;
		resize: none;
		transition: var(--field-transition);
		min-width: 100px;

		&:hover:not(:disabled),
		&:focus {
			border-color: var(--field-border-color-hover);
		}
	}
</style>
