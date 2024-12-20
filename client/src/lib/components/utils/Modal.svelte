<script lang="ts">
	import type { Snippet } from 'svelte';
	import type { ModalClass } from './types';

	import IconButton from './button/IconButton.svelte';

	interface Props {
		children: Snippet;
		class?: ModalClass[];
		isOpen: boolean;
		onclose: () => void;
	}

	let { children, class: CLASS = [], isOpen, onclose }: Props = $props();

	$effect(() => {
		if (isOpen) {
			dialog.showModal();
		} else {
			dialog.close();
		}
	});

	let dialog: HTMLDialogElement;

	function onpointerdown(event: PointerEvent) {
		event.stopPropagation();
		const dialogDimensions = dialog.getBoundingClientRect();
		if (
			event.clientX < dialogDimensions.left ||
			event.clientX > dialogDimensions.right ||
			event.clientY < dialogDimensions.top ||
			event.clientY > dialogDimensions.bottom
		) {
			onclose();
		}
	}
</script>

<dialog bind:this={dialog} class={CLASS.join(' ')} {onclose} {onpointerdown}>
	<IconButton iconName="cross" class={['round', 'position-absolute']} onclick={onclose} />
	{@render children()}
</dialog>

<style>
	dialog {
		background-color: var(--tertiary-bg);
		border: none;
		border-radius: 5px;
		padding: 50px 40px;
		max-height: 85dvh;

		&.right {
			height: 85dvh;
			right: 20px;
			left: auto;
			width: 400px;
		}

		&.bottom {
			width: 80dvw;
			top: auto;
			bottom: 20px;
		}
	}
</style>
