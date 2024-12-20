import { type Result, ok, err } from '$lib/Result';

interface Endpoint {
	method: string;
	resource: string;
	params: object;
	body: object | null;
}

export interface ApiResponse<T> {
	data: T;
	success: boolean;
}

export type LoadFetch = (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>;

export enum RequestError {
	InternalServerError = 'InternalServerError'
}

const API_BASE_URL = import.meta.env.VITE_API_BASE_URL;

export async function preLoadRequest(
	resource: string,
	loadFetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>
): Promise<Result<any, RequestError>> {
	try {
		const res = await loadFetch(API_BASE_URL + resource, {
			method: 'GET',
			credentials: 'include'
		});
		const json = await res.json();
		return ok(json);
	} catch (error) {
		console.log('Error:', error);
		return err(RequestError.InternalServerError);
	}
}

export async function postLoadRequest(endpoint: Endpoint): Promise<Result<any, RequestError>> {
	try {
		const headers: HeadersInit =
			endpoint.method === 'POST' || endpoint.method === 'PUT' || endpoint.method === 'PATCH'
				? { 'Content-Type': 'application/json' }
				: {};

		const res = await fetch(API_BASE_URL + endpoint.resource, {
			method: endpoint.method,
			headers,
			credentials: 'include',
			body: endpoint.body ? JSON.stringify(endpoint.body) : null
		});
		const json = await res.json();
		return ok(json);
	} catch (error) {
		console.log('Error:', error);
		return err(RequestError.InternalServerError);
	}
}
