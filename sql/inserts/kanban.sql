INSERT INTO
    kanban (label, owner_id, is_shared)
VALUES
    ('Project Alpha', 1, true),
    ('Project Beta', 2, true);

INSERT INTO
    shared_kanban (id, user_id)
VALUES
    (1, 2),
    (1, 3),
    (1, 5),
    (2, 1),
    (2, 3);

INSERT INTO
    list (label, kanban_id, owner_id)
VALUES
    ('Service A', 1, 1),
    ('Service B', 1, 1),
    ('Service C', 1, 1),
    ('To Write', 2, 2),
    ('To Review', 2, 2),
    ('Exchanged', 2, 2);

INSERT INTO
    task (label, owner_id, list_id, completion_id)
VALUES
    ('Set up project repo', 1, 1, 1),
    ('Organize the team', 2, 1, 3),
    ('Contact all clients', 3, 1, 2),
    ('Write initial documentation', 1, 1, 1),
    ('Implement feature A', 1, 2, 2),
    ('Review code', 1, 3, 1),
    ('Finalize design', 2, 4, 1),
    ('Gather requirements', 2, 4, 3),
    ('Development Phase 1', 2, 5, 2),
    ('Testing Phase', 2, 6, 1);
