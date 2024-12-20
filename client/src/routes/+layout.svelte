<script lang="ts">
	import Toasts from '$lib/components/toaster/toast/Toasts.svelte';
	import SpecialCharClipboard from '$lib/components/SpecialCharClipboard.svelte';

	let { children } = $props();
</script>

<Toasts />

<SpecialCharClipboard />

{@render children()}

<style>
	:root {
		color-scheme: light dark;
		/* Ne pas utiliser px pour fonts */

		--btn-border-dark-color: light-dark(rgb(219, 219, 219), #5c5470);
		--btn-border-light-color: light-dark(rgb(235, 235, 235), #352f44);
		--btn-border-hover-color: light-dark(#cdcdcd, #2a2438);

		--app-color: #8f71ff;
		--border-app-light-color: #8a2be2;
		--border-app-dark-color: #662d91;

		--success-color: #43a277;
		--border-success-light-color: rgb(96, 210, 58);
		--border-success-dark-color: #008878;

		--danger-color: #ff3846;
		--border-danger-light-color: rgb(248, 96, 58);
		--border-danger-dark-color: rgb(172, 39, 6);

		--warning-color: #efd510;
		--border-warning-light-color: #ffd700;
		--border-warning-dark-color: #dbc414;

		/* Light - Dark variables */
		--primary-bg: light-dark(#fff, #222);
		--secondary-bg: light-dark(#f1f1f1, #352f44);
		--tertiary-bg: light-dark(#eee, #1a1a2e);
		--quaternary-bg: light-dark(#e5e1da, #635985);

		--icon-color: light-dark(#000, #fff);

		--primary-text: light-dark(#000, #fff);
		--secondary-text: light-dark(#222, #ddd);

		--primary-border: light-dark(#363a43, #f9f5f6);
		--secondary-border: light-dark(#d3d3d3, #c4b6b6);

		--field-border-color: light-dark(#14141480, #aaa);
		--field-border-color-hover: light-dark(#333, #eee);

		/* Form fields variables */
		--field-border-width: 3px;

		--field-border: var(--field-border-width) solid var(--field-border-color);

		--field-transition-duration: 150ms;
		--field-transition: border-color var(--field-transition-duration);

		--field-disabled-opacity: 0.5;

		--box-checked-color: #00853e;

		/* Main navigation bar */
		--nav-height: 100px;
	}

	:global(body) {
		background-color: var(--secondary-bg);
		display: flex;
		gap: 30px;
		overflow-y: hidden;
		margin: 0;
		height: 100dvh;
		width: 100dvw;
	}

	:global(html) {
		line-height: 1.15;
		-webkit-text-size-adjust: 100%;
	}

	:global(h1) {
		font-size: 2em;
		margin: 0.67em 0;
	}

	:global(hr) {
		box-sizing: content-box;
		height: 0;
		overflow: visible;
	}

	:global(pre) {
		font-family: monospace, monospace;
		font-size: 1em;
	}

	:global(abbr[title]) {
		border-bottom: none;
		text-decoration: underline;
		text-decoration: underline dotted;
	}

	:global(b),
	:global(strong) {
		font-weight: bolder;
	}

	:global(code),
	:global(kbd),
	:global(samp) {
		font-family: monospace, monospace;
		font-size: 1em;
	}

	:global(small) {
		font-size: 80%;
	}

	:global(sub),
	:global(sup) {
		font-size: 75%;
		line-height: 0;
		position: relative;
		vertical-align: baseline;
	}

	:global(sub) {
		bottom: -0.25em;
	}

	:global(sup) {
		top: -0.5em;
	}

	:global(button),
	:global(input),
	:global(optgroup),
	:global(select),
	:global(textarea) {
		font-family: inherit;
		line-height: 1.15;
		margin: 0;
	}

	:global(button),
	:global(select) {
		text-transform: none;
	}

	:global(button),
	:global([type='button']),
	:global([type='reset']),
	:global([type='submit']) {
		-webkit-appearance: button;
		appearance: button;
	}

	:global(button::-moz-focus-inner),
	:global([type='button']::-moz-focus-inner),
	:global([type='reset']::-moz-focus-inner),
	:global([type='submit']::-moz-focus-inner) {
		border-style: none;
		padding: 0;
	}

	:global(button:-moz-focusring),
	:global([type='button']:-moz-focusring),
	:global([type='reset']:-moz-focusring),
	:global([type='submit']:-moz-focusring) {
		outline: 1px dotted ButtonText;
	}

	:global(fieldset) {
		padding: 0.35em 0.75em 0.625em;
	}

	:global(legend) {
		box-sizing: border-box;
		color: inherit;
		display: table;
		max-width: 100%;
		padding: 0;
		white-space: normal;
	}

	:global(progress) {
		vertical-align: baseline;
	}

	:global([type='number']::-webkit-inner-spin-button),
	:global([type='number']::-webkit-outer-spin-button) {
		height: auto;
	}

	:global([type='search']) {
		-webkit-appearance: textfield;
		appearance: textfield;
		outline-offset: -2px;
	}

	:global([type='search']::-webkit-search-decoration) {
		-webkit-appearance: none;
	}

	:global(::-webkit-file-upload-button) {
		-webkit-appearance: button;
		font: inherit;
	}

	:global(details) {
		display: block;
	}

	:global(summary) {
		display: list-item;
	}
</style>
