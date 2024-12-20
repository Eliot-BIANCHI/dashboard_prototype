CREATE TABLE IF NOT EXISTS calendar (
    id SERIAL PRIMARY KEY,
    label VARCHAR(31) NOT NULL,
    owner_id INT NOT NULL REFERENCES app_user (id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS shared_calendar (
    id INT NOT NULL REFERENCES calendar (id) ON DELETE CASCADE,
    user_id INT NOT NULL REFERENCES app_user (id) ON DELETE CASCADE,
    PRIMARY KEY (id, user_id)
);

CREATE TABLE IF NOT EXISTS schedule (
    id SERIAL PRIMARY KEY,
    label VARCHAR(63) NOT NULL,
    takes_place DATE NOT NULL,
    start_time TIME NOT NULL,
    end_time TIME NOT NULL,
    all_day BOOLEAN NOT NULL DEFAULT FALSE,
    calendar_id INT NOT NULL REFERENCES calendar (id) ON DELETE CASCADE,
    owner_id INT NOT NULL REFERENCES app_user (id) ON DELETE CASCADE,
    CHECK (end_time > start_time)
);
