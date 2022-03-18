create table channels (
	id bigint generated always as identity primary key,
	name text not null unique,
	description text not null,
	url text not null,
	lang char(2) not null,
	icon_path text not null,
	active boolean not null default true
);
