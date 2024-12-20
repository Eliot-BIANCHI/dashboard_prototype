import Task, { type TaskT } from './Task.svelte';

export interface ListT {
	id: number;
	label: string;
	ownerId: number;
	tasks: Task[];
}

export default class List {
	id: number;
	label: string = $state('');
	ownerId: number;
	tasks: Task[] = $state([]);

	constructor({ id, label, ownerId, tasks }: ListT) {
		this.id = id;
		this.label = label;
		this.ownerId = ownerId;
		this.tasks = tasks;
	}

	update(label: string) {
		this.label = label;
	}

	addTask({ id, label, completionId, priority, owner, createdAt }: TaskT) {
		const task = new Task({
			id,
			label,
			completionId,
			priority,
			owner,
			createdAt
		});

		this.tasks.push(task);
	}

	deleteTask(taskId: number) {
		const index = this.tasks.findIndex((task) => task.id === taskId);
		this.tasks.splice(index, 1);
	}
}
