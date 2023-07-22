insert into channels
(id, name, hosts, description, url, lang, icon_path, rust_centered, active)
values
(1, '-', null,'-', '-', 'en', null, false, true);

insert into channels
(id, name, hosts, description, url, lang, icon_path, rust_centered, active)
values
(2, 'New Rustacean', 'Chris Krycho',
'This is a podcast about learning the programming language Rust--from scratch!. Created by Chris Krycho. You can hear more about why I''m doing this in e000: Why Am I Here?',
'https://newrustacean.com/', 'en', 'https://newrustacean.com/podcast.png', true, false);


insert into channels
(id, name, hosts, description, url, lang, icon_path, rust_centered, active)
values
(3, 'Building with Rust', 'Sean Chen', 'Chats with folks who work with and within the Rust programming language',
'https://anchor.fm/building-with-rust', 'en',
'https://s3-us-west-2.amazonaws.com/anchor-generated-image-bank/production/podcast_uploaded_nologo400/12174063/12174063-1611952651089-34ad25ae71f66.jpg',
true, true);

insert into channels
(id, name, hosts, description, url, lang, icon_path, rust_centered, active)
values
(4, 'The Request for Explanation Podcast', null, 'A weekly discussion of Rust RFCs. This is an unofficial podcast created by Rust community members. 20 minutes once a week discussing a Rust RFC',
'https://request-for-explanation.github.io/podcast/', 'en',
null,
true, false);

insert into channels
(id, name, hosts, description, url, lang, icon_path, rust_centered, active)
values
(5, 'Chats with James', 'James Munns', 'Chats with James is a podcast discussing everything that James or his guests are passionate about. Embedded Systems, the Rust Programming Language, and a variety of other technical topics are the most commonly discussed items',
'https://jamesmunns.com/podcast/', 'en',
null,
true, true);

insert into channels
(id, name, hosts, description, url, lang, icon_path, rust_centered, active)
values
(6, 'The Rust Workshop Podcast', 'Tim Abell', 'All things Rust. Discoveries, Learnings, Interviews This show is brought to you by The Rust Workshop https://rustworkshop.co/ for all your Rust coding needs.',
'https://feeds.transistor.fm/the-rust-workshop-podcast', 'en', 'https://images.transistor.fm/file/transistor/images/show/33931/full_1681173292-artwork.jpg',
false, true);

insert into channels
(id, name, hosts, description, url, lang, icon_path, rust_centered, active)
values
(7, 'The InfoQ Podcast', null,
'Software engineers, architects and team leads have found inspiration to drive change and innovation in their team by listening to the weekly InfoQ Podcast. They have received essential information that helped them validate their software development map. We have achieved that by interviewing some of the top CTOs, engineers and technology directors from companies like Uber, Netflix and more.',
'https://www.infoq.com/podcasts/', 'en', 'https://www.infoq.com/styles/static/images/podcast/podcast-infoq.png', false, true);

insert into channels
(id, name, hosts, description, url, lang, icon_path, rust_centered, active)
values
(8, 'SalesforceWay Podcast', 'Xi Xiao',
'In this podcast, I interview Software technologists to pick their brains.',
'https://salesforceway.com/salesforce-podcast/', 'en',
'https://salesforceway.com/wp-content/uploads/2019/09/podcast-768x614.jpg', false, true);

insert into channels
(id, name, hosts, description, url, lang, icon_path, rust_centered, active)
values
(9, 'Software Engineering Radio', null,
'Software Engineering Radio is a podcast targeted at the professional software developer. The goal is to be a lasting educational resource, not a newscast. Now a weekly show, we talk to experts from throughout the software engineering world about the full range of topics that matter to professional developers',
'https://www.se-radio.net/', 'en', 'https://www.se-radio.net/wp-content/uploads/2017/01/seradio-100.jpg', false, true);


insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(1, 'Code[ish] episode 34: An introduction to Rust', 'Carol Nichols,Jake Goulding',
'Carol Nichols and Jake Goulding are Rust instructors and enthusiasts, and they join Chris Castle to talk about Rust''s underlying strengths as an ideal blend of simpler languages, like Ruby, with more memory conscious ones, like C',
'en', 'https://www.heroku.com/podcasts/codeish/34-an-introduction-to-rust', '2019-09-10', 43*60+58,
'https://heroku-www-files.s3.amazonaws.com/podcasts/uploads/610d8a9b-b983-4670-ad93-88fdf7a929f8/codeish-cover-art.png');

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(8, 'SalesforceWay episode 92: Learning Rust Language | Scott Lee', 'Scott Lee',
'Scott Lee, who joins to talk about Rust Language, is CEO of Elega Corporation, Senior Salesforce Developer, Game Builder, and PluralSight Author',
'en', 'https://salesforceway.com/podcast/rust-lang/', '2021-12-02', 22*60+19, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(7, 'The InfoQ Podcast: Bryan Cantrill on Rust and Why He Feels It''s The Biggest Change in Systems Development in His Career',
'Bryan Cantrill', 'Bryan Cantrill is the CTO of Joyent and well known for the development of DTrace at Sun Microsystems. Today on the podcast, Bryan discusses with Wes Reisz a bit about the origins of DTrace and then spends the rest of the time discussing why he feels Rust is the "biggest development in systems development in his career." The podcast wraps with a bit about why Bryan feels we should be rewriting parts of the operating system in Rust',
'en', 'https://www.infoq.com/podcasts/rust-systems-programming/', '2019-04-12', 38*60+41, null);

-- insert into episodes
-- (channel_id, title, guest, description, lang, url, date_published, duration_seconds, icon_path)
-- values
-- (1, 'KubeOps Tech Talk EP16: คุยเรื่อง Rust สไตล์ Cloud Native Rustacean', null,
-- 'KubeOps Tech Talk EP16 ครั้งนี้คุยกันเรื่อง Rust programming language ตามสไตล์ Cloud Native Rustacean พบกับคุณเนตรชวินทร์ สุทธิสันธิ์ ผู้พัฒนาที่ใช้ภาษา Rust',
-- 'th', 'https://podcasts.kubeops.guru/e/kubeops-tech-talk-ep16-%e0%b8%84%e0%b8%b8%e0%b8%a2%e0%b9%80%e0%b8%a3%e0%b8%b7%e0%b9%88%e0%b8%ad%e0%b8%87-rust-%e0%b8%aa/',
-- '2021-05-21', 3600+29*60+37, 'https://pbcdn1.podbean.com/imglogo/image-logo/9845650/kubeops-logo.png');


insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(9, 'SE-Radio Episode 279: Florian Gilcher on Rust', 'Florian Gilcher',
'Eberhard talks with Florian Gilcher about the programming language Rust. Rust originates from Mozilla research. Its focus is on system programming and it is often used to replace C or C++. Topics include the concepts behind Rust; concurrent and safe programming; advanced and unique features like ownership and borrowing; the rust type system (which supports other features like traits, generics and macros). The show finishes with: the evolution of Rust based, features of libraries, and how the community works',
'en', 'https://www.se-radio.net/2017/01/se-radio-episode-279-florian-gilcher-on-rust/',
'2017-01-10', 3600+10*60+40, 'https://www.se-radio.net/wp-content/uploads/2017/01/fgilcher-100.jpg');

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(9, 'SE-Radio Episode 490: Tim McNamara on Rust 2021 Edition', 'Tim McNamara',
'Tim McNamara, author of Rust in Action, an introduction Rust for programmers who have never used a systems programming language, discusses the top three benefits of Rust and why they make it a performant, reliable and productive programming language. Host Gavin Henry spoke with McNamara about its rich type system, ownership models, memory safety, thread safety, concurrency, UnSafe Rust, build tools, compilers, build times, running Rust with no OS, Rust 2021 edition, Rusts'' future, package managers, managing memory, memory leaks, runtimes, garbage collection, closures, version migrations, static analysis, what''s changed since our 2017 show on Rust, when to use Rust and how Rust compares to C, C++ and Go',
'en', 'https://www.se-radio.net/2021/12/episode-490-tim-mcnamara-on-rust-2021-edition/',
'2021-12-15', 50*60+52, 'https://www.se-radio.net/wp-content/uploads/2021/12/tim-mcnamara.jpg');

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(2, 'e000: Hello, world!', null,
'Today''s show is pretty meta. You can skip it if you just want to start with something more technical, but I thought listeners might want to know a little about the origins of the show and my own background, so that''s what you get today',
'en', 'https://newrustacean.com/show_notes/e000/index.html', '2015-09-24', 17*60+11,
'https://newrustacean.com/podcast.png');

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(1, 'Humans of Open Source: Niko Matsakis on how Rust has Matured', 'Niko Matsakis',
'Niko Matsakis works at Mozilla as a Principle Research Engineer and has been working on Rust since 2011. In that time he''s seen the language and the community go through many changes.
We discuss topics such as what features have come and gone in the language, how to continue fostering the culture of the community, and struggling with work-life balance when you just really love your job',
'en', 'https://anchor.fm/humans-of-open-source/episodes/Niko-Matsakis-on-how-Rust-has-Matured-emh819',
'2020-11-07', 46*60+10, 'https://s3-us-west-2.amazonaws.com/anchor-generated-image-bank/staging/podcast_uploaded_nologo400/10693218/57d56c07c6002d65.jpeg');

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(1, 'Understanding Vulnerability Analysis in the Rust Programming Language', 'David Svoboda, Garret Wassermann',
'David Svoboda and Garret Wassermann explore tools for understanding vulnerabilities in Rust whether the original source code is available or not.',
'en', 'https://resources.sei.cmu.edu/library/asset-view.cfm?assetid=980217', '2023-06-08', 30*60+44, 'https://deow9bq0xqvbj.cloudfront.net/dir-logo/428457/428457_300x300.png');

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(6, 'What even is this Rust thing -> Naïve Excitement', null,'Tim and Jim, two coders with 20 years each coding, one more c# one more c++ but both with broad experience have discovered Rust, and capture their learnings, musings and enthusiasms for your entertainment.',
'en', 'https://share.transistor.fm/s/8350a4a2', '2022-09-08', 102*60+51, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(6, 'Jim and Tim go deeper into Rust and compare with other languages', null,'An update on what we''ve learned from spending more time with Rust with real coding. The perspective from C#, C++, Ruby etc.',
'en', 'https://share.transistor.fm/s/f22c961a', '2023-05-02', 161*60+26, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(6, 'Rust meetups and DevOps pipelines with end-to-end testing for confident fast shipping', null, 'Quick update about what''s going on in the Rust Workshop, and new Rust meetup in Reading, UK',
'en', 'https://share.transistor.fm/s/f4dbb201', '2023-06-28', 15*60+18, null);


insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(1, 'Code Savvy Presents: The Rust Programming Language', 'Chris Eckhardt', 'This month''s episode features Chris Eckhardt of Rust MN, a local group dedicated to learning, sharing, and celebrating the Rust Programming Language.',
'en', 'https://www.codesavvy.org/podcast/episode/e02da11c/code-savvy-presents-the-rust-programming-language', '2023-01-24', 19*60+26,
'https://d3t3ozftmdmh3i.cloudfront.net/production/podcast_uploaded_nologo/20933747/20933747-1641863468645-5b8d9cfd1367e.jpg');

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(9, 'SE Radio 562: Bastian Gruber on Rust Web Development', 'Bastian Gruber',
'Bastian Gruber, author of the book Rust Web Development, speaks with host Philip Winston about creating server-based web applications with Rust. They explore Rust language features, tooling, and web frameworks such as Warp and Tokio. From there, they examine the steps to build a simple web server and a RESTful API, as well as modules, logging and tracing, and other aspects of web development with Rust.',
'en', 'https://www.se-radio.net/2023/05/se-radio-562-bastian-gruber-on-rust-web-development/', '2023-05-03', 68*60+23,
'https://www.se-radio.net/wp-content/uploads/2023/05/bastian-gruber-100.png');

