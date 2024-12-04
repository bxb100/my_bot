-- Add up migration script here
-- Add migration script here
create table wager
(
    id      integer primary key autoincrement,
    time_id text    not null,
    user_id integer not null,
    action  text    not null,
    amount  int null
);

create index if not exists wager_time_id on wager (time_id);

create table user
(
    user_id      integer not null primary key,
    name         text,
    points       integer default 0,
    daily_reward integer default 0
);
