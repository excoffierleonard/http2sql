-- @block Wipe Everything
DROP TABLE IF EXISTS `tags`;
DROP TABLE IF EXISTS `users`;
-- @block User Table
CREATE TABLE IF NOT EXISTS `users` (
    `id` INT NOT NULL AUTO_INCREMENT,
    `email` VARCHAR(255) NOT NULL,
    `password` VARCHAR(255) NOT NULL,
    `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`)
);
-- @block Creating a tags Table
CREATE TABLE IF NOT EXISTS `tags` (
    `id` INT NOT NULL AUTO_INCREMENT,
    `user_id` INT NOT NULL,
    `name` VARCHAR(255) NOT NULL,
    `created_at` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`id`),
    FOREIGN KEY (`user_id`) REFERENCES `users`(`id`)
);
-- @block Creating a new mock user
INSERT INTO `users` (`email`, `password`)
VALUES ('john.doe@gmail.com', 'john48');
-- @block Creating a new mock tag
INSERT INTO `tags` (`user_id`, `name`)
VALUES (1, 'file1');
-- @block Listing the tables
SHOW TABLES;