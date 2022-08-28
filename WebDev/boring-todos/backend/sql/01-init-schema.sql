CREATE TYPE task_state AS ENUM (
    'Ready',
    'InProgress',
    'Completed'
);
CREATE TABLE task(
    id bigserial,
    user_id bigint NOT NULL,
    create_time timestamp with time zone DEFAULT now(),
    title text NOT NULL,
    state task_state NOT NULL DEFAULT 'Ready'
);
ALTER SEQUENCE task_id_sequence RESTART WITH 1000;