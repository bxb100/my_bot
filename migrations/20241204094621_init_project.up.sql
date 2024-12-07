-- Add up migration script here
-- Add migration script here
create table wager
(
    id        integer not null primary key autoincrement,
    time_id   text    not null,
    user_id   integer not null,
    user_name text,
    action    text    not null,
    amount    int     null
);

create index if not exists wager_time_id on wager (time_id);
create unique index if not exists wager_unique_id on wager (time_id, user_id);

create table users
(
    id           integer not null primary key,
    name         text,
    points       integer not null default 0,
    daily_reward integer not null default 0
);


create table jobs
(
    id            integer primary key autoincrement,
    name          text      not null,
    scheduled_at  timestamp not null default current_timestamp,
    medata        text,
    executed_at   timestamp,
    error_message text
);

create unique index jobs_name_scheduled_at_unique_index on jobs (name, scheduled_at);
