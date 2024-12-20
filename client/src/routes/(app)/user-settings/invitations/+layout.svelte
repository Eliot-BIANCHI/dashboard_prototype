<script lang="ts">
	import { page } from '$app/state';
	import Button from '$lib/components/utils/button/Button.svelte';

	let { children } = $props();
</script>

<div class="invitations">
	<div class="change">
		<Button isSelected={page.url.pathname === '/user-settings/invitations/received'}>
			<a href="/user-settings/invitations/received">Received</a>
		</Button>

		<Button isSelected={page.url.pathname === '/user-settings/invitations/sent'}>Sent</Button>
	</div>

	{@render children()}
</div>

<style>
	.invitations {
		display: flex;
		flex-direction: column;
		gap: 20px;
		flex-grow: 1;
		margin-block: 16px;
	}

	.change {
		background-color: var(--primary-bg);
		border-radius: 5px;
		display: flex;
		justify-content: center;
		gap: 20px;
		padding: 15px;
	}

	a {
		color: var(--primary-text);
		text-decoration: none;
	}
</style>
