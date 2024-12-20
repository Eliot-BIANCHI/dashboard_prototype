import preLoadAuth from './pre-load-endpoints/users/auth';
import preLoadUserSettings from './pre-load-endpoints/users/user-settings';
import preLoadUsers from './pre-load-endpoints/users/users';
import preLoadCalendars from './pre-load-endpoints/calendars';
import preLoadKanbans from './pre-load-endpoints/kanbans';
import preLoadRolesPermissions from './pre-load-endpoints/roles_permissions';
import preLoadInvitations from './pre-load-endpoints/invitations';

import postLoadAuth from './post-load-endpoints/users/auth';
import postLoadUserSettings from './post-load-endpoints/users/user-settings';
import postLoadUsers from './post-load-endpoints/users/users';
import postLoadKanbans from './post-load-endpoints/kanbans/kanbans';
import postLoadLists from './post-load-endpoints/kanbans/list';
import postLoadTasks from './post-load-endpoints/kanbans/tasks';
import postLoadCalendars from './post-load-endpoints/calendars/calendars';
import postLoadSchedules from './post-load-endpoints/calendars/schedules';
import postLoadRolesPermissions from './post-load-endpoints/roles-permissions/roles_permissions';
import postLoadPermissions from './post-load-endpoints/roles-permissions/permissions';
import postLoadRoles from './post-load-endpoints/roles-permissions/roles';
import postLoadInvitations from './post-load-endpoints/invitations';

export const apiPreLoadRequest = {
	auth: preLoadAuth,
	calendars: preLoadCalendars,
	kanbans: preLoadKanbans,
	userSettings: preLoadUserSettings,
	users: preLoadUsers,
	rolesPermissions: preLoadRolesPermissions,
	invitations: preLoadInvitations
};

export const apiPostLoadRequest = {
	auth: postLoadAuth,
	kanbans: postLoadKanbans,
	lists: postLoadLists,
	tasks: postLoadTasks,
	calendars: postLoadCalendars,
	schedules: postLoadSchedules,
	userSettings: postLoadUserSettings,
	users: postLoadUsers,
	rolesPermissions: postLoadRolesPermissions,
	permissions: postLoadPermissions,
	roles: postLoadRoles,
	invitations: postLoadInvitations
};
