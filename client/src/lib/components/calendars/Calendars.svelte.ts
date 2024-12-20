export default class Calendars {
	overviews: CalendarOverview[] = $state([]);

	constructor(overviews: CalendarOverview[]) {
		this.overviews = overviews;
	}

	addCalendar({ id, label }: CalendarOverviewT) {
		const overview = new CalendarOverview({ id, label });
		this.overviews.push(overview);
	}

	deleteCalendar(calendarId: number) {
		const index = this.overviews.findIndex((overview) => overview.id === calendarId);
		this.overviews.splice(index, 1);
	}
}

export interface CalendarOverviewT {
	id: number;
	label: string;
}

export class CalendarOverview {
	id: number;
	label: string = $state('');

	constructor({ id, label }: CalendarOverviewT) {
		this.id = id;
		this.label = label;
	}

	update(label: string) {
		this.label = label;
	}
}
