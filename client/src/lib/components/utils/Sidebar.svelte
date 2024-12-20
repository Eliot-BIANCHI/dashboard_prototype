<script lang="ts">
	import type { Snippet } from 'svelte';
	import IconButton from './button/IconButton.svelte';

	interface Props {
		children: Snippet;
	}

	let { children }: Props = $props();

	let isOpen = $state(false);
</script>

<div class="sidebar" class:active={isOpen}>
	<div class="content">
		{@render children()}
	</div>
	<span class="close-sidebar">
		<IconButton
			iconName="chevron-left"
			class={['round', 'primary', 'hide-above-1024-px']}
			onclick={() => (isOpen = false)}
		/>
	</span>
</div>
<span class="open-sidebar">
	<IconButton
		iconName="chevron-right"
		class={['round', 'primary', 'hide-above-1024-px']}
		onclick={() => (isOpen = true)}
	/>
</span>

<style>
	.sidebar {
		--content-width: 250px;
		border-radius: 5px;
		box-sizing: border-box;
		display: flex;
		gap: 10px;
		height: 100dvh;
		transition: translate 250ms;
	}

	.content {
		background-color: var(--tertiary-bg);
		overflow-y: auto;
		padding: 10px;
		min-width: var(--content-width);
		width: var(--content-width);
	}

	.open-sidebar,
	.close-sidebar {
		display: flex;
		flex-direction: column;
		justify-content: center;
	}

	@media all and (width <= 1024px) {
		.sidebar {
			position: fixed;
			translate: -100%;
			z-index: 10;

			&.active {
				translate: 0%;
			}
		}
	}
</style>
