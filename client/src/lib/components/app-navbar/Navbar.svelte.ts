import type { IconName } from '$lib/components/utils/types';
import { Temporal } from '@js-temporal/polyfill';
import type { Permission } from '$lib/tools';

export interface Link {
	href: string;
	routeName: string;
	iconName: IconName;
	permissionRequired?: Permission;
}

interface NavbarLink {
	href: string;
	routeName: string;
	iconName: IconName;
	permissionRequired?: Permission;
	isHidden: boolean;
}

const hiddenLinks: string[] = JSON.parse(
	localStorage.getItem('favorites:hidden-app-navbar-links') || '[]'
);

class Navbar {
	links: NavbarLink[] = $state([]);

	constructor(links: Link[]) {
		for (const link of links) {
			this.links.push({ ...link, isHidden: hiddenLinks.includes(link.routeName.toLowerCase()) });
		}
	}

	toggleLinkVisibility(routeName: string) {
		const link = this.links.find((link) => link.routeName === routeName)!;
		link.isHidden = !link.isHidden;

		const hiddenLinks: string[] = JSON.parse(
			localStorage.getItem('favorites:hidden-app-navbar-links') || '[]'
		);

		if (link.isHidden) {
			hiddenLinks.push(routeName.toLowerCase());
		} else {
			const index = hiddenLinks.indexOf(routeName.toLowerCase());
			hiddenLinks.splice(index, 1);
		}

		localStorage.setItem('favorites:hidden-app-navbar-links', JSON.stringify(hiddenLinks));
	}

	setCalendarRoute(calendarId: string | null) {
		const link = this.links.find((link) => link.href.startsWith('/calendars'))!;

		if (calendarId === null) {
			link.href = '/calendars';
		} else {
			const date = Temporal.Now.plainDateTimeISO();

			const year = date.year;
			const week = date.weekOfYear;

			link.href = `/calendars/${calendarId}/${year}/weeks/${week}`;
		}
	}

	setKanbanRoute(kanbanId: string | null) {
		const link = this.links.find((link) => link.href.startsWith('/kanbans'))!;

		if (kanbanId === null) {
			link.href = '/kanbans';
		} else {
			link.href = `/kanbans/${kanbanId}`;
		}
	}
}

const links: Link[] = [
	{ href: '/', routeName: 'Home', iconName: 'home' },
	{
		href: '/calendars',
		routeName: 'Calendars',
		iconName: 'calendar',
		permissionRequired: 'section:calendars'
	},
	{
		href: '/kanbans',
		routeName: 'Kanbans',
		iconName: 'kanban',
		permissionRequired: 'section:kanbans'
	},
	{ href: '/users', routeName: 'Users', iconName: 'users' },
	{
		href: '/admin/users',
		routeName: 'Admin',
		iconName: 'admin',
		permissionRequired: 'section:admin'
	},
	{ href: '/user-settings/profile', routeName: 'Settings', iconName: 'user-settings' },
	{ href: '/log-in?log-out=true', routeName: 'Log out', iconName: 'log-out' }
];

const navbar = new Navbar(links);

export default navbar;
