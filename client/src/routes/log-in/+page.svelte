<script lang="ts">
	import { page } from '$app/state';
	import { apiPostLoadRequest as apiRequest } from '$lib/api-request/ApiRequest';
	import Form from '$lib/components/utils/form/Form.svelte';
	import Input from '$lib/components/utils/form/Input.svelte';
	import Button from '$lib/components/utils/button/Button.svelte';
	import { goto } from '$app/navigation';

	async function logIn(e: SubmitEvent) {
		const form = e.target as HTMLFormElement;
		const formData = new FormData(form);

		const username = formData.get('username') as string;

		const res = await apiRequest.auth.logIn({
			data: { username, password: 'azerty123' }
		});

		if (res.type === 'ok') {
			const redirectTo = page.url.searchParams.get('redirectTo');

			if (redirectTo !== null) {
				goto(`/${redirectTo.slice(1)}`);
			} else {
				goto('/');
			}
		}
	}
</script>

<svelte:head>
	<title>Log in</title>
</svelte:head>

<Form onsubmit={logIn} class={['align-self-center']}>
	<Input name="username" text="Username" />

	<Button>Log-in</Button>
</Form>
