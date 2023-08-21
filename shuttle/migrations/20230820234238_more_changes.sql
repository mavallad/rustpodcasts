update channels set icon_path = 'https://delivery.jamescdn.com/logo0.png' where id = 5;

insert into channels
(id, name, hosts, description, url, lang, icon_path, rust_centered, active)
values
(13, 'DevTalles', 'Fernando Herrera',
'Podcast de Fernando Herrera, un full-stack developer y profesor en línea con cientos de miles de alumnos, cuyo objetivo es que pasemos un rato agradable, vertiendo mis opiniones profesionales y experiencias que puede que te sirvan mucho en tu día a día como profesional en tecnologías de información y así ayudarte a formar mejores opiniones sobre diferentes temas.',
'https://www.ivoox.com/podcast-devtalles_sq_f11186399_1.html', 'es',
'https://img-static.ivoox.com/index.php?w=141&h=141&url=https://static-2.ivoox.com/audios/9/7/f/a/97fa6b7442695675b596f99cc962def8_XXL.jpg', false, true);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(13, '101- Rust', null,
'Rust es un lenguaje de programación compilado, de propósito general y multiparadigma que está siendo desarrollado por Fundación Rust. Es un lenguaje de programación multiparadigmático que soporta programación funcional pura, por procedimientos, imperativa y orientada a objetos.',
'es', 'https://www.ivoox.com/101-rust-audios-mp3_rf_102577785_1.html', '2023-02-05', 22*60+56, null);


insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(5, '011 - Eliza Weisman', 'Eliza Weisman',
'James chats with Eliza about systems, systems of systems, operating systems, java, java cards, what posix did wrong, and a ton of other rust adjacent things.',
'en', 'https://jamesmunns.com/podcast/011-eliza/', '2022-04-11', 119*60+43, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(5, '010 - Dion Doktor', 'Dion Doktor',
'James chats with Dion Doktor about device drivers, bootloaders, and embedded Rust.',
'en', 'https://jamesmunns.com/podcast/010-dion/', '2021-03-02', 35*60+50, null);

insert into episodehttps://jamesmunns.com/podcast/008-yosh/
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(5, '009 - Sylvan Morris', 'Sylvan Morris',
'James chats with Sylvan Morris about introducing Rust into a company, working at an embedded startup, over the air bootloaders, and managing risk.',
'en', 'https://jamesmunns.com/podcast/009-sylvan/', '2021-02-23', 27*60+44, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(5, '008 - Yoshua Wuyts', 'Yoshua Wuyts',
'James chats with Yoshua Wuyts to discuss Control Theory in traditional software domains, the possibilities of RISC-V, and open source hardware.',
'en', 'https://jamesmunns.com/podcast/008-yosh/', '2021-02-16', 53*60+18, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(5, '007 - Michael Nitschinger', 'Michael Nitschinger',
'James chats with Michael Nitschinger of Couchbase to discuss Coffee Machines, PID control loops, Bluetooth Low Energy, and Databases.',
'en', 'https://jamesmunns.com/podcast/007-michael/', '2021-02-09', 53*60+8, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(5, '006 - Bryan Cantrill', 'Bryan Cantrill',
'James chats with Bryan Cantrill of Oxide Computer to discuss the open source hardware explosion, Oxide''s experience with Embedded Rust, the importance of a culture of sharing knowledge, and the joy of fixing hard-to-diagnose systems problems.',
'en', 'https://jamesmunns.com/podcast/006-bryan/', '2021-02-02', 84*60+22, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(5, '005 - Clifford Heath', 'Clifford Heath',
'James chats with Clifford Heath to discuss the differences between procedural and structural code, using natural language processing to describe software, code generation, and programming education approaches.',
'en', 'https://jamesmunns.com/podcast/005-clifford/', '2021-01-26', 64*60+45, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(5, '004 - François Baldassari', 'François Baldassari',
'James chats with François Baldassari to discuss embedded systems, engineering practices (and where the industry is lacking), and the social side of improving the skills of embedded systems teams.',
'en', 'https://jamesmunns.com/podcast/004-francois/', '2021-01-19', 48*60+36, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(5, '003 - Michael Gattozzi', 'Michael Gattozzi',
'James chats with Michael Gattozzi to discuss developer tooling, message serialization, and undefined behavior in Rust.',
'en', 'https://jamesmunns.com/podcast/003-michael/', '2021-01-12', 71*60+28, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(5, '002 - Christopher Hunt', 'Christopher Hunt',
'James chats with Christopher Hunt to discuss programming languages, the cost of software abstraction, Bluetooth, and state machines.',
'en', 'https://jamesmunns.com/podcast/002-chris/', '2021-01-05', 71*60+47, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(5, '001 - Lachezar Lechev', 'Lachezar Lechev',
'James chats with Lachezar Lechev to discuss Rust in Avionics, drone simulation technologies, and burnout in software development.',
'en', 'https://jamesmunns.com/podcast/001-lachezar/', '2020-12-29', 45*60+35, null);

insert into episodes
(channel_id, title, guests, description, lang, url, date_published, duration_seconds, icon_path)
values
(5, '000 - Foreward', null,
'James introduces the Chats with James Podcast.',
'en', 'https://jamesmunns.com/podcast/000-foreward/', '2020-12-28', 1*60+13, null);
