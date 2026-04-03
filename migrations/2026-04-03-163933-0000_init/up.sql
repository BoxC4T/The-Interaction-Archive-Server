-- Your SQL goes here
CREATE TABLE `connections`(
	`id` TEXT NOT NULL PRIMARY KEY,
	`status` TEXT NOT NULL,
	`removal_date` DATE NOT NULL
);

CREATE TABLE `details`(
	`id` TEXT NOT NULL,
	`connection_id` TEXT NOT NULL,
	PRIMARY KEY(`id`, `connection_id`)
);

