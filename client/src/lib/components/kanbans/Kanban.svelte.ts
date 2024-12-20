import List, { type ListT } from './List.svelte';

export interface KanbanUser {
	id: number;
	firstName: string;
	lastName: string;
	imageUrl: string | null;
	kanbanRoleId: number;
	kanbanRoleLabel: string;
}

interface KanbanT {
	id: number;
	owner: KanbanUser;
	lists: List[];
	isShared: boolean;
	sharedWith: KanbanUser[];
}

export default class Kanban {
	id: number;
	owner: KanbanUser;
	lists: List[] = $state([]);
	isShared: boolean;
	sharedWith: KanbanUser[] = $state([]);

	constructor({ id, owner, lists, isShared, sharedWith }: KanbanT) {
		this.id = id;
		this.owner = owner;
		this.lists = lists;
		this.isShared = isShared;
		this.sharedWith = sharedWith;
	}

	addList({ id, label, ownerId, tasks }: ListT) {
		const list = new List({
			id,
			label,
			ownerId,
			tasks
		});

		this.lists.push(list);
	}

	deleteList(listId: number) {
		const index = this.lists.findIndex((list) => list.id === listId);
		this.lists.splice(index, 1);
	}

	addUser(user: KanbanUser) {
		this.sharedWith.push(user);
	}

	removeUser(userId: number) {
		const index = this.sharedWith.findIndex((user) => user.id === userId);
		this.sharedWith.splice(index, 1);
	}
}
