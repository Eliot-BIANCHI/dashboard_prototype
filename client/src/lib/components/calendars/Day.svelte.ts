import Schedule from './Schedule.svelte';

interface DayT {
	label: string;
	number: number;
	schedules: Schedule[];
}

export default class Day {
	label: string;
	number: number;
	schedules: Schedule[] = $state([]);

	constructor({ label, number, schedules }: DayT) {
		this.label = label;
		this.number = number;
		this.schedules = schedules;
	}

	addSchedule(schedule: Schedule) {
		this.schedules.push(schedule);
	}

	deleteSchedule(scheduleId: number) {
		const index = this.schedules.findIndex((schedule) => schedule.id === scheduleId);
		this.schedules.splice(index, 1);
	}
}
