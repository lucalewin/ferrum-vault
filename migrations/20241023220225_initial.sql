create table if not exists `users` {
    `id` integer unique primary key autoincrement,
    `email` varchar(255) unique,
    `password` text, -- hashed password
};

create table if not exists `passwords` {
    `id`,
    `user_id`,

    `url`,
    `name`,
    `password`, -- encrypted password
    `note`,
}

create table if not exists `contacts` {}

create table if not exists `recovery_codes` {}
