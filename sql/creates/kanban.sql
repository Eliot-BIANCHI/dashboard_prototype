CREATE TABLE IF NOT EXISTS kanban (
    id SERIAL PRIMARY KEY,
    label VARCHAR(63) NOT NULL,
    owner_id INT NOT NULL REFERENCES app_user (id) ON DELETE CASCADE,
    is_shared BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS shared_kanban (
    id INT NOT NULL REFERENCES kanban (id) ON DELETE CASCADE,
    user_id INT NOT NULL REFERENCES app_user (id) ON DELETE CASCADE,
    PRIMARY KEY (id, user_id)
);

CREATE TABLE IF NOT EXISTS list (
    id SERIAL PRIMARY KEY,
    label VARCHAR(63) NOT NULL,
    kanban_id INT NOT NULL REFERENCES kanban (id) ON DELETE CASCADE,
    owner_id INT NOT NULL REFERENCES app_user (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS task (
    id SERIAL PRIMARY KEY,
    label VARCHAR(255) NOT NULL,
    completion_id INT NOT NULL,
    list_id INT NOT NULL REFERENCES list (id) ON DELETE CASCADE,
    owner_id INT NOT NULL REFERENCES app_user (id) ON DELETE CASCADE,
    priority BOOLEAN NOT NULL DEFAULT FALSE,
    created_at DATE NOT NULL DEFAULT CURRENT_DATE,
    CHECK (
        completion_id >= 1
        AND completion_id <= 3
    )
);

CREATE TABLE IF NOT EXISTS task_assignee (
    id INT NOT NULL REFERENCES task (id) ON DELETE CASCADE,
    user_id INT NOT NULL REFERENCES app_user (id) ON DELETE CASCADE,
    PRIMARY KEY (id, user_id)
);
