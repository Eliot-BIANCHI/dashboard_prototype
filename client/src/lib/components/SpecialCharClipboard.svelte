<script lang="ts">
	import Modal from '$lib/components/utils/Modal.svelte';
	import Clipboard from '$lib/components/utils/Clipboard.svelte';

	let openSpecialCharacters = $state(false);

	let keyPressed: 'Alt' | undefined = $state();

	function onkeydown(event: KeyboardEvent) {
		if (event.key === 'Alt') {
			keyPressed = event.key;
		}

		if (keyPressed === 'Alt' && (event.key === 's' || event.key === 'S')) {
			event.preventDefault();
			openSpecialCharacters = !openSpecialCharacters;
		}
	}

	function onkeyup(event: KeyboardEvent) {
		if (event.key === 'Alt') {
			keyPressed = undefined;
		}
	}

	const characters = [
		'é',
		'É',
		'è',
		'È',
		'ê',
		'Ê',
		'ë',
		'Ë',
		'à',
		'À',
		'â',
		'Â',
		'î',
		'Î',
		'ï',
		'Ï',
		'ô',
		'Ô',
		'ç',
		'Ç',
		'ù',
		'Ù',
		'û',
		'Û',
		'ü',
		'Ü'
	];
</script>

<svelte:window {onkeydown} {onkeyup} />

<Modal isOpen={openSpecialCharacters} onclose={() => (openSpecialCharacters = false)}>
	<section>
		<h2>Special characters clipboard</h2>

		<ul class="special-characters">
			{#each characters as character}
				<li class="special-character">
					<span>{character}</span>
					<Clipboard textToCopy={character} />
				</li>
			{/each}
		</ul>
	</section>
</Modal>

<style>
	.special-characters {
		display: grid;
		grid-template-columns: repeat(4, minmax(75px, 1fr));
		font-size: 30px;
		padding-left: 0;
	}

	.special-character {
		border-bottom: 2px solid var(--primary-border);
		border-right: 2px solid var(--primary-border);
		display: flex;
		align-items: center;
		justify-content: center;
		padding: 5px;

		&:nth-child(4n) {
			border-right: none;
		}

		&:nth-last-child(2) {
			border-bottom: none;
		}

		&:last-child {
			border-bottom: none;
			border-right: none;
		}

		& span:first-child {
			width: 2ch;
		}
	}
</style>
