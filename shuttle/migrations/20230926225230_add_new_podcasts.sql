insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(10, 'Pitching Rust to decision-makers, with Joel Marcey', 'Joel Marcey',
'Marco Otto-Witte discusses how to pitch Rust to decision-makers with Joel Marcey, the Director of Technology at the Rust Foundation.',
'en', 'https://rustacean-station.org/episode/joel-marcey-pitching-rust/',
'2023-09-11', 41*60+5, null);


insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(10, 'What''s New in Rust 1.68 and 1.69', null,
'Jon and Ben discuss the highlights of the 1.68 and 1.69 releases of Rust.',
'en', 'https://rustacean-station.org/episode/rust-1.68-1.69/',
'2023-09-19', 51*60+24, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(20, 'The SemVer Rabbit Hole with Predrag Gruevski', 'Predrag Gruevski',
'Richard talks with Predrag Gruevski, author of the cargo-semver-checks tool for detecting accidental semantic versioning mistakes in Rust packages, as well as Trustfall, which is an incredibly flexible query engine. They talk about why semantic versioning is so especially tricky to get right in Rust, tradeoffs in different package managers'' approaches to semver in general, and how his work on cargo-semver-checks motivated him to create a tool for querying data in just about any format.',
'en', 'https://open.spotify.com/episode/0f5vTE3XG5HBLw5EQXCtiu',
'2023-09-12', 58*60+14, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(9, 'SE Radio 581: Zach Lloyd on Terminal Emulators', 'Zach Lloyd',
'Zach Lloyd, CEO of Warp.dev, discusses how to implement and effectively use command-line terminals. Host Gregory M. Kapfhammer speaks with Lloyd about how command-line terminals work and how the Warp terminal uses the GPU and AI to enhance a software developer’s productivity. They also discuss the trade-offs associated with using the Rust programming language to implement a command-line terminal.',
'en', 'https://www.se-radio.net/2023/09/se-radio-581-zach-lloyd-on-terminal-emulators/',
'2023-09-14', 64*60+43, 'https://www.se-radio.net/wp-content/uploads/2023/09/zach_lloyd-100.jpg');

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(16, 'Atuin - Shell history sync, search and backup - Ellie Huxtable', 'Ellie Huxtable',
'In RustShip #3, Ellie Huxtable walks us through Atuin, a Rust CLI tool she created to boost your CTRL+R shell history search. 🐢 Atuin replaces your existing shell history with an SQLite database and records additional context for your commands to give you a faster and better search of your shell history! Additionally, Atuin (optionally) syncs your shell history between your machines! Learn more at https://atuin.sh/',
'en', 'https://podcasters.spotify.com/pod/show/marco-ieni',
'2023-09-23', 69*60+53, null);
