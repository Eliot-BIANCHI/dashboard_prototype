<script lang="ts">
	import { completions } from '../Task.svelte';
	import Button from '$lib/components/utils/button/Button.svelte';

	interface Props {
		hasPermissionToIncrease: boolean;
		taskCompletionId: number;
		increaseCompletion: () => void;
	}

	let { hasPermissionToIncrease, taskCompletionId, increaseCompletion }: Props = $props();

	let index = $derived(completions.findIndex((completion) => completion.id === taskCompletionId));

	let completion = $derived(completions[index]);

	let completionClass: 'danger' | 'primary' | 'success' = $derived.by(() => {
		if (completion.id === 1) return 'danger';
		else if (completion.id === 2) return 'primary';
		else return 'success';
	});
</script>

<Button
	onclick={increaseCompletion}
	class={['min-width-150-px', 'align-self-center', 'grid-area-progress', completionClass]}
	isDisabled={!hasPermissionToIncrease}
>
	{completion.label}
</Button>
<div class="progress" class:no-permission={!hasPermissionToIncrease}>
	{#each completions as _, i}
		<span class="progress-bar" class:active={i <= index}></span>
	{/each}
</div>

<style>
	.progress {
		align-self: center;
		display: flex;
		gap: 5px;

		&.no-permission {
			padding-right: 20px;
		}

		& .progress-bar {
			background-color: var(--quaternary-bg);
			display: inline-block;
			height: 10px;
			width: 30px;

			&.active {
				background-color: var(--app-color);
			}

			&:first-child {
				border-top-left-radius: 5px;
				border-bottom-left-radius: 5px;
			}

			&:last-child {
				border-top-right-radius: 5px;
				border-bottom-right-radius: 5px;
			}
		}
	}

	@media all and (width < 1300px) {
		.progress {
			display: none;
		}
	}
</style>
