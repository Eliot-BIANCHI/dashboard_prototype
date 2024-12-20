<script lang="ts">
	import { tick } from 'svelte';
	import type { Option } from './types';

	interface Props {
		name: string;
		text: string;
		options: Option[];
		isDisabled?: boolean;
		onchange?: (value: string | number) => void;
	}

	let { name, text, options, isDisabled = false, onchange = () => {} }: Props = $props();

	let dropdown: HTMLDivElement;
	let select: HTMLSelectElement;
	let selectedOption: HTMLButtonElement | null = $state(null);

	let selected = $state('');
	let isOpen = $state(false);

	$effect(() => {
		const option = options.find((option) => option.isSelected);

		selected = option === undefined ? options[0].text : option.text;
		if (option === undefined) selectFirstOption();

		const form = select.parentElement;
		if (form instanceof HTMLFormElement) {
			form.addEventListener('reset', selectFirstOption);
		}
	});

	async function toggleDropdown() {
		isOpen = !isOpen;
		if (isOpen && selectedOption !== null) {
			await tick();
			selectedOption.scrollIntoView(true);
		}
	}

	function selectOption(value: string | number) {
		options = options.map((option) => ({
			...option,
			isSelected: option.value === value
		}));
		onchange(value);
		isOpen = false;
	}

	function selectFirstOption() {
		options = options.map((option, index) => ({
			...option,
			isSelected: index === 0
		}));
	}

	function onpointerdown(event: PointerEvent) {
		if (event.target instanceof Node && !dropdown.contains(event.target)) {
			isOpen = false;
		}
	}
</script>

<svelte:window {onpointerdown} />

<select bind:this={select} {name} tabindex="-1" aria-hidden="true">
	{#each options as { value, text, isSelected, isDisabled }}
		<option {value} selected={isSelected} disabled={isDisabled}>{text}</option>
	{/each}
</select>

<div bind:this={dropdown} class="form-dropdown">
	<label for={name}>{text}:</label>
	<div class="dropdown">
		<button
			type="button"
			class="select"
			class:active={isOpen}
			id={name}
			disabled={isDisabled}
			onclick={toggleDropdown}
		>
			<span class="selected">{selected}</span>
			<span class="caret" class:rotate={isOpen}></span>
		</button>
		<ul class="menu" class:open={isOpen}>
			{#each options as { text, value, isSelected, isDisabled }}
				<li class="option">
					{#if isSelected}
						<button
							bind:this={selectedOption}
							class="active"
							type="button"
							disabled={isDisabled}
							onclick={() => selectOption(value)}
						>
							{text}
						</button>
					{:else}
						<button type="button" disabled={isDisabled} onclick={() => selectOption(value)}>
							{text}
						</button>
					{/if}
				</li>
			{/each}
		</ul>
	</div>
</div>

<style>
	select {
		opacity: 0;
		pointer-events: none;
		position: absolute;
		top: -999px;
		left: -999px;
	}

	.form-dropdown {
		cursor: pointer;
		display: flex;
		flex-direction: column;
		gap: 10px;

		&:has(button.select:disabled) {
			cursor: not-allowed;
			opacity: var(--field-disabled-opacity);
		}
	}

	label {
		color: var(--primary-text);
		cursor: inherit;
		font-size: 1.5rem;
	}

	.dropdown {
		position: relative;
		min-width: 100px;
	}

	.select {
		background-color: var(--primary-bg);
		border: var(--field-border);
		border-radius: 0.5rem;
		cursor: inherit;
		display: flex;
		align-items: center;
		justify-content: space-between;
		font-size: 1.5rem;
		outline: none;
		padding: 15px;
		transition: var(--field-transition);
		width: 100%;

		&:hover:not(:disabled),
		&:focus,
		&.active {
			border-color: var(--field-border-color-hover);
		}
	}

	.selected {
		color: var(--primary-text);
		height: max-content;
		min-height: 30px;
		width: 100%;
	}

	.caret {
		border-left: 5px solid transparent;
		border-right: 5px solid transparent;
		border-top: 6px solid var(--primary-border);
		display: inline-block;
		height: 0;
		transition: rotate 300ms;
		width: 0;

		&.rotate {
			rotate: 180deg;
		}
	}

	.menu {
		background-color: var(--primary-bg);
		border: 2px solid var(--primary-border);
		border-radius: 0.5rem;
		display: none;
		list-style: none;
		margin-block: 0;
		max-height: 400px;
		opacity: 0;
		overflow-y: auto;
		padding: 0.2rem 0.5rem;
		position: absolute;
		top: 70px;
		left: 50%;
		width: 100%;
		translate: -50%;
		z-index: 1;

		&.open {
			display: block;
			opacity: 1;
		}
	}

	.menu .option {
		margin: 0.3em 0;

		& button {
			background-color: transparent;
			border: none;
			border-radius: 0.5rem;
			color: var(--primary-text);
			cursor: pointer;
			font-size: 1.25rem;
			height: 100%;
			outline: none;
			padding: 0.7rem 0.5rem;
			width: 100%;

			&:disabled {
				cursor: not-allowed;
				opacity: var(--field-disabled-opacity);
			}

			&:hover:not(:disabled),
			&:focus,
			&.active {
				background-color: var(--quaternary-bg);
			}
		}
	}
</style>
