update channels set icon_path = '/images/channels/se-radio-icon.png' where id = 9;

insert into channels
(id, name, hosts, description, url, lang, icon_path, rust_centered, active)
values
(17, 'Are we podcast yet?', 'Vijay Kiran, Wouter Dullaert', 'A new fn Podcast about all things Rust by https://twitter.com/vijaykiran and https://twitter.com/wouterdullaert',
'https://arewepodcastyet.com/', 'en', '/images/channels/arewepodcastyet.jpg',
true, false);


insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(17, 'AWPY - 00 - Hello, World!', null,
'Hello, World! We just kick-started a new podcast about Rust - we both are rust newbies, and our goal is to make a small contribution to rust community by providing a nice podcast that shines spotlight on several amazing people and their work in Rust.

The secret subversive goal is to accelerate our learning of Rust and also for someone who is in the same boat like us :)',
'en', 'https://soundcloud.com/arewepodcastyet/awpy-00-hello-world', '2019-10-21', 31*60+11, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(17, 'AWPY - 01 Ashley @ag_dubs', 'Ashley Williams',
'In our very first episode we are honoured to have the inimitable Ashley as our guest! We get to know about her journey into Rust from unlikely beginnings, learn about how Rust core team and working groups'' inner workings and more!

After listening to the episode - don''t forget to tweet @ag_dubs and "spam" with your response :)',
'en', 'https://soundcloud.com/arewepodcastyet/awpy-01', '2019-11-05', 92*60+41, 'https://production.listennotes.com/podcasts/are-we-podcast-yet/awpy-01-ashley-ag_dubs-URt7g5GYIFB-ZausmiQtXH3.300x300.jpg');

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(17, 'AWPY - 02 Andy Grove aka @andygrove73', 'Andy Grove',
'After the holiday slumber, we got around to prepare new episode for you :) We are excited to talk to Andy Grove and talked about his extensive experience with Big Data and his projects in Rust - Data Fusion (Apache Arrow), rdbc and more :)',
'en', 'https://soundcloud.com/arewepodcastyet/awpy-02-andy-grove-aka-andygrove73', '2020-01-15', 67*60+40, 'https://production.listennotes.com/podcasts/are-we-podcast-yet/awpy-02-andy-grove-aka-GexfijAv-9N-t_FDCSNcigA.300x300.jpg');

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(17, 'AWPY - 03 Carol (Nichols || Goulding)', 'Carol Nichols',
'We got to talk to Carol co-author of Rustbook, creator of Rustlings, co-author of Rust in Motion video series!',
'en', 'https://soundcloud.com/arewepodcastyet/awpy-03-carol-nichols-goulding', '2020-02-03', 64*60+14, 'https://production.listennotes.com/podcasts/are-we-podcast-yet/awpy-03-carol-nichols-goulding-fe3oTupPYAR-rRYN59iKChH.300x300.jpg');

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(17, 'AWPY - 04 Jane Lusby (@yaahc_)', 'Jane Lusby',
'We are back with inimitable and awesome Jane! talking about error handling/community mentoring and more :)',
'en', 'https://soundcloud.com/arewepodcastyet/awpy-04-jane-lusby-yaahc', '2020-05-05', 69*60+40, 'https://production.listennotes.com/podcasts/are-we-podcast-yet/awpy-04-jane-lusby-yaahc_-xlWbckMLH5O-u_aeQjx71s6.300x300.jpg');

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(17, 'AWPY - 05 Tim McNamara (@timclicks)', 'Tim McNamara',
'In yet another super interesting episode, we present you Mr. Tim McNamara from beautiful COVID-free New Zealand, Software developer at Canonical, Data Scientist, and most importantly author of Rust In Action book :)

We woke up Tim super early to pick his brain on lot of Rusty goodness, and behind the scenes of his 4 years effort to write best book ever. Checkout www.rustinaction.com to buy the book',
'en', 'https://soundcloud.com/arewepodcastyet/awpy-05-tim-mcnamara-timclicks', '2020-06-13', 99*60+17, 'https://production.listennotes.com/podcasts/are-we-podcast-yet/awpy-05-tim-mcnamara-timclicks-ltbNk_RTp3w-xZchMiwdY_W.300x300.jpg');

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(17, 'AWPY - 06 Jonathan Turner', 'Jonathan Turner',
'We had plenty of fun discussing rust and all sort of other things: retro computing, programming languages, and even some poetry with Jonathan!',
'en', 'https://soundcloud.com/arewepodcastyet/awpy-06-jonathan-turner', '2020-08-11', 101*60+46, 'https://production.listennotes.com/podcasts/are-we-podcast-yet/awpy-06-jonathan-turner-p6VWM8kGEAO-gA4fXH2lKSL.300x300.jpg');

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(17, 'AWPY - 07 Georg Semmler on Diesel !', 'Georg Semmler',
'We are back with season 2 of AWPY!

We deep dive into Diesel with Georg and he tells us how he is taking over the (rust) world by adding Diesel test suite to one project at a time :D

Checkout his GH profile https://github.com/weiznich and you can Sponsor Georg''s work by going to https://github.com/sponsors/weiznich',
'en', 'https://soundcloud.com/arewepodcastyet/awpy-07-georg-semmler-on-diesel', '2021-07-26', 70*60+18, 'https://production.listennotes.com/podcasts/are-we-podcast-yet/awpy-07-georg-semmler-on-gWMOc0wRFfm-h6GSv8IZzDq.300x300.jpg');

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(17, 'AWPY - 08 Jon Ferdinand Ronge Gjengset', 'Jon Gjengset',
'ARRRRRR we podcast yet is back with an awesome guest! Jon (https://thesquareplanet.com), a Rustacean at AWS, author of Rust of Rustaceans (https://nostarch.com/rust-rustaceans), an Open source contributor(https://github.com/jonhoo?tab=repositories) and Rust steamer and educational video maker (https://www.youtube.com/c/JonGjengset)',
'en', 'https://soundcloud.com/arewepodcastyet/awpy-08-jon-ferdinand-ronge-gjengset', '2021-09-16', 88*60+46, 'https://production.listennotes.com/podcasts/are-we-podcast-yet/awpy-08-jon-ferdinand-ronge-fXuWZ_gEJ8n-QrfDVIOAZFK.300x300.jpg');


