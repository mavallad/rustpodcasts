update channels set rust_centered = true where id = 6;

update channels set icon_path = 'images/channels/se-radio-icon.png' where id = 9;

insert into channels
(id, name, hosts, description, url, lang, icon_path, rust_centered, active)
values
(14, 'Les Cast Codeurs Podcast', 'Emmanuel Bernard, Arnaud Héritier, Guillaume Laforge, Antonio Goncalves, Vincent Massol, Audrey Neveu',
'Les Cast Codeurs est un podcast en français de, par et pour les développeurs.

Prenez connaissance des dernières nouvelles de la sphère Java et du développement en général. Plongez sur un sujet précis avec les épisodes interview.',
'https://lescastcodeurs.com/', 'fr', 'https://lescastcodeurs.com/images/logo_carre_transparent_150height.png', false, true);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(14, 'LCC 239 - Interview Rust avec François Teychene, Sylvain Wallez et Geoffroy Couprie', 'François Teychene, Sylvain Wallez, Geoffroy Couprie',
'Dans cet épisode, Audrey a fait appel à l’aide d’un ami, François Teychene pour interview Sylvain Wallez et Geoffroy Couprie sur le langage le plus populaire de ces dernières années : Rust.',
'fr', 'https://lescastcodeurs.com/2020/10/05/lcc-239-interview-rust-avec-francois-teychene-sylvain-wallez-et-geoffroy-couprie/', '2020-10-02', 70*60+48, null);

insert into channels
(id, name, hosts, description, url, lang, icon_path, rust_centered, active)
values
(15, 'The Coder Career', 'Cameron Blackwood', 'A Podcast and Meetup Group to help you get the most out of your technology career!',
'https://www.thecodercareer.com/', 'en', 'https://www.thecodercareer.com/_next/image?url=%2Ftcchero.png&w=256&q=75', false, true);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(15, '#66: Joshua Mo | DevRel at ShuttleRS', 'Joshua Mo',
'In this episode of The Coder Career Podcast, host Cameron interviews Joshua Mo, a software engineer with expertise in Rust and web development. The conversation revolves around Joshua''s career journey, including how he entered the industry and advanced his career. They also delve into the benefits of Rust, such as memory safety, low-level control, and speed, and its growing popularity in web development. Additionally, they discuss the future of web development and emerging technologies that are transforming the industry. This episode is a must-listen for programmers who want to learn from Joshua''s experience and stay up-to-date on the latest trends in the tech industry.',
'en', 'https://shows.acast.com/the-coder-career/episodes/66-joshua-mo', '2023-05-15', 42*60+26, null);

insert into channels
(id, name, hosts, description, url, lang, icon_path, rust_centered, active)
values
(16, 'RustShip', 'Marco Ieni', 'Marco Ieni interviews other Rust developers to learn from their experience. RustShip is the podcast for developers who ship Rust code!',
'https://podcasters.spotify.com/pod/show/marco-ieni', 'en', 'https://s3-us-west-2.amazonaws.com/anchor-generated-image-bank/staging/podcast_uploaded_nologo400/38514371/38514371-1689958767366-3e8730e399d79.jpg',
true, true);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(16, 'Creating successful open source projects - Orhun Parmaksız', 'Orhun Parmaksız',
'In the first episode, I interviewed Orhun, Arch Linux package maintainer and author of tens of open-source projects used and loved by thousands of people, including myself.

Orhun shared what it’s like maintaining Rust code and his advice on increasing the adoption of our open-source projects.',
'en', 'https://podcasters.spotify.com/pod/show/marco-ieni', '2023-08-01', 95*60+14, null);



insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(16, 'Trustfall and cargo-semver-checks - Predrag Gruevski', 'Predrag Gruevski',
'Querying Rust API and beyond. In this episode, Predrag Gruevski walks us through Trustfall and Cargo-semver-checks, two Rust projects he created.',
'en', 'https://podcasters.spotify.com/pod/show/marco-ieni', '2023-08-27', 86*60+19, null);




insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(5, '013 - Ryan Summers', 'Ryan Summers',
'In this episode, James chats with Ryan Summers about the process of developing protocols, the guidelines of working in safety critical and embedded engineering.',
'en', 'https://jamesmunns.com/podcast/013-ryan/', '2023-08-03', 80*60+28, null);
