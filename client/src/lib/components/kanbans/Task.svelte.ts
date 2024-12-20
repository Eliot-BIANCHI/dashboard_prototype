import type { User } from '$lib/Account.svelte';
import type { Temporal } from '@js-temporal/polyfill';

export interface Completion {
	id: number;
	label: string;
}

export const completions: Completion[] = [
	{ id: 1, label: 'Pending' },
	{ id: 2, label: 'Started' },
	{ id: 3, label: 'Done' }
];

export interface TaskT {
	id: number;
	label: string;
	completionId: number;
	owner: User;
	priority: boolean;
	assignees: User[];
	createdAt: Temporal.PlainDate;
}

export default class Task {
	id: number;
	label: string = $state('');
	createdAt: Temporal.PlainDate;
	completionId: number = $state(-1);
	owner: User;
	priority: boolean = $state(false);
	assignees: User[] = $state([]);

	constructor({ id, label, completionId, owner, priority, createdAt, assignees }: TaskT) {
		this.id = id;
		this.label = label;
		this.createdAt = createdAt;
		this.completionId = completionId;
		this.owner = owner;
		this.priority = priority;
		this.assignees = assignees;
	}

	increaseCompletion(newIndex: number) {
		this.completionId = completions[newIndex].id;
	}

	update({
		label,
		completionId,
		priority
	}: Omit<TaskT, 'id' | 'owner' | 'createdAt' | 'assignees'>) {
		this.label = label;
		this.completionId = completionId;
		this.priority = priority;
	}

	assignUser(user: User) {
		this.assignees.push(user);
	}

	unassignUser(userId: number) {
		const index = this.assignees.findIndex((assignee) => assignee.id === userId);
		this.assignees.splice(index, 1);
	}
}
