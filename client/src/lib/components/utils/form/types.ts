export type InputType =
	| 'button'
	| 'color'
	| 'date'
	| 'datetime-local'
	| 'email'
	| 'file'
	| 'hidden'
	| 'image'
	| 'month'
	| 'number'
	| 'password'
	| 'range'
	| 'reset'
	| 'search'
	| 'submit'
	| 'tel'
	| 'text'
	| 'time'
	| 'url'
	| 'week';

export type InputAutocomplete = 'off' | 'on';

export type BoxesType = 'checkbox' | 'radio';

export interface Box {
	text: string;
	value: string | number;
	isChecked?: boolean;
	isDisabled?: boolean;
}

export interface Option {
	text: string;
	value: string | number;
	isSelected?: boolean;
	isDisabled?: boolean;
}

export type FormClass = 'no-border' | 'no-padding' | 'no-padding-bottom' | 'align-self-center';

export type SubmitClass = 'add' | 'default' | 'delete' | 'update';
