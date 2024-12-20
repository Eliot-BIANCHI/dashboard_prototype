import Toast, { toastsTypes, type ToastConfig } from './Toast.svelte';

class Toaster {
	toasts: Toast[] = $state([]);
}

const toaster = new Toaster();

export function addToast({ message, type = 'info', hasCountdown = true }: ToastConfig) {
	const toastType = toastsTypes.find((toastType) => toastType.value === type)!;

	if (toastType.isHidden) return;

	const id = Math.floor(Math.random() * 10000);

	const toast = new Toast({ id, message, type, hasCountdown });

	toaster.toasts.push(toast);

	if (toast.hasCountdown) {
		setTimeout(() => dismissToast(id), 4000);
	}
}

export function dismissToast(toastId: number) {
	const index = toaster.toasts.findIndex((toast) => toast.id === toastId);
	if (index === -1) return;
	toaster.toasts.splice(index, 1);
}

export default toaster;
