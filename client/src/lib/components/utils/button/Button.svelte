<script lang="ts">
	import type { Snippet } from 'svelte';
	import Icon from '../Icon.svelte';
	import type { ButtonClass, ButtonType, IconName } from '../types';

	interface Props {
		children: Snippet;
		class?: ButtonClass[];
		isFocusable?: boolean;
		iconName?: IconName;
		type?: ButtonType;
		onclick?: () => void;
		isDisabled?: boolean;
		isSelected?: boolean;
	}

	let {
		children,
		class: CLASS = [],
		isFocusable = true,
		iconName,
		type = 'button',
		onclick = () => {},
		isSelected = false,
		isDisabled = false
	}: Props = $props();
</script>

<button
	class={CLASS.join(' ')}
	class:selected={isSelected}
	{type}
	tabindex={isFocusable ? 0 : -1}
	{onclick}
	disabled={isDisabled}
>
	{#if iconName !== undefined}
		<Icon name={iconName} />
	{/if}
	{@render children()}
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
		color: var(--primary-text);
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 10px;
		font-size: 1.75rem;
		outline-offset: 2px;
		overflow-x: hidden;
		overflow-y: auto;
		padding: 5px 10px;
		min-height: 30px;
		min-width: 30px;

		&:disabled {
			cursor: not-allowed;
			opacity: 0.5;
		}

		&.align-self-center {
			align-self: center;
		}

		&.column {
			flex-direction: column;
			align-items: flex-start;
			justify-content: flex-start;
		}

		&.round {
			border-radius: 100px;
			height: 50px;
			width: 50px;
		}

		&.no-padding {
			padding: 0;
		}

		&.min-width-150-px {
			min-width: 150px;
		}

		&.width-100 {
			width: 100%;
		}

		&.height-100 {
			height: 100%;
		}

		&.grid-area-progress {
			grid-area: progress;
		}

		&.grid-area-no-assignee {
			grid-area: users;
		}

		&:not(:disabled):hover {
			border-color: var(--btn-border-hover-color);
		}

		&:focus-visible {
			outline: 3px solid orange;
		}

		&.primary,
		&.selected {
			background-color: var(--app-color);
			border-top-color: var(--border-app-light-color);
			border-right-color: var(--border-app-dark-color);
			border-bottom-color: var(--border-app-dark-color);
			border-left-color: var(--border-app-light-color);
		}

		&.success {
			background-color: var(--success-color);
			border-top-color: var(--border-success-light-color);
			border-right-color: var(--border-success-dark-color);
			border-bottom-color: var(--border-success-dark-color);
			border-left-color: var(--border-success-light-color);
		}

		&.danger {
			background-color: var(--danger-color);
			border-top-color: var(--border-danger-light-color);
			border-right-color: var(--border-danger-dark-color);
			border-bottom-color: var(--border-danger-dark-color);
			border-left-color: var(--border-danger-light-color);
		}

		&.warning {
			background-color: var(--warning-color);
			border-top-color: var(--border-warning-light-color);
			border-right-color: var(--border-warning-dark-color);
			border-bottom-color: var(--border-warning-dark-color);
			border-left-color: var(--border-warning-light-color);
		}

		&:is(.primary, .success, .danger, .warning)&:hover {
			border-color: #454545;
		}
	}
</style>
