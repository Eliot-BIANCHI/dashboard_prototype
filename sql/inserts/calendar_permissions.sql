INSERT INTO
    calendar_role (label, is_default)
VALUES
    ('Gestionnaire', TRUE),
    ('Éditeur', FALSE),
    ('Contributeur', FALSE);

INSERT INTO
    calendar_role_permission (calendar_role_id, permission_id)
VALUES
    (1, 51),
    (1, 52),
    (1, 53),
    (1, 54),
    (1, 55),
    (1, 56),
    (1, 57),
    (1, 58),
    (2, 54),
    (2, 55),
    (2, 56),
    (2, 57),
    (3, 54),
    (3, 55),
    (3, 56);
