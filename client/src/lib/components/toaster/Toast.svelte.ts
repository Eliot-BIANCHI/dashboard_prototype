type Type = 'info' | 'success' | 'danger' | 'warning';

interface ToastType {
	value: Type;
	isHidden: boolean;
}

const hiddenTypes: string[] = JSON.parse(
	localStorage.getItem('favorites:hidden-notification-types') || '[]'
);

export const toastsTypes: ToastType[] = $state(
	['info', 'success', 'danger', 'warning'].map((type) => ({
		value: type as Type,
		isHidden: hiddenTypes.includes(type)
	}))
);

interface Constructor {
	id: number;
	message: string;
	type: Type;
	hasCountdown: boolean;
}

export interface ToastConfig {
	message: string;
	type?: Type;
	hasCountdown?: boolean;
}

export default class Toast {
	id: number;
	message: string;
	type: Type;
	hasCountdown: boolean;

	constructor({ id, message, type, hasCountdown }: Constructor) {
		this.id = id;
		this.message = message;
		this.type = type;
		this.hasCountdown = hasCountdown;
	}
}
