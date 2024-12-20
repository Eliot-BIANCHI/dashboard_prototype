export type Permission =
	| 'section:admin'
	| 'admin-section:users'
	| 'admin-section:authorizations'
	| 'role-permissions:view-all'
	| 'role-permissions:create'
	| 'role-permissions:delete-all'
	| 'permission:create'
	| 'role:create'
	| 'section:calendars'
	| 'calendar:view-own'
	| 'calendar:create'
	| 'calendar:update-own'
	| 'calendar:delete-own'
	| 'calendar:view-invited-to'
	| 'calendar:view-all'
	| 'calendar:update-all'
	| 'calendar:delete-all'
	| 'section:kanbans'
	| 'kanban:view-own'
	| 'kanban:create'
	| 'kanban:update-own'
	| 'kanban:delete-own'
	| 'kanban:view-invited-to'
	| 'kanban:view-all'
	| 'kanban:update-all'
	| 'kanban:delete-all';

export type KanbanPermission =
	| 'kanban:invite-user'
	| 'kanban:delete-user'
	| 'kanban:kick-user'
	| 'list:create'
	| 'list:update-own'
	| 'list:delete-own'
	| 'list:update-all'
	| 'list:delete-all'
	| 'task:create'
	| 'task:move-own'
	| 'task:update-own'
	| 'task:delete-own'
	| 'task:move-all'
	| 'task:update-all'
	| 'task:delete-all'
	| 'task:assign-self'
	| 'task:unassign-self';

export function debounce<T extends (...args: any[]) => any>(
	callback: T,
	delay: number
): (...args: Parameters<T>) => void {
	let timeoutId: ReturnType<typeof setTimeout> | null;
	return function (...args: Parameters<T>) {
		if (timeoutId) clearTimeout(timeoutId);
		timeoutId = setTimeout(() => callback(...args), delay);
	};
}
