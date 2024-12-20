<script lang="ts">
	import Button from '$lib/components/utils/button/Button.svelte';
	import type { SortTaskOption } from '$lib/components/kanbans/tools.js';

	interface Props {
		by: SortTaskOption;
		setBy: (option: SortTaskOption) => void;
	}

	let { by, setBy }: Props = $props();

	const options: SortTaskOption[] = $state([
		{ ascendingOrder: true, value: 'alpha' },
		{ ascendingOrder: true, value: 'date' },
		{ ascendingOrder: true, value: 'progress' },
		{ ascendingOrder: true, value: 'priority' }
	]);

	$effect(() => {
		const option = options.find((option) => option.value === by.value)!;
		option.ascendingOrder = by.ascendingOrder;
	});
</script>

<ul class="sort-options">
	{#each options as option}
		<li class="sort-option">
			<Button
				iconName={option.ascendingOrder ? 'arrow-up' : 'arrow-down'}
				isSelected={option.value === by.value}
				onclick={() => {
					if (option.value === by.value) option.ascendingOrder = !option.ascendingOrder;
					localStorage.setItem(
						'kanban:sort-tasks-ascending-order',
						JSON.stringify(option.ascendingOrder)
					);
					localStorage.setItem('kanban:sort-tasks-value', option.value);
					setBy(option);
				}}>{option.value}</Button
			>
		</li>
	{/each}
</ul>

<style>
	.sort-options {
		display: flex;
		flex-wrap: wrap;
		list-style: none;
		padding-left: 0;
	}
</style>
