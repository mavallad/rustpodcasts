create table episodes (
	id bigint generated always as identity primary key,
	channel_id bigint not null references channels(id),
	title text not null,
	guest text,
	description text,
	url text not null,
	date_emited date,
	duration_seconds integer,
	icon_path text
);

