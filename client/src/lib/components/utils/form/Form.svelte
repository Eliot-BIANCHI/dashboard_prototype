<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { FormClass } from './types';

	interface Props {
		children: Snippet;
		class?: FormClass[];
		onsubmit: (e: SubmitEvent) => void;
	}

	let { children, class: CLASS = [], onsubmit: handler }: Props = $props();

	function preventDefault(fn: (e: SubmitEvent) => void) {
		return function (this: HTMLFormElement, event: SubmitEvent) {
			event.preventDefault();
			fn.call(this, event);
		};
	}
</script>

<form class={CLASS.join(' ')} onsubmit={preventDefault(handler)}>
	{@render children()}
</form>

<style>
	form {
		border: 2px solid var(--secondary-border);
		border-radius: 10px;
		box-sizing: border-box;
		display: flex;
		flex-direction: column;
		gap: 25px;
		height: max-content;
		margin-inline: auto;
		max-width: 600px;
		min-width: 200px;
		padding: 20px;

		&.no-border {
			border: none;
		}

		&.no-padding {
			padding: 0;
		}

		&.no-padding-bottom {
			padding-bottom: 0;
		}

		&.align-self-center {
			align-self: center;
		}
	}
</style>
