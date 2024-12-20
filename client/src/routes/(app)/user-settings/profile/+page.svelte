<script lang="ts">
	import ImageCropper from '$lib/components/utils/ImageCropper.svelte';

	let { data } = $props();

	let imageSrc: string | ArrayBuffer | null = $state('');
	let croppedImage = $state('');

	const ASSETS_BASE_URL = import.meta.env.VITE_ASSETS_BASE_URL;

	const src = $derived(`${ASSETS_BASE_URL}/images/${data.account.imageUrl || 'default.svg'}`);

	const handleFileChange = (event: Event) => {
		const target = event.target as HTMLInputElement;
		const file = target.files?.[0];
		if (file) {
			const reader = new FileReader();
			reader.onload = () => (imageSrc = reader.result);
			reader.readAsDataURL(file);
		}
	};

	function updateProfilePicture(dataUrl: string) {
		croppedImage = dataUrl;
		// Optionally, upload `croppedImage` to your server here
	}
</script>

<div class="profile">
	<img class="profile-picture" {src} alt="User profile" width="150" height="150" />
	<div>
		<input type="file" accept="image/*" onchange={handleFileChange} />
		{#if imageSrc}
			<ImageCropper {imageSrc} onCrop={updateProfilePicture} />
		{/if}
		{#if croppedImage}
			<div class="preview">
				<h3>Preview:</h3>
				<img src={croppedImage} alt="Cropped profile" />
			</div>
		{/if}
	</div>
	<p>{data.account.firstName} {data.account.lastName}</p>
	<span class="role">{data.account.roleLabel}</span>
</div>

<style>
	.profile {
		background-color: var(--primary-bg);
		border-radius: 5px;
		display: flex;
		align-items: center;
		gap: 50px;
		font-size: 25px;
		flex-grow: 1;
		height: max-content;
		margin-block: 16px;
		overflow-y: auto;
		padding: 30px;
		position: relative;
	}

	.profile-picture {
		border-radius: 100px;
		object-fit: cover;
	}

	.role {
		background-color: var(--quaternary-bg);
		border-radius: 3px;
		padding: 10px;
		position: absolute;
		top: 10px;
		right: 10px;
	}
</style>
