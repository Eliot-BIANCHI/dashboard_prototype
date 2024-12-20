export default class Kanbans {
	overviews: KanbanOverview[] = $state([]);

	constructor(overviews: KanbanOverview[]) {
		this.overviews = overviews;
	}

	addKanban({ id, label, isShared }: KanbanOverviewT) {
		const overview = new KanbanOverview({ id, label, isShared });
		this.overviews.push(overview);
	}

	deleteKanban(kanbanId: number) {
		const index = this.overviews.findIndex((overview) => overview.id === kanbanId);
		this.overviews.splice(index, 1);
	}
}

export interface KanbanOverviewT {
	id: number;
	label: string;
	isShared: boolean;
}

export class KanbanOverview {
	id: number;
	label: string = $state('');
	isShared: boolean = $state(false);

	constructor({ id, label, isShared }: KanbanOverviewT) {
		this.id = id;
		this.label = label;
		this.isShared = isShared;
	}

	update(label: string) {
		this.label = label;
	}
}
