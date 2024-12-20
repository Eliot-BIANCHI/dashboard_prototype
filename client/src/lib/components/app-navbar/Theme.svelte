<script lang="ts">
	const prefersDarkScheme = window.matchMedia('(prefers-color-scheme: dark)');

	let darkTheme = $state(prefersDarkScheme.matches);
</script>

<button
	class="toggle-theme"
	onclick={() => (darkTheme = !darkTheme)}
	aria-label="toggle-light-dark-theme"
>
	<span class="theme" class:sun={!darkTheme} class:moon={darkTheme}></span>
</button>

<style>
	.toggle-theme {
		background-color: var(--primary-bg);
		border: 2px solid var(--primary-text);
		border-radius: 99px;
		cursor: pointer;
		height: 40px;
		margin-top: auto;
		min-height: 40px;
		position: relative;
		transition: background-color 200ms;
		width: 75px;

		&:hover {
			background-color: var(--secondary-bg);
		}
	}

	.theme {
		background-color: var(--secondary-bg);
		background-image: url('$lib/assets/icons/sun.svg');
		background-repeat: no-repeat;
		background-position: center;
		border-radius: 99px;
		height: 30px;
		position: absolute;
		top: 3px;
		left: 5px;
		width: 30px;
		transition: translate 250ms;

		&.sun {
			background-image: url('$lib/assets/icons/sun.svg') !important;
			translate: 0% !important;
		}

		&.moon {
			background-image: url('$lib/assets/icons/moon.svg') !important;
			translate: 100% !important;
		}
	}

	@media (prefers-color-scheme: dark) {
		.theme {
			background-image: url('$lib/assets/icons/moon.svg');
			translate: 100%;
		}
	}

	:global(html:has(.theme.sun)) {
		color-scheme: light !important;
	}

	:global(html:has(.theme.moon)) {
		color-scheme: dark !important;
	}
</style>
