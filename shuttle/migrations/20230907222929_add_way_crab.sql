insert into channels
(id, name, hosts, description, url, lang, icon_path, rust_centered, active)
values
(19, 'The Way of the Crab', 'Ecton, ToggleBit',
'We are two Rustaceans who are embarking on a journey to try creating a multiplayer, community-driven game. Both of us were already developing our own open-source projects, and we thought it would be entertaining to others to listen to our progress from the beginning.',
'https://wayofthecrab.com/', 'en', 'https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcSijI8cOGnDuoezmkKQzW51k6Iml7OICDePA7KyjDnhTaKHRxoyOw0FHcibPnMVLENZdmA&usqp=CAU',
true, true);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(19, 'Episode 12: Code reuse could lead to bugs', null,
'Ecton and ToggleBit discuss wordle, optimizing user interfaces, Rust 1.72, and more.',
'en', 'https://wayofthecrab.com/episodes/012/', '2023-08-31', 66*60+5, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(19, 'Episode 11: Always check your inputs and outputs', null,
'Ecton and ToggleBit discuss writing programming languages, the Rust type system, and a little about the user interface of their pending game.',
'en', 'https://wayofthecrab.com/episodes/011/', '2023-08-08', 53*60+30, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(19, 'Episode 10: They''re making a programming language now?!', null,
'Ecton and ToggleBit get back to podcasting and talk about what they''ve been working on, their recording setup, webserver frameworks, and GUI frameworks in Rust.',
'en', 'https://wayofthecrab.com/episodes/010/', '2023-07-29', 105*60+18, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(19, 'Episode 9: Did you start Steam?', null,
'Ecton and ToggleBit talk about the Rust projects they''ve been working on, work/life balance, modern mobile gaming experiences, and how they are going to start working together.',
'en', 'https://wayofthecrab.com/episodes/009/', '2023-06-30', 56*60+4, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(19, 'Episode 8: Beware of spiders', null,
'Ecton and ToggleBit discuss their adventures into graphics programming with Rust, the worst projects they''ve worked on, and more.',
'en', 'https://wayofthecrab.com/episodes/008/', '2023-06-22', 56*60+28, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(19, 'Episode 7: Shoehorning unsafe code', null,
'Ecton and ToggleBit discuss unsafe code, user experience testing, benchmarking, and what they''ve been working on.',
'en', 'https://wayofthecrab.com/episodes/007/', '2023-06-17', 54*60+17, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(19, 'Episode 6: Making sure the happy path is happy', null,
'Ecton and ToggleBit meet for the sixth episode of The Way of the Crab. In this episode, they discuss what they''ve been working on, their unit testing philosophies, and how they evaluate crates. And most importantly... they avoid discussing their game project for another episode!',
'en', 'https://wayofthecrab.com/episodes/006/', '2023-06-09', 57*60+21, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(19, 'Episode 5: Putting Lipstick on a Pig', null,
'Ecton and ToggleBit meet for the fifth episode of The Way of the Crab. Ecton and ToggleBit discuss what they''ve been working on, their personal development practices, and Rust 1.70. They discuss everything except their game project.',
'en', 'https://wayofthecrab.com/episodes/005/', '2023-06-01', 66*60+8, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(19, 'Episode 4: Bunnies, Boxes, and Style', null,
'Ecton and ToggleBit meet for the fourth episode of The Way of the Crab.

In this episode they discuss:

- What they''ve been working on
- User interface styling
- Premature optimization
- Box<str> vs String
- Will their game have questing in our game?
- Making a programmable game?
- Backwards compatibility',
'en', 'https://wayofthecrab.com/episodes/004/', '2023-05-26', 60*60+2, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(19, 'Episode 3: The Pit of Stupid', null,
'Ecton and ToggleBit meet for the third episode of The Way of the Crab.

In this episode they discuss:

- What have they been working on?
- Premature allocation optimization
- Squashing Commits vs Keeping Commit History
- Domain Specific Languages (DSLs) vs Pure Rust
- Our game project',
'en', 'https://wayofthecrab.com/episodes/003/', '2023-05-18', 60*60+41, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(19, 'Episode 2: select * from crabs', null,
'Ecton and ToggleBit meet for the second episode of The Way of the Crab.

The main discussion topics are:

- What have we been working on?
- Learning Rust for new programmers
- What makes Rust so alluring to us?
- What are we hoping to get out of future Rust releases?',
'en', 'https://wayofthecrab.com/episodes/002/', '2023-05-12', 57*60+20, null);


insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(19, 'Episode 1: Hello, World!', null,
'Ecton and ToggleBit meet for an introductory episode of The Way of the Crab.

The main topics are:

- Who are we?
- How did we start programming?
- When did we start using Rust?
- What are we trying to build?
- What does each of us think our favorite/most-often-used crate is?

We also talk about our user interface projects that we''ve been working on recently.',
'en', 'https://wayofthecrab.com/episodes/001/', '2023-05-05', 69*60+21, null);




insert into channels
(id, name, hosts, description, url, lang, icon_path, rust_centered, active)
values
(20, 'Software Unscripted', 'Richard Feldman',
'A weekly podcast of casual conversations about code, hosted by @rtfeldman',
'https://open.spotify.com/show/0cWHExP9zgcPaaSRScHVtO', 'en',
'https://pbs.twimg.com/profile_images/1475862358488760326/0frttBfU_400x400.jpg',
false, true);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(20, 'Niche Domain Knowledge with Ashley Williams', 'Ashley Williams',
'Richard talks with former Rust core team member Ashley Williams, aka ag_dubs,, about various different types of niche domain knowledge - from CSS tricks in web development to low-level systems programming, package managers, and even organization-specific domain knowledge.',
'en', 'https://open.spotify.com/episode/6ou50FO7hFs31Ga2BIzyUM', '2023-08-08', 54*60+40, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(20, 'Speeding up Rust''s Compiler', 'Nicholas Nethercote',
'Richard talks with Nicholas Nethercote, a member of the Rust programming language''s Performance Working Group and author of the Rust Performance Book. They discuss how he and others have worked to speed up Rust''s compiler, different strategies for speeding up compilers in general, and how compiler performance fits into',
'en', 'https://open.spotify.com/episode/2hRbSDIHK6cOhDkORzevFV', '2023-03-15', 49*60+12, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(20, 'The Rust + Elm Stack', 'Dan Bruder',
'Dan Bruder talks with Richard about his experiences using a stack of Rust + Elm at StructionSite, a company that makes software for construction workers to use on job sites.',
'en', 'https://open.spotify.com/episode/6cnAHvdCXedoHxG4w9pWOV', '2023-02-20', 48*60+8, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(20, 'Scratch-Building an Operating System with Steve Klabnik', 'Steve Klabnik',
'Richard talks with Steve Klabnik about his experiences being a major contributor to Ruby on Rails, and then to Rust, and now to a scratch-built operating system at Oxide.',
'en', 'https://open.spotify.com/episode/5PjAThS9ehqwf2ceRMyxTH', '2023-01-16', 56*60+59, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(20, 'Functional Programming in Rust with Luke Westby', 'Luke Westby',
'Richard and Luke discuss functional programming in Rust, based on their experiences doing FP in Rust and in other languages.',
'en', 'https://open.spotify.com/episode/1PD9ZJ9Uy1SrfD4dpqlkDV', '2022-06-30', 52*60+10, null);
