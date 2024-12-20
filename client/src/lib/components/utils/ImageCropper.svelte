<script lang="ts">
	import Cropper from 'cropperjs';
	// import 'cropperjs/dist/cropper.css';

	interface Props {
		imageSrc: string | ArrayBuffer | null;
		onCrop: (dataUrl: string) => void;
	}

	let { imageSrc, onCrop }: Props = $props();

	let imageElement: HTMLImageElement;
	let cropper: Cropper;

	function initCropper() {
		cropper = new Cropper(imageElement, {
			aspectRatio: 1,
			viewMode: 2,
			autoCropArea: 1,
			crop(event) {
				console.log(event.detail);
			}
		});
	}

	const getCroppedImage = () => {
		if (cropper) {
			const canvas = cropper.getCroppedCanvas({
				width: 300,
				height: 300
			});
			onCrop(canvas.toDataURL('image/jpeg'));
		}
	};

	$effect(() => {
		initCropper();
		return () => {
			cropper?.destroy();
		};
	});
</script>

<div class="image-container">
	<img bind:this={imageElement} src={imageSrc} alt="Update profile" />
</div>
<button onclick={getCroppedImage}>Crop</button>

<style>
	.image-container {
		width: 100%;
		max-width: 400px; /* Adjust based on your design */
		margin: auto;
	}

	img {
		display: block;
		max-width: 100%;
	}
</style>
