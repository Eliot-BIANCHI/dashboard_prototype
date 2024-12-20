import List from './List.svelte';
import Task from './Task.svelte';
import type { KanbanUser } from './Kanban.svelte';
import type { User } from '$lib/Account.svelte';
import { Temporal } from '@js-temporal/polyfill';

export type SortTaskOptionValue = 'alpha' | 'date' | 'progress' | 'priority';

export interface SortTaskOption {
	ascendingOrder: boolean;
	value: SortTaskOptionValue;
}

export interface TaskDragged {
	id: number;
	listId: number;
}

export interface TaskLoaded {
	id: number;
	label: string;
	completionId: number;
	createdAt: string;
	listId: number;
	ownerId: number;
	priority: boolean;
}

export interface ListLoaded {
	id: number;
	label: string;
	ownerId: number;
}

export interface TaskAssignee {
	id: number;
	userId: number;
}

export function extractLists(
	listsLoaded: ListLoaded[],
	tasksLoaded: TaskLoaded[],
	kanbanOwner: KanbanUser,
	users: KanbanUser[],
	tasksAssignees: TaskAssignee[]
): List[] {
	const lists = listsLoaded.map((list) => {
		const tasks = tasksLoaded
			.filter((task) => task.listId === list.id)
			.map((task) => {
				const user = users.find((user) => user.id === task.ownerId);

				const owner: User = {
					id: user === undefined ? kanbanOwner.id : user.id,
					firstName: user === undefined ? kanbanOwner.firstName : user.firstName,
					lastName: user === undefined ? kanbanOwner.lastName : user.lastName,
					imageUrl: user === undefined ? kanbanOwner.imageUrl : user.imageUrl
				};

				const assignees: User[] = tasksAssignees
					.filter((taskAssignee) => taskAssignee.id === task.id)
					.map((taskAssignee) => {
						const user = users.find((user) => user.id === taskAssignee.userId);

						return {
							id: user === undefined ? kanbanOwner.id : user.id,
							firstName: user === undefined ? kanbanOwner.firstName : user.firstName,
							lastName: user === undefined ? kanbanOwner.lastName : user.lastName,
							imageUrl: user === undefined ? kanbanOwner.imageUrl : user.imageUrl
						};
					});

				return new Task({
					id: task.id,
					label: task.label,
					createdAt: Temporal.PlainDate.from(task.createdAt),
					completionId: task.completionId,
					owner,
					priority: task.priority,
					assignees
				});
			});

		return new List({ id: list.id, label: list.label, ownerId: list.ownerId, tasks });
	});

	return lists;
}
