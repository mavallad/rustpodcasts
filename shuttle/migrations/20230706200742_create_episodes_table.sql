create table if not exists episodes (
    id bigint generated always as identity primary key,
    channel_id bigint not null references channels(id),
    title text not null,
    guests text,
    description text not null,
    lang char(2) not null,
    url text not null,
    date_published date not null,
    duration_seconds integer not null,
    icon_path text,
    unique(channel_id, title)
);
