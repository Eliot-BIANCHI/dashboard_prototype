import type { Permission } from './tools';

interface AccountT {
	id: number;
	firstName: string;
	lastName: string;
	imageUrl: string | null;
	roleId: number;
	roleLabel: string;
	permissions: Permission[];
}

export default class Account {
	id: number;
	firstName: string;
	lastName: string;
	imageUrl: string | null = $state(null);
	roleId: number;
	roleLabel: string;
	permissions: Permission[];
	favoriteCalendarId: string | null = $state(null);
	favoriteKanbanId: string | null = $state(null);

	constructor({ id, firstName, lastName, imageUrl, roleId, roleLabel, permissions }: AccountT) {
		this.id = id;
		this.firstName = firstName;
		this.lastName = lastName;
		this.imageUrl = imageUrl;
		this.roleId = roleId;
		this.roleLabel = roleLabel;
		this.permissions = permissions;
		this.favoriteCalendarId = localStorage.getItem(`${id}-favorites:calendar-id`);
		this.favoriteKanbanId = localStorage.getItem(`${id}-favorites:kanban-id`);
	}

	setFavoriteCalendarId(calendarId: string | null) {
		this.favoriteCalendarId = calendarId;

		if (calendarId === null) {
			localStorage.removeItem(`${this.id}-favorites:calendar-id`);
		} else {
			localStorage.setItem(`${this.id}-favorites:calendar-id`, calendarId);
		}
	}

	setFavoriteKanbanId(kanbanId: string | null) {
		this.favoriteKanbanId = kanbanId;

		if (kanbanId === null) {
			localStorage.removeItem(`${this.id}-favorites:kanban-id`);
		} else {
			localStorage.setItem(`${this.id}-favorites:kanban-id`, kanbanId);
		}
	}
}

export interface User {
	id: number;
	firstName: string;
	lastName: string;
	imageUrl: string | null;
}

export type SearchBy = 'username' | 'first_name' | 'last_name';

export type PaginatedSearchBy = 'name' | 'role' | 'newest' | 'oldest';
