<script lang="ts">
	import Icon from '../Icon.svelte';
	import type { IconButtonClass, ButtonType, IconName } from '../types';

	interface Props {
		class?: IconButtonClass[];
		isFocusable?: boolean;
		iconName: IconName;
		isDisabled?: boolean;
		type?: ButtonType;
		onclick?: () => void;
	}

	let {
		class: CLASS = [],
		isFocusable = true,
		type,
		iconName,
		isDisabled = false,
		onclick = () => {}
	}: Props = $props();
</script>

<button
	class={CLASS.join(' ')}
	tabindex={isFocusable ? 0 : -1}
	{type}
	aria-label="{iconName} icon"
	{onclick}
	disabled={isDisabled}
>
	<Icon name={iconName} />
</button>

<style>
	button {
		background-color: transparent;
		border-top: 0.9px solid var(--btn-border-light-color);
		border-right: 1.8px solid var(--btn-border-dark-color);
		border-bottom: 1.8px solid var(--btn-border-dark-color);
		border-left: 0.9px solid var(--btn-border-light-color);
		border-radius: 3px;
		box-sizing: border-box;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		min-height: 25px;
		min-width: 25px;
		max-height: 75px;
		outline-offset: 2px;
		padding: 3px;

		&:disabled {
			cursor: not-allowed;
			opacity: 0.5;
		}

		&.round {
			border-radius: 100px;
			padding: 6px;
		}

		&.right-border-radius {
			border-top-left-radius: 0;
			border-bottom-left-radius: 0;
			border-top-right-radius: 5px;
			border-bottom-right-radius: 5px;
		}

		&.position-absolute {
			position: absolute;
			top: 10px;
			right: 10px;
		}

		&.primary {
			background-color: var(--app-color);
		}

		&.grid-area-settings {
			grid-area: settings;
		}

		&:not(:disabled):hover {
			border-color: var(--btn-border-hover-color);
		}

		&:focus-visible {
			outline: 3px solid orange;
		}
	}

	@media all and (width > 1024px) {
		button.hide-above-1024-px {
			display: none;
		}
	}

	@media all and (width <= 1024px) {
		button.hide-below-1024-px {
			display: none;
		}
	}
</style>
