export interface ScheduleT {
	id: number;
	label: string;
	takesPlace: string;
	startTime: string;
	endTime: string;
	duration: number;
	allDay: boolean;
}

export default class Schedule {
	id: number;
	label: string = $state('');
	takesPlace: string = $state('');
	startTime: string = $state('');
	endTime: string = $state('');
	duration: number = $state(1);
	allDay: boolean = $state(false);

	constructor({ id, label, takesPlace, startTime, endTime, duration, allDay }: ScheduleT) {
		this.id = id;
		this.label = label;
		this.takesPlace = takesPlace;
		this.startTime = startTime;
		this.endTime = endTime;
		this.duration = duration;
		this.allDay = allDay;
	}

	update({ label, takesPlace, startTime, endTime, duration, allDay }: Omit<ScheduleT, 'id'>) {
		this.label = label;
		this.takesPlace = takesPlace;
		this.startTime = startTime;
		this.endTime = endTime;
		this.duration = duration;
		this.allDay = allDay;
	}
}
