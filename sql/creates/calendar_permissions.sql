CREATE TABLE IF NOT EXISTS calendar_role (
    id SERIAL PRIMARY KEY,
    label VARCHAR(31) UNIQUE NOT NULL,
    is_default BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS calendar_role_permission (
    calendar_role_id INT NOT NULL REFERENCES calendar_role (id) ON DELETE CASCADE,
    permission_id INT NOT NULL REFERENCES permission (id) ON DELETE CASCADE,
    PRIMARY KEY (calendar_role_id, permission_id)
);

CREATE TABLE IF NOT EXISTS user_calendar_role (
    calendar_id INT NOT NULL REFERENCES calendar (id) ON DELETE CASCADE,
    calendar_role_id INT NOT NULL REFERENCES calendar_role (id) ON DELETE CASCADE,
    user_id INT NOT NULL REFERENCES app_user (id) ON DELETE CASCADE,
    PRIMARY KEY (calendar_id, calendar_role_id, user_id)
);
