CREATE TABLE IF NOT EXISTS kanban_role (
    id SERIAL PRIMARY KEY,
    label VARCHAR(31) UNIQUE NOT NULL,
    is_default BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS kanban_role_permission (
    kanban_role_id INT NOT NULL REFERENCES kanban_role (id) ON DELETE CASCADE,
    permission_id INT NOT NULL REFERENCES permission (id) ON DELETE CASCADE,
    PRIMARY KEY (kanban_role_id, permission_id)
);

CREATE TABLE IF NOT EXISTS user_kanban_role (
    kanban_id INT NOT NULL REFERENCES kanban (id) ON DELETE CASCADE,
    kanban_role_id INT NOT NULL REFERENCES kanban_role (id) ON DELETE CASCADE,
    user_id INT NOT NULL REFERENCES app_user (id) ON DELETE CASCADE,
    PRIMARY KEY (kanban_id, user_id)
);

CREATE TABLE IF NOT EXISTS kanban_invitation (
    id SERIAL PRIMARY KEY,
    kanban_id INT NOT NULL REFERENCES kanban (id) ON DELETE CASCADE,
    inviter_id INT NOT NULL REFERENCES app_user (id) ON DELETE CASCADE,
    invitee_id INT NOT NULL REFERENCES app_user (id) ON DELETE CASCADE,
    kanban_role_id INT NOT NULL REFERENCES kanban_role (id) ON DELETE CASCADE,
    status VARCHAR(15) NOT NULL DEFAULT 'pending',
    created_at DATE NOT NULL DEFAULT CURRENT_DATE,
    UNIQUE (kanban_id, inviter_id, invitee_id),
    CHECK (
        status = 'pending'
        OR status = 'accepted'
        OR status = 'rejected'
    )
);
