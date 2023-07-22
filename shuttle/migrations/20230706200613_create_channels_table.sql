create table if not exists channels (
    id bigint primary key,
    name text not null unique,
    hosts text,
    description text not null,
    url text not null,
    lang char(2) not null,
    icon_path text,
    rust_centered boolean not null,
    active boolean not null default true
);

