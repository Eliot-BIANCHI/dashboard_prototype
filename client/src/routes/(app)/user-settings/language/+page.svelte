<script lang="ts">
	import Icon from '$lib/components/utils/Icon.svelte';
	import type { IconName } from '$lib/components/utils/types';

	interface Language {
		label: string;
		iconName: IconName;
	}

	const languages: Language[] = [
		{ label: 'Français', iconName: 'french' },
		{ label: 'English', iconName: 'english' }
	];
</script>

<ul class="languages">
	{#each languages as { label, iconName }}
		<li class="language">
			<a href="/">
				<Icon name={iconName} />
				{label}
			</a>
		</li>
	{/each}
</ul>

<style>
	.languages {
		flex-grow: 1;
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 10px;
		justify-content: center;
		list-style: none;
		margin-block: 0;
		margin-inline: 10px;
		padding-left: 0;
	}

	.language {
		border-radius: 10px;
		max-width: 300px;
		width: 100%;

		& a {
			background-color: var(--primary-bg);
			color: var(--primary-text);
			display: flex;
			align-items: center;
			gap: 20px;
			font-size: 22px;
			padding: 10px;
			text-decoration: none;

			&:hover,
			&:active {
				background-color: var(--quaternary-bg);
			}
		}
	}
</style>
