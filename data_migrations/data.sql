insert into channels
(name, description, url, lang, icon_path, active)
values
('-', '-', '-', 'en', '-', true);

insert into channels
(name, description, url, lang, icon_path, active)
values
('New Rustacean',
'This is a podcast about learning the programming language Rust--from scratch!. Created by Chris Krycho. You can hear more about why I''m doing this in e000: Why Am I Here?',
'https://newrustacean.com/', 'en', '-', false);


insert into channels
(name, description, url, lang, icon_path, active)
values
('Building with Rust', 'Chats with folks who work with and within the Rust programming language',
'https://anchor.fm/building-with-rust', 'en',
'https://s3-us-west-2.amazonaws.com/anchor-generated-image-bank/production/podcast_uploaded_nologo400/12174063/12174063-1611952651089-34ad25ae71f66.jpg',
true);

insert into episodes
(channel_id, title, guest, description, lang, url, date_published, duration_seconds, icon_path)
values
(1, 'Code[ish] episode 34: An introduction to Rust', 'Carol Nichols, Jake GGoulding',
'Carol Nichols and Jake Goulding are Rust instructors and enthusiasts, and they join Chris Castle to talk about Rust''s underlying strengths as an ideal blend of simpler languages, like Ruby, with more memory conscious ones, like C',
'en', 'https://www.heroku.com/podcasts/codeish/34-an-introduction-to-rust', '2019-09-10', 43*60+58,
'https://heroku-www-files.s3.amazonaws.com/podcasts/uploads/610d8a9b-b983-4670-ad93-88fdf7a929f8/codeish-cover-art.png');

insert into episodes
(channel_id, title, guest, description, lang, url, date_published, duration_seconds, icon_path)
values
(1, 'SalesforceWay episode 92: Learning Rust Language | Scott Lee', 'Scott Lee',
'Scott Lee, who joins to talk about Rust Language, is CEO of Elega Corporation, Senior Salesforce Developer, Game Builder, and PluralSight Author',
'en', 'https://salesforceway.com/podcast/rust-lang/', '2021-12-02', 22*60+19, null);

insert into episodes
(channel_id, title, guest, description, lang, url, date_published, duration_seconds, icon_path)
values
(1, 'The InfoQ Podcast: Bryan Cantrill on Rust and Why He Feels It''s The Biggest Change in Systems Development in His Career',
'Bryan Cantrill', 'Bryan Cantrill is the CTO of Joyent and well known for the development of DTrace at Sun Microsystems. Today on the podcast, Bryan discusses with Wes Reisz a bit about the origins of DTrace and then spends the rest of the time discussing why he feels Rust is the "biggest development in systems development in his career." The podcast wraps with a bit about why Bryan feels we should be rewriting parts of the operating system in Rust',
'en', 'https://www.infoq.com/podcasts/rust-systems-programming/', '2019-04-12', 38*60+41, null);

insert into episodes
(channel_id, title, guest, description, lang, url, date_published, duration_seconds, icon_path)
values
(1, 'KubeOps Tech Talk EP16: คุยเรื่อง Rust สไตล์ Cloud Native Rustacean', null,
'KubeOps Tech Talk EP16 ครั้งนี้คุยกันเรื่อง Rust programming language ตามสไตล์ Cloud Native Rustacean พบกับคุณเนตรชวินทร์ สุทธิสันธิ์ ผู้พัฒนาที่ใช้ภาษา Rust',
'th', 'https://podcasts.kubeops.guru/e/kubeops-tech-talk-ep16-%e0%b8%84%e0%b8%b8%e0%b8%a2%e0%b9%80%e0%b8%a3%e0%b8%b7%e0%b9%88%e0%b8%ad%e0%b8%87-rust-%e0%b8%aa/',
'2021-05-21', 3600+29*60+37, 'https://pbcdn1.podbean.com/imglogo/image-logo/9845650/kubeops-logo.png');

insert into episodes
(channel_id, title, guest, description, lang, url, date_published, duration_seconds, icon_path)
values
(2, 'e000: Hello, world!', null,
'Today''s show is pretty meta. You can skip it if you just want to start with something more technical, but I thought listeners might want to know a little about the origins of the show and my own background, so that''s what you get today',
'en', 'https://newrustacean.com/show_notes/e000/index.html', '2015-09-24', 17*60+11,
'https://newrustacean.com/podcast.png');
