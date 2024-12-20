<script lang="ts">
	import toaster from '../Toaster.svelte';
	import Toast from './Toast.svelte';
</script>

<div class="toasts">
	{#each toaster.toasts as toast (toast.id)}
		<Toast {toast} />
	{/each}
</div>

<style>
	.toasts {
		display: flex;
		flex-direction: column-reverse;
		gap: 15px;
		position: fixed;
		top: 30px;
		left: 50%;
		translate: -50%;
		z-index: 100;
	}
</style>
