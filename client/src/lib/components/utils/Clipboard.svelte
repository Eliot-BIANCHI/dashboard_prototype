<script lang="ts">
	import IconButton from '$lib/components/utils/button/IconButton.svelte';

	interface Props {
		textToCopy: string;
	}

	let { textToCopy }: Props = $props();

	let isClicked = $state(false);
	let timeoutId: ReturnType<typeof setTimeout> | null = null;

	async function setClipboard(text: string) {
		const type = 'text/plain';
		const blob = new Blob([text], { type });
		const data = [new ClipboardItem({ [type]: blob })];
		await navigator.clipboard.write(data);

		if (timeoutId) {
			clearTimeout(timeoutId);
		}

		isClicked = true;
		timeoutId = setTimeout(() => (isClicked = false), 2000);
	}
</script>

<span class="clipboard" class:clicked={isClicked}>
	<IconButton iconName="clipboard" onclick={() => setClipboard(textToCopy)} />
</span>

<style>
	.clipboard {
		position: relative;
		z-index: 10;

		&::before {
			background-color: var(--secondary-bg);
			border-radius: 5px;
			box-shadow:
				rgba(0, 0, 0, 0.1) 0px 1px 3px 0px,
				rgba(0, 0, 0, 0.06) 0px 1px 2px 0px;
			content: 'Copied';
			display: none;
			font-size: 20px;
			padding: 3px;
			position: absolute;
			left: 50%;
			top: -35px;
			translate: -50%;
		}

		&.clicked::before {
			display: initial;
		}
	}
</style>
