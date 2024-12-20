type Ok<T> = { type: 'ok'; value: T };
type Err<E> = { type: 'err'; error: E };

export type Result<T, E> = Ok<T> | Err<E>;

export function ok<T>(value: T): Ok<T> {
	return { type: 'ok', value };
}

export function err<E>(error: E): Err<E> {
	return { type: 'err', error };
}
