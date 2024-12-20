INSERT INTO
    kanban_role (label, is_default)
VALUES
    ('Gestionnaire', TRUE),
    ('Éditeur', FALSE),
    ('Contributeur', FALSE);

INSERT INTO
    kanban_role_permission (kanban_role_id, permission_id)
VALUES
    (1, 34),
    (1, 35),
    (1, 36),
    (1, 37),
    (1, 38),
    (1, 39),
    (1, 40),
    (1, 41),
    (1, 42),
    (1, 43),
    (1, 44),
    (1, 45),
    (1, 46),
    (1, 47),
    (1, 48),
    (1, 49),
    (1, 50),
    (2, 37),
    (2, 38),
    (2, 39),
    (2, 40),
    (2, 42),
    (2, 43),
    (2, 44),
    (2, 45),
    (2, 46),
    (2, 47),
    (2, 49),
    (2, 50),
    (3, 37),
    (3, 38),
    (3, 39),
    (3, 42),
    (3, 43),
    (3, 44),
    (3, 45),
    (3, 49),
    (3, 50);

INSERT INTO
    user_kanban_role (kanban_id, user_id, kanban_role_id)
VALUES
    (1, 1, 1),
    (1, 2, 2),
    (1, 3, 2),
    (1, 5, 3),
    (2, 2, 1),
    (2, 1, 3),
    (2, 3, 2);
