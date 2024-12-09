-- Add up migration script here
-- Add migration script here
create table gambles
(
    id        integer not null primary key autoincrement,
    serial_id text    not null,
    user_id   integer not null,
    user_name text,
    action    text    not null,
    amount    int     null
);

create unique index if not exists gambles_serial_id_user_id_unique_id on gambles (serial_id, user_id);

create table users
(
    id           integer not null primary key,
    name         text,
    points       integer not null default 0,
    daily_reward timestamp
);


create table jobs
(
    id            integer primary key autoincrement,
    name          text      not null,
--     current_timestamp equals DATETIME('now') it's UTC timestamp
    scheduled_at  timestamp not null default current_timestamp,
    medata        text,
    executed_at   timestamp,
    error_message text
);

create unique index jobs_name_scheduled_at_unique_index on jobs (name, scheduled_at);
