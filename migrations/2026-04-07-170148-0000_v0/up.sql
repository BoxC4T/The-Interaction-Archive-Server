-- Your SQL goes here
CREATE TABLE `connections`(
	`id` TEXT NOT NULL PRIMARY KEY,
	`status` TEXT NOT NULL,
	`removal_datetime` TEXT
);

CREATE TABLE `details`(
	`id` TEXT NOT NULL,
	`connection_id` TEXT NOT NULL,
	`data` JSON NOT NULL,
	PRIMARY KEY(`id`, `connection_id`)
);

CREATE TABLE `interactions`(
	`id` TEXT NOT NULL PRIMARY KEY,
	`data` JSON NOT NULL
);

