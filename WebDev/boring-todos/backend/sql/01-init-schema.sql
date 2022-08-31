CREATE TYPE task_state AS ENUM (
    'ready',
    'inprogress',
    'completed'
);
CREATE TABLE task(
    id bigserial,
    user_id bigint NOT NULL,
    create_time timestamp with time zone DEFAULT now(),
    modify_user_id bigint,
    modify_date timestamp with time zone,
    title text NOT NULL,
    state task_state NOT NULL DEFAULT 'ready'
);
ALTER SEQUENCE task_id_sequence RESTART WITH 1000;