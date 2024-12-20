import {
	preLoadRequest as request,
	RequestError,
	type LoadFetch,
	type ApiResponse
} from '../../tool';
import { type Result, ok, err } from '$lib/Result';

export interface UserSettings {
	getAccount: (loadFetch: LoadFetch) => Promise<Result<Account, string>>;
}

interface Account {
	id: number;
	firstName: string;
	lastName: string;
	imageUrl: string | undefined;
}

const userSettingsEndpoints: UserSettings = {
	getAccount: async (loadFetch) => {
		const resource = '/account';
		const res: Result<ApiResponse<{ account: Account }>, RequestError> = await request(
			resource,
			loadFetch
		);

		if (res.type === 'ok') {
			if (res.value.success) {
				return ok(res.value.data.account);
			} else {
				return err("Couldn't fetch the account");
			}
		}

		return err('INTERNAL SERVER ERROR');
	}
};

export default userSettingsEndpoints;
