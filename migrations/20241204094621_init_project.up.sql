-- Add up migration script here
-- Add migration script here
create table gambles
(
    id        BIGSERIAL PRIMARY KEY,
    serial_id TEXT   NOT NULL,
    user_id   BIGINT NOT NULL,
    user_name TEXT,
    action    TEXT   NOT NULL,
    amount    INTEGER
);

create unique index if not exists gambles_serial_id_user_id_unique_id on gambles (serial_id, user_id);

create table users
(
    id           BIGINT  NOT NULL PRIMARY KEY,
    name         TEXT,
    points       INTEGER NOT NULL DEFAULT 0,
    daily_reward TIMESTAMP WITH TIME ZONE
);


create table jobs
(
    id            BIGSERIAL                NOT NULL PRIMARY KEY,
    name          TEXT                     NOT NULL,
    scheduled_at  TIMESTAMP WITH TIME ZONE not null,
    metadata      JSONB,
    executed_at   TIMESTAMP WITH TIME ZONE,
    error_message TEXT
);

create unique index jobs_name_scheduled_at_unique_index on jobs (name, scheduled_at);
