<script lang="ts">
	import { apiPostLoadRequest as apiRequest } from '$lib/api-request/ApiRequest.js';
	import { addToast } from '$lib/components/toaster/Toaster.svelte.js';

	import AddRole from '$lib/components/authorization/role/Add.svelte';
	import AddPermission from '$lib/components/authorization/permission/Add.svelte';

	import Button from '$lib/components/utils/button/Button.svelte';
	import Modal from '$lib/components/utils/Modal.svelte';
	import Boxes from '$lib/components/utils/form/Boxes.svelte';

	let { data } = $props();

	let selectedRoleId = $state(data.calendarAuthorizations.roles[0].id);

	let openAddRole = $state(false);
	let openAddPermission = $state(false);

	async function toggleCalendarRolePermission(calendarRoleId: number, permissionId: number) {
		if (doesRolePermissionExists(calendarRoleId, permissionId)) {
			const res = await apiRequest.rolesPermissions.deleteCalendar({
				calendarRoleId,
				permissionId
			});

			if (res.type === 'ok') {
				data.calendarAuthorizations.deleteRolePermission(calendarRoleId, permissionId);
			} else {
				addToast({ message: 'Failed to delete the role permission', type: 'danger' });
			}
		} else {
			const res = await apiRequest.rolesPermissions.addCalendar({ calendarRoleId, permissionId });

			if (res.type === 'ok') {
				data.calendarAuthorizations.addRolePermission({ calendarRoleId, permissionId });
			} else {
				addToast({ message: 'Failed to add the role permission', type: 'danger' });
			}
		}
	}

	async function addCalendarRole(e: SubmitEvent) {
		const form = e.target as HTMLFormElement;
		const formData = new FormData(form);

		const roleData = {
			label: formData.get('create-calendar-role-label') as string
		};

		const res = await apiRequest.roles.add({
			data: { label: roleData.label }
		});

		if (res.type === 'ok') {
			data.calendarAuthorizations.addRole({
				id: res.value,
				label: roleData.label,
				isDefault: false
			});
			addToast({ message: 'Role added', type: 'success' });
		} else {
			addToast({ message: 'Failed to add the role', type: 'danger' });
		}

		form.reset();
		openAddRole = false;
	}

	async function addCalendarPermission(e: SubmitEvent) {
		const form = e.target as HTMLFormElement;
		const formData = new FormData(form);

		const permissionData = {
			label: formData.get('create-calendar-permission-label') as string
		};

		const res = await apiRequest.permissions.add({
			data: { label: permissionData.label }
		});

		if (res.type === 'ok') {
			data.calendarAuthorizations.addPermission({ id: res.value, label: permissionData.label });

			addToast({ message: 'Permission added', type: 'success' });
		} else {
			addToast({ message: 'Failed to add the permission', type: 'danger' });
		}

		form.reset();
		openAddPermission = false;
	}

	function doesRolePermissionExists(calendarRoleId: number, permissionId: number): boolean {
		return data.calendarAuthorizations.rolesPermissions.some(
			(rolePermission) =>
				rolePermission.calendarRoleId === calendarRoleId &&
				rolePermission.permissionId === permissionId
		);
	}
</script>

<Modal isOpen={openAddRole} onclose={() => (openAddRole = false)}>
	<AddRole onsubmit={addCalendarRole} />
</Modal>

<Modal isOpen={openAddPermission} onclose={() => (openAddPermission = false)}>
	<AddPermission onsubmit={addCalendarPermission} />
</Modal>

<div class="roles-permissions">
	<Boxes
		name="default-calendar-role"
		text="Default role given to the creator of a calendar"
		type="radio"
		boxes={data.calendarAuthorizations.roles.map((role) => ({
			text: role.label,
			value: role.id,
			isChecked: role.isDefault
		}))}
	/>

	<div class="utils">
		<Button onclick={() => (openAddRole = true)} iconName="add" class={['success']}>Role</Button>
		<Button onclick={() => (openAddPermission = true)} iconName="add" class={['success']}>
			Permission
		</Button>
	</div>

	<table>
		<thead>
			<tr>
				<th></th>
				{#each data.calendarAuthorizations.roles as role}
					<th>
						<Button
							isSelected={selectedRoleId === role.id}
							onclick={() => (selectedRoleId = role.id)}>{role.label}</Button
						>
					</th>
				{/each}
			</tr>
		</thead>
		<tbody>
			{#each data.calendarAuthorizations.permissions as permission}
				<tr>
					<th>
						<Button>
							<label for="calendar-role-{selectedRoleId}-permission-{permission.id}">
								{permission.label}
							</label>
						</Button>
					</th>
					{#each data.calendarAuthorizations.roles as role}
						<td>
							<input
								type="checkbox"
								id="calendar-role-{role.id}-permission-{permission.id}"
								name="calendar-role-{role.id}-permission-{permission.id}"
								value="{role.id}-{permission.id}"
								checked={doesRolePermissionExists(role.id, permission.id)}
								onclick={() => toggleCalendarRolePermission(role.id, permission.id)}
							/>
						</td>
					{/each}
				</tr>
			{/each}
		</tbody>
	</table>
</div>

<style>
	.roles-permissions {
		display: flex;
		flex-direction: column;
		gap: 25px;
		flex-grow: 1;
		margin-block: 16px;
		overflow-y: scroll;
	}

	.utils {
		display: flex;
		align-items: center;
		flex-wrap: wrap;
		gap: 30px;
	}

	table {
		border-collapse: collapse;
		text-align: center;
		width: 100%;
	}

	thead > tr > th {
		background-color: var(--quaternary-bg);
	}

	tbody > tr:nth-child(odd) {
		& th,
		& td {
			background-color: var(--primary-bg);
		}
	}

	tbody > tr:nth-child(even) {
		& th,
		& td {
			background-color: var(--secondary-bg);
		}
	}

	th,
	td {
		border: 1px solid #ddd;
		padding: 8px;
	}

	th {
		font-weight: bold;
	}

	label {
		cursor: pointer;
	}

	input {
		appearance: none;
		background-color: var(--quaternary-bg);
		border-radius: 10px;
		cursor: pointer;
		display: flex;
		align-items: center;
		justify-content: center;
		outline: none;
		height: 4em;
		width: 4em;

		&:hover {
			background-color: var(--tertiary-bg);
		}

		&:checked {
			background-color: var(--success-color);

			&::after {
				display: block;
			}
		}

		&::after {
			content: '\2713';
			display: none;
			font-size: 3.12em;
		}
	}
</style>
