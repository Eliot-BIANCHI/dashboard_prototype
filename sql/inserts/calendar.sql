INSERT INTO
    calendar (label, owner_id)
VALUES
    ('Eliot''s Calendar', 1),
    ('Alice''s Calendar', 2),
    ('Bob''s Calendar', 3),
    ('Charlie''s Calendar', 4);

INSERT INTO
    shared_calendar (id, user_id)
VALUES
    (1, 2),
    (1, 3),
    (2, 1),
    (3, 2),
    (4, 5);

INSERT INTO
    schedule (
        label,
        takes_place,
        start_time,
        end_time,
        all_day,
        calendar_id,
        owner_id
    )
VALUES
    (
        'Meeting',
        CURRENT_DATE - INTERVAL '3 day',
        '09:00',
        '10:00',
        FALSE,
        1,
        1
    ),
    (
        'Workshop',
        CURRENT_DATE - INTERVAL '2 day',
        '11:00',
        '13:00',
        FALSE,
        1,
        1
    ),
    (
        'Conference',
        CURRENT_DATE - INTERVAL '1 day',
        '10:00',
        '15:00',
        TRUE,
        1,
        1
    ),
    (
        'Lunch with Team',
        CURRENT_DATE,
        '12:00',
        '13:00',
        FALSE,
        1,
        1
    ),
    (
        'Project Review',
        CURRENT_DATE + INTERVAL '1 day',
        '14:00',
        '15:00',
        FALSE,
        1,
        1
    ),
    (
        'Client Call',
        CURRENT_DATE - INTERVAL '3 day',
        '10:00',
        '11:00',
        FALSE,
        2,
        2
    ),
    (
        'Product Launch',
        CURRENT_DATE,
        '13:00',
        '14:00',
        FALSE,
        2,
        2
    ),
    (
        'Brainstorming',
        CURRENT_DATE + INTERVAL '1 day',
        '09:00',
        '11:00',
        FALSE,
        2,
        2
    ),
    (
        'Interview',
        CURRENT_DATE + INTERVAL '2 day',
        '10:00',
        '12:00',
        FALSE,
        2,
        2
    ),
    (
        'Training Session',
        CURRENT_DATE - INTERVAL '1 day',
        '14:00',
        '16:00',
        FALSE,
        2,
        2
    ),
    (
        'Team Meeting',
        CURRENT_DATE,
        '10:00',
        '11:00',
        FALSE,
        3,
        3
    ),
    (
        'Standup',
        CURRENT_DATE - INTERVAL '1 day',
        '09:00',
        '09:30',
        FALSE,
        3,
        3
    ),
    (
        'Retrospective',
        CURRENT_DATE + INTERVAL '1 day',
        '14:00',
        '15:00',
        FALSE,
        3,
        3
    ),
    (
        'Planning',
        CURRENT_DATE + INTERVAL '2 day',
        '11:00',
        '12:00',
        FALSE,
        3,
        3
    ),
    (
        'One-on-One',
        CURRENT_DATE - INTERVAL '3 day',
        '15:00',
        '15:30',
        FALSE,
        3,
        3
    ),
    (
        'Design Review',
        CURRENT_DATE + INTERVAL '1 day',
        '11:00',
        '12:00',
        FALSE,
        4,
        4
    ),
    (
        'Code Review',
        CURRENT_DATE,
        '14:00',
        '15:30',
        FALSE,
        4,
        4
    ),
    (
        'Testing',
        CURRENT_DATE - INTERVAL '1 day',
        '09:00',
        '10:30',
        FALSE,
        4,
        4
    ),
    (
        'Deployment',
        CURRENT_DATE - INTERVAL '2 day',
        '16:00',
        '17:00',
        FALSE,
        4,
        4
    ),
    (
        'Post-Launch Meeting',
        CURRENT_DATE,
        '13:00',
        '14:00',
        FALSE,
        4,
        4
    );
