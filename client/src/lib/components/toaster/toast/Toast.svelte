<script lang="ts">
	import type Toast from '../Toast.svelte';
	import { dismissToast } from '../Toaster.svelte';
	import IconButton from '$lib/components/utils/button/IconButton.svelte';
	import { fly } from 'svelte/transition';

	interface Constructor {
		toast: Toast;
	}

	let { toast }: Constructor = $props();
</script>

<div
	class="toast"
	class:success={toast.type === 'success'}
	class:danger={toast.type === 'danger'}
	class:warning={toast.type === 'warning'}
	transition:fly={{ y: -10, duration: 250 }}
>
	{toast.message}
	{#if toast.hasCountdown}
		<div class="progress-bar"></div>
	{/if}
	<IconButton
		onclick={() => dismissToast(toast.id)}
		iconName="cross"
		class={['round', 'position-absolute']}
	/>
</div>

<style>
	.toast {
		--progress-bar-height: 6px;
		background-color: var(--primary-bg);
		border-radius: 3px;
		box-shadow:
			rgba(50, 50, 93, 0.25) 0px 6px 12px -2px,
			rgba(0, 0, 0, 0.3) 0px 3px 7px -3px;
		font-size: 25px;
		padding: 15px;
		position: relative;
		width: 300px;

		&.success {
			background-color: var(--success-color);
		}

		&.danger {
			background-color: var(--danger-color);
		}

		&.warning {
			background-color: var(--warning-color);
		}
	}

	.progress-bar {
		background-color: light-dark(#000, #fff);
		border-bottom-left-radius: 3px;
		position: absolute;
		bottom: 0;
		left: 0;
		height: var(--progress-bar-height);
		width: 100%;
		animation: decrease 4s linear forwards;
	}

	@keyframes decrease {
		from {
			width: 100%;
		}
		to {
			width: 0%;
		}
	}
</style>
