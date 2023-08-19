insert into channels
(id, name, hosts, description, url, lang, icon_path, rust_centered, active)
values
(10, 'Rustacean Station', 'Allen Wyma',
'A community project for creating podcast content for the Rust programming language. Come journey with us into the weird, wonderful, and wily world of Rust',
'https://rustacean-station.org/', 'en', 'https://rustacean-station.org/images/artwork-2x.jpg', true, true);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(10, 'Adopting Rust: present and future of the Rust web ecosystem, with Luca Palmieri', 'Luca Palmieri',
'Marco Otto-Witte discusses with Luca Palmieri the present and future of the Rust web ecosystem',
'en', 'https://rustacean-station.org/episode/luca-palmieri-web-ecosystem/', '2023-08-12', 47*60+2, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(10, 'Scanner.dev with Cliff Crosland', 'Cliff Crosland',
'Allen Wyma talks with Cliff Crosland about his work on Scanner.dev that is powered by Rust',
'en', 'https://rustacean-station.org/episode/cliff-crosland/', '2023-08-11', 63*60+25, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(10, 'Bootstrapping Rust with Albert Larsan', 'Albert Larsan',
'Allen Wyma talks with Albert Larsan about his work on bootstrapping the Rust compiler',
'en', 'https://rustacean-station.org/episode/albert-larsan/', '2023-07-14', 33*60+4, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(10, 'Shuttle with Ivan Cernja', 'Ivan Cernja',
'Allen Wyma and Zeeshan Ali Khan talk with Ivan Cernja, DevRel at Shuttle, a platform for deploying Rust apps',
'en', 'https://rustacean-station.org/episode/ivan-cernja/', '2023-06-30', 38*60+49, null);


insert into channels
(id, name, hosts, description, url, lang, icon_path, rust_centered, active)
values
(11, 'ADSP: The Podcast', 'Bryce Adelstein Lelbach, Conor Hoekstra',
'ADSP: The Podcast is a programming podcast hosted by two NVIDIA software engineers that focuses on the C++ and Rust programming languages. Topics discussed include algorithms, data structures, programming languages, latest news in tech and more. The podcast was initially inspired by Magic Read Along.',
'https://adspthepodcast.com/', 'en', 'https://is1-ssl.mzstatic.com/image/thumb/Podcasts115/v4/ed/18/9a/ed189ac7-7a0d-2257-213e-67ac1ca64456/mza_3592480773119772773.jpg/313x0w.webp', false, true);



insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(11, 'Episode 132: RustConf Drama', null,
'In this episode, Conor and Bryce chat about the Rust Conf drama and other upcoming conferences',
'en', 'https://adspthepodcast.com/2023/06/02/Episode-132.html', '2023-06-02', 31*60+11, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(11, 'Episode 106: Jane Losare-Lusby on Rust!', 'Jane Losare-Lusby',
'In this episode, Conor and Bryce talk to Jane Losare-Lusby about the Rust Programming Language',
'en', 'https://adspthepodcast.com/2022/12/02/Episode-106.html', '2022-12-02', 31*60+43, null);


insert into channels
(id, name, hosts, description, url, lang, icon_path, rust_centered, active)
values
(12, 'AWS Developers Podcast', 'Dave Isbitski, Emily Freeman',
'Dave Isbitski and Emily Freeman have a weekly chat with the people behind Amazon Web Services (AWS) and the developers who are building on it.',
'https://aws.amazon.com/developer/podcast/', 'en',
'https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcSL07Sb8-qshPsooECi8pDnZXwrGpS7XEemevRTH8WT84AWwlrn', false, true);


insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(12, 'Episode 092 - Moving to Rust in the Age of AI with Noah Gift', 'Noah Gift',
'Join Brooke and Dave as they dive into an enthralling conversation with Noah Gift, Duke EIR for Data Science and AI, exploring the powerful synergy between Rust and the dynamic world of AI/ML. Journey through Noah''s illustrious career, including his remarkable stint working on blockbuster movies like Avatar, and his transition into the fascinating sphere of Machine Learning.

This episode uncovers the strengths of the Rust programming language, the evolving landscape of MLOps, the ways Rust enhances developers'' work, its integration with Lambda, the substantial cost savings Rust can usher in, and the thrilling developments on the horizon for developers.',
'en', 'https://podcasts.google.com/feed/aHR0cHM6Ly9mZWVkcy5zb3VuZGNsb3VkLmNvbS91c2Vycy9zb3VuZGNsb3VkOnVzZXJzOjk5NDM2MzU0OS9zb3VuZHMucnNz/episode/dGFnOnNvdW5kY2xvdWQsMjAxMDp0cmFja3MvMTU5MDkyMDQ4Mw?sa=X&ved=0CAUQkfYCahcKEwjIpJySueeAAxUAAAAAHQAAAAAQEw', '2023-08-12', 36*60+34, null);
