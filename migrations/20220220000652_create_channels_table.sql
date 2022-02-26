create table channels (
	id bigint generated always as identity primary key,
	name text not null,
	description text,
	url text not null,
	lang text not null,
	icon_path text not null,
	active boolean not null default true
);