insert into channels
(id, name, hosts, description, url, lang, icon_path, rust_centered, active)
values
(18, 'The Changelog', 'Adam Stacoviak, Jerod Santo', 'Software''s best weekly news brief, deep technical interviews & talk show',
'https://changelog.com/podcast', 'en', 'https://cdn.changelog.com/static/images/podcasts/podcast-medium-126fc11a345517eb5ae5708daee38390.png',
false, true);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(18, 'The Rust Programming Language', 'Steve Klabnik, Yehuda Katz',
'Steve Klabnik and Yehuda Katz joined the show to talk about the Rust Programming Language, a systems programming language from Mozilla Research. We covered memory safety without garbage collection, security, the Rust 1.0 Beta, getting started with Rust, and we even hypothesize about the future of the Rust.',
'en', 'https://changelog.com/podcast/151', '2015-04-11', 82*60+26, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(18, 'Servo and Rust', 'Jack Moffitt',
'Jack Moffitt joined the show to talk about Servo, an experimental web browser layout engine. We talked about what the Servo project aims to achieve, six areas of performance, and what makes Rust a good fit for this effort.',
'en', 'https://changelog.com/podcast/228', '2016-11-18', 77*60+10, 'https://secure.gravatar.com/avatar/405d91268cec8f5e75be74c75cce1c79.jpg?s=150&d=mm');

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(18, 'Rails as a day job, Diesel on the side', 'Sean Griffin',
'Sean Griffin joins the show to talk about doing Rails full-time, his love of Rust. and his project Diesel - a safe, extensible ORM and query builder for Rust. We discuss Sean''s path to working full-time on Rails, what he works on specifically, why Rust, why Diesel, and how much of Diesel''s design and featureset is a product of his experience with ActiveRecord and Rails.',
'en', 'https://changelog.com/podcast/270', '2017-11-04', 76*60+08, 'https://secure.gravatar.com/avatar/0f674817f8c6e149518f0a4b4ad3d560.jpg?s=150&d=mm');

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(18, 'Building a secure Operating System (Redox OS) with Rust', 'Jeremy Soller',
'We talked with Jeremy Soller, the BDFL of Redox OS, a Unix-like Operating System written in Rust, aiming to bring the innovations of Rust to a modern microkernel and full set of applications. In this episode we talk about; OS design principals, Jeremy''s goals for Redox, why is Rust, the Micro-kernel, the Filesystem, how Linux isn''t secure enough, how he''s funding this his development, and a coding style in Rust called Safe Rust.',
'en', 'https://changelog.com/podcast/280', '2018-01-19', 78*60+46, 'https://cdn.changelog.com/uploads/avatars/people/YXNk/avatar_small.jpeg?v=63676699810');

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(18, 'Build tiny multi-platform apps with Tauri and web tech', 'Daniel Thompson',
'This week we''re talking with Daniel Thompson about Tauri and their journey to their recent 1.0 release. Tauri is often compared to Electron - it''s a toolkit that lets you build software for all major desktop operating systems using web technologies. It was built for the security-focused, privacy-respecting, and environmentally-conscious software engineering community. The core libraries are written in Rust and the UI layer can be written using virtually any frontend framework. We get into all the details, why Rust, how the project was formed, their resistance (thus far) to venture capital, their full commitment to the freedom virtues of open source, and all the technical bits you need to know to consider it for your next multi-platform project.',
'en', 'https://changelog.com/podcast/497', '2022-07-15', 97*60+17, 'https://cdn.changelog.com/uploads/avatars/people/dVzja/avatar_small.jpeg?v=63823038416');

